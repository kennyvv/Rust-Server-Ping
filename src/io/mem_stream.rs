use std::fmt;

#[derive(Debug)]
pub enum ReadError {
    EndOfStream
}

impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReadError::EndOfStream => write!(f, "End of Stream"),
        }
    }
}

pub struct MemStream {
    buffer: Vec<u8>,
    pub(crate) position: i32,
    pub(crate) length: i32
}

impl MemStream {
    pub fn read_exact(&mut self, buffer: &mut [u8]) -> Result<(), ReadError> {
        if (self.position + buffer.len() as i32) > self.length {
            return Err(ReadError::EndOfStream);
        }

        let position = self.position as usize;
        let (left, _) = self.buffer.split_at_mut(position + (buffer.len()));
        buffer.copy_from_slice(&left[position..]);
        self.position += buffer.len() as i32;

        Ok(())
    }

    pub fn write(&mut self, data: &[u8]){
        let len = data.len() as i32;
        let mut vector = data.to_vec();

        self.buffer.append(&mut vector);
        self.position += len;
        self.length = self.buffer.len() as i32;
    }

    pub fn get_data(&mut self) -> &[u8] {
        return self.buffer.as_mut_slice();
    }
}

pub fn new(buffer: &Vec<u8>) -> MemStream {
    let stream = MemStream {
        position: 0,
        buffer: buffer.to_vec(),
        length: buffer.len() as i32
    };
    return stream;
}