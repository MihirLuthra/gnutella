use std::{thread};

use gnutella::transmittable::{Serializable, Deserializable, Transmittable};
use gnutella_transmittable_derive::Transmittable;
use std::net::Ipv4Addr;

#[derive(Debug, Transmittable)]
struct Abc<T> {
    a: T,
    z: Ipv4Addr,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let x = Ipv4Addr::new(1, 2, 3, 4);

    let iabc = Abc {a: 32, z: Ipv4Addr::new(1, 2, 3, 4)};

    let f = iabc.serialize()?;

    println!("f = {:?}", f);

    let (q, bytes) = Abc::<i32>::deserialize(&f).unwrap();

    println!("iabc = {:#?}", iabc);
    println!("q = {:#?}, bytes = {}", q, bytes);

    
    println!("x = {}", x);

    let y = x.serialize().unwrap();

    println!("y = {:?}", y);

    let (z, bytes) = Ipv4Addr::deserialize(&y).unwrap();

    println!("z = {:?}, bytes = {}", z, bytes);


    match u32::deserialize(&42u128.serialize().unwrap()) {
        Ok((val, bytes)) => println!("val = {}, bytes = {}", val, bytes),
        Err(e) => println!("err = {}", e),
    }
    
    //start_server()?;

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
