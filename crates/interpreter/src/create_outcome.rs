use crate::{Energy, InstructionResult, InterpreterResult};
use revm_primitives::{Address, Bytes};

/// Represents the outcome of a create operation in an interpreter.
///
/// This struct holds the result of the operation along with an optional address.
/// It provides methods to determine the next action based on the result of the operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateOutcome {
    // The result of the interpreter operation.
    pub result: InterpreterResult,
    // An optional address associated with the create operation.
    pub address: Option<Address>,
}

impl CreateOutcome {
    /// Constructs a new `CreateOutcome`.
    ///
    /// # Arguments
    ///
    /// * `result` - An `InterpreterResult` representing the result of the interpreter operation.
    /// * `address` - An optional `Address` associated with the create operation.
    ///
    /// # Returns
    ///
    /// A new `CreateOutcome` instance.
    pub fn new(result: InterpreterResult, address: Option<Address>) -> Self {
        Self { result, address }
    }

    /// Retrieves a reference to the `InstructionResult` from the `InterpreterResult`.
    ///
    /// This method provides access to the `InstructionResult` which represents the
    /// outcome of the instruction execution. It encapsulates the result information
    /// such as whether the instruction was executed successfully, resulted in a revert,
    /// or encountered a fatal error.
    ///
    /// # Returns
    ///
    /// A reference to the `InstructionResult`.
    pub fn instruction_result(&self) -> &InstructionResult {
        &self.result.result
    }

    /// Retrieves a reference to the output bytes from the `InterpreterResult`.
    ///
    /// This method returns the output of the interpreted operation. The output is
    /// typically used when the operation successfully completes and returns data.
    ///
    /// # Returns
    ///
    /// A reference to the output `Bytes`.
    pub fn output(&self) -> &Bytes {
        &self.result.output
    }

    /// Retrieves a reference to the `Energy` details from the `InterpreterResult`.
    ///
    /// This method provides access to the energy details of the operation, which includes
    /// information about energy used, remaining, and refunded. It is essential for
    /// understanding the energy consumption of the operation.
    ///
    /// # Returns
    ///
    /// A reference to the `Energy` details.
    pub fn energy(&self) -> &Energy {
        &self.result.energy
    }
}
