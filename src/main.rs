use std::fs;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
mod threadpool;
fn main() {
    let ip_and_port = "192.168.1.93:7878".to_string();
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
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    if request_line == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("src/html_files/index.html").unwrap();
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        let _ = stream.write_all(response.as_bytes());
    } else if request_line == "GET /ip HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = format![
            "<!DOCTYPE html>
        <head>
            <title>IP display</title>
        </head>
        <body>
            <h1>Hey, your ip is: \"{}\"</h1>
            <p>This is a website written in rust.</p>
        </body>", stream.peer_addr().expect("Failed, sorry")];
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        let _ = stream.write_all(response.as_bytes());
    }
}
