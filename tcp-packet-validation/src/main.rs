use std::{env::current_dir, fs, net::Ipv4Addr};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for num in 0..=9 {
        check_packet(num)?;
    }
    Ok(())
}

fn check_packet(num: usize) -> Result<(), Box<dyn std::error::Error>> {
    let addr_file = fs::read_to_string(current_dir()?.join(format!("tcp_data/tcp_addrs_{num}.txt")))?;
    let seg = fs::read(current_dir()?.join(format!("tcp_data/tcp_data_{num}.dat")))?;

    let [source, dest] = *addr_file
        .split_ascii_whitespace()
        .map(|ip_str| ip_str.parse::<Ipv4Addr>().unwrap().octets())
        .collect::<Vec<_>>()
        .as_slice() else { panic!() };

    let mut ip_pseudo_header: Vec<u8> = vec![];
    ip_pseudo_header.extend_from_slice(&source);
    ip_pseudo_header.extend_from_slice(&dest);
    ip_pseudo_header.push(0);
    ip_pseudo_header.push(0x06);
    ip_pseudo_header.extend_from_slice(&(seg.len() as u16).to_be_bytes());

    // pull out checksum value from byte offset 16-17
    let checksum = u16::from_be_bytes([seg[16], seg[17]]);

    // duplicate tcp segment with checksum set to 0
    let mut seg_zero_cksum = seg.clone();
    seg_zero_cksum[16] = 0x0;
    seg_zero_cksum[17] = 0x0;

    // pad to an even number of bytes for addition
    if seg_zero_cksum.len() % 2 == 1 { seg_zero_cksum.push(0x0); }

    // compute checksum using psuedo header + tcp header/data segment concat'd
    // sum each 16 bit (2 byte) word using 1s compliment addition (wrap around overflow)
    let mut combined = ip_pseudo_header.clone();
    combined.extend(&seg_zero_cksum);
    let computed_checksum = !(combined
        .chunks(2)
        .fold(0u32, |sum, word| {
            let s = sum + u16::from_be_bytes([word[0], word[1]]) as u32;
            (s & 0xffff) + (s >> 16)
        }) as u16);

    // if computed checksum matches checksum from header, packet is correct:
    println!("Test {num}: {}", if checksum == computed_checksum { "PASS" } else { "FAIL" });

    Ok(())
} 


