use super::{Deserializable, Serializable};

pub trait Transmittable: Serializable + Deserializable {}
