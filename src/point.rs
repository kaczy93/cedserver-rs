use bytes::Buf;

pub struct PointU16(pub u16, pub u16);

pub trait ReadPointU16{
    fn get_point_u16(&mut self)->PointU16;
}
impl ReadPointU16 for &[u8]{
    fn get_point_u16(&mut self) -> PointU16 {
        let x = self.get_u16_le();
        let y = self.get_u16_le();
        PointU16(x, y)
    }
}