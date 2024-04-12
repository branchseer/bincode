use core::hash::{BuildHasher, Hash};

use crate::{
    de::{BorrowDecoder, Decoder},
    enc::Encoder,
    error::{DecodeError, EncodeError},
    BorrowDecode, Decode, Encode,
};
use indexmap::{IndexMap, IndexSet};

impl<K: Encode, V: Encode, S> Encode for IndexMap<K, V, S> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        (self.len() as u64).encode(encoder)?;
        for (k, v) in self.iter() {
            k.encode(encoder)?;
            v.encode(encoder)?;
        }
        Ok(())
    }
}

impl<C, K: Decode<C> + Hash + Eq, V: Decode<C>, S: BuildHasher + Default> Decode<C>
    for IndexMap<K, V, S>
{
    fn decode<D: Decoder<Ctx = C>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let len = crate::de::decode_slice_len(decoder)?;
        decoder.claim_container_read::<(K, V)>(len)?;

        let hash_builder: S = Default::default();
        let mut map = IndexMap::with_capacity_and_hasher(len, hash_builder);
        for _ in 0..len {
            // See the documentation on `unclaim_bytes_read` as to why we're doing this here
            decoder.unclaim_bytes_read(core::mem::size_of::<(K, V)>());

            let k = K::decode(decoder)?;
            let v = V::decode(decoder)?;
            map.insert(k, v);
        }
        Ok(map)
    }
}

impl<'de, K, V, S, Ctx> BorrowDecode<'de, Ctx> for IndexMap<K, V, S>
where
    K: BorrowDecode<'de, Ctx> + Eq + std::hash::Hash,
    V: BorrowDecode<'de, Ctx>,
    S: BuildHasher + Default,
{
    fn borrow_decode<D: BorrowDecoder<'de, Ctx = Ctx>>(
        decoder: &mut D,
    ) -> Result<Self, DecodeError> {
        let len = crate::de::decode_slice_len(decoder)?;
        decoder.claim_container_read::<(K, V)>(len)?;

        let hash_builder: S = Default::default();
        let mut map = IndexMap::with_capacity_and_hasher(len, hash_builder);
        for _ in 0..len {
            // See the documentation on `unclaim_bytes_read` as to why we're doing this here
            decoder.unclaim_bytes_read(core::mem::size_of::<(K, V)>());

            let k = K::borrow_decode(decoder)?;
            let v = V::borrow_decode(decoder)?;
            map.insert(k, v);
        }
        Ok(map)
    }
}

impl<C, T, S> Decode<C> for IndexSet<T, S>
where
    T: Decode<C> + Eq + Hash,
    S: BuildHasher + Default,
{
    fn decode<D: Decoder<Ctx = C>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let len = crate::de::decode_slice_len(decoder)?;
        decoder.claim_container_read::<T>(len)?;

        let hash_builder: S = Default::default();
        let mut map: IndexSet<T, S> = IndexSet::with_capacity_and_hasher(len, hash_builder);
        for _ in 0..len {
            // See the documentation on `unclaim_bytes_read` as to why we're doing this here
            decoder.unclaim_bytes_read(core::mem::size_of::<T>());

            let key = T::decode(decoder)?;
            map.insert(key);
        }
        Ok(map)
    }
}

impl<'de, T, S, Ctx> BorrowDecode<'de, Ctx> for IndexSet<T, S>
where
    T: BorrowDecode<'de, Ctx> + Eq + Hash,
    S: BuildHasher + Default,
{
    fn borrow_decode<D: BorrowDecoder<'de, Ctx = Ctx>>(
        decoder: &mut D,
    ) -> Result<Self, DecodeError> {
        let len = crate::de::decode_slice_len(decoder)?;
        decoder.claim_container_read::<T>(len)?;

        let mut map = IndexSet::with_capacity_and_hasher(len, S::default());
        for _ in 0..len {
            // See the documentation on `unclaim_bytes_read` as to why we're doing this here
            decoder.unclaim_bytes_read(core::mem::size_of::<T>());

            let key = T::borrow_decode(decoder)?;
            map.insert(key);
        }
        Ok(map)
    }
}

impl<T, S> Encode for IndexSet<T, S>
where
    T: Encode,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        crate::enc::encode_slice_len(encoder, self.len())?;
        for item in self.iter() {
            item.encode(encoder)?;
        }
        Ok(())
    }
}
