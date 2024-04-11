# The `energy.rs` Module

The `energy.rs` module in this Rust EVM implementation manages the concept of "energy" within the Ethereum network. In Ethereum, "energy" signifies the computational effort needed to execute operations, whether a simple transfer of ether or the execution of a smart contract function. Each operation carries a energy cost, and transactions must specify the maximum amount of energy they are willing to consume.

## Data Structures
- `Energy` Struct

    The `Energy` struct represents the energy state for a particular operation or transaction. The struct is defined as follows:

    ### Fields in `Energy` Struct

    - `limit`: The maximum amount of energy allowed for the operation or transaction.
    - `all_used_energy`: The total energy used, inclusive of memory expansion costs.
    - `used`: The energy used, excluding memory expansion costs.
    - `memory`: The energy used for memory expansion.
    - `refunded`: The energy refunded. Certain operations in Ethereum allow for energy refunds, up to half the energy used by a transaction.

## Methods of the `Energy` Struct

The `Energy` struct also includes several methods to manage the energy state. Here's a brief summary of their functions:

- `new`: Creates a new `Energy` instance with a specified energy limit and zero usage and refunds.
- `limit`, `memory`, `refunded`, `spend`, `remaining`: These getters return the current state of the corresponding field.
- `erase_cost`: Decreases the energy usage by a specified amount.
- `record_refund`: Increases the refunded energy by a specified amount.
- `record_cost`: Increases the used energy by a specified amount. It also checks for energy limit overflow. If the new total used energy would exceed the energy limit, it returns `false` and doesn't change the state.
- `record_memory`: This method works similarly to `record_cost`, but specifically for memory expansion energy. It only updates the state if the new memory energy usage is greater than the current usage.
- `energy_refund`: Increases the refunded energy by a specified amount.

