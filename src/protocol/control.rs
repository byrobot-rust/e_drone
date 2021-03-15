use crate::protocol::Serializable;
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

