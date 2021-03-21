use std::{error::Error, thread};
mod core;

use crate::core::transmittable::{Serializable, Deserializable, Transmittable};
use std::net::Ipv4Addr;

fn main() -> Result<(), Box<dyn Error>> {
    
    let x = Ipv4Addr::new(1, 2, 3, 4);
    
    println!("x = {}", x);

    let y = x.serialize().unwrap();

    println!("y = {:?}", y);

    let z = Ipv4Addr::deserialize(&y).unwrap();

    println!("z = {:?}", z);

    match u32::deserialize(&[4, 0, 0, 0, 0]) {
        Ok(val) => println!("val = {}", val),
        Err(e) => println!("err = {}", e),
    }
    
    //start_server()?;

    Ok(())
}

fn start_server() -> Result<(), Box<dyn Error>> {
    // Start listening to other nodes for
    // Ping, Pong, Query, QueryHit and Push
    let server_thread_join_handle = thread::Builder::new()
        .name("Server thread".to_string())
        .spawn(|| {
           // gnutella::start_server();
        })?;

    server_thread_join_handle.join().unwrap();

    Ok(())
}
