use crate::transmittable::{Deserializable, Error, Serializable, Transmittable};
use std::net::Ipv4Addr;

impl Serializable for Ipv4Addr {
    fn serialize_append(&self, mut v: Vec<u8>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        v.extend(&self.octets());
        Ok(v)
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
