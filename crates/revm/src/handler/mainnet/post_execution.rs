use crate::{
    interpreter::{Energy, SuccessOrHalt},
    primitives::{
        db::Database, EVMError, ExecutionResult, ResultAndState, Spec, SpecId::LONDON, U256,
    },
    Context, FrameResult,
};

/// Mainnet end handle does not change the output.
#[inline]
pub fn end<EXT, DB: Database>(
    _context: &mut Context<EXT, DB>,
    evm_output: Result<ResultAndState, EVMError<DB::Error>>,
) -> Result<ResultAndState, EVMError<DB::Error>> {
    evm_output
}

/// Reward beneficiary with energy fee.
#[inline]
pub fn reward_beneficiary<SPEC: Spec, EXT, DB: Database>(
    context: &mut Context<EXT, DB>,
    energy: &Energy,
) -> Result<(), EVMError<DB::Error>> {
    let beneficiary = context.evm.env.block.coinbase;
    let effective_energy_price = context.evm.env.effective_energy_price();

    // transfer fee to coinbase/beneficiary.
    // EIP-1559 discard basefee for coinbase transfer. Basefee amount of energy is discarded.
    let coinbase_energy_price = if SPEC::enabled(LONDON) {
        effective_energy_price.saturating_sub(context.evm.env.block.basefee)
    } else {
        effective_energy_price
    };

    let (coinbase_account, _) = context
        .evm
        .inner
        .journaled_state
        .load_account(beneficiary, &mut context.evm.inner.db)?;

    coinbase_account.mark_touch();
    coinbase_account.info.balance = coinbase_account.info.balance.saturating_add(
        coinbase_energy_price * U256::from(energy.spent() - energy.refunded() as u64),
    );

    Ok(())
}

#[inline]
pub fn reimburse_caller<SPEC: Spec, EXT, DB: Database>(
    context: &mut Context<EXT, DB>,
    energy: &Energy,
) -> Result<(), EVMError<DB::Error>> {
    let caller = context.evm.env.tx.caller;
    let effective_energy_price = context.evm.env.effective_energy_price();

    // return balance of not spend energy.
    let (caller_account, _) = context
        .evm
        .inner
        .journaled_state
        .load_account(caller, &mut context.evm.inner.db)?;

    caller_account.info.balance = caller_account.info.balance.saturating_add(
        effective_energy_price * U256::from(energy.remaining() + energy.refunded() as u64),
    );

    Ok(())
}

/// Main return handle, returns the output of the transaction.
#[inline]
pub fn output<EXT, DB: Database>(
    context: &mut Context<EXT, DB>,
    result: FrameResult,
) -> Result<ResultAndState, EVMError<DB::Error>> {
    core::mem::replace(&mut context.evm.error, Ok(()))?;
    // used energy with refund calculated.
    let energy_refunded = result.energy().refunded() as u64;
    let final_energy_used = result.energy().spent() - energy_refunded;
    let output = result.output();
    let instruction_result = result.into_interpreter_result();

    // reset journal and return present state.
    let (state, logs) = context.evm.journaled_state.finalize();

    let result = match instruction_result.result.into() {
        SuccessOrHalt::Success(reason) => ExecutionResult::Success {
            reason,
            energy_used: final_energy_used,
            energy_refunded,
            logs,
            output,
        },
        SuccessOrHalt::Revert => ExecutionResult::Revert {
            energy_used: final_energy_used,
            output: output.into_data(),
        },
        SuccessOrHalt::Halt(reason) => ExecutionResult::Halt {
            reason,
            energy_used: final_energy_used,
        },
        // Only two internal return flags.
        flag @ (SuccessOrHalt::FatalExternalError
        | SuccessOrHalt::InternalContinue
        | SuccessOrHalt::InternalCallOrCreate) => {
            panic!(
                "Encountered unexpected internal return flag: {:?} with instruction result: {:?}",
                flag, instruction_result
            )
        }
    };

    Ok(ResultAndState { result, state })
}
