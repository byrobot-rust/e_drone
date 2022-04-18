use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

use crate::protocol::Serializable;
use crate::communication::extractor::Extractor;


// -- Position -----------------------------------------------------------------------------------------------
#[derive(Debug, Copy, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub error: u8,
}


impl Position {
    pub fn new() -> Position{
        Position {
            x: 0.0_f32,
            y: 0.0_f32,
            z: 0.0_f32,
            error: 0_u8,
        }
    }


    pub const fn size() -> usize { 13 }


    pub fn parse(slice_data: &[u8]) -> Result<Position, &'static str> {
        if slice_data.len() == Position::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(Position{
                x: ext.get_f32(),
                y: ext.get_f32(),
                z: ext.get_f32(),
                error: ext.get_u8(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for Position {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.x.to_le_bytes());
        vec_data.extend_from_slice(&self.y.to_le_bytes());
        vec_data.extend_from_slice(&self.z.to_le_bytes());
        vec_data.extend_from_slice(&self.error.to_le_bytes());

        vec_data
    }
}

