use std::fmt;
use std::io::{Read, Write};
use std::net::TcpStream;
use crate::io::mem_stream::MemStream;

#[derive(Debug)]
pub enum ReadError {
    EndOfStream,
    VarIntTooBig,
    VarLongTooBig
}

impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReadError::EndOfStream => write!(f, "End of Stream"),
            ReadError::VarIntTooBig => write!(f, "VarInt too big"),
            ReadError::VarLongTooBig => write!(f, "VarLong too big")
        }
    }
}

pub trait MinecraftStream {
    fn read_byte(&mut self) -> Result<u8, ReadError>;
    fn write_byte(&mut self, value: u8);

    fn read_bytes(&mut self, buf: &mut [u8]);
    fn write_bytes(&mut self, buf: &[u8]);

    fn read_short(&mut self) -> Result<i16, ReadError> {
        let mut buf = [0;2];
        self.read_bytes(&mut buf);

        Ok(i16::from_le_bytes(buf))
    }

    fn write_short(&mut self, value: i16) {
        self.write_bytes(value.to_le_bytes().as_slice());
    }

    fn read_ushort(&mut self) -> Result<u16, ReadError> {
        let mut buf = [0;2];
        self.read_bytes(&mut buf);

        Ok(u16::from_le_bytes(buf))
    }

    fn write_ushort(&mut self, value: u16) {
        self.write_bytes(value.to_le_bytes().as_slice());
    }

    fn read_int(&mut self) -> Result<i32, ReadError> {
        let mut buf = [0;4];
        self.read_bytes(&mut buf);

        Ok(i32::from_le_bytes(buf))
    }

    fn write_int(&mut self, value: i32) {
        self.write_bytes(value.to_le_bytes().as_slice());
    }

    fn read_uint(&mut self) -> Result<u32, ReadError> {
        let mut buf = [0;4];
        self.read_bytes(&mut buf);

        Ok(u32::from_le_bytes(buf))
    }

    fn write_uint(&mut self, value: u32) {
        self.write_bytes(value.to_le_bytes().as_slice());
    }

    fn read_long(&mut self) -> Result<i64, ReadError> {
        let mut buf = [0;8];
        self.read_bytes(&mut buf);

        Ok(i64::from_le_bytes(buf))
    }

    fn write_long(&mut self, value: i64) {
        self.write_bytes(value.to_le_bytes().as_slice());
    }

    fn read_ulong(&mut self) -> Result<u64, ReadError> {
        let mut buf = [0;8];
        self.read_bytes(&mut buf);

        Ok(u64::from_le_bytes(buf))
    }

    fn write_ulong(&mut self, value: u64) {
        self.write_bytes(value.to_le_bytes().as_slice());
    }

    fn read_float(&mut self) -> Result<f32, ReadError> {
        let mut buf = [0;4];
        self.read_bytes(&mut buf);

        Ok(f32::from_le_bytes(buf))
    }

    fn write_float(&mut self, value: f32) {
        self.write_bytes(value.to_le_bytes().as_slice());
    }

    fn read_varint(&mut self) -> Result<i32, ReadError> {
        let mut num_read : i32 = 0;
        let mut result : i32 = 0;
        let mut read: u8 = 0;

        loop {
            match self.read_byte(){
                Err(error) => {
                    return Err(error);
                }
                Ok(value) => {
                    read = value;
                }
            }

            let value = (read & 0x7f) as i32;
            result |= value << (7 * num_read);
            num_read += 1;

            if (read & 0x80) == 0{
                break;
            }

            if num_read > 5 {
                return Err(ReadError::VarIntTooBig)
            }
        }

        Ok(result)
    }

    fn write_varint(&mut self, value: i32) -> i32 {
        let mut length = 0;
        let mut v: i32 = value;
        loop {
            if (v & !0x7F) == 0 {
                self.write_byte(v as u8);
                return length +1;
            }

            self.write_byte(((v & 0x7F) as u8) | 0x80);
            v >>= 7;
            length += 1;
        }
    }

    fn read_varlong(&mut self) -> Result<i64, ReadError> {
        let mut num_read : i64 = 0;
        let mut result : i64 = 0;
        let mut read: u8 = 0;

        loop {

            match self.read_byte(){
                Err(error) => {
                    return Err(error);
                }
                Ok(value) => {
                    read = value;
                }
            }

            let value = (read & 0x7f) as i64;
            result |= value << (7 * num_read);
            num_read += 1;

            if (read & 0x80) == 0{
                break;
            }

            if num_read > 10 {
                return Err(ReadError::VarLongTooBig)
            }
        }

        Ok(result)
    }

    fn write_varlong(&mut self, value: i64) -> i32 {
        let mut length = 0;
        let mut v: i64 = value;
        loop {
            if (v & !0x7F) == 0 {
                self.write_byte(v as u8);
                return length +1;
            }

            self.write_byte(((v & 0x7F) as u8) | 0x80);
            v >>= 7;
            length += 1;
        }
    }

    fn read_string(&mut self) -> Result<String, ReadError> {
        let length = self.read_varint().unwrap();

        let mut buf = vec![0; length as usize];
        self.read_bytes(buf.as_mut_slice());

        Ok(String::from_utf8(buf).expect("Found invalid UTF-8"))
    }

    fn write_string(&mut self, value: &str) {
        let bytes = value.as_bytes();
        self.write_varint(bytes.len() as i32);
        self.write_bytes(bytes);
    }
}

impl MinecraftStream for TcpStream {
    fn read_byte(&mut self) -> Result<u8, ReadError> {
        let mut buf: [u8;1] = [0;1];

        if self.read_exact(&mut buf).is_err() {
            return Err(ReadError::EndOfStream);
        }

        Ok(buf[0])
    }

    fn write_byte(&mut self, value: u8) {
        let buf: [u8;1] = [value];
        self.write_bytes(&buf);
    }

    fn read_bytes(&mut self, buf: &mut [u8]) {
        self.read_exact(buf).unwrap();
    }

    fn write_bytes(&mut self, buf: &[u8]) {
        self.write(buf).expect("Failed to write to stream.");
    }
}

impl MinecraftStream for MemStream {
    fn read_byte(&mut self) -> Result<u8, ReadError> {
        let mut buf: [u8;1] = [0;1];

        if self.read_exact(&mut buf).is_err() {
            return Err(ReadError::EndOfStream);
        }

        Ok(buf[0])
    }

    fn write_byte(&mut self, value: u8) {
        let buf: [u8;1] = [value];
        self.write_bytes(&buf);
    }

    fn read_bytes(&mut self, buf: &mut [u8]) {
        self.read_exact(buf).unwrap();
    }

    fn write_bytes(&mut self, buf: &[u8]) {
        self.write(buf);
    }
}