use crate::protocol::{*};
use crate::communication::extractor::Extractor;


// -- Quad8 -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct Quad8 {
    pub roll: i8,
    pub pitch: i8,
    pub yaw: i8,
    pub throttle: i8,
}


impl Quad8 {
    pub fn new() -> Quad8{
        Quad8 {
            roll: 0,
            pitch: 0,
            yaw: 0,
            throttle: 0,
        }
    }
    
    pub fn parse(slice_data: &[u8]) -> Result<Quad8, &'static str> {
        if slice_data.len() == Quad8::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(Quad8{
                roll: ext.get_i8(),
                pitch: ext.get_i8(),
                yaw: ext.get_i8(),
                throttle: ext.get_i8(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for Quad8 {
    fn size() -> usize { 4 }


    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.roll.to_le_bytes());
        vec_data.extend_from_slice(&self.pitch.to_le_bytes());
        vec_data.extend_from_slice(&self.yaw.to_le_bytes());
        vec_data.extend_from_slice(&self.throttle.to_le_bytes());

        vec_data
    }
}


// -- Quad8AndRequestData -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct Quad8AndRequestData {
    pub roll: i8,
    pub pitch: i8,
    pub yaw: i8,
    pub throttle: i8,
    pub data_type: DataType,
}


impl Quad8AndRequestData {
    pub fn new() -> Quad8AndRequestData{
        Quad8AndRequestData {
            roll: 0,
            pitch: 0,
            yaw: 0,
            throttle: 0,
            data_type: DataType::None,
        }
    }
    
    pub fn parse(slice_data: &[u8]) -> Result<Quad8AndRequestData, &'static str> {
        if slice_data.len() == Quad8AndRequestData::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(Quad8AndRequestData{
                roll: ext.get_i8(),
                pitch: ext.get_i8(),
                yaw: ext.get_i8(),
                throttle: ext.get_i8(),
                data_type: DataType::from_u8(ext.get_u8()),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for Quad8AndRequestData {
    fn size() -> usize { 5 }


    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.roll.to_le_bytes());
        vec_data.extend_from_slice(&self.pitch.to_le_bytes());
        vec_data.extend_from_slice(&self.yaw.to_le_bytes());
        vec_data.extend_from_slice(&self.throttle.to_le_bytes());
        vec_data.push(self.data_type.into());

        vec_data
    }
}


// -- Position16 -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct Position16 {
    pub position_x: i16,
    pub position_y: i16,
    pub position_z: i16,
    pub velocity: i16,
    pub heading: i16,
    pub rotational_velocity: i16,
}


impl Position16 {
    pub fn new() -> Position16{
        Position16 {
            position_x: 0,
            position_y: 0,
            position_z: 0,
            velocity: 0,
            heading: 0,
            rotational_velocity: 0,
        }
    }
    
    pub fn parse(slice_data: &[u8]) -> Result<Position16, &'static str> {
        if slice_data.len() == Position16::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(Position16{
                position_x: ext.get_i16(),
                position_y: ext.get_i16(),
                position_z: ext.get_i16(),
                velocity: ext.get_i16(),
                heading: ext.get_i16(),
                rotational_velocity: ext.get_i16(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for Position16 {
    fn size() -> usize { 12 }


    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.position_x.to_le_bytes());
        vec_data.extend_from_slice(&self.position_y.to_le_bytes());
        vec_data.extend_from_slice(&self.position_z.to_le_bytes());
        vec_data.extend_from_slice(&self.velocity.to_le_bytes());
        vec_data.extend_from_slice(&self.heading.to_le_bytes());
        vec_data.extend_from_slice(&self.rotational_velocity.to_le_bytes());

        vec_data
    }
}


// -- Position -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub velocity: f32,
    pub heading: i16,
    pub rotational_velocity: i16,
}


impl Position {
    pub fn new() -> Position{
        Position {
            x: 0.0_f32,
            y: 0.0_f32,
            z: 0.0_f32,
            velocity: 0.0_f32,
            heading: 0,
            rotational_velocity: 0,
        }
    }
    
    pub fn parse(slice_data: &[u8]) -> Result<Position, &'static str> {
        if slice_data.len() == Position::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(Position{
                x: ext.get_f32(),
                y: ext.get_f32(),
                z: ext.get_f32(),
                velocity: ext.get_f32(),
                heading: ext.get_i16(),
                rotational_velocity: ext.get_i16(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for Position {
    fn size() -> usize { 20 }


    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.x.to_le_bytes());
        vec_data.extend_from_slice(&self.y.to_le_bytes());
        vec_data.extend_from_slice(&self.z.to_le_bytes());
        vec_data.extend_from_slice(&self.velocity.to_le_bytes());
        vec_data.extend_from_slice(&self.heading.to_le_bytes());
        vec_data.extend_from_slice(&self.rotational_velocity.to_le_bytes());

        vec_data
    }
}



