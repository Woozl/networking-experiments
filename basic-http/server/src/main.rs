use std::{
    env, io::{Read, Write}, net::{Shutdown, TcpListener, TcpStream}, str
};

const PAYLOAD: &str = "\
    HTTP/1.1 200 OK\r\n\
    Content-Type: text/plain\r\n\
    Content-Length: 6\r\n\
    Connection: close\r\n\r\n\
    Hello!
";

// cargo r <optional port>
// defaults to port 12345
// run the server and send an http request to localhost:12345 (or your specified port)
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = env::args()
        .collect::<Vec<_>>()
        .get(1)
        .unwrap_or(&String::from("12345"))
        .parse::<u16>()
        .unwrap_or(12345);

    let listener = TcpListener::bind(format!("localhost:{port}"))?;
    println!("Listening on port {}", listener.local_addr()?.port());

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Incoming client request from {:?}:", stream.peer_addr().unwrap());
                handle_connection(stream)
            }
            Err(e) => {
                eprint!("Error in connection: {e:?}");
            }
        }
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf: [u8; 16] = [0; 16];
    let mut message: String = String::from("");

    loop {
        match stream.read(&mut buf) {
           Err(e) => {
                eprintln!("Error reading from stream! {e:?}");
                stream.shutdown(Shutdown::Both).unwrap();
            },
           Ok(num_bytes) => {
               if num_bytes == 0 {
                   break;
                }

                // push utf8 text to a growable type, we can't directly check
                // the buffer for a blank line directly since it could be split
                // between two blocks of data
                message.push_str(str::from_utf8(&buf).unwrap());
                
                // Blank line indicates end of HTTP header block
                if message.contains("\r\n\r\n") {
                    print!("{message}");

                    // respond with payload
                    stream.write_all(PAYLOAD.as_bytes()).unwrap();
                    stream.shutdown(Shutdown::Both).unwrap();
                    return;
                }

                // reset buffer for next chunk
                buf = [0; 16];
           } 
        };
    }
}
