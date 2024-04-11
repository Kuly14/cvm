use crate::{
    handler::mainnet,
    primitives::{db::Database, EVMError, Env, Spec},
    Context,
};
use std::sync::Arc;

/// Handle that validates env.
pub type ValidateEnvHandle<'a, DB> =
    Arc<dyn Fn(&Env) -> Result<(), EVMError<<DB as Database>::Error>> + 'a>;

/// Handle that validates transaction environment against the state.
/// Second parametar is initial energy.
pub type ValidateTxEnvAgainstState<'a, EXT, DB> =
    Arc<dyn Fn(&mut Context<EXT, DB>) -> Result<(), EVMError<<DB as Database>::Error>> + 'a>;

/// Initial energy calculation handle
pub type ValidateInitialTxEnergyHandle<'a, DB> =
    Arc<dyn Fn(&Env) -> Result<u64, EVMError<<DB as Database>::Error>> + 'a>;

/// Handles related to validation.
pub struct ValidationHandler<'a, EXT, DB: Database> {
    /// Validate and calculate initial transaction energy.
    pub initial_tx_energy: ValidateInitialTxEnergyHandle<'a, DB>,
    /// Validate transactions against state data.
    pub tx_against_state: ValidateTxEnvAgainstState<'a, EXT, DB>,
    /// Validate Env.
    pub env: ValidateEnvHandle<'a, DB>,
}

impl<'a, EXT: 'a, DB: Database + 'a> ValidationHandler<'a, EXT, DB> {
    /// Create new ValidationHandles
    pub fn new<SPEC: Spec + 'a>() -> Self {
        Self {
            initial_tx_energy: Arc::new(mainnet::validate_initial_tx_energy::<SPEC, DB>),
            env: Arc::new(mainnet::validate_env::<SPEC, DB>),
            tx_against_state: Arc::new(mainnet::validate_tx_against_state::<SPEC, EXT, DB>),
        }
    }
}

impl<'a, EXT, DB: Database> ValidationHandler<'a, EXT, DB> {
    /// Validate env.
    pub fn env(&self, env: &Env) -> Result<(), EVMError<DB::Error>> {
        (self.env)(env)
    }

    /// Initial energy
    pub fn initial_tx_energy(&self, env: &Env) -> Result<u64, EVMError<DB::Error>> {
        (self.initial_tx_energy)(env)
    }

    /// Validate ttansaction against the state.
    pub fn tx_against_state(
        &self,
        context: &mut Context<EXT, DB>,
    ) -> Result<(), EVMError<DB::Error>> {
        (self.tx_against_state)(context)
    }
}
