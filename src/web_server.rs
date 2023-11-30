use std::{net::{TcpListener, TcpStream}, fs, io::Write};
use chrono::{self, Local};
use crate::threadpool;

pub fn host_server<F>(port: String, handler: F) 
    where F: Fn(TcpStream) + Send + 'static + std::marker::Sync + Clone
{
    let listener = TcpListener::bind(format!["0.0.0.0:{}", port]).unwrap();
    let threadpool = threadpool::ThreadPool::new(4);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let x = handler.clone();
                threadpool.execute(move || x(stream));
            }
            Err(_) => {
                print_time(format!("Warn: Failed to connect."));
            }
        };
    }
}
pub fn print_time(text: String) {
    let since_the_epoch = Local::now()
    .format("%Y-%m-%d][%H:%M:%S");
println!("[{}]: {}", since_the_epoch, text);
}
pub fn send_file(path: &str, stream: &mut TcpStream) {
    let status_line = "HTTP/1.1 200 OK";
    let contents = match fs::read(path) {
        Ok(value) => value,
        Err(_) => {
            print_time(format!("Error: Failed to read/find requested file"));
            return;
        },
    };
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n");
    let _ = stream.write_all(response.as_bytes());
    let _ = stream.write_all(&contents);
}