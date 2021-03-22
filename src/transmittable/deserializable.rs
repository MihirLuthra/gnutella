pub trait Deserializable {
    /// Input data can be variable length. Implementer has to verify the length.
    /// If the bytes are not sufficient to construct the object, error should be returned.
    /// Otherwise if the bytes >= required number of bytes, the object should be constructed
    /// with initial bytes that are required and upon success deserialized object should bytes
    /// returned along with number of bytes parsed.
    fn deserialize(data: &[u8]) -> Result<(Self, usize), Box<dyn std::error::Error>>
    where
        Self: Sized;
}
