use chrono::Local;
use std::fs;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
mod threadpool;
struct rule {
    url: &'static str,
    path: &'static str,
}
fn main() {
    let rules = vec![
        rule {
            url: "/mossy",
            path: "/src/webpage/mossy/index.html",
        },
        rule {
            url: "/mossy/Mossy.jpeg",
            path: "/src/webpage/mossy/assets/Mossy.jpg"
        }
    ];
    let port = "80".to_string();
    host_server(port);
    loop {}
}
fn host_server(port: String) {
    let listener = TcpListener::bind(format!["0.0.0.0:{}", port]).unwrap();
    let threadpool = threadpool::ThreadPool::new(4);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                print_time();
                println!("Info: Worker dispatched to serve port {}", port);
                threadpool.execute(|| handle_connection(stream));
            }
            Err(_) => {
                print_time();
                println!("Warn: Failed to connect.")
            }
        };
    }
}
fn handle_connection(mut stream: TcpStream) {
    let ip = stream.peer_addr().expect("Failed to read peer ip").to_string();
    let buf_reader = BufReader::new(&mut stream);
    let request = match buf_reader.lines().next() {
        Some(value) => match value {
            Ok(request) => request,
            Err(_) => {
                print_time();
                println!("Error: Encountered problem while reading request from {}", ip);
                return;
            },
        },
        None => {
            print_time();
            println!("Error: No request supplied by {}", ip);
            return;
        },
    };
    print_time();
    println!("Info: Serving request \"{}\" for \"{}\"", request, ip);
    if request == "GET / HTTP/1.1" {
        send_text("src\\webpage\\index.htm", &mut stream)
    } else if request == "GET /favicon.ico HTTP/1.1" {
        send_img("src\\webpage\\assets\\Mossy.jpg", &mut stream)
    } else if request == "GET /style.css HTTP/1.1" {
        send_text("src\\webpage\\mossy\\assets\\style.css", &mut stream)
    } else if request == "GET /Mossy.jpeg HTTP/1.1" {
        send_img("src\\webpage\\assets\\Mossy.jpg", &mut stream)
    } else if request == "GET /Wikipedia.png HTTP/1.1" {
        send_img("src\\webpage\\assets\\Wikipedia.png", &mut stream)
    }
}
fn send_text(path: &str, stream: &mut TcpStream) {
    let status_line = "HTTP/1.1 200 OK";
    let contents = match fs::read_to_string(path) {
        Ok(value) => value,
        Err(_) => {
            print_time();
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
            print_time();
            println!("Error: Failed to read/find requested file");
            return;
        },
    };
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n");
    let _ = stream.write_all(response.as_bytes());
    let _ = stream.write_all(&contents);
}
fn print_time() {
    let since_the_epoch = Local::now()
    .format("%Y-%m-%d][%H:%M:%S");
    print!("[{}]: ", since_the_epoch);
}
