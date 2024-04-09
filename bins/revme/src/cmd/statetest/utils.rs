use libgoldilocks::SigningKey;
use revm::primitives::Address;

/// Recover the address from a private key (SigningKey).
pub fn recover_address(private_key: &[u8]) -> Option<Address> {
    let key = SigningKey::from_slice(private_key);
    let public_key = key.verifying_key();
    Some(Address::from_raw_public_key(&public_key.as_bytes()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use revm::primitives::{address, hex};

    #[test]
    fn sanity_test() {
        assert_eq!(
            Some(address!("1afe4bd57060cb20be3da71729151922bcbf3947")),
            recover_address(&hex!(
                "445a915e4d060149eb4365960e6a7a45f334393093061116b197e3240065ff2d85a915e4d060149eb4365960e6a7a45f334393093061110068"
            ))
        )
    }
}
