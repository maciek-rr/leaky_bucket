use std::net::TcpStream;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Error;

pub struct Connection {
}

impl Connection {
    pub fn new() -> Self {
        Connection {

        }
    }

    pub fn handle(&mut self, stream: Result<TcpStream,Error>) {
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
                Ok(l) => {

                }
                Err(e) => {
                    println!("Error reading line {:?}", e);
                }
            }
        }
    }
}
