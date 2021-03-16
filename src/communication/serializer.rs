/*
    2021.3.11

    Serializer를 추가함.
 */


pub fn add_u8(vec: &mut Vec<u8>, value: u8)
{
    vec.push(value);
}

pub fn add_u16(vec: &mut Vec<u8>, value: u16)
{
    vec.extend_from_slice(&value.to_le_bytes());
}

pub fn add_u32(vec: &mut Vec<u8>, value: u32)
{
    vec.extend_from_slice(&value.to_le_bytes());
}

pub fn add_u64(vec: &mut Vec<u8>, value: u64)
{
    vec.extend_from_slice(&value.to_le_bytes());
}

pub fn add_i8(vec: &mut Vec<u8>, value: i8)
{
    vec.push(value as u8);
}

pub fn add_i16(vec: &mut Vec<u8>, value: i16)
{
    vec.extend_from_slice(&value.to_le_bytes());
}

pub fn add_i32(vec: &mut Vec<u8>, value: i32)
{
    vec.extend_from_slice(&value.to_le_bytes());
}

pub fn add_i64(vec: &mut Vec<u8>, value: i64)
{
    vec.extend_from_slice(&value.to_le_bytes());
}

pub fn add_f32(vec: &mut Vec<u8>, value: f32)
{
    vec.extend_from_slice(&value.to_le_bytes());
}

pub fn add_f64(vec: &mut Vec<u8>, value: f64)
{
    vec.extend_from_slice(&value.to_le_bytes());
}

pub fn add_slice(vec: &mut Vec<u8>, slice: &[u8])
{
    vec.extend_from_slice(slice);
}

