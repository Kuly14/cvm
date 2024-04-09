use crate::{utilities::right_pad, Error, Precompile, PrecompileResult, PrecompileWithAddress};
use libgoldilocks::goldilocks::ed448_verify_with_error;
use revm_primitives::{alloy_primitives::B1368, sha3, Bytes, B256};

pub const ECRECOVER: PrecompileWithAddress = PrecompileWithAddress(
    crate::u64_to_address(1),
    Precompile::Standard(ec_recover_run),
);

pub fn ecrecover(
    sig: &B1368,
    msg: &B256,
) -> Result<B256, libgoldilocks::errors::LibgoldilockErrors> {
    let mut sig_bytes = [0u8; 114];
    let mut pub_bytes = [0u8; 57];
    sig_bytes.copy_from_slice(&sig[0..114]);
    pub_bytes.copy_from_slice(&sig[114..171]);

    // Not sure whether this returns address(0) on invliad message
    ed448_verify_with_error(&pub_bytes, &sig_bytes, msg.as_ref())?;

    let mut hash = sha3(pub_bytes);
    // truncate to 20 bytes
    hash[..12].fill(0);
    Ok(hash)
}

pub fn ec_recover_run(input: &Bytes, gas_limit: u64) -> PrecompileResult {
    const ECRECOVER_BASE: u64 = 3_000;

    if ECRECOVER_BASE > gas_limit {
        return Err(Error::OutOfGas);
    }

    let input = right_pad::<267>(input);

    let msg = <&B256>::try_from(&input[0..32]).unwrap();
    let sig = <&B1368>::try_from(&input[96..32 * 3 + 171]).unwrap();

    let out = ecrecover(sig, msg)
        .map(|o| o.to_vec().into())
        .unwrap_or_default();
    Ok((ECRECOVER_BASE, out))
}

#[cfg(test)]
mod tests {
    // use super::*;
    use crate::{
        secp256k1::{ec_recover_run, ecrecover},
        Bytes, B256,
    };

    #[test]
    fn test_recover() {
        let sig = hex::decode("611d178b128095022653965eb0ed3bc8bbea8e7891b5a121a102a5b29bb895770d204354dbbc67c5567186f92cdb58a601397dfe0022e0ce002c1333b6829c37c732fb909501f719df200ceaaa0e0a1533dc22e4c9c999406c071fee2858bc7c76c66d113ff1ac739564d465cd541b0d1e003761457fcdd53dba3dea5848c43aa54fe468284319f032945a3acb9bd4cd0fa7b7c901d978e9acd9eca43fa5b3c32b648c33dcc3f3169e8080").unwrap();
        let sig: [u8; 171] = sig.try_into().unwrap();
        let msg = hex::decode("f092a4af1f2103fe7be067df44370097c444f3bf877783ba56f21cf70ba365a3")
            .unwrap();
        let msg: [u8; 32] = msg.try_into().unwrap();
        let msg = B256::from(msg);
        let recovered = ecrecover((&sig).into(), &msg).unwrap();
        let expected: [u8; 32] =
            hex::decode("000000000000000000000000fc37a3b370a1f22e2fe2f819c210895e098845ed")
                .unwrap()
                .try_into()
                .unwrap();
        assert_eq!(recovered, expected);
    }

    #[test]
    fn test_ecrecover() {
        let sig = hex::decode("f092a4af1f2103fe7be067df44370097c444f3bf877783ba56f21cf70ba365a300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000611d178b128095022653965eb0ed3bc8bbea8e7891b5a121a102a5b29bb895770d204354dbbc67c5567186f92cdb58a601397dfe0022e0ce002c1333b6829c37c732fb909501f719df200ceaaa0e0a1533dc22e4c9c999406c071fee2858bc7c76c66d113ff1ac739564d465cd541b0d1e003761457fcdd53dba3dea5848c43aa54fe468284319f032945a3acb9bd4cd0fa7b7c901d978e9acd9eca43fa5b3c32b648c33dcc3f3169e8080").unwrap();
        let recovered = ec_recover_run(&sig.into(), 5000).unwrap().1;
        let expected: Bytes =
            hex::decode("000000000000000000000000fc37a3b370a1f22e2fe2f819c210895e098845ed")
                .unwrap()
                .into();
        assert_eq!(recovered, expected);
    }
}
