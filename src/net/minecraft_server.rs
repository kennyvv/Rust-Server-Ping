use std::net::{TcpListener};
use std::{io, thread};
use crate::net::client_connection::handle_connection;

pub fn bind(port: i32) -> MinecraftServer {
    let server = MinecraftServer {
        port
    };

    return server;
}

pub struct MinecraftServer {
    port: i32,
}

impl MinecraftServer {
    pub fn run(self) {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", self.port)).expect("Failed to bind server.");
        listener.set_nonblocking(false).expect("Cannot set blocking");

        println!("Accepting connections...");
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("Accepted connection");
                    stream.set_nonblocking(false).expect("Failed to set stream as blocking");

                    thread::Builder::new().name("Client Conn".to_string()).spawn(|| {
                        let connection = handle_connection(stream);
                        connection.run();
                    }).expect("Failed to spawn thread.");
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => { /* connection failed */ }
            }
        }
    }
}