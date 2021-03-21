pub trait Serializable {
    fn serialize(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>>;
}
