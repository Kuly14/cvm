use super::constants::*;
use crate::inner_models::SelfDestructResult;
use crate::primitives::{Address, SpecId, U256};
use std::vec::Vec;

/// `const` Option `?`.
macro_rules! tri {
    ($e:expr) => {
        match $e {
            Some(v) => v,
            None => return None,
        }
    };
}

/// `const` unwrap.
macro_rules! opt_unwrap {
    ($e:expr) => {
        match $e {
            Some(v) => v,
            None => panic!("unwrap failed"),
        }
    };
}

/// `SSTORE` opcode refund calculation.
#[allow(clippy::collapsible_else_if)]
pub fn sstore_refund(spec_id: SpecId, original: U256, current: U256, new: U256) -> i64 {
    if spec_id.is_enabled_in(SpecId::ISTANBUL) {
        // EIP-3529: Reduction in refunds
        let sstore_clears_schedule = if spec_id.is_enabled_in(SpecId::LONDON) {
            (SSTORE_RESET - COLD_SLOAD_COST + ACCESS_LIST_STORAGE_KEY) as i64
        } else {
            REFUND_SSTORE_CLEARS
        };
        if current == new {
            0
        } else {
            if original == current && new == U256::ZERO {
                sstore_clears_schedule
            } else {
                let mut refund = 0;

                if original != U256::ZERO {
                    if current == U256::ZERO {
                        refund -= sstore_clears_schedule;
                    } else if new == U256::ZERO {
                        refund += sstore_clears_schedule;
                    }
                }

                if original == new {
                    let (energy_sstore_reset, energy_sload) =
                        if spec_id.is_enabled_in(SpecId::BERLIN) {
                            (SSTORE_RESET - COLD_SLOAD_COST, WARM_STORAGE_READ_COST)
                        } else {
                            (SSTORE_RESET, sload_cost(spec_id, false))
                        };
                    if original == U256::ZERO {
                        refund += (SSTORE_SET - energy_sload) as i64;
                    } else {
                        refund += (energy_sstore_reset - energy_sload) as i64;
                    }
                }

                refund
            }
        }
    } else {
        if current != U256::ZERO && new == U256::ZERO {
            REFUND_SSTORE_CLEARS
        } else {
            0
        }
    }
}

/// `CREATE2` opcode cost calculation.
#[inline]
pub const fn create2_cost(len: u64) -> Option<u64> {
    let sha_addup_base = len.div_ceil(32);
    let sha_addup = tri!(SHA3WORD.checked_mul(sha_addup_base));
    CREATE.checked_add(sha_addup)
}

#[inline]
fn log2floor(value: U256) -> u64 {
    assert!(value != U256::ZERO);
    let mut l: u64 = 256;
    for i in 0..4 {
        let i = 3 - i;
        if value.as_limbs()[i] == 0u64 {
            l -= 64;
        } else {
            l -= value.as_limbs()[i].leading_zeros() as u64;
            if l == 0 {
                return l;
            } else {
                return l - 1;
            }
        }
    }
    l
}

/// `EXP` opcode cost calculation.
#[inline]
pub fn exp_cost(spec_id: SpecId, power: U256) -> Option<u64> {
    if power == U256::ZERO {
        Some(EXP)
    } else {
        // EIP-160: EXP cost increase
        let energy_byte = U256::from(if spec_id.is_enabled_in(SpecId::SPURIOUS_DRAGON) {
            50
        } else {
            10
        });
        let energy = U256::from(EXP)
            .checked_add(energy_byte.checked_mul(U256::from(log2floor(power) / 8 + 1))?)?;

        u64::try_from(energy).ok()
    }
}

/// `*COPY` opcodes cost calculation.
#[inline]
pub const fn verylowcopy_cost(len: u64) -> Option<u64> {
    VERYLOW.checked_add(tri!(cost_per_word(len, COPY)))
}

/// `EXTCODECOPY` opcode cost calculation.
#[inline]
pub const fn extcodecopy_cost(spec_id: SpecId, len: u64, is_cold: bool) -> Option<u64> {
    let base_energy = if spec_id.is_enabled_in(SpecId::BERLIN) {
        if is_cold {
            COLD_ACCOUNT_ACCESS_COST
        } else {
            WARM_STORAGE_READ_COST
        }
    } else if spec_id.is_enabled_in(SpecId::TANGERINE) {
        700
    } else {
        20
    };
    base_energy.checked_add(tri!(cost_per_word(len, COPY)))
}

