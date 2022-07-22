use crate::net::minecraft_stream::{MinecraftStream, ReadError};

pub trait Packet {
    fn decode(&mut self, stream: impl MinecraftStream);
    //fn encode(&mut self, &mut stream: MinecraftStream)
}

pub struct Handshake {
    protocol_version: i32,
    server_address: String,
    server_port: u16,
    pub next_state: i32
}

impl Handshake {
    pub fn get_next_state(&mut self) -> i32 {
        self.next_state
    }
}

impl Packet for Handshake {
    fn decode(&mut self, mut stream: impl MinecraftStream) {
        self.protocol_version = stream.read_varint().expect("Failed to read protocol version");
        self.server_address = stream.read_string().expect("Failed to read server address");
        self.server_port = stream.read_ushort().expect("Failed to read server port");
        self.next_state = stream.read_varint().expect("Failed to read next state");
    }
}

pub fn read_handshake(mut stream: impl MinecraftStream) -> Handshake {
    let mut handshake = Handshake {
        next_state: 0,
        protocol_version: 0,
        server_port: 0,
        server_address: String::new()
    };

    handshake.decode(stream);

    handshake
}

pub struct PingRequest {
    pub payload: i64
}

impl Packet for PingRequest {
    fn decode(&mut self, mut stream: impl MinecraftStream) {
        self.payload = stream.read_long().expect("Failed to read payload");
    }
}

pub fn read_ping_request(mut stream: impl MinecraftStream) -> PingRequest {
    let mut packet = PingRequest {
        payload: 0
    };

    packet.decode(stream);

    packet
}