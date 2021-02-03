use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;
use std::thread;

pub mod client {
  pub fn connect() {
    println!("Connecting to server...");
    while match TcpStream::connect("localhost:3333") {
        Ok(mut stream) => {
            let mut buffer = String::new();
            println!("Connected > send a message:");
            std::io::stdin().read_line(&mut buffer).unwrap();

            stream.write(buffer.as_bytes()).unwrap();
            println!("waiting for reply...");

            let mut data = [0 as u8; 50];
            match stream.read(&mut data) {
                Ok(_) => {
                    if true == false {
                    } else {
                        let text = str::from_utf8(&data).unwrap();
                        println!("Received reply: {}", text);
                    };
                    true
                }
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                    false
                }
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
            false
        }
    } {}

    println!("Terminated");
  }
}
