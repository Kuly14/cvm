use revm::{
    primitives::{address, AccountInfo, Bytecode, Bytes, TransactTo, U256},
    Evm, InMemoryDB,
};
use std::str::FromStr;

fn main() {
    let code = Bytecode::new_raw(Bytes::from_str("6080604052600a6000806101000a81548175ffffffffffffffffffffffffffffffffffffffffffff021916908375ffffffffffffffffffffffffffffffffffffffffffff16021790555034801561005557600080fd5b50610129806100656000396000f3fe6080604052348015600f57600080fd5b506004361060285760003560e01c80633b21bc1414602d575b600080fd5b60336035565b005b60008075ffffffffffffffffffffffffffffffffffffffffffff16670de0b6b3a764000060405160639060d2565b60006040518083038185875af1925050503d8060008114609e576040519150601f19603f3d011682016040523d82523d6000602084013e60a3565b606091505b505090508060b057600080fd5b50565b600060be60008360e5565b915060c78260f0565b600082019050919050565b600060db8260b3565b9150819050919050565b600081905092915050565b5056fea2646970667358221220ed4f28df46ec962a084c6e05408d34a7fe93d0366f307176375229425f42982664736f6c63430101000033").unwrap());
    // BenchmarkDB is dummy state that implements Database trait.
    let mut evm = Evm::builder()
        .with_db(InMemoryDB::default())
        .modify_db(|db| {
            db.insert_account_info(
                address!("1111111111111111111111111111111111111111"),
                AccountInfo::new(U256::MAX, 0, code.hash_slow(), code),
            )
        })
        .modify_tx_env(|tx| {
            // execution globals block hash/gas_limit/coinbase/timestamp..
            tx.caller = "0x1000000000000000000000000000000000000000"
                .parse()
                .unwrap();
            tx.value = U256::from(0);
            tx.transact_to = TransactTo::Call(
                "0x1111111111111111111111111111111111111111"
                    .parse()
                    .unwrap(),
            );
        })
        .build();

        let s = evm.transact().unwrap();
        println!("{:?}", s);
}