/// `BALANCE` opcode cost calculation.
#[inline]
pub const fn account_access_energy(spec_id: SpecId, is_cold: bool) -> u64 {
    if spec_id.is_enabled_in(SpecId::BERLIN) {
        if is_cold {
            COLD_ACCOUNT_ACCESS_COST
        } else {
            WARM_STORAGE_READ_COST
        }
    } else if spec_id.is_enabled_in(SpecId::ISTANBUL) {
        700
    } else {
        20
    }
}

/// `LOG` opcode cost calculation.
#[inline]
pub const fn log_cost(n: u8, len: u64) -> Option<u64> {
    tri!(LOG.checked_add(tri!(LOGDATA.checked_mul(len)))).checked_add(LOGTOPIC * n as u64)
}

/// `SHA3` opcode cost calculation.
#[inline]
pub const fn sha3_cost(len: u64) -> Option<u64> {
    SHA3.checked_add(tri!(cost_per_word(len, SHA3WORD)))
}

/// Cost for memory length. `ceil(len / 32) * multiple`.
#[inline]
pub const fn cost_per_word(len: u64, multiple: u64) -> Option<u64> {
    len.div_ceil(32).checked_mul(multiple)
}

/// EIP-3860: Limit and meter initcode
///
/// Apply extra energy cost of 2 for every 32-byte chunk of initcode.
///
/// This cannot overflow as the initcode length is assumed to be checked.
#[inline]
pub const fn initcode_cost(len: u64) -> u64 {
    opt_unwrap!(cost_per_word(len, INITCODE_WORD_COST))
}

/// `SLOAD` opcode cost calculation.
#[inline]
pub const fn sload_cost(spec_id: SpecId, is_cold: bool) -> u64 {
    if spec_id.is_enabled_in(SpecId::BERLIN) {
        if is_cold {
            COLD_SLOAD_COST
        } else {
            WARM_STORAGE_READ_COST
        }
    } else if spec_id.is_enabled_in(SpecId::ISTANBUL) {
        // EIP-1884: Repricing for trie-size-dependent opcodes
        INSTANBUL_SLOAD_ENERGY
    } else if spec_id.is_enabled_in(SpecId::TANGERINE) {
        // EIP-150: Energy cost changes for IO-heavy operations
        200
    } else {
        50
    }
}

/// `SSTORE` opcode cost calculation.
#[inline]
pub fn sstore_cost(
    spec_id: SpecId,
    original: U256,
    current: U256,
    new: U256,
    energy: u64,
    is_cold: bool,
) -> Option<u64> {
    // EIP-1706 Disable SSTORE with energyleft lower than call stipend
    if spec_id.is_enabled_in(SpecId::ISTANBUL) && energy <= CALL_STIPEND {
        return None;
    }

    if spec_id.is_enabled_in(SpecId::BERLIN) {
        // Berlin specification logic
        let mut energy_cost = istanbul_sstore_cost::<WARM_STORAGE_READ_COST, WARM_SSTORE_RESET>(
            original, current, new,
        );

        if is_cold {
            energy_cost += COLD_SLOAD_COST;
        }
        Some(energy_cost)
    } else if spec_id.is_enabled_in(SpecId::ISTANBUL) {
        // Istanbul logic
        Some(istanbul_sstore_cost::<INSTANBUL_SLOAD_ENERGY, SSTORE_RESET>(original, current, new))
    } else {
        // Frontier logic
        Some(frontier_sstore_cost(current, new))
    }
}

/// EIP-2200: Structured Definitions for Net Energy Metering
#[inline]
fn istanbul_sstore_cost<const SLOAD_ENERGY: u64, const SSTORE_RESET_ENERGY: u64>(
    original: U256,
    current: U256,
    new: U256,
) -> u64 {
    if new == current {
        SLOAD_ENERGY
    } else if original == current && original == U256::ZERO {
        SSTORE_SET
    } else if original == current {
        SSTORE_RESET_ENERGY
    } else {
        SLOAD_ENERGY
    }
}

/// Frontier sstore cost just had two cases set and reset values.
#[inline]
fn frontier_sstore_cost(current: U256, new: U256) -> u64 {
    if current == U256::ZERO && new != U256::ZERO {
        SSTORE_SET
    } else {
        SSTORE_RESET
    }
}

