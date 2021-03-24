use crate::transmittable::{Deserializable, Error, Serializable, Transmittable};
use std::convert::TryInto;
use uuid::Uuid;

impl Serializable for Uuid {
    fn serialize_append(&self, mut v: Vec<u8>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        v.extend(self.as_bytes().iter().rev());
        Ok(v)
    }
}

impl Deserializable for Uuid {
    fn deserialize(data: &[u8]) -> Result<(Self, usize), Box<dyn std::error::Error>> {
        if data.len() >= 16 {
            let mut data: [u8; 16] = data[0..16].try_into()?;
            data.reverse(); // gnutella protocol requires uuid in little endian
            Ok((Uuid::from_bytes(data), 16))
        } else {
            Err(Box::new(Error::DeserializationFailed {
                reason: format!(
                    "16 bytes input data required for constructing uuid::Uuid.\n\
                 Input array should have length 16 but found data.len() = {}\n\
                 where data = {:?}",
                    data.len(),
                    data
                ),
            }))
        }
    }
}

impl Transmittable for Uuid {}

#[cfg(test)]
mod tests {
    use super::{Deserializable, Serializable};
    use uuid::Uuid;

    #[test]
    fn test_uuid_serialize() {
        let uuid = Uuid::from_bytes([1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

        match uuid.serialize() {
            Ok(bytes) => assert_eq!(bytes, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]),
            Err(err) => panic!("{}", err),
        };

        let random_uuid = Uuid::new_v4();

        match random_uuid.serialize() {
            Ok(bytes) => {
                assert_eq!(
                    bytes.iter().collect::<Vec<&u8>>(),
                    random_uuid.as_bytes().iter().rev().collect::<Vec<&u8>>()
                );
            }
            Err(err) => panic!("{}", err),
        }
    }

    #[test]
    fn test_uuid_deserialize() {
        let uuid_bytes_little_endian: uuid::Bytes =
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];

        let (uuid, bytes_parsed) =
            match <Uuid as Deserializable>::deserialize(&uuid_bytes_little_endian) {
                Ok(tuple) => tuple,
                Err(err) => panic!("{}", err),
            };

        assert_eq!(
            uuid,
            Uuid::from_bytes([1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
        );
        assert_eq!(bytes_parsed, 16);
    }

    #[test]
    fn test_uuid_transmittable() {
        let uuid = Uuid::new_v4();

        let serialized_uuid = match uuid.serialize() {
            Ok(bytes) => bytes,
            Err(err) => panic!("{}", err),
        };

        let new_uuid = match <Uuid as Deserializable>::deserialize(&serialized_uuid) {
            Ok((uuid, bytes_parsed)) => {
                assert_eq!(bytes_parsed, 16);
                uuid
            }
            Err(err) => panic!("{}", err),
        };

        assert_eq!(uuid, new_uuid);
    }
}
