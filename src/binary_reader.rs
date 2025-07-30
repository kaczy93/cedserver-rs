use std::io::Read;

pub struct BinaryReader<'a>{
    buffer: &'a [u8],
    position: usize,
}

impl BinaryReader<'_>{
    pub fn new(buffer: &[u8]) -> BinaryReader{
        BinaryReader { buffer, position: 0}
    }

    pub fn has_next(&self) -> bool {
        self.position < self.buffer.len()
    }

    pub fn read_u64(&mut self) -> u64{
        let slice = &self.buffer[self.position..self.position + 8];
        let result = u64::from_le_bytes(slice.try_into().unwrap());
        self.position += 8;
        result
    }

    pub fn read_u32(&mut self) -> u32{
        let slice = &self.buffer[self.position..self.position + 4];
        let result = u32::from_le_bytes(slice.try_into().unwrap());
        self.position += 4;
        result
    }

    pub fn read_u16(&mut self) -> u16{
        let slice = &self.buffer[self.position..self.position + 2];
        let result = u16::from_le_bytes(slice.try_into().unwrap());
        self.position += 2;
        result
    }

    pub fn read_u8(&mut self) -> u8{
        let result = self.buffer[self.position];
        self.position += 1;
        result
    }

    pub fn read_i8(&mut self) -> i8{
        let slice = &self.buffer[self.position..self.position + 1];
        let result = i8::from_le_bytes(slice.try_into().unwrap());
        self.position += 1;
        result
    }
}

#[cfg(test)]
mod tests {
    use rand::random;
    use crate::binary_reader::BinaryReader;

    #[test]
    fn test_read_u64(){
        let test_val: u64 = random();
        let buf: [u8; 8] = test_val.to_le_bytes();

        let mut reader = BinaryReader::new(&buf);
        assert_eq!(test_val, reader.read_u64())
    }
}