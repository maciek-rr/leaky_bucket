#[macro_use]
extern crate serde_derive;

extern crate bincode;
extern crate serde;

use std::net::{Shutdown, TcpListener};
use std::thread;
use std::sync::{Arc, Mutex};
mod storage;
mod hash_storage;
mod connection;
use storage::Storage;
use hash_storage::HashStorage;

const TCP_PORT: u16 = 1984;
const HOST: &'static str = "127.0.0.1";
const MAX_CONNECTIONS: usize = 128;
// const MAX_MEMORY: u64 = 1024;

fn main() {
    let storage = Arc::new(Mutex::new(HashStorage::new()));
    storage.lock().unwrap().load();
    let addr = format!("{}:{}", HOST, TCP_PORT);
    let listener = TcpListener::bind(addr).unwrap();
    let conn_count = Arc::new(Mutex::new(0 as usize));

    for stream_result in listener.incoming() {
        if stream_result.is_err() {
            continue;
        }
        let mut stream = stream_result.unwrap();
        {
            let mut count = conn_count.lock().unwrap();
            if *count >= MAX_CONNECTIONS {
                let _ = stream.shutdown(Shutdown::Both);
                println!("Too many connections (max connections: {})", *count);
                continue;
            }
            *count += 1;
        }

        let storage_ref = Arc::clone(&storage);
        let counter_ref = Arc::clone(&conn_count);
        thread::spawn(move || {
            let mut conn = connection::Connection::new();
            conn.handle(stream, storage_ref);
            let mut count = counter_ref.lock().unwrap();
            *count -= 1;
        });
    }
}
