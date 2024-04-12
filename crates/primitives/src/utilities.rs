use crate::{
    b256, B256, BLOB_ENERGYPRICE_UPDATE_FRACTION, MIN_BLOB_ENERGYPRICE,
    TARGET_BLOB_ENERGY_PER_BLOCK,
};
pub use alloy_primitives::sha3;

/// The Sha3 hash of the empty string `""`.
pub const SHA3_EMPTY: B256 =
    b256!("a7ffc6f8bf1ed76651c14756a061d662f580ff4de43b49fa82d80a4b80f8434a");

/// Calculates the `excess_blob_energy` from the parent header's `blob_energy_used` and `excess_blob_energy`.
///
/// See also [the EIP-4844 helpers]<https://eips.ethereum.org/EIPS/eip-4844#helpers>
/// (`calc_excess_blob_energy`).
#[inline]
pub fn calc_excess_blob_energy(
    parent_excess_blob_energy: u64,
    parent_blob_energy_used: u64,
) -> u64 {
    (parent_excess_blob_energy + parent_blob_energy_used)
        .saturating_sub(TARGET_BLOB_ENERGY_PER_BLOCK)
}

/// Calculates the blob energy price from the header's excess blob energy field.
///
/// See also [the EIP-4844 helpers](https://eips.ethereum.org/EIPS/eip-4844#helpers)
/// (`get_blob_energyprice`).
#[inline]
pub fn calc_blob_energyprice(excess_blob_energy: u64) -> u128 {
    fake_exponential(
        MIN_BLOB_ENERGYPRICE,
        excess_blob_energy,
        BLOB_ENERGYPRICE_UPDATE_FRACTION,
    )
}

/// Approximates `factor * e ** (numerator / denominator)` using Taylor expansion.
///
/// This is used to calculate the blob price.
///
/// See also [the EIP-4844 helpers](https://eips.ethereum.org/EIPS/eip-4844#helpers)
/// (`fake_exponential`).
///
/// # Panics
///
/// This function panics if `denominator` is zero.
#[inline]
pub fn fake_exponential(factor: u64, numerator: u64, denominator: u64) -> u128 {
    assert_ne!(denominator, 0, "attempt to divide by zero");
    let factor = factor as u128;
    let numerator = numerator as u128;
    let denominator = denominator as u128;

    let mut i = 1;
    let mut output = 0;
    let mut numerator_accum = factor * denominator;
    while numerator_accum > 0 {
        output += numerator_accum;

        // Denominator is asserted as not zero at the start of the function.
        numerator_accum = (numerator_accum * numerator) / (denominator * i);
        i += 1;
    }
    output / denominator
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ENERGY_PER_BLOB;

    // https://github.com/ethereum/go-ethereum/blob/28857080d732857030eda80c69b9ba2c8926f221/consensus/misc/eip4844/eip4844_test.go#L27
    #[test]
    fn test_calc_excess_blob_energy() {
        for t @ &(excess, blobs, expected) in &[
            // The excess blob energy should not increase from zero if the used blob
            // slots are below - or equal - to the target.
            (0, 0, 0),
            (0, 1, 0),
            (0, TARGET_BLOB_ENERGY_PER_BLOCK / ENERGY_PER_BLOB, 0),
            // If the target blob energy is exceeded, the excessBlobEnergy should increase
            // by however much it was overshot
            (
                0,
                (TARGET_BLOB_ENERGY_PER_BLOCK / ENERGY_PER_BLOB) + 1,
                ENERGY_PER_BLOB,
            ),
            (
                1,
                (TARGET_BLOB_ENERGY_PER_BLOCK / ENERGY_PER_BLOB) + 1,
                ENERGY_PER_BLOB + 1,
            ),
            (
                1,
                (TARGET_BLOB_ENERGY_PER_BLOCK / ENERGY_PER_BLOB) + 2,
                2 * ENERGY_PER_BLOB + 1,
            ),
            // The excess blob energy should decrease by however much the target was
            // under-shot, capped at zero.
            (
                TARGET_BLOB_ENERGY_PER_BLOCK,
                TARGET_BLOB_ENERGY_PER_BLOCK / ENERGY_PER_BLOB,
                TARGET_BLOB_ENERGY_PER_BLOCK,
            ),
            (
                TARGET_BLOB_ENERGY_PER_BLOCK,
                (TARGET_BLOB_ENERGY_PER_BLOCK / ENERGY_PER_BLOB) - 1,
                TARGET_BLOB_ENERGY_PER_BLOCK - ENERGY_PER_BLOB,
            ),
            (
                TARGET_BLOB_ENERGY_PER_BLOCK,
                (TARGET_BLOB_ENERGY_PER_BLOCK / ENERGY_PER_BLOB) - 2,
                TARGET_BLOB_ENERGY_PER_BLOCK - (2 * ENERGY_PER_BLOB),
            ),
            (
                ENERGY_PER_BLOB - 1,
                (TARGET_BLOB_ENERGY_PER_BLOCK / ENERGY_PER_BLOB) - 1,
                0,
            ),
        ] {
            let actual = calc_excess_blob_energy(excess, blobs * ENERGY_PER_BLOB);
            assert_eq!(actual, expected, "test: {t:?}");
        }
    }

    // https://github.com/ethereum/go-ethereum/blob/28857080d732857030eda80c69b9ba2c8926f221/consensus/misc/eip4844/eip4844_test.go#L60
    #[test]
    fn test_calc_blob_fee() {
        let blob_fee_vectors = &[
            (0, 1),
            (2314057, 1),
            (2314058, 2),
            (10 * 1024 * 1024, 23),
            // calc_blob_energyprice approximates `e ** (excess_blob_energy / BLOB_ENERGYPRICE_UPDATE_FRACTION)` using Taylor expansion
            //
            // to roughly find where boundaries will be hit:
            // 2 ** bits = e ** (excess_blob_energy / BLOB_ENERGYPRICE_UPDATE_FRACTION)
            // excess_blob_energy = ln(2 ** bits) * BLOB_ENERGYPRICE_UPDATE_FRACTION
            (148099578, 18446739238971471609), // output is just below the overflow
            (148099579, 18446744762204311910), // output is just after the overflow
            (161087488, 902580055246494526580),
        ];

        for &(excess, expected) in blob_fee_vectors {
            let actual = calc_blob_energyprice(excess);
            assert_eq!(actual, expected, "test: {excess}");
        }
    }

    // https://github.com/ethereum/go-ethereum/blob/28857080d732857030eda80c69b9ba2c8926f221/consensus/misc/eip4844/eip4844_test.go#L78
    #[test]
    fn fake_exp() {
        for t @ &(factor, numerator, denominator, expected) in &[
            (1u64, 0u64, 1u64, 1u128),
            (38493, 0, 1000, 38493),
            (0, 1234, 2345, 0),
            (1, 2, 1, 6), // approximate 7.389
            (1, 4, 2, 6),
            (1, 3, 1, 16), // approximate 20.09
            (1, 6, 2, 18),
            (1, 4, 1, 49), // approximate 54.60
            (1, 8, 2, 50),
            (10, 8, 2, 542), // approximate 540.598
            (11, 8, 2, 596), // approximate 600.58
            (1, 5, 1, 136),  // approximate 148.4
            (1, 5, 2, 11),   // approximate 12.18
            (2, 5, 2, 23),   // approximate 24.36
            (1, 50000000, 2225652, 5709098764),
            (1, 380928, BLOB_ENERGYPRICE_UPDATE_FRACTION, 1),
        ] {
            let actual = fake_exponential(factor, numerator, denominator);
            assert_eq!(actual, expected, "test: {t:?}");
        }
    }
}
