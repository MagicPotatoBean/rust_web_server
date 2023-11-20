mod web_server;
fn main() {
    let server = web_server::spawn(
        "172.25.35.234:13333".to_owned(),
        "<h1> test </h1>".to_owned(),
        1,
    );
    loop {}
}
