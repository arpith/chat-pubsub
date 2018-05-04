use std::thread;
use std::sync::mpsc;
use std::io;
use std::net::{TcpListener};
use std::sync::{Arc, RwLock};
use std::io::Read;
use std::fs::OpenOptions;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    let (tx, rx) = mpsc::channel();
    let bus = Arc::new(RwLock::new(Vec::new()));
    let bus_clone1 = bus.clone();
    let bus_clone2 = bus.clone();
    thread::spawn(move || {
        loop {
            let bytes = rx.recv().unwrap();
            let mut bus_write = bus_clone1.write().unwrap();
            bus_write.extend(bytes);
        }
    });
    thread::spawn(move || {
        let mut file = OpenOptions::new().append(true).create(true).open("log.txt").unwrap();
        let mut last_read = 0;
        loop {
            let bus_read = &bus_clone2.read().unwrap();
            let _res = io::copy(&mut &bus_read[last_read..], &mut file);
            last_read = bus_read.len();
        }
    });
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut stream_clone = stream.try_clone().unwrap();
        let bus_clone = bus.clone();
        let thread_tx = tx.clone();
        thread::spawn(move || {
            let mut last_read = 0;
            loop {
                let bus_read = &bus_clone.read().unwrap();
                let _res = io::copy(&mut &bus_read[last_read..], &mut stream);
                last_read = bus_read.len();
            }
        });
        thread::spawn(move || {
            for b in stream_clone.bytes() {
                let _res = thread_tx.send(b);
            }
        });
    }
}
