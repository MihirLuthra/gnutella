use gnutella::{
    transmittable::{Deserializable, Serializable, Transmittable},
    Transmittable,
};

#[derive(Transmittable)]
struct Test {
    x: i32,
    y: String,
}

fn main() {
    let instance = Test {
        x: 3,
        y: "hello".into(),
    };
}
