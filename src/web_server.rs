use std::clone;
use std::io::{prelude::*, BufReader, Read};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, Builder, JoinHandle, Thread};
pub struct Server {
    web_page: Arc<Mutex<String>>,
    join_handle: JoinHandle<()>,
    error_log: Receiver<MessageFromServer>,
    max_threads: u32,
}
/// This will not block the current thread
pub fn spawn(
    ip_and_port: String,
    page: String,
    max_threads: u32,
) -> Result<Server, std::io::Error> {
    let (webtx, rx) = mpsc::channel::<MessageFromServer>();
    let thread_builder: thread::Builder = Builder::new();
    let thread_safe_page = Arc::new(Mutex::new(page));
    let copied_page = thread_safe_page.clone();
    let copied_max_threads = max_threads.clone();
    match thread_builder
        .name("Web Server".to_string())
        .spawn(move || server_process(ip_and_port, webtx, copied_page, copied_max_threads))
    {
        Ok(ok_value) => Ok(Server {
            web_page: thread_safe_page.clone(),
            join_handle: ok_value,
            error_log: rx,
            max_threads: max_threads,
        }),
        Err(err_value) => Err(err_value),
    }
}
struct Worker {
    web_page: String,
    join_handle: JoinHandle<()>,
    error_log: Receiver<MessageFromServer>,
    tx: Sender<MessageFromThread>,
}
fn server_process(
    ip_and_port: String,
    tx: Sender<MessageFromServer>,
    page: Arc<Mutex<String>>,
    max_threads: u32,
) {
    let (thread_tx, server_rx) = mpsc::channel::<MessageFromThread>();
    let mut thread_count: u32 = 0;
    let listener_result = TcpListener::bind(&ip_and_port);
    match listener_result {
        Ok(listener) => {
            for stream in listener.incoming() {
                loop {
                    let incoming_message = server_rx.try_recv();
                    match incoming_message {
                        Ok(message) => match message {
                            MessageFromThread::ThreadClosing => {
                                thread_count -= 1;
                            }
                        },
                        Err(_) => break,
                    }
                }
                match stream {
                    Ok(mut stream) => {
                        match stream.peer_addr() {
                            Ok(peer_addr) => {
                                let _ = tx.send(MessageFromServer::ClientConnected(
                                    peer_addr.to_string(),
                                ));
                                let page = page.to_owned();
                                let copied_thread_tx = thread_tx.clone();
                                let thread = thread::Builder::new()
                                    .name("Handler thread".to_string())
                                    .spawn(move || {
                                        handle_connection(page, stream, copied_thread_tx)
                                    });
                                if thread.is_ok() {
                                    thread_count += 1;
                                }
                            }
                            Err(_) => {
                                let _ = tx
                                    .send(MessageFromServer::FailedToConnect(ip_and_port.clone()));
                            }
                        };
                    }
                    Err(_) => {
                        let _ = tx.send(MessageFromServer::FailedToConnect(ip_and_port.clone()));
                        return ();
                    }
                };
            }
        }
        Err(_) => {
            let _ = tx.send(MessageFromServer::FailedToBind(ip_and_port));
            return ();
        }
    }
}
fn handle_connection(
    web_page: Arc<Mutex<String>>,
    mut stream: TcpStream,
    tx: Sender<MessageFromThread>,
) {
    let buf_reader = BufReader::new(&mut stream);
    let mut lines = buf_reader.lines();
    let request_line = &lines.next().unwrap().unwrap();

    /*println!("{:#?}", lines
    .map(|result| result.unwrap())
    .take_while(|line| !line.is_empty())
    .collect::<Vec<String>>());*/

    if request_line == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = std::fs::read_to_string(
            "C:/Users/terra/source/repos/VSCode/rust/web_server/src/html_files/index.html",
        )
        .unwrap();
        let length = contents.len();
        let response = format!["{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"];

        stream.write_all(response.as_bytes()).unwrap();
    } else {
        let status_line = "HTTP/1.1 200 OK";
        let contents = format![
        "<!DOCTYPE html>
        <head>
            <title>Rust website</title>
        </head>
        <body>
            <h1>Error 404:</h1>
            <p>Page \"{}\" not found.</p>
        </body>", request_line.split_ascii_whitespace().nth(1).or(Some("Unknown")).expect("Failed to show err message")
        ];
        let length = contents.len();
        let response = format!["{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"];

        stream.write_all(response.as_bytes()).unwrap();
    }
}
enum MessageFromServer {
    FailedToSpawnThread,
    ClientConnected(String), // Contains the remote IP
    UnknownRequest,
    FailedToBind(String),    // Contains the local IP
    FailedToConnect(String), // Contains the local IP
}
enum MessageFromThread {
    ThreadClosing,
}
