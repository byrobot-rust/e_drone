use crate::protocol::Serializable;
use crate::communication::extractor::Extractor;


// -- LostConnection -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct LostConnection {
    pub time_neutral: u16,
    pub time_landing: u16,
    pub time_stop: u32,
}


impl LostConnection {
    pub fn new() -> LostConnection{
        LostConnection {
            time_neutral: 0,
            time_landing: 0,
            time_stop: 0,
        }
    }
    
    pub fn parse(slice_data: &[u8]) -> Result<LostConnection, &'static str> {
        if slice_data.len() == LostConnection::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(LostConnection{
                time_neutral: ext.get_u16(),
                time_landing: ext.get_u16(),
                time_stop: ext.get_u32(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for LostConnection {
    fn size() -> usize { 8 }


    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.time_neutral.to_le_bytes());
        vec_data.extend_from_slice(&self.time_landing.to_le_bytes());
        vec_data.extend_from_slice(&self.time_stop.to_le_bytes());

        vec_data
    }
}