/// `SELFDESTRUCT` opcode cost calculation.
#[inline]
pub const fn selfdestruct_cost(spec_id: SpecId, res: SelfDestructResult) -> u64 {
    // EIP-161: State trie clearing (invariant-preserving alternative)
    let should_charge_topup = if spec_id.is_enabled_in(SpecId::SPURIOUS_DRAGON) {
        res.had_value && !res.target_exists
    } else {
        !res.target_exists
    };

    // EIP-150: Energy cost changes for IO-heavy operations
    let selfdestruct_energy_topup =
        if spec_id.is_enabled_in(SpecId::TANGERINE) && should_charge_topup {
            25000
        } else {
            0
        };

    // EIP-150: Energy cost changes for IO-heavy operations
    let selfdestruct_energy = if spec_id.is_enabled_in(SpecId::TANGERINE) {
        5000
    } else {
        0
    };

    let mut energy = selfdestruct_energy + selfdestruct_energy_topup;
    if spec_id.is_enabled_in(SpecId::BERLIN) && res.is_cold {
        energy += COLD_ACCOUNT_ACCESS_COST
    }
    energy
}

/// Basic `CALL` opcode cost calculation, see [`call_cost`].
#[inline]
pub const fn call_energy(spec_id: SpecId, is_cold: bool) -> u64 {
    if spec_id.is_enabled_in(SpecId::BERLIN) {
        if is_cold {
            COLD_ACCOUNT_ACCESS_COST
        } else {
            WARM_STORAGE_READ_COST
        }
    } else if spec_id.is_enabled_in(SpecId::TANGERINE) {
        // EIP-150: Energy cost changes for IO-heavy operations
        700
    } else {
        40
    }
}

/// `CALL` opcode cost calculation.
#[inline]
pub const fn call_cost(
    spec_id: SpecId,
    transfers_value: bool,
    is_new: bool,
    is_cold: bool,
    is_call_or_callcode: bool,
    is_call_or_staticcall: bool,
) -> u64 {
    call_energy(spec_id, is_cold)
        + xfer_cost(is_call_or_callcode, transfers_value)
        + new_cost(spec_id, is_call_or_staticcall, is_new, transfers_value)
}

#[inline]
const fn xfer_cost(is_call_or_callcode: bool, transfers_value: bool) -> u64 {
    if is_call_or_callcode && transfers_value {
        CALLVALUE
    } else {
        0
    }
}

#[inline]
const fn new_cost(
    spec_id: SpecId,
    is_call_or_staticcall: bool,
    is_new: bool,
    transfers_value: bool,
) -> u64 {
    if !is_call_or_staticcall || !is_new {
        return 0;
    }

    // EIP-161: State trie clearing (invariant-preserving alternative)
    if spec_id.is_enabled_in(SpecId::SPURIOUS_DRAGON) && !transfers_value {
        return 0;
    }

    NEWACCOUNT
}

/// Memory expansion cost calculation.
#[inline]
pub const fn memory_energy(a: usize) -> u64 {
    let a = a as u64;
    MEMORY
        .saturating_mul(a)
        .saturating_add(a.saturating_mul(a) / 512)
}

/// Initial energy that is deducted for transaction to be included.
/// Initial energy contains initial stipend energy, energy for access list and input data.
pub fn validate_initial_tx_energy(
    spec_id: SpecId,
    input: &[u8],
    is_create: bool,
    access_list: &[(Address, Vec<U256>)],
) -> u64 {
    let mut initial_energy = 0;
    let zero_data_len = input.iter().filter(|v| **v == 0).count() as u64;
    let non_zero_data_len = input.len() as u64 - zero_data_len;

    // initdate stipend
    initial_energy += zero_data_len * TRANSACTION_ZERO_DATA;
    // EIP-2028: Transaction data energy cost reduction
    initial_energy += non_zero_data_len
        * if spec_id.is_enabled_in(SpecId::ISTANBUL) {
            16
        } else {
            68
        };

    // get number of access list account and storages.
    if spec_id.is_enabled_in(SpecId::BERLIN) {
        let accessed_slots = access_list
            .iter()
            .fold(0, |slot_count, (_, slots)| slot_count + slots.len() as u64);
        initial_energy += access_list.len() as u64 * ACCESS_LIST_ADDRESS;
        initial_energy += accessed_slots * ACCESS_LIST_STORAGE_KEY;
    }

    // base stipend
    initial_energy += if is_create {
        if spec_id.is_enabled_in(SpecId::HOMESTEAD) {
            // EIP-2: Homestead Hard-fork Changes
            53000
        } else {
            21000
        }
    } else {
        21000
    };

    // EIP-3860: Limit and meter initcode
    // Initcode stipend for bytecode analysis
    if spec_id.is_enabled_in(SpecId::SHANGHAI) && is_create {
        initial_energy += initcode_cost(input.len() as u64)
    }

    initial_energy
}
