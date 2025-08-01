use bytes::{Buf};

pub trait BytesUtf8{
    fn get_str_nul(&mut self) -> &str;
}

impl BytesUtf8 for &[u8] {
    fn get_str_nul(&mut self) -> &str{
        let endIdx = self.iter().position(|&c| c == 0).expect("Missing string terminator");
        let result = std::str::from_utf8(&self[..endIdx]).expect("Invalid UTF-8 string");
        self.advance(endIdx+1);
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::bytes_utf8::BytesUtf8;

    #[test]
    fn test_get_str_nul(){
        let test_val = "hello\0";
        let mut test_bytes = test_val.as_bytes();

        assert_eq!("hello", test_bytes.get_str_nul())
    }

    #[test]
    fn test_get_two_str_nul(){
        let test_val = "hello\0world\0";
        let mut test_bytes = test_val.as_bytes();

        assert_eq!("hello", test_bytes.get_str_nul());
        assert_eq!("world", test_bytes.get_str_nul());
    }
}