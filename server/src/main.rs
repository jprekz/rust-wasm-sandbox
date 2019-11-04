use std::fs::File;
use std::io::{copy, Read, Write};
use std::net::{TcpListener, TcpStream};

use std::path::Path;
use std::thread;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!(
        "Listening for connections on port {0} ( http://localhost:{0}/ )",
        8080
    );

    for stream in listener.incoming() {
        thread::spawn(|| handle_client(stream.unwrap()));
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buf = [0; 4096];

    stream.read(&mut buf).unwrap();
    let mut parsed_headers = [httparse::EMPTY_HEADER; 16];
    let mut req = httparse::Request::new(&mut parsed_headers);
    req.parse(&buf).unwrap();

    if let Some(mut path) = req.path {
        while path.starts_with("/") {
            path = &path[1..];
        }

        let mut file_path = Path::new("./static").join(path);
        if !file_path.is_file() {
            file_path.push("index.html");
        };

        let exists = file_path.exists();

        let status = if exists {
            "HTTP/1.1 200 OK\r\n"
        } else {
            "HTTP/1.1 404 Not Found\r\n"
        };

        let ext = file_path.extension().map(|ext| ext.to_str().unwrap());
        let header = match ext {
            Some("html") => "Content-Type: text/html\r\n\r\n",
            Some("js") => "Content-Type: application/javascript\r\n\r\n",
            Some("wasm") => "Content-Type: application/wasm\r\n\r\n",
            _ => "Content-Type: text/html\r\n\r\n",
        };

        let _ = stream.write_all(status.as_bytes());
        let _ = stream.write_all(header.as_bytes());

        if exists {
            let _ = copy(&mut File::open(file_path).unwrap(), &mut stream);
        } else {
            let _ = stream.write_all(path.as_bytes());
            let _ = stream.write_all("\r\n".as_bytes());
        }
    }
}
