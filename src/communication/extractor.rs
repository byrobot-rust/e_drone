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
            let value: u8 = self.vec_data[self.index];
            self.index = self.index + 1;
            value
        }
        else { 0 }
    }

    pub fn get_u16(&mut self) -> u16
    {
        if self.index + 2 <= self.vec_data.len() {
            let value: u16 = LittleEndian::read_u16(&self.vec_data[self.index..(self.index+2)]);
            self.index = self.index + 2;
            value
        }
        else { 0 }
    }

    pub fn get_u32(&mut self) -> u32
    {
        if self.index + 4 <= self.vec_data.len() {
            let value: u32 = LittleEndian::read_u32(&self.vec_data[self.index..(self.index+4)]);
            self.index = self.index + 4;
            value
        }
        else { 0 }
    }

    pub fn get_u64(&mut self) -> u64
    {
        if self.index + 8 <= self.vec_data.len() {
            let value: u64 = LittleEndian::read_u64(&self.vec_data[self.index..(self.index+8)]);
            self.index = self.index + 8;
            value
        }
        else { 0 }
    }

    pub fn get_i8(&mut self) -> i8
    {
        if self.index + 1 <= self.vec_data.len() {
            let value: i8 = self.vec_data[self.index] as i8;
            self.index = self.index + 1;
            value
        }
        else { 0 }
    }

    pub fn get_i16(&mut self) -> i16
    {
        if self.index + 2 <= self.vec_data.len() {
            let value: i16 = LittleEndian::read_i16(&self.vec_data[self.index..(self.index+2)]);
            self.index = self.index + 2;
            value
        }
        else { 0 }
    }

    pub fn get_i32(&mut self) -> i32
    {
        if self.index + 4 <= self.vec_data.len() {
            let value: i32 = LittleEndian::read_i32(&self.vec_data[self.index..(self.index+4)]);
            self.index = self.index + 4;
            value
        }
        else { 0 }
    }

    pub fn get_i64(&mut self) -> i64
    {
        if self.index + 8 <= self.vec_data.len() {
            let value: i64 = LittleEndian::read_i64(&self.vec_data[self.index..(self.index+8)]);
            self.index = self.index + 8;
            value
        }
        else { 0 }
    }

    pub fn get_f32(&mut self) -> f32
    {
        if self.index + 4 <= self.vec_data.len() {
            let value: f32 = LittleEndian::read_f32(&self.vec_data[self.index..(self.index+4)]);
            self.index = self.index + 4;
            value
        }
        else { 0_f32 }
    }

    pub fn get_f64(&mut self) -> f64
    {
        if self.index + 8 <= self.vec_data.len() {
            let value: f64 = LittleEndian::read_f64(&self.vec_data[self.index..(self.index+8)]);
            self.index = self.index + 8;
            value
        }
        else { 0_f64 }
    }

    pub fn get_vec_clone(&self) -> Vec<u8>
    {
        self.vec_data.clone()
    }
}
