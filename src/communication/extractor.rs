/*
    2021.3.11

    Extractor를 사용하는 이유는 슬라이스로 된 데이터 배열에서
    구조체를 추출하는 과정에서 인덱스를 잘못 지정하기가 매우 쉽고,
    그 과정에서 중간에 인덱스의 변경 사항이 발생하면
    해당 데이터 이후의 모든 인덱스를 수정해주어야 하는 불편함이 있기 때문.
    또한 코드가 길어지고 읽기가 어려워지는 것을 줄이기 위함.

    데이터 반환형이 Result가 아닌 이유는 데이터 파싱 시작 전에 이미
    데이터의 길이를 확인한다고 가정하기 때문임.
    Result형으로 반환하는 경우 Extractor를 주로 사용하는 parse 함수에서
    코드가 필요 이상으로 길어지고 지저분해질 수 있다.
 */

use byteorder::{ByteOrder, LittleEndian};

pub struct Extractor
{
    vec_data: Vec<u8>,
    index : usize,
}

impl Extractor
{
    pub fn new() -> Extractor {
        Extractor {
            vec_data: Vec::new(),
            index: 0,
        }
    }

    pub fn from_vec(source: &Vec<u8>) -> Extractor {
        Extractor {
            vec_data: source.clone(),
            index: 0,
        }
    }

    pub fn from_slice(source:  &[u8]) -> Extractor {
        Extractor {
            vec_data: source.to_vec(),
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

    pub fn get_array(&mut self, length : usize) -> Vec<u8>
    {
        if self.index + length <= self.vec_data.len() {
            let value = self.vec_data[self.index..(self.index+length)].to_vec();
            self.index = self.index + length;
            return value;
        }

        [].to_vec()
    }
    
    pub fn get_vec_clone(&self) -> Vec<u8>
    {
        self.vec_data.clone()
    }
}
