use super::{Deserializable, Error, Serializable, Transmittable};
use std::{convert::TryInto, mem::size_of, net::Ipv4Addr};

macro_rules! impl_integer_serializable {
    ($($ty: ty),*) => {
        $(
            /// All integer types are serialized to little endian.
            /// Gnutella specification asks everything to be little ending
            /// unless specified explicitly.
            impl Serializable for $ty {
                fn serialize(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>>  {
                    Ok(self.to_le_bytes().into())
                }
            }
        )*
    };
}

macro_rules! impl_integer_deserializable {
    ($($ty: ty),*) => {
        $(
            /// All integer types are serialized to little endian.
            /// Gnutella specification asks everything to be little ending
            /// unless specified explicitly.
            impl Deserializable for $ty {
                fn deserialize(data: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
                    if data.len() == size_of::<$ty>() {
                        let data: [u8; size_of::<$ty>()] = data.try_into()?;
                        Ok(<$ty>::from_le_bytes(data))
                    } else {
                        Err(Box::new(
                            Error::DeserializationFailed(
                                format!(
                                    "{} bytes input data is required for constructing {}.\n\
                                     Input array should have length {0} but found data.len() = {}\n\
                                     where data = {:?}", size_of::<$ty>(), stringify!($ty), data.len(), data
                                )
                            )
                        ))

                    }
                }
            }
        )*
    };
}

macro_rules! impl_integer_transmittable {
    ($($ty: ty),*) => {
        $( impl Transmittable for $ty {} )*
    };
}

impl_integer_serializable!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128);
impl_integer_deserializable!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128);
impl_integer_transmittable!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128);

impl Serializable for Ipv4Addr {
    fn serialize(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        Ok(self.octets().into())
    }
}

impl Deserializable for Ipv4Addr {
    fn deserialize(data: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        if data.len() == 4 {
            Ok(Ipv4Addr::new(data[0], data[1], data[2], data[3]))
        } else {
            Err(Box::new(Error::DeserializationFailed(format!(
                "4 bytes input data required for constructing Ipv4Addr.\n\
                         Input array should have length 4 but found data.len() = {}\n\
                         where data = {:?}",
                data.len(),
                data
            ))))
        }
    }
}

impl Transmittable for Ipv4Addr {}
