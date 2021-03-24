use std::thread;

use gnutella::{
    transmittable::{Deserializable, Serializable, Transmittable},
    Transmittable,
};
use std::net::Ipv4Addr;
use uuid::Uuid;

#[derive(Debug, Transmittable)]
struct Test<T, U> {
    a: T,
    b: U,
    c: Ipv4Addr,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let test = Test {
        a: Uuid::new_v4(),
        b: 6_u32,
        c: Ipv4Addr::new(1, 2, 3, 4),
    };

    println!("test = {:#?}", test);
    println!("test.a.as_bytes() = {:?}", test.a.as_bytes());

    let serialized_test = test.serialize()?;

    println!("serialized_test = {:?}", serialized_test);

    let deserialized_test = Test::<Uuid, u32>::deserialize(&serialized_test)?;

    println!("deserialized_test = {:#?}", deserialized_test);

    Ok(())
}

// fn start_server() -> Result<(), Box<dyn std::error::Error>> {
//     // Start listening to other nodes for
//     // Ping, Pong, Query, QueryHit and Push
//     let server_thread_join_handle = thread::Builder::new()
//         .name("Server thread".to_string())
//         .spawn(|| {
//            // gnutella::start_server();
//         })?;

//     server_thread_join_handle.join().unwrap();

//     Ok(())
// }
