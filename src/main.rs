extern crate byteorder;

use byteorder::{BigEndian, WriteBytesExt};
use std::env;
use std::net::UdpSocket;

const MAGIC: u32 = 0xae4fa983;

fn main() -> std::io::Result<()> {

    let args: Vec<String> = env::args().collect();

    let on_off = if args.len() == 1 {
        2
    } else if args.len() == 2 {

        match args[1].as_str() {
            "on" => 1,
            "off" => 0,
            "toggle" => 2,
            _ => {
                println!("Invalid argument: {}", args[1]);
                println!("Usage: {} on/off", args[0]);
                return Ok(());
            },
        }
    } else {
        println!("Usage: {} on/off/toggle", args[0]);
        return Ok(());
    };

    let socket = UdpSocket::bind("0.0.0.0:0")?;
    
    let mut wtr = Vec::new();

    wtr.write_u32::<BigEndian>(MAGIC)?;
    wtr.write_u8(1)?;
    wtr.write_u8(on_off)?;
    
    socket.send_to(&wtr, "192.168.10.109:6319")?;

    Ok(())
}
