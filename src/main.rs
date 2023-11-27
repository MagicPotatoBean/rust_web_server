use chrono::Local;
use colored::Colorize;
use std::fs;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
mod threadpool;
fn main() {
    let port = "80".to_string();
    host_server(port);
    loop {

    }
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
fn print_time(text: String) {
    let since_the_epoch = Local::now()
    .format("[%Y-%m-%d][%H:%M:%S]");
println!("{}: {}", since_the_epoch, text);
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
fn send_data(path: &str, stream: &mut TcpStream) {
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
fn handle_connection(mut stream: TcpStream) {
    let ip = stream.peer_addr().expect("Failed to read peer ip").to_string();
    let buf_reader = BufReader::new(&mut stream);

    let mut request_lines = buf_reader.lines();
    let header = match request_lines.next() {
        Some(value) => match value {
            Ok(request) => request,
            Err(_) => {
                print_time(format!("{}: Encountered problem while reading request from {}", "Error".red() , ip));
                return;
            },
        },
        None => {
            print_time(format!("{}: \"{}\" supplied no request", "Error".red(), ip));
            return;
        },
    };
    let body: Vec<_> = request_lines.map(|result| result.unwrap())
    .take_while(|line| !line.is_empty())
    .collect();


    print_time(format!("Info: Serving {} request \"{}\"", ip, header));
    if        header == "GET / HTTP/1.1" {
        send_text("src\\webpage\\index.html", &mut stream)
    } else if header == "GET /favicon.ico HTTP/1.1" {
        send_img("src\\webpage\\assets\\favicon.ico", &mut stream)
    } else  if header == "GET /records HTTP/1.1" {
        send_text("src\\webpage\\records\\records.htm", &mut stream)
    } else {
        print_time(format!("{}: Unknown request from {}: \"{}\", data: \n\"{:#?}\"", "Warn".yellow(), ip, header, body));
        send_text("src\\webpage\\404.html", &mut stream)
    }
}
