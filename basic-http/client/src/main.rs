use std::{
    env,
    io::{Read, Write},
    net::{TcpStream, ToSocketAddrs},
};

// cargo r <host> <optional port>
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let port = args
        .get(2)
        .unwrap_or(&String::from("80"))
        .parse::<u16>()
        .unwrap_or(80);

    let url = args.get(1).expect("Please provide url!");

    let socket = (&url[..], port).to_socket_addrs().unwrap().next().unwrap();

    let mut stream = TcpStream::connect(socket)?;

    let payload = format!(
        "\
        GET / HTTP/1.1\r\n\
        Host: {}\r\n\
        Connection: close\r\n\r\n\
    ",
        url
    );

    stream.write_all(payload.as_bytes())?;
    println!("Request:\n{}", payload);

    let mut buf = String::from("");
    stream.read_to_string(&mut buf)?;

    println!("Response:\n{}", buf);

    Ok(())
}
