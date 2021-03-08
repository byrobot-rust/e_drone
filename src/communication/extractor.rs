use byteorder::{ByteOrder, LittleEndian};

pub struct Extractor
{
    vec_data: Vec<u8>,
    index : usize,
}

impl Extractor
{
    pub fn new(vec_data_source: &Vec<u8>) -> Extractor {
        Extractor {
            vec_data: vec_data_source.clone(),
            index: 0,
        }
    }

    pub fn get_u8(&mut self) -> u8
    {
        if self.index + 1 <= self.vec_data.len() {
            self.index = self.index + 1;
            let value: u8 = self.vec_data[self.index];
            value
        }
        else { 0 }
    }

    pub fn get_u16(&mut self) -> u16
    {
        if self.index + 2 <= self.vec_data.len() {
            self.index = self.index + 2;
            /*
            let value: u16 =    ((self.vec_data[self.index + 1] as u16) << 8) |
                                ( self.vec_data[self.index + 0] as u16);
            value
            // */
            LittleEndian::read_u16(&self.vec_data[self.index..(self.index+1)])
        }
        else { 0 }
    }

    pub fn get_u32(&mut self) -> u32
    {
        if self.index + 4 <= self.vec_data.len() {
            self.index = self.index + 4;
            /*
            let value: u32 =    ((self.vec_data[self.index + 3] as u32) << (8 * 3)) |
                                ((self.vec_data[self.index + 2] as u32) << (8 * 2)) |
                                ((self.vec_data[self.index + 1] as u32) << (8 * 1)) |
                                ( self.vec_data[self.index + 0] as u32);
            value
            // */
            LittleEndian::read_u32(&self.vec_data[self.index..(self.index+3)])
        }
        else { 0 }
    }

    pub fn get_u64(&mut self) -> u64
    {
        if self.index + 8 <= self.vec_data.len() {
            self.index = self.index + 8;
            /*
            let value: u64 =    ((self.vec_data[self.index + 7] as u64) << (8 * 7)) |
                                ((self.vec_data[self.index + 6] as u64) << (8 * 6)) |
                                ((self.vec_data[self.index + 5] as u64) << (8 * 5)) |
                                ((self.vec_data[self.index + 4] as u64) << (8 * 4)) |
                                ((self.vec_data[self.index + 3] as u64) << (8 * 3)) |
                                ((self.vec_data[self.index + 2] as u64) << (8 * 2)) |
                                ((self.vec_data[self.index + 1] as u64) << (8 * 1)) |
                                ( self.vec_data[self.index + 0] as u64);
            value
            // */
            LittleEndian::read_u64(&self.vec_data[self.index..(self.index+7)])
        }
        else { 0 }
    }

    pub fn get_i8(&mut self) -> i8
    {
        if self.index + 1 <= self.vec_data.len() {
            self.index = self.index + 1;
            let value: i8 = self.vec_data[self.index] as i8;
            value
        }
        else { 0 }
    }

    pub fn get_i16(&mut self) -> i16
    {
        if self.index + 2 <= self.vec_data.len() {
            /*
            self.get_u16() as i16
            // */
            self.index = self.index + 2;
            LittleEndian::read_i16(&self.vec_data[self.index..(self.index+1)])
        }
        else { 0 }
    }

    pub fn get_i32(&mut self) -> i32
    {
        if self.index + 4 <= self.vec_data.len() {
            /*
            self.get_u32() as i32
            // */
            self.index = self.index + 4;
            LittleEndian::read_i32(&self.vec_data[self.index..(self.index+3)])
        }
        else { 0 }
    }

    pub fn get_i64(&mut self) -> i64
    {
        if self.index + 8 <= self.vec_data.len() {
            /*
            self.get_u64() as i64
            // */
            self.index = self.index + 8;
            LittleEndian::read_i64(&self.vec_data[self.index..(self.index+7)])
        }
        else { 0 }
    }

    pub fn get_f32(&mut self) -> f32
    {
        if self.index + 4 <= self.vec_data.len() {
            self.index = self.index + 4;
            LittleEndian::read_f32(&self.vec_data[self.index..(self.index+3)])
        }
        else { 0_f32 }
    }

    pub fn get_f64(&mut self) -> f64
    {
        if self.index + 8 <= self.vec_data.len() {
            self.index = self.index + 8;
            LittleEndian::read_f64(&self.vec_data[self.index..(self.index+7)])
        }
        else { 0_f64 }
    }

    pub fn get_vec_clone(&self) -> Vec<u8>
    {
        self.vec_data.clone()
    }
}
