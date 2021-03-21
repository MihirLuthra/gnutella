pub trait Deserializable {
    fn deserialize(data: &[u8]) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;
}
