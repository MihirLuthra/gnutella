use crate::transmittable::{Deserializable, Error, Serializable, Transmittable};
use std::{convert::TryInto, mem::size_of};

macro_rules! impl_integer_transmittable {
    ($($ty: ty),*) => {$(

        /// All integer types are serialized to little endian.
        /// Gnutella specification asks everything to be little ending
        /// unless specified explicitly.
        impl Serializable for $ty {
            fn serialize_append(&self, mut v: Vec<u8>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
                v.extend(&self.to_le_bytes());
                Ok(v)
            }
        }

        /// All integer types are serialized to little endian.
        /// Gnutella specification asks everything to be little ending
        /// unless specified explicitly.
        impl Deserializable for $ty {
            fn deserialize(data: &[u8]) -> Result<(Self, usize), Box<dyn std::error::Error>> {
                if data.len() >= size_of::<$ty>() {
                    let data: [u8; size_of::<$ty>()] = data[0..size_of::<$ty>()].try_into()?;
                    Ok((<$ty>::from_le_bytes(data), data.len()))
                } else {
                    Err(Box::new(Error::DeserializationFailed {
                         reason: format!(
                            "{} bytes input data is required for constructing {}.\n\
                             Input array should have length {0} but found data.len() = {}\n\
                             where data = {:?}", size_of::<$ty>(), stringify!($ty), data.len(), data)
                    }))
                }
            }
        }

        impl Transmittable for $ty {}
    )*};
}

impl_integer_transmittable!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128);

#[cfg(test)]
mod tests {
    use super::{Deserializable, Serializable};

    #[test]
    fn test_integer_transmittable() {
        let x = 12345_i64;

        let x_serialized = match x.serialize() {
            Ok(bytes) => bytes,
            Err(err) => panic!("{}", err),
        };

        let x_deserialized = match <i64 as Deserializable>::deserialize(&x_serialized) {
            Ok((x, bytes_parsed)) => {
                assert_eq!(bytes_parsed, std::mem::size_of::<i64>());
                x
            }
            Err(err) => panic!("{}", err),
        };

        assert_eq!(x_deserialized, x);
    }
}
