use std::net::TcpStream;
use std::io::{BufReader, Write};
use crate::net::minecraft_stream::{MinecraftStream, ReadError};
use crate::io::mem_stream;
use crate::io::mem_stream::MemStream;
use crate::net::packets::{Packet, Handshake, read_handshake, read_ping_request};

pub struct ClientConnection{
    stream: TcpStream,
    connected: bool,
    compression_enabled: bool,
    compression_threshold: i32,
    connection_state: i8
}

struct PacketData {
    data: Vec<u8>
}

impl ClientConnection {
    pub fn run(mut self){
        while self.connected {
            match self.read_packet_data(){
                Ok(packet_data) => {
                    self.handle_packet_data(&packet_data);
                }
                Err(read_error) => {
                    println!("Read error: {}", read_error);
                    self.connected = false;
                    break;
                }
            }
            //self.process_packet(buf.as_mut_slice());
        }
        println!("Closed");
    }

    fn read_packet_data(&mut self) -> Result<PacketData, ReadError> {
        let mut packet_length = self.stream.read_varint().expect("Failed to read packet length");

        let mut buf = vec![0; packet_length as usize];
        self.stream.read_bytes(buf.as_mut_slice());

        if self.compression_enabled {
            let mut uncompressed_length = 0;
            match self.stream.read_varint() {
                Ok(length) => {
                    uncompressed_length = length;
                }
                Err(error) => {
                    return Err(error);
                }
            }

            if uncompressed_length > 0 {
                //Data is compressed, decompress before reading!!!
            } else {
                //
            }
        }

        //TODO: Only return remaining data (Without compression header etc)

        Ok(PacketData{
            data: buf
        })
    }

    fn write_packet_data(&mut self, data: &[u8]) {
        self.stream.write_varint(data.len() as i32);
        self.stream.write_bytes(data);
        self.stream.flush().expect("Failed to flush data");
    }

    fn handle_packet_data(&mut self, packet_data: &PacketData){
        let mut stream = mem_stream::new(&packet_data.data);

        let mut packet_id: i32 = 0;
        match stream.read_varint(){
            Ok(value) => {
                packet_id = value;
            }
            Err(error) => {
                println!("Invalid packet! Error={}", error);
                return;
            }
        }

        let packet_length = stream.length - stream.position;
            //let packet_id = stream.read_varint();
        println!("Received packet. PacketID={:#04x} Length={} bytes", packet_id, packet_length);

        if self.connection_state == 0  //Handshake
        {
            self.handle_handshake(stream, packet_id);
        }
        else if self.connection_state == 1 //Status
        {
            self.handle_status(stream, packet_id);
        }
    }

    fn handle_handshake(&mut self, stream: impl MinecraftStream, packet_id: i32) {
        if packet_id == 0x00 {
            let mut handshake_packet: Handshake = read_handshake(stream);
            self.connection_state = handshake_packet.get_next_state() as i8;
        }
    }

    fn handle_status(&mut self, stream: impl MinecraftStream, packet_id: i32) {
        if packet_id == 0x00 {
            let mut buffer: Vec<u8> =  Vec::new();
            let mut mem_stream = mem_stream::new(&buffer);
            mem_stream.write_varint(0x00);
            mem_stream.write_string("{\"version\":{\"name\":\"1.19\",\"protocol\":759},\"players\":{\"max\":100,\"online\":5,\"sample\":[{\"name\":\"thinkofdeath\",\"id\":\"4566e69f-c907-48ee-8d71-d7ba5aa00d20\"}]},\"description\":{\"text\":\"Hello world\"},\"previewsChat\":true}");

            let data = mem_stream.get_data();
            self.write_packet_data(data);
        } else if packet_id == 0x01 {
            let ping_request = read_ping_request(stream);
            let mut buffer: Vec<u8> =  Vec::new();
            let mut mem_stream = mem_stream::new(&buffer);
            mem_stream.write_varint(0x01);
            mem_stream.write_long(ping_request.payload);

            let data = mem_stream.get_data();
            self.write_packet_data(data);
        }
    }
}

pub fn handle_connection(stream: TcpStream) -> ClientConnection{
    let connection = ClientConnection{
        stream,
        connected: true,
        compression_enabled: false,
        compression_threshold: -1,
        connection_state: 0
    };

    return connection;
}