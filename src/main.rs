#![allow(unused_imports)]
use std::{fs::File, io::Write, net::TcpListener};

use bytes::Bytes;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                stream
                    .write_all("+PONG\r\n".as_bytes())
                    .expect("Failed write to client");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
