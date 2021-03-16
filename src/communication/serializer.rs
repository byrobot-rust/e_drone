/*
    2021.3.11

    Serializer를 추가함.
 */

use byteorder::{ByteOrder, LittleEndian};


pub fn add_u8(vec: &mut Vec<u8>, value: u8)
{
    vec.push(value);
}

pub fn add_u16(vec: &mut Vec<u8>, value: u16)
{
    let mut array = [0; 2];
    LittleEndian::write_u16(&mut array, value);
    vec.extend_from_slice(&array);
}

pub fn add_u32(vec: &mut Vec<u8>, value: u32)
{
    let mut array = [0; 4];
    LittleEndian::write_u32(&mut array, value);
    vec.extend_from_slice(&array);
}

pub fn add_u64(vec: &mut Vec<u8>, value: u64)
{
    let mut array = [0; 8];
    LittleEndian::write_u64(&mut array, value);
    vec.extend_from_slice(&array);
}

pub fn add_i8(vec: &mut Vec<u8>, value: i8)
{
    vec.push(value as u8);
}

pub fn add_i16(vec: &mut Vec<u8>, value: i16)
{
    let mut array = [0; 2];
    LittleEndian::write_i16(&mut array, value);
    vec.extend_from_slice(&array);
}

pub fn add_i32(vec: &mut Vec<u8>, value: i32)
{
    let mut array = [0; 4];
    LittleEndian::write_i32(&mut array, value);
    vec.extend_from_slice(&array);
}

pub fn add_i64(vec: &mut Vec<u8>, value: i64)
{
    let mut array = [0; 8];
    LittleEndian::write_i64(&mut array, value);
    vec.extend_from_slice(&array);
}

pub fn add_f32(vec: &mut Vec<u8>, value: f32)
{
    let mut array = [0; 4];
    LittleEndian::write_f32(&mut array, value);
    vec.extend_from_slice(&array);
}

pub fn add_f64(vec: &mut Vec<u8>, value: f64)
{
    let mut array = [0; 8];
    LittleEndian::write_f64(&mut array, value);
    vec.extend_from_slice(&array);
}

pub fn add_slice(vec: &mut Vec<u8>, slice: &[u8])
{
    vec.extend_from_slice(slice);
}

