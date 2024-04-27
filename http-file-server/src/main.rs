use core::str;
use std::{
    fs, io::{Read, Write}, net::{Shutdown, SocketAddr, TcpListener, TcpStream}
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = SocketAddr::from(([127, 0, 0, 1], 12345));
    let listener = TcpListener::bind(socket)?;

    loop {
        match listener.accept() {
            Ok((stream, addr)) => {
                println!("New TCP connection initiated from client address: {addr:?}");
                handle_connection(stream)?;
            }
            Err(e) => {
                eprintln!("Error getting client {e:?}");
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf: [u8; 128] = [0; 128];
    let mut request = String::from("");

    loop {
        match stream.read(&mut buf) {
            Ok(num_bytes) => {
                if num_bytes == 0 {
                    return Ok(());
                } // client closed connection

                request.push_str(str::from_utf8(&buf)?);
                buf = [0; 128];

                if request.contains("\r\n\r\n") {
                    let path = request
                        .lines()
                        .next()
                        .unwrap()
                        .split_ascii_whitespace()
                        .nth(1)
                        .unwrap();
                    println!("Requesting path: {path}");

                    let response = build_response(path);
                    println!("Sending file!");

                    stream.write_all(response.as_slice())?;
                    stream.shutdown(Shutdown::Both)?;
                    break;
                }
            }
            Err(e) => {
                eprintln!("Issue reading from stream: {e:?}");
                stream.shutdown(Shutdown::Both)?;
            }
        }
    }

    Ok(())
}

const NOT_FOUND_RESPONSE: &str = "\
    HTTP/1.1 404 Not Found\r\n\
    Content-Type: text/plain\r\n\
    Content-Length: 13\r\n\
    Connection: close\r\n\r\n\
    404 not found\r\n
";

fn build_response(path: &str) -> Vec<u8> {
    // for this simple test only support files served from the root level of `/serve`
    let route = path.split("/").last().unwrap();
    let fs_path = std::env::current_dir().unwrap().join("src/serve").join(route);

    if fs_path.is_dir() {
        return server_err_response("Requested directory, only files are supported");
    }

    match fs::read(&fs_path) {
        Ok(mut file_contents) => {
            let file_length = file_contents.len();
            let mime_type = match fs_path.extension() {
                Some(extension) => match extension.to_str().unwrap() {
                    "html" => "text/html",
                    "txt" => "text/plain",
                    "jpg" | "jpeg" => "image/jpeg",
                    _ => { return server_err_response("Only .html, .jpg/.jpeg, and .txt files supported"); }
                },
                None => { return server_err_response("Requested routes must contain an extension"); }
            };

            let mut response: Vec<u8> = format!("\
                HTTP/1.1 200 OK\r\n\
                Content-Type: {mime_type}\r\n\
                Content-Length: {file_length}\r\n\
                Connection: close\r\n\r\n").bytes().collect();

            response.append(&mut file_contents);

            return response;
        }
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => {
                eprintln!("Could not find file {:?}, returning 404", &fs_path);
                return NOT_FOUND_RESPONSE.bytes().collect();
            },
            error => { panic!("Errored reading {:?}: {error:?}", &fs_path); }
        }
    }
}

fn server_err_response(message: &str) -> Vec<u8> {
    return format!("\
        HTTP/1.1 500 Internal Server Error\r\n\
        Content-Type: text/plain\r\n\
        Content-Length: {}\r\n\
        Connection: close\r\n\r\n\
        {}\r\n
    ", message.len(), message).bytes().collect();
}
