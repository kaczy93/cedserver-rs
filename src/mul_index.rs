use bytes::Buf;

pub struct MulIndex {
    pub lookup: u32,
    pub length: u32,
    pub _extra: u32
}

impl MulIndex {
    pub fn _new(lookup: u32, length: u32, extra: u32) -> MulIndex {
        MulIndex { lookup, length, _extra: extra }
    }

    pub fn deserialize(mut data: &[u8]) -> MulIndex {
        let lookup = data.get_u32_le();
        let length = data.get_u32_le();
        let extra = data.get_u32_le();
        MulIndex { lookup, length, _extra: extra }
    }

    pub fn is_valid(&self) -> bool {
        self.length != u32::MAX
    }
}
