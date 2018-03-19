use std::net::TcpStream;
use std::io::{BufRead, BufReader, Error, Write};
use std::sync::{Arc, Mutex};
use storage;

#[derive(Debug)]
enum Command {
    Pop { count: usize },
    Push { priority: u16, data: Box<Vec<u8>> },
    Clear,
}

struct ProtocolParser {}
impl ProtocolParser {
    pub fn parse_line(line: &String) -> Result<Command, String> {
        let tokens: Vec<&str> = line.splitn(3, ' ').collect();

        if tokens.is_empty() {
            return Err(format!("Unrecognized command {}", line));
        }

        let cmd = String::from(tokens[0]).to_lowercase();

        match &cmd as &str {
            "pop" => Self::pop(tokens),
            "push" => Self::push(tokens),
            "clear" => Self::clear(),
            _ => Err(format!("Unrecognized command {}", line)),
        }
    }

    // push [priority] data without newline
    fn push(tokens: Vec<&str>) -> Result<Command, String> {
        if tokens.len() < 3 {
            return Err(format!("Unrecognized push command {:?}", tokens));
        }
        let priority_result = tokens[1].parse::<u16>();

        if priority_result.is_err() {
            return Err(format!("Unrecognized push command {:?}", tokens));
        };

        let bytes = String::from(tokens[2]).into_bytes();
        let cmd = Command::Push {
            priority: priority_result.unwrap(),
            data: Box::new(bytes),
        };
        Ok(cmd)
    }

    // pop [number or no number]
    fn pop(tokens: Vec<&str>) -> Result<Command, String> {
        let count: usize = if tokens.len() < 2 {
            1
        } else {
            match tokens[1].parse::<usize>() {
                Ok(num) => num,
                Err(_e) => 1,
            }
        };
        Ok(Command::Pop { count: count })
    }

    fn clear() -> Result<Command, String> {
        Ok(Command::Clear)
    }
}

pub struct Connection {}

impl Connection {
    pub fn new() -> Self {
        Connection {}
    }

    pub fn handle<T: storage::Storage>(
        &mut self,
        stream: Result<TcpStream, Error>,
        storage: Arc<Mutex<T>>,
    ) {
        match stream {
            Ok(stream) => {
                self.handle_stream(stream, storage);
            }
            Err(e) => println!("Error handling connection {:?}", e),
        }
    }

    fn handle_stream<T: storage::Storage>(&mut self, stream: TcpStream, storage: Arc<Mutex<T>>) {
        let mut writer = stream.try_clone().expect("Clone failed");
        let reader = BufReader::new(stream);

        for line_result in reader.lines() {
            match line_result {
                Err(e) => println!("Error reading line {:?}", e),
                Ok(l) => match ProtocolParser::parse_line(&l) {
                    Err(e) => println!("{:?}", e),
                    Ok(cmd) => {
                        let mut s = storage.lock().unwrap();
                        match cmd {
                            Command::Pop { count } => match s.pop(count) {
                                Some(storage_items) => for storage_item in storage_items {
                                    writer.write(&storage_item.data).unwrap();
                                    writer.write(b"\n").unwrap();
                                },
                                None => {
                                    writer.write(b"\n").unwrap();
                                }
                            },
                            Command::Push { priority, data } => {
                                s.push(priority, data);
                                writer.write(b"OK\n").unwrap();
                            }
                            Command::Clear => {
                                s.clear();
                                writer.write(b"OK\n").unwrap();
                            }
                        };
                    }
                },
            }
        }
    }
}
