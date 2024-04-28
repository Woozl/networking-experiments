use std::{io::Read, net::{Shutdown, TcpStream, ToSocketAddrs}, time::{SystemTime, UNIX_EPOCH}};

const EPOCH_DELTA: u32 = 2_208_988_800; // num seconds between 1900 -> 1970 (Unix epoch)

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = "time.nist.gov:37".to_socket_addrs()?.next().expect("Couldn't get socket");

    let mut stream = TcpStream::connect(socket)?;
    println!("Connected to {:?}", stream.peer_addr()?);

    let mut buf: [u8; 4] = [0; 4];
    stream.read(&mut buf)?;
    stream.shutdown(Shutdown::Both)?;

    println!("NIST time: {}", u32::from_be_bytes(buf) - EPOCH_DELTA);
    println!("System time: {}", SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs());

    Ok(())
}

