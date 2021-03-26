use crate::protocol::Serializable;
use crate::communication::extractor::Extractor;


// -- LostConnection -----------------------------------------------------------------------------------------------
#[derive(Debug, Copy, Clone)]
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


    pub const fn size() -> usize { 8 }


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
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.time_neutral.to_le_bytes());
        vec_data.extend_from_slice(&self.time_landing.to_le_bytes());
        vec_data.extend_from_slice(&self.time_stop.to_le_bytes());

        vec_data
    }
}


// -- Rssi -----------------------------------------------------------------------------------------------
#[derive(Debug, Copy, Clone)]
pub struct Rssi {
    pub rssi: i8,
}


impl Rssi {
    pub fn new() -> Rssi{
        Rssi {
            rssi: 0,
        }
    }


    pub const fn size() -> usize { 1 }


    pub fn parse(slice_data: &[u8]) -> Result<Rssi, &'static str> {
        if slice_data.len() == Rssi::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(Rssi{
                rssi: ext.get_i8(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for Rssi {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.rssi.to_le_bytes());

        vec_data
    }
}


// -- Pairing -----------------------------------------------------------------------------------------------
#[derive(Debug, Copy, Clone)]
pub struct Pairing {
    pub address: [u16; 3],
    pub scramble: u8,
    pub channel: [u8; 4],
}


impl Pairing {
    pub fn new() -> Pairing{
        Pairing {
            address: [0; 3],
            scramble: 0,
            channel: [0; 4],
        }
    }


    pub const fn size() -> usize { 11 }


    pub fn parse(slice_data: &[u8]) -> Result<Pairing, &'static str> {
        if slice_data.len() == Pairing::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(Pairing{
                address: [ext.get_u16(), ext.get_u16(), ext.get_u16()],
                scramble: ext.get_u8(),
                channel: [ext.get_u8(), ext.get_u8(), ext.get_u8(), ext.get_u8()],
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for Pairing {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.address[0].to_le_bytes());
        vec_data.extend_from_slice(&self.address[1].to_le_bytes());
        vec_data.extend_from_slice(&self.address[2].to_le_bytes());
        vec_data.extend_from_slice(&self.scramble.to_le_bytes());
        vec_data.extend_from_slice(&self.channel[0].to_le_bytes());
        vec_data.extend_from_slice(&self.channel[1].to_le_bytes());
        vec_data.extend_from_slice(&self.channel[2].to_le_bytes());
        vec_data.extend_from_slice(&self.channel[3].to_le_bytes());

        vec_data
    }
}


