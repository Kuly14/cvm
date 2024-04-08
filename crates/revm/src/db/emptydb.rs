use core::{convert::Infallible, fmt, marker::PhantomData};
use revm_interpreter::primitives::{
    db::{Database, DatabaseRef},
    sha3, AccountInfo, Address, Bytecode, B256, U256,
};

/// An empty database that always returns default values when queried.
pub type EmptyDB = EmptyDBTyped<Infallible>;

/// An empty database that always returns default values when queried.
///
/// This is generic over a type which is used as the database error type.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EmptyDBTyped<E> {
    _phantom: PhantomData<E>,
}

// Don't derive traits, because the type parameter is unused.
impl<E> Clone for EmptyDBTyped<E> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<E> Copy for EmptyDBTyped<E> {}

impl<E> Default for EmptyDBTyped<E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<E> fmt::Debug for EmptyDBTyped<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EmptyDB").finish_non_exhaustive()
    }
}

impl<E> PartialEq for EmptyDBTyped<E> {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl<E> Eq for EmptyDBTyped<E> {}

impl<E> EmptyDBTyped<E> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }

    #[doc(hidden)]
    #[deprecated = "use `new` instead"]
    pub fn new_keccak_block_hash() -> Self {
        Self::new()
    }
}

impl<E> Database for EmptyDBTyped<E> {
    type Error = E;

    #[inline]
    fn basic(&mut self, address: Address) -> Result<Option<AccountInfo>, Self::Error> {
        <Self as DatabaseRef>::basic_ref(self, address)
    }

    #[inline]
    fn code_by_hash(&mut self, code_hash: B256) -> Result<Bytecode, Self::Error> {
        <Self as DatabaseRef>::code_by_hash_ref(self, code_hash)
    }

    #[inline]
    fn storage(&mut self, address: Address, index: U256) -> Result<U256, Self::Error> {
        <Self as DatabaseRef>::storage_ref(self, address, index)
    }

    #[inline]
    fn block_hash(&mut self, number: U256) -> Result<B256, Self::Error> {
        <Self as DatabaseRef>::block_hash_ref(self, number)
    }
}

impl<E> DatabaseRef for EmptyDBTyped<E> {
    type Error = E;

    #[inline]
    fn basic_ref(&self, _address: Address) -> Result<Option<AccountInfo>, Self::Error> {
        Ok(None)
    }

    #[inline]
    fn code_by_hash_ref(&self, _code_hash: B256) -> Result<Bytecode, Self::Error> {
        Ok(Bytecode::new())
    }

    #[inline]
    fn storage_ref(&self, _address: Address, _index: U256) -> Result<U256, Self::Error> {
        Ok(U256::default())
    }

    #[inline]
    fn block_hash_ref(&self, number: U256) -> Result<B256, Self::Error> {
        Ok(sha3(number.to_string().as_bytes()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::b256;

    #[test]
    fn conform_block_hash_calculation() {
        let db = EmptyDB::new();
        assert_eq!(
            db.block_hash_ref(U256::from(0)),
            Ok(b256!(
                "f9e2eaaa42d9fe9e558a9b8ef1bf366f190aacaa83bad2641ee106e9041096e4"
            ))
        );

        assert_eq!(
            db.block_hash_ref(U256::from(1)),
            Ok(b256!(
                "67b176705b46206614219f47a05aee7ae6a3edbe850bbbe214c536b989aea4d2"
            ))
        );

        assert_eq!(
            db.block_hash_ref(U256::from(100)),
            Ok(b256!(
                "46b55626ab805350ea5f08f3592bd81298c12f2fee1d6040d1b8b3c7b490d966"
            ))
        );
    }
}
