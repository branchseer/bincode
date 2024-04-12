use crate::{
    enc::Encoder,
    error::{DecodeError, EncodeError},
    BorrowDecode, Decode, Encode,
};
use alloc::vec::Vec;
use index_vec::{Idx, IndexVec};

impl<I: Idx, T: Encode> Encode for IndexVec<I, T> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.as_vec().encode(encoder)
    }
}

impl<C, I: Idx, T: Decode<C>> Decode<C> for IndexVec<I, T> {
    fn decode<D: crate::de::Decoder<Ctx = C>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let vec = Vec::<T>::decode(decoder)?;
        Ok(Self::from_vec(vec))
    }
}

impl<'de, C, I: Idx, T: BorrowDecode<'de, C>> BorrowDecode<'de, C> for IndexVec<I, T> {
    fn borrow_decode<D: crate::de::BorrowDecoder<'de, Ctx = C>>(
        decoder: &mut D,
    ) -> Result<Self, DecodeError> {
        let vec = Vec::<T>::borrow_decode(decoder)?;
        Ok(Self::from_vec(vec))
    }
}
