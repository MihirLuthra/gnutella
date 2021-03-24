use gnutella::{
    transmittable::{Deserializable, Serializable, Transmittable},
    Transmittable,
};
use std::net::Ipv4Addr;

#[derive(Debug, PartialEq, Transmittable)]
struct Test {
    a: u32,
    b: Ipv4Addr,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let instance = Test {
        a: 4,
        b: Ipv4Addr::new(1, 2, 3, 4),
    };

    let serialized_instance = instance.serialize()?;

    assert_eq!(serialized_instance, [4, 0, 0, 0, 1, 2, 3, 4]);

    let (deserialized_instance, bytes_parsed) =
        <Test as Deserializable>::deserialize(&serialized_instance)?;

    assert_eq!(bytes_parsed, 8);

    assert_eq!(instance, deserialized_instance);

    Ok(())
}
