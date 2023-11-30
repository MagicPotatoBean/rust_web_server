use std::fs;
use std::io::{prelude::*, BufReader};
use std::net::TcpStream;
use std::ops::Index;

use web_server::send_file;
mod threadpool;
mod web_server;
fn main() {
    let port = "80".to_string();
    web_server::host_server(port, handle_connection);
}
fn handle_connection(mut stream: TcpStream) {
    let ip = stream
        .peer_addr()
        .expect("Failed to read peer ip")
        .to_string();
    let buf_reader = BufReader::new(&mut stream);

    let mut request_lines = buf_reader.lines();
    let header = if let Some(Ok(temp)) = request_lines.next() {
        temp
    } else {
        return;
    };

    web_server::print_time(format!("Info: Serving {} request \"{}\"", ip, header));
    if header.starts_with("GET /create") {
        if !header.contains("?") {
            send_file("src\\webpage\\main\\create.html", &mut stream)
        } else {
            let data = split_url_get_data(&header);
        }
    } else if header.starts_with("GET /delete") {
        if !header.contains("?") {
            send_file("src\\webpage\\main\\delete.html", &mut stream)
        }
    } else {
        println!("Root");
        send_file("src\\webpage\\main\\index.html", &mut stream)
    }
}

fn send_text_replace_one(path: &str, stream: &mut TcpStream, replace_value: &str) {
    let status_line = "HTTP/1.1 200 OK";
    let contents = match fs::read_to_string(path) {
        Ok(value) => value.replace("<v>x</v>", replace_value),
        Err(_) => {
            web_server::print_time(format!("Error: Failed to read/find requested file"));

            return;
        }
    };
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    let _ = stream.write_all(response.as_bytes());
}
fn split_url_get_data(url: &str) -> Vec<String> {
    let mut data: Vec<String> = vec!["".to_owned();0];
    let mut ascii_split = url.split_ascii_whitespace();
    let path = ascii_split.nth(1).expect("URL was in wrong form");
    let question_split = path.split("?").into_iter();
    for data_item in question_split {
        data.push(data_item.to_owned());
    }
    data
}