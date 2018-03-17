#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate bincode;

use std::net::TcpListener;
use std::thread;
use std::sync::{Arc, Mutex};
mod storage;
mod simple_storage;
mod ordered_storage;
mod connection;
use storage::Storage;
use simple_storage::SimpleStorage;
use ordered_storage::OrderedStorage;

const TCP_PORT: u16 = 1984;
const HOST: &'static str = "127.0.0.1";
// const MAX_CONNECTIONS: u8 = 16;
// const MAX_MEMORY: u64 = 1024;

fn main() {
    let storage = Arc::new(Mutex::new(SimpleStorage::new()));
    storage.lock().unwrap().load();
    let addr = format!("{}:{}", HOST, TCP_PORT);
    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        let storage_ref = Arc::clone(&storage);
        thread::spawn(move || {
            let mut conn = connection::Connection::new();
            conn.handle(stream, storage_ref);
        });
    }
}
