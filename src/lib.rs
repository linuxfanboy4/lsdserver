use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;

pub fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer);
    let path = request.split_whitespace().nth(1).unwrap_or("/index.html");
    let file_path = if path == "/" { "index.html" } else { path.trim_start_matches("/") };
    let path = Path::new(file_path);

    let extension = path.extension().unwrap_or_default().to_str().unwrap_or_default();
    let content_type = match extension {
        "html" => "text/html",
        "css" => "text/css",
        "js" => "application/javascript",
        _ => "application/octet-stream",
    };

    if path.exists() {
        let mut file = File::open(path).unwrap();
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).unwrap();
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: {}; charset=utf-8\r\n\r\n",
            content_type
        );
        stream.write_all(response.as_bytes()).unwrap();
        stream.write_all(&contents).unwrap();
    } else {
        let response = "HTTP/1.1 404 NOT FOUND\r\n\r\nFile not found.";
        stream.write_all(response.as_bytes()).unwrap();
    }
}

pub fn start_server(addr: &str) {
    let listener = TcpListener::bind(addr).unwrap();
    println!("Server started at http://{}", addr);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(_) => {
                eprintln!("Failed to handle connection");
            }
        }
    }
                         }
