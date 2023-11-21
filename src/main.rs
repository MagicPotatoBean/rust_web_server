use std::fs;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
mod threadpool;
fn main() {
    let ip_and_port = "0.0.0.0:80".to_string();
    host_server(ip_and_port);
    loop {}
}
fn host_server(ip_and_port: String) {
    let listener = TcpListener::bind(ip_and_port).unwrap();
    let threadpool = threadpool::ThreadPool::new(4);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Info: Worker dispatched.");
                threadpool.execute(|| handle_connection(stream));
            }
            Err(_) => {println!("Warn: Failed to connect.")}
        };
    }
}
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request = match buf_reader.lines().next() {
        Some(value) => match value {
            Ok(request) => request,
            Err(_) => {
                println!("Error: Encountered problem while reading request");
                return;
            },
        },
        None => {
            println!("Error: No request supplied");
            return;
        },
    };
    println!("{}", request);
    if request == "GET / HTTP/1.1" {
        send_text("src\\webpage\\index.html", &mut stream)
    } else if request == "GET /favicon.ico HTTP/1.1" {
        send_img("src\\webpage\\assets\\favicon.ico", &mut stream)
    }
}


fn send_text(path: &str, stream: &mut TcpStream) {
    let status_line = "HTTP/1.1 200 OK";
    let contents = match fs::read_to_string(path) {
        Ok(value) => value,
        Err(_) => {
            println!("Error: Failed to read/find requested file");
            return;
        },
    };
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    let _ = stream.write_all(response.as_bytes());
}
fn send_img(path: &str, stream: &mut TcpStream) {
    let status_line = "HTTP/1.1 200 OK";
    let contents = match fs::read(path) {
        Ok(value) => value,
        Err(_) => {
            println!("Error: Failed to read/find requested file");
            return;
        },
    };
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n");
    let _ = stream.write_all(response.as_bytes());
    let _ = stream.write_all(&contents);
}