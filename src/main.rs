use std::error::Error;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    match run_server() {
        Ok(()) => {
            println!("Stopping server");
        }
        Err(_e) => {
            println!("Error running server");
        }
    }
}

fn run_server() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream)?;
            }
            Err(_e) => {
                println!("Connection failed");
            }
        }
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0; 512];
    stream.read(&mut buffer)?;
    let get = b"GET / HTTP/1.1\r\n";
    if buffer.starts_with(get) {
        let contents = fs::read_to_string("html/hello.html")?;
        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
        stream.write(response.as_bytes())?;
    } else {
        let response = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        stream.write(response.as_bytes())?;
    }
    stream.flush()?;
    Ok(())
}
