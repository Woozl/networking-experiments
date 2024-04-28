use std::{env::current_dir, fs, net::Ipv4Addr};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr_file = fs::read_to_string(current_dir()?.join("tcp_data/tcp_addrs_0.txt"))?;
    let data_file = fs::read(current_dir()?.join("tcp_data/tcp_data_0.dat"))?;

    let [source, dest] = *addr_file
        .split_ascii_whitespace()
        .map(|ip_str| ip_str.parse::<Ipv4Addr>().unwrap().octets())
        .collect::<Vec<_>>()
        .as_slice() else { panic!() };

    // println!("Source: {source:?}, Dest: {dest:?}");
    // println!("{data_file:?}");

    let mut ip_pseudo_header: Vec<u8> = vec![];
    ip_pseudo_header.extend_from_slice(&source);
    ip_pseudo_header.extend_from_slice(&dest);
    ip_pseudo_header.push(0);
    ip_pseudo_header.push(0x06);
    ip_pseudo_header.extend_from_slice(&(data_file.len() as u16).to_be_bytes());

    // println!("Psuedo header: {:?}", ip_pseudo_header);

    let checksum = u16::from_be_bytes([*data_file.get(16).unwrap(), *data_file.get(17).unwrap()]);

    println!("Checksum: {checksum}");

    Ok(())
}


