use std::net::TcpStream;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Error;

#[derive(Debug)]
enum Command {
    Pop { number: usize },
    Push { priority: u16, data: Box<Vec<u8>> },
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
        let cmd = Command::Push { priority: priority_result.unwrap(), data: Box::new(bytes) };
        Ok(cmd)
    }

    // pop [number or no number]
    fn pop(tokens: Vec<&str>) -> Result<Command, String> {
        let pop_number: usize = if tokens.len() < 2 {
            1
        } else {
            match tokens[1].parse::<usize>() {
                Ok(num) => { num }
                Err(e) => { 1 }
            }
        };
        Ok(Command::Pop { number: pop_number })
    }
}

pub struct Connection {}

impl Connection {
    pub fn new() -> Self {
        Connection {}
    }

    pub fn handle(&mut self, stream: Result<TcpStream, Error>) {
        match stream {
            Ok(stream) => {
                self.handle_stream(stream);
            }
            Err(e) => println!("Error handling connection {:?}", e),
        }
    }

    fn handle_stream(&mut self, stream: TcpStream) {
        let reader = BufReader::new(stream);

        for line_result in reader.lines() {
            match line_result {
                Ok(l) => match ProtocolParser::parse_line(&l) {
                    Ok(cmd) => {
                        println!("Parsed command: {:?}", cmd);
                    }
                    Err(e) => {
                        println!("{:?}", e);
                    }
                },
                Err(e) => {
                    println!("Error reading line {:?}", e);
                }
            }
        }
    }
}