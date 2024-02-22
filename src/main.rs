use chrono::Local;
use colored::Colorize;
use std::collections::HashMap;
use std::fs;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
mod threadpool;
fn main() {
    let port = "80".to_string();
    host_server(port);

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
                print_time(format!("{}: Failed to connect.", "Warn".yellow()));
            }
        };
    }
}
fn print_time(text: String) {
    let since_the_epoch = Local::now()
    .format("%Y-%m-%d][%H:%M:%S");
println!("[{}]: {}", since_the_epoch, text);
}
fn send_data(path: &str, stream: &mut TcpStream) {
    let status_line = "HTTP/1.1 200 OK";
    let contents = match fs::read(path) {
        Ok(value) => value,
        Err(_) => {
            print_time(format!("{}: Failed to read/find requested file", "Error".red()));
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
    let mut header_iter = header.split_whitespace();
    let mut get_redirects: HashMap<&str, &str> = HashMap::new();


    get_redirects.insert("/", "src\\webpage\\main\\index.html");
    get_redirects.insert("/favicon.ico", "src\\webpage\\main\\assets\\favicon.ico");
    get_redirects.insert("/catch_project/writeup", "C:\\Users\\terra\\source\\repos\\VSCode\\Rust\\web_server\\src\\webpage\\catch_project\\writeup.html");


    if let (Some(method), Some(path), Some(protocol)) = (header_iter.next(),header_iter.next(),header_iter.next()) {
        if method == "GET" {
            print_time(format!("Info: Serving {} request \"{}\"", ip, header));
            if let Some(path) = get_redirects.get(path) {
                send_data(path, &mut stream);
            } else {
                send_data("src\\webpage\\404.html", &mut stream);
            }
        }
    } else {
        print_time(format!("Info: Client {} requested a nonsense header \"{}\"", ip, header));
    }
}