use std::net::TcpListener;
mod storage;
mod connection;
use std::thread;

const TCP_PORT: u16 = 1984;
const HOST: &'static str = "127.0.0.1";
// const MAX_CONNECTIONS: u8 = 16;
// const MAX_MEMORY: u64 = 1024;

fn main() {
    let storage = storage::Storage::new();
    let addr = format!("{}:{}", HOST, TCP_PORT);
    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        thread::spawn(move || {
            let mut conn = connection::Connection::new();
            conn.handle(stream);
        });
    }
}
