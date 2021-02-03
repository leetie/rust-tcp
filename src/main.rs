use std::io;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str;
use std::thread;
fn main() {
    // println!("Printing some variables as utf8 encoded bytes");
    // let mut a = "Hello, world!";
    // println!("a.as_bytes() {:?}", a.as_bytes());
    // let mut b = a.as_bytes();
    // let c = str::from_utf8(b).unwrap();
    // println!("c is {}", c);

    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // accept connections and process them, spawning new thread for each
    println!("Server listening on port 3333");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                // connection failed
            }
        }
    }

    // close the socket server
    drop(listener);
}
// receive data, send that data right back
fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50]; // 50 byte buffer
    while match stream.read(&mut data) {
        Ok(_size) => {
            // echo everything
            // stream.write(&data[0..size]).unwrap();
            // println for data received
            println!("Received data: {}", str::from_utf8(&data).unwrap());
            println!("Connected > send a message:");
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).unwrap(); // handle this
                                                         // buffer is a string to be converted to bytes to send to stream

            stream.write(buffer.as_bytes()).unwrap();
            true
        }
        Err(_) => {
            println!(
                "Error occured, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            false
        }
    } {}
}

