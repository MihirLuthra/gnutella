use std::{thread};

use gnutella::{Serializable, Deserializable};
use std::net::Ipv4Addr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let x = Ipv4Addr::new(1, 2, 3, 4);
    
    println!("x = {}", x);

    let y = x.serialize().unwrap();

    println!("y = {:?}", y);

    let z = Ipv4Addr::deserialize(&y).unwrap();

    println!("z = {:?}", z);


    match i32::deserialize(&42u128.serialize().unwrap()) {
        Ok(val) => println!("val = {}", val),
        Err(e) => println!("err = {}", e),
    }
    
    //start_server()?;

    Ok(())
}

fn start_server() -> Result<(), Box<dyn std::error::Error>> {
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
