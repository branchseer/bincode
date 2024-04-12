use compact_str::CompactString;

use crate::{
    de::{read::Reader, Decoder},
    error::{DecodeError, EncodeError},
    Decode, Encode,
};

impl Encode for CompactString {
    fn encode<E: crate::enc::Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.as_str().encode(encoder)
    }
}

impl<C> Decode<C> for CompactString {
    fn decode<D: Decoder<Ctx = C>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let len = crate::de::decode_slice_len(decoder)?;
        let mut s = CompactString::with_capacity(len);
        decoder.claim_container_read::<u8>(len)?;
        unsafe {
            let buf = &mut s.as_mut_bytes()[..len];
            decoder.reader().read(buf)?;
            if let Err(utf8_error) = std::str::from_utf8(buf) {
                return Err(DecodeError::Utf8 { inner: utf8_error });
            }
            s.set_len(len);
        }
        Ok(s)
    }
}

#[cfg(test)]
mod tests {
    use compact_str::CompactString;

    use crate::{config, decode_from_slice, encode_to_vec, error::DecodeError};

    #[test]
    fn decode_small_str() {
        let config = config::standard();
        let data = encode_to_vec("aaa", config).unwrap();

        let s: CompactString = decode_from_slice(&data, config).unwrap().0;
        assert_eq!(s, "aaa")
    }
    #[test]
    fn decode_small_invalid() {
        let config = config::standard();

        let result: Result<(CompactString, usize), DecodeError> =
            decode_from_slice(&[3, 0xFF, 0xFF, 0xFF], config);
        assert!(matches!(result, Err(DecodeError::Utf8 { .. })))
    }
    #[test]
    fn decode_large_str() {
        let config = config::standard();
        let data = encode_to_vec("123456789123456789123456789123456789", config).unwrap();

        let s: CompactString = decode_from_slice(&data, config).unwrap().0;
        assert_eq!(s, "123456789123456789123456789123456789")
    }

    #[test]
    fn decode_large_invalid() {
        let config = config::standard();

        let mut data = [0xFFu8; 40];
        data[0] = (data.len() - 1) as u8;

        let result: Result<(CompactString, usize), DecodeError> =
            decode_from_slice(data.as_slice(), config);
        assert!(matches!(result, Err(DecodeError::Utf8 { .. })))
    }
}

crate::impl_borrow_decode!(CompactString);
