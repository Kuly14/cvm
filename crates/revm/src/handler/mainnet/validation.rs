use revm_interpreter::energy;

use crate::{
    primitives::{db::Database, EVMError, Env, InvalidTransaction, Spec},
    Context,
};

/// Validate environment for the mainnet.
pub fn validate_env<SPEC: Spec, DB: Database>(env: &Env) -> Result<(), EVMError<DB::Error>> {
    // Important: validate block before tx.
    env.validate_block_env::<SPEC>()?;
    env.validate_tx::<SPEC>()?;
    Ok(())
}

/// Validates transaction against the state.
pub fn validate_tx_against_state<SPEC: Spec, EXT, DB: Database>(
    context: &mut Context<EXT, DB>,
) -> Result<(), EVMError<DB::Error>> {
    // load acc
    let tx_caller = context.evm.env.tx.caller;
    let (caller_account, _) = context
        .evm
        .inner
        .journaled_state
        .load_account(tx_caller, &mut context.evm.inner.db)?;

    context
        .evm
        .inner
        .env
        .validate_tx_against_state::<SPEC>(caller_account)
        .map_err(EVMError::Transaction)?;

    Ok(())
}

/// Validate initial transaction energy.
pub fn validate_initial_tx_energy<SPEC: Spec, DB: Database>(
    env: &Env,
) -> Result<u64, EVMError<DB::Error>> {
    let input = &env.tx.data;
    let is_create = env.tx.transact_to.is_create();
    let access_list = &env.tx.access_list;

    let initial_energy_spend =
        energy::validate_initial_tx_energy(SPEC::SPEC_ID, input, is_create, access_list);

    // Additional check to see if limit is big enough to cover initial energy.
    if initial_energy_spend > env.tx.energy_limit {
        return Err(InvalidTransaction::CallEnergyCostMoreThanEnergyLimit.into());
    }
    Ok(initial_energy_spend)
}
