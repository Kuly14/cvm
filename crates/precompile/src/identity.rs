use super::calc_linear_cost_u32;
use crate::{Error, Precompile, PrecompileResult, PrecompileWithAddress};
use revm_primitives::Bytes;

pub const FUN: PrecompileWithAddress =
    PrecompileWithAddress(crate::u64_to_address(4), Precompile::Standard(identity_run));

/// The base cost of the operation.
pub const IDENTITY_BASE: u64 = 15;
/// The cost per word.
pub const IDENTITY_PER_WORD: u64 = 3;

/// Takes the input bytes, copies them, and returns it as the output.
///
/// See: <https://ethereum.github.io/yellowpaper/paper.pdf>
/// See: <https://etherscan.io/address/0000000000000000000000000000000000000004>
pub fn identity_run(input: &Bytes, energy_limit: u64) -> PrecompileResult {
    let energy_used = calc_linear_cost_u32(input.len(), IDENTITY_BASE, IDENTITY_PER_WORD);
    if energy_used > energy_limit {
        return Err(Error::OutOfEnergy);
    }
    Ok((energy_used, input.clone()))
}
