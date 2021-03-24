use crate::transmittable::{Deserializable, Error, Serializable, Transmittable};
use std::net::Ipv4Addr;

impl Serializable for Ipv4Addr {
    fn serialize_append(&self, mut v: Vec<u8>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        v.extend(&self.octets());
        Ok(v)
    }
}

impl Deserializable for Ipv4Addr {
    fn deserialize(data: &[u8]) -> Result<(Self, usize), Box<dyn std::error::Error>> {
        if data.len() >= 4 {
            Ok((Ipv4Addr::new(data[0], data[1], data[2], data[3]), 4))
        } else {
            Err(Box::new(Error::DeserializationFailed {
                reason: format!(
                    "4 bytes input data required for constructing Ipv4Addr.\n\
                 Input array should have length 4 but found data.len() = {}\n\
                 where data = {:?}",
                    data.len(),
                    data
                ),
            }))
        }
    }
}

impl Transmittable for Ipv4Addr {}

#[cfg(test)]
mod tests {
    use super::{Deserializable, Serializable};
    use std::net::Ipv4Addr;

    #[test]
    fn test_ipv4_addr_transmittable() {
        let x = Ipv4Addr::new(32, 43, 11, 234);

        let x_serialized = match x.serialize() {
            Ok(bytes) => bytes,
            Err(err) => panic!("{}", err),
        };

        let x_deserialized = match <Ipv4Addr as Deserializable>::deserialize(&x_serialized) {
            Ok((x, bytes_parsed)) => {
                assert_eq!(bytes_parsed, 4);
                x
            }
            Err(err) => panic!("{}", err),
        };

        assert_eq!(x_deserialized, x);
    }
}
