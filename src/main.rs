use std::thread;
use std::io;
use std::net::{TcpListener};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut stream_clone = stream.try_clone().unwrap();
        thread::spawn(move || {
            io::copy(&mut io::stdin(), &mut stream);
        });
        thread::spawn(move || {
            io::copy(&mut stream_clone, &mut io::stdout());
        });
    }
}
