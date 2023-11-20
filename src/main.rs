use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, prelude::*};
use std::fs;
mod threadpool;
fn main() {
    let threadpool = threadpool::ThreadPool::new(1);
    let ip_and_port = "127.0.0.1:7878".to_string();
    threadpool.execute(||host_server(ip_and_port));
    loop{}
}
fn host_server(ip_and_port: String) {
    let listener = TcpListener::bind(ip_and_port).unwrap();
    let threadpool = threadpool::ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
       threadpool.execute(||handle_connection(stream)); 
    }

}
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    if request_line == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("src/html_files/index.html").unwrap();
        let length = contents.len();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );

        stream.write_all(response.as_bytes()).unwrap();
    } else {
        // some other request
    }
}