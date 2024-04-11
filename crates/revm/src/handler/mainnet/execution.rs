use crate::{
    db::Database,
    interpreter::{
        return_ok, return_revert, CallInputs, CreateInputs, CreateOutcome, Energy,
        InstructionResult, SharedMemory,
    },
    primitives::{EVMError, Env, Spec, SpecId},
    CallFrame, Context, CreateFrame, Frame, FrameOrResult, FrameResult,
};
use revm_interpreter::{CallOutcome, InterpreterResult};
use std::boxed::Box;
/// Helper function called inside [`last_frame_return`]
#[inline]
pub fn frame_return_with_refund_flag<SPEC: Spec>(
    env: &Env,
    frame_result: &mut FrameResult,
    refund_enabled: bool,
) {
    let instruction_result = frame_result.interpreter_result().result;
    let energy = frame_result.energy_mut();
    let remaining = energy.remaining();
    let refunded = energy.refunded();

    // Spend the energy limit. Energy is reimbursed when the tx returns successfully.
    *energy = Energy::new(env.tx.energy_limit);
    energy.record_cost(env.tx.energy_limit);

    match instruction_result {
        return_ok!() => {
            energy.erase_cost(remaining);
            energy.record_refund(refunded);
        }
        return_revert!() => {
            energy.erase_cost(remaining);
        }
        _ => {}
    }

    // Calculate energy refund for transaction.
    // If config is set to disable energy refund, it will return 0.
    // If spec is set to london, it will decrease the maximum refund amount to 5th part of
    // energy spend. (Before london it was 2th part of energy spend)
    if refund_enabled {
        // EIP-3529: Reduction in refunds
        energy.set_final_refund(SPEC::SPEC_ID.is_enabled_in(SpecId::LONDON));
    }
}

/// Handle output of the transaction
#[inline]
pub fn last_frame_return<SPEC: Spec, EXT, DB: Database>(
    context: &mut Context<EXT, DB>,
    frame_result: &mut FrameResult,
) -> Result<(), EVMError<DB::Error>> {
    frame_return_with_refund_flag::<SPEC>(&context.evm.env, frame_result, true);
    Ok(())
}

/// Handle frame sub call.
#[inline]
pub fn call<SPEC: Spec, EXT, DB: Database>(
    context: &mut Context<EXT, DB>,
    inputs: Box<CallInputs>,
) -> Result<FrameOrResult, EVMError<DB::Error>> {
    context.evm.make_call_frame(&inputs)
}

#[inline]
pub fn call_return<EXT, DB: Database>(
    context: &mut Context<EXT, DB>,
    frame: Box<CallFrame>,
    interpreter_result: InterpreterResult,
) -> Result<CallOutcome, EVMError<DB::Error>> {
    context
        .evm
        .call_return(&interpreter_result, frame.frame_data.checkpoint);
    Ok(CallOutcome::new(
        interpreter_result,
        frame.return_memory_range,
    ))
}

#[inline]
pub fn insert_call_outcome<EXT, DB: Database>(
    context: &mut Context<EXT, DB>,
    frame: &mut Frame,
    shared_memory: &mut SharedMemory,
    outcome: CallOutcome,
) -> Result<(), EVMError<DB::Error>> {
    core::mem::replace(&mut context.evm.error, Ok(()))?;
    frame
        .frame_data_mut()
        .interpreter
        .insert_call_outcome(shared_memory, outcome);
    Ok(())
}

/// Handle frame sub create.
#[inline]
pub fn create<SPEC: Spec, EXT, DB: Database>(
    context: &mut Context<EXT, DB>,
    inputs: Box<CreateInputs>,
) -> Result<FrameOrResult, EVMError<DB::Error>> {
    context.evm.make_create_frame(SPEC::SPEC_ID, &inputs)
}

#[inline]
pub fn create_return<SPEC: Spec, EXT, DB: Database>(
    context: &mut Context<EXT, DB>,
    frame: Box<CreateFrame>,
    mut interpreter_result: InterpreterResult,
) -> Result<CreateOutcome, EVMError<DB::Error>> {
    context.evm.create_return::<SPEC>(
        &mut interpreter_result,
        frame.created_address,
        frame.frame_data.checkpoint,
    );
    Ok(CreateOutcome::new(
        interpreter_result,
        Some(frame.created_address),
    ))
}

#[inline]
pub fn insert_create_outcome<EXT, DB: Database>(
    context: &mut Context<EXT, DB>,
    frame: &mut Frame,
    outcome: CreateOutcome,
) -> Result<(), EVMError<DB::Error>> {
    core::mem::replace(&mut context.evm.error, Ok(()))?;
    frame
        .frame_data_mut()
        .interpreter
        .insert_create_outcome(outcome);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use revm_interpreter::primitives::CancunSpec;
    use revm_precompile::Bytes;

    /// Creates frame result.
    fn call_last_frame_return(instruction_result: InstructionResult, energy: Energy) -> Energy {
        let mut env = Env::default();
        env.tx.energy_limit = 100;

        let mut first_frame = FrameResult::Call(CallOutcome::new(
            InterpreterResult {
                result: instruction_result,
                output: Bytes::new(),
                energy,
            },
            0..0,
        ));
        frame_return_with_refund_flag::<CancunSpec>(&env, &mut first_frame, true);
        *first_frame.energy()
    }

    #[test]
    fn test_consume_energy() {
        let energy = call_last_frame_return(InstructionResult::Stop, Energy::new(90));
        assert_eq!(energy.remaining(), 90);
        assert_eq!(energy.spent(), 10);
        assert_eq!(energy.refunded(), 0);
    }

    // TODO
    #[test]
    fn test_consume_energy_with_refund() {
        let mut return_energy = Energy::new(90);
        return_energy.record_refund(30);

        let energy = call_last_frame_return(InstructionResult::Stop, return_energy);
        assert_eq!(energy.remaining(), 90);
        assert_eq!(energy.spent(), 10);
        assert_eq!(energy.refunded(), 2);

        let energy = call_last_frame_return(InstructionResult::Revert, return_energy);
        assert_eq!(energy.remaining(), 90);
        assert_eq!(energy.spent(), 10);
        assert_eq!(energy.refunded(), 0);
    }

    #[test]
    fn test_revert_energy() {
        let energy = call_last_frame_return(InstructionResult::Revert, Energy::new(90));
        assert_eq!(energy.remaining(), 90);
        assert_eq!(energy.spent(), 10);
        assert_eq!(energy.refunded(), 0);
    }
}
