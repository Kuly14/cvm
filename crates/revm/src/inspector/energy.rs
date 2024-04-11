//! EnergyIspector. Helper Inspector to calculate energy for others.

use revm_interpreter::CallOutcome;

use crate::{
    interpreter::{CallInputs, CreateInputs, CreateOutcome},
    primitives::db::Database,
    EvmContext, Inspector,
};

/// Helper [Inspector] that keeps track of energy.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Default)]
pub struct EnergyInspector {
    energy_remaining: u64,
    last_energy_cost: u64,
}

impl EnergyInspector {
    pub fn energy_remaining(&self) -> u64 {
        self.energy_remaining
    }

    pub fn last_energy_cost(&self) -> u64 {
        self.last_energy_cost
    }
}

impl<DB: Database> Inspector<DB> for EnergyInspector {
    fn initialize_interp(
        &mut self,
        interp: &mut crate::interpreter::Interpreter,
        _context: &mut EvmContext<DB>,
    ) {
        self.energy_remaining = interp.energy.limit();
    }

    fn step(
        &mut self,
        interp: &mut crate::interpreter::Interpreter,
        _context: &mut EvmContext<DB>,
    ) {
        self.energy_remaining = interp.energy.remaining();
    }

    fn step_end(
        &mut self,
        interp: &mut crate::interpreter::Interpreter,
        _context: &mut EvmContext<DB>,
    ) {
        let remaining = interp.energy.remaining();
        self.last_energy_cost = self.energy_remaining.saturating_sub(remaining);
        self.energy_remaining = remaining;
    }

    fn call_end(
        &mut self,
        _context: &mut EvmContext<DB>,
        _inputs: &CallInputs,
        mut outcome: CallOutcome,
    ) -> CallOutcome {
        if outcome.result.result.is_error() {
            outcome
                .result
                .energy
                .record_cost(outcome.result.energy.remaining());
            self.energy_remaining = 0;
        }
        outcome
    }

    fn create_end(
        &mut self,
        _context: &mut EvmContext<DB>,
        _inputs: &CreateInputs,
        mut outcome: CreateOutcome,
    ) -> CreateOutcome {
        if outcome.result.result.is_error() {
            outcome
                .result
                .energy
                .record_cost(outcome.result.energy.remaining());
            self.energy_remaining = 0;
        }
        outcome
    }
}

#[cfg(test)]
mod tests {

    use revm_interpreter::CallOutcome;
    use revm_interpreter::CreateOutcome;

    use crate::{
        inspectors::EnergyInspector,
        interpreter::{CallInputs, CreateInputs, Interpreter},
        primitives::Log,
        Database, EvmContext, Inspector,
    };
    use std::vec::Vec;

    #[derive(Default, Debug)]
    struct StackInspector {
        pc: usize,
        energy_inspector: EnergyInspector,
        energy_remaining_steps: Vec<(usize, u64)>,
    }

    impl<DB: Database> Inspector<DB> for StackInspector {
        fn initialize_interp(&mut self, interp: &mut Interpreter, context: &mut EvmContext<DB>) {
            self.energy_inspector.initialize_interp(interp, context);
        }

        fn step(&mut self, interp: &mut Interpreter, context: &mut EvmContext<DB>) {
            self.pc = interp.program_counter();
            self.energy_inspector.step(interp, context);
        }

        fn log(&mut self, context: &mut EvmContext<DB>, log: &Log) {
            self.energy_inspector.log(context, log);
        }

        fn step_end(&mut self, interp: &mut Interpreter, context: &mut EvmContext<DB>) {
            self.energy_inspector.step_end(interp, context);
            self.energy_remaining_steps
                .push((self.pc, self.energy_inspector.energy_remaining()));
        }

        fn call(
            &mut self,
            context: &mut EvmContext<DB>,
            call: &mut CallInputs,
        ) -> Option<CallOutcome> {
            self.energy_inspector.call(context, call)
        }

        fn call_end(
            &mut self,
            context: &mut EvmContext<DB>,
            inputs: &CallInputs,
            outcome: CallOutcome,
        ) -> CallOutcome {
            self.energy_inspector.call_end(context, inputs, outcome)
        }

        fn create(
            &mut self,
            context: &mut EvmContext<DB>,
            call: &mut CreateInputs,
        ) -> Option<CreateOutcome> {
            self.energy_inspector.create(context, call);
            None
        }

        fn create_end(
            &mut self,
            context: &mut EvmContext<DB>,
            inputs: &CreateInputs,
            outcome: CreateOutcome,
        ) -> CreateOutcome {
            self.energy_inspector.create_end(context, inputs, outcome)
        }
    }

    #[test]
    fn test_energy_inspector() {
        use crate::{
            db::BenchmarkDB,
            inspector::inspector_handle_register,
            interpreter::opcode,
            primitives::{address, Bytecode, Bytes, TransactTo},
            Evm,
        };

        let contract_data: Bytes = Bytes::from(vec![
            opcode::PUSH1,
            0x1,
            opcode::PUSH1,
            0xb,
            opcode::JUMPI,
            opcode::PUSH1,
            0x1,
            opcode::PUSH1,
            0x1,
            opcode::PUSH1,
            0x1,
            opcode::JUMPDEST,
            opcode::STOP,
        ]);
        let bytecode = Bytecode::new_raw(contract_data);

        let mut evm: Evm<'_, StackInspector, BenchmarkDB> = Evm::builder()
            .with_db(BenchmarkDB::new_bytecode(bytecode.clone()))
            .with_external_context(StackInspector::default())
            .modify_tx_env(|tx| {
                tx.clear();
                tx.caller = address!("1000000000000000000000000000000000000000");
                tx.transact_to =
                    TransactTo::Call(address!("0000000000000000000000000000000000000000"));
                tx.energy_limit = 21100;
            })
            .append_handler_register(inspector_handle_register)
            .build();

        // run evm.
        evm.transact().unwrap();

        let inspector = evm.into_context().external;

        // starting from 100energy
        let steps = vec![
            // push1 -3
            (0, 97),
            // push1 -3
            (2, 94),
            // jumpi -10
            (4, 84),
            // jumpdest 1
            (11, 83),
            // stop 0
            (12, 83),
        ];

        assert_eq!(inspector.energy_remaining_steps, steps);
    }
}
