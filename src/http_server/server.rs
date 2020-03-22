use rscam;
use std::fs;
use std::io::prelude::*;
use std::net;

use super::{
    log::debug,
    result::Result,
};

pub fn run() -> Result<()> {
    let listener = net::TcpListener::bind("127.0.0.1:8080")?;
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream)?;
            }
            Err(_e) => {
                debug("Connection failed");
            }
        }
    }
    Ok(())
}

fn handle_connection(mut stream: net::TcpStream) -> Result<()> {
    let mut buffer = [0; 128];
    stream.read(&mut buffer)?;
    if request_is(&buffer, "GET", "/") {
        get_root(&mut stream)?;
    } else if request_is(&buffer, "GET", "/index") {
        get_root(&mut stream)?;
    } else if request_is(&buffer, "GET", "/index.html") {
        get_root(&mut stream)?;
    } else if request_is(&buffer, "GET", "/info") {
        get_info(&mut stream)?;
    } else if request_is(&buffer, "GET", "/image.jpg") {
        get_image(&mut stream)?;
    } else if request_is(&buffer, "POST", "/info") {
        post_info(&mut stream)?;
    } else if request_is(&buffer, "GET", "/favicon.ico") {
        get_favicon(&mut stream)?;
    } else {
        write_not_found(&mut stream)?;
    }
    Ok(())
}

fn get_image(stream: &mut net::TcpStream) -> Result<()> {
    debug("GET /image.jpg");

    let mut camera = rscam::new("/dev/video0")?;

    camera.set_control(rscam::CID_BRIGHTNESS, &100)?;
    camera.start(&rscam::Config {
        interval: (1, 10),
        resolution: (640, 480),
        format: b"MJPG",
        ..Default::default()
    })?;

    let frame = camera.capture()?;
    let mut file = fs::File::create(&format!("image.jpg"))?;
    file.write_all(&frame[..])?;
    write_ok_bytes(stream, "image/jpg", &frame)
}

fn get_favicon(stream: &mut net::TcpStream) -> Result<()> {
    debug("GET /favicon.ico");

    let path = "img/favicon.ico";
    let mut buffer = [0; 32038]; //TODO dynamically size
    let mut f = fs::File::open(path)?;
    f.read(&mut buffer)?;
    write_ok_bytes(stream, "image/vnd.microsoft.icon", &buffer)
}

fn get_root(stream: &mut net::TcpStream) -> Result<()> {
    debug("GET /");
    let index = fs::read_to_string("html/index.html")?;
    write_ok(stream, "text/html", &index)
}

fn get_info(stream: &mut net::TcpStream) -> Result<()> {
    debug("GET /info");
    let data = 10;
    write_ok(stream, "text/plain", &data.to_string())
}

fn post_info(stream: &mut net::TcpStream) -> Result<()> {
    debug("POST /info");
    write_ok(stream, "text/plain", "")
}

fn write_not_found(stream: &mut net::TcpStream) -> Result<()> {
    debug("HTTP 404");
    let header = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
    stream.write(header.as_bytes())?;
    stream.flush()?;
    Ok(())
}

fn write_ok(stream: &mut net::TcpStream, content_type: &str, data: &str) -> Result<()> {
    write_ok_bytes(stream, content_type, &data.as_bytes())
}

fn write_ok_bytes(stream: &mut net::TcpStream, content_type: &str, data: &[u8]) -> Result<()> {
    let header = format!("HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n", content_type, data.len());
    stream.write(header.as_bytes())?;
    stream.write(&data)?;
    stream.flush()?;
    Ok(())
}

fn request_is(request: &[u8], verb: &str, uri: &str) -> bool {
    let expected_str = format!("{} {} HTTP/1.1\r\n", verb, uri);
    let expected = expected_str.as_bytes();
    request.starts_with(expected)
}
