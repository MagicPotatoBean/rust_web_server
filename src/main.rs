use chrono::Local;
use colored::Colorize;
use std::fs;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
mod threadpool;
fn main() {
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
                threadpool.execute(|| handle_connection(stream));
            }
            Err(_) => {
                print_time(format!("Warn: Failed to connect."));
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
                print_time(format!("{}: Encountered problem while reading request from {}", "Error".red() , ip));
                return;
            },
        },
        None => {
            print_time(format!("{}: No request supplied by {}", "Error".red(), ip));
            return;
        },
    };

    print_time(format!("Info: Serving {} request \"{}\"", ip, request));
    if        request == "GET / HTTP/1.1" {
        send_text("src\\webpage\\main\\index.html", &mut stream)
    } else if request == "GET /favicon.ico HTTP/1.1" {
        send_img("src\\webpage\\main\\assets\\favicon.ico", &mut stream)
    } else if request == "GET /mossy HTTP/1.1" {
        send_text("src\\webpage\\mossy\\index.html", &mut stream)
    } else if request == "GET /mossy/style.css HTTP/1.1" {
        send_text("src\\webpage\\mossy\\assets\\style.css", &mut stream)
    } else if request == "GET /mossy/Mossy.jpeg HTTP/1.1" {
        send_img("src\\webpage\\mossy\\assets\\Mossy.jpg", &mut stream)
    } else if request == "GET /mossy/Wikipedia.png HTTP/1.1" {
        send_img("src\\webpage\\mossy\\assets\\Wikipedia.png", &mut stream)
    } else {
        print_time(format!("{}: Unknown request \"{}\" from {}", "Warn".yellow(), request, ip));
        send_text("src\\webpage\\404.html", &mut stream)
    }
}
fn send_text(path: &str, stream: &mut TcpStream) {
    let status_line = "HTTP/1.1 200 OK";
    let contents = match fs::read_to_string(path) {
        Ok(value) => value,
        Err(_) => {
            print_time(format!("Error: Failed to read/find requested file"));
            
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
            print_time(format!("Error: Failed to read/find requested file"));
            return;
        },
    };
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n");
    let _ = stream.write_all(response.as_bytes());
    let _ = stream.write_all(&contents);
}
fn print_time(text: String) {
    let since_the_epoch = Local::now()
    .format("%Y-%m-%d][%H:%M:%S");
    println!("[{}]: {}", since_the_epoch, text);
}
