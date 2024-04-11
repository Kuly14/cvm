use crate::{
    energy,
    primitives::{Spec, SpecId::*, U256},
    Host, Interpreter,
};

/// EIP-1344: ChainID opcode
pub fn chainid<H: Host + ?Sized, SPEC: Spec>(interpreter: &mut Interpreter, host: &mut H) {
    check!(interpreter, ISTANBUL);
    energy!(interpreter, energy::BASE);
    push!(interpreter, U256::from(host.env().cfg.network_id));
}

pub fn coinbase<H: Host + ?Sized>(interpreter: &mut Interpreter, host: &mut H) {
    energy!(interpreter, energy::BASE);
    push_b256!(interpreter, host.env().block.coinbase.into_word());
}

pub fn timestamp<H: Host + ?Sized>(interpreter: &mut Interpreter, host: &mut H) {
    energy!(interpreter, energy::BASE);
    push!(interpreter, host.env().block.timestamp);
}

pub fn number<H: Host + ?Sized>(interpreter: &mut Interpreter, host: &mut H) {
    energy!(interpreter, energy::BASE);
    push!(interpreter, host.env().block.number);
}

pub fn difficulty<H: Host + ?Sized, SPEC: Spec>(interpreter: &mut Interpreter, host: &mut H) {
    energy!(interpreter, energy::BASE);
    if SPEC::enabled(MERGE) {
        push_b256!(interpreter, host.env().block.prevrandao.unwrap());
    } else {
        push!(interpreter, host.env().block.difficulty);
    }
}

pub fn energylimit<H: Host + ?Sized>(interpreter: &mut Interpreter, host: &mut H) {
    energy!(interpreter, energy::BASE);
    push!(interpreter, host.env().block.energy_limit);
}

pub fn energyprice<H: Host + ?Sized>(interpreter: &mut Interpreter, host: &mut H) {
    energy!(interpreter, energy::BASE);
    push!(interpreter, host.env().effective_energy_price());
}

/// EIP-3198: BASEFEE opcode
pub fn basefee<H: Host + ?Sized, SPEC: Spec>(interpreter: &mut Interpreter, host: &mut H) {
    check!(interpreter, LONDON);
    energy!(interpreter, energy::BASE);
    push!(interpreter, host.env().block.basefee);
}

pub fn origin<H: Host + ?Sized>(interpreter: &mut Interpreter, host: &mut H) {
    energy!(interpreter, energy::BASE);
    push_b256!(interpreter, host.env().tx.caller.into_word());
}

// EIP-4844: Shard Blob Transactions
pub fn blob_hash<H: Host + ?Sized, SPEC: Spec>(interpreter: &mut Interpreter, host: &mut H) {
    check!(interpreter, CANCUN);
    energy!(interpreter, energy::VERYLOW);
    pop_top!(interpreter, index);
    let i = as_usize_saturated!(index);
    *index = match host.env().tx.blob_hashes.get(i) {
        Some(hash) => U256::from_be_bytes(hash.0),
        None => U256::ZERO,
    };
}

/// EIP-7516: BLOBBASEFEE opcode
pub fn blob_basefee<H: Host + ?Sized, SPEC: Spec>(interpreter: &mut Interpreter, host: &mut H) {
    check!(interpreter, CANCUN);
    energy!(interpreter, energy::BASE);
    push!(
        interpreter,
        U256::from(host.env().block.get_blob_energyprice().unwrap_or_default())
    );
}
