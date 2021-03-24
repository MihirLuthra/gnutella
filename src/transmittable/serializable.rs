use std::mem::size_of;

pub trait Serializable {
    /// This is to be defined by the implementer such that
    /// the serialized version of self extends the argument vector `v`.
    /// In general, the the vector `v` received in the arguments such be returned.
    fn serialize_append(&self, v: Vec<u8>) -> Result<Vec<u8>, Box<dyn std::error::Error>>;

    fn serialize(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>>
    where
        Self: Sized,
    {
        // Generally the needed vector capacity should be equal to
        // the number of bytes
        let v = Vec::<u8>::with_capacity(size_of::<Self>());
        self.serialize_append(v)
    }
}
