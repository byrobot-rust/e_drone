use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
use byteorder::{ByteOrder, LittleEndian};

use crate::protocol::Serializable;
use crate::communication::extractor::Extractor;


// -- Mode ----------------------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Mode {
    #[num_enum(default)]
    Stop                = 0,    // 정지(Mode에서의 Stop은 통신에서 받았을 때 Buzzer를 끄는 용도로 사용, set으로만 호출)
    
    MuteInstantly       = 1,    // 묵음 즉시 적용
    MuteContinually     = 2,    // 묵음 예약
    
    ScaleInstantly      = 3,    // 음계 즉시 적용
    ScaleContinually    = 4,    // 음계 예약
    
    HzInstantly         = 5,    // 주파수 즉시 적용
    HzContinually       = 6,    // 주파수 예약
}


impl Mode {
    pub fn from_u8(data_u8: u8) -> Mode {
        match Mode::try_from( data_u8 ) {
            Ok(data) => { data },
            _ => { Mode::Stop },
        }
    }
}


// -- Scale ----------------------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
pub enum Scale {
    #[num_enum(default)]

    C1, CS1, D1, DS1, E1, F1, FS1, G1, GS1, A1, AS1, B1,
    C2, CS2, D2, DS2, E2, F2, FS2, G2, GS2, A2, AS2, B2,
    C3, CS3, D3, DS3, E3, F3, FS3, G3, GS3, A3, AS3, B3,
    C4, CS4, D4, DS4, E4, F4, FS4, G4, GS4, A4, AS4, B4,
    C5, CS5, D5, DS5, E5, F5, FS5, G5, GS5, A5, AS5, B5,
    C6, CS6, D6, DS6, E6, F6, FS6, G6, GS6, A6, AS6, B6,
    C7, CS7, D7, DS7, E7, F7, FS7, G7, GS7, A7, AS7, B7,
    C8, CS8, D8, DS8, E8, F8, FS8, G8, GS8, A8, AS8, B8,

    Mute    = 0xEE,     // 묵음
    Fin     = 0xFF,     // 악보의 끝
}


impl Scale {
    pub fn from_u16(data_u8: u16) -> Scale {
        match Scale::try_from( data_u8 ) {
            Ok(data) => { data },
            _ => { Scale::Mute },
        }
    }

    pub fn to_array(&self) -> [u8; 2] {
        let mut buf = [0; 2];
        LittleEndian::write_u16(&mut buf, self.clone().into());
        buf
    }
}


// -- Melody -----------------------------------------------------------------------------------------------
#[derive(Debug, Copy, Clone)]
pub struct Melody {
    pub melody: u8,
    pub repeat: u8,
}


impl Melody {
    pub fn new() -> Melody{
        Melody {
            melody: 0,
            repeat: 0,
        }
    }


    pub const fn size() -> usize { 2 }


    pub fn parse(slice_data: &[u8]) -> Result<Melody, &'static str> {
        if slice_data.len() == Melody::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(Melody{
                melody: ext.get_u8(),
                repeat: ext.get_u8(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for Melody {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.push(self.melody);
        vec_data.push(self.repeat);

        vec_data
    }
}


// -- BuzzerScale -----------------------------------------------------------------------------------------------
#[derive(Debug, Copy, Clone)]
pub struct BuzzerScale {
    pub mode: Mode,
    pub scale: Scale,
    pub time: u16,
}


impl BuzzerScale {
    pub fn new() -> BuzzerScale{
        BuzzerScale {
            mode: Mode::ScaleInstantly,
            scale: Scale::C5,
            time: 0,
        }
    }


    pub const fn size() -> usize { 5 }


    pub fn parse(slice_data: &[u8]) -> Result<BuzzerScale, &'static str> {
        if slice_data.len() == BuzzerScale::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(BuzzerScale{
                mode: Mode::from_u8(ext.get_u8()),
                scale: Scale::from_u16(ext.get_u16()),
                time: ext.get_u16(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for BuzzerScale {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.push(self.mode.into());
        vec_data.extend_from_slice(&self.scale.to_array());
        vec_data.extend_from_slice(&self.time.to_le_bytes());

        vec_data
    }
}


// -- BuzzerHz -----------------------------------------------------------------------------------------------
#[derive(Debug, Copy, Clone)]
pub struct BuzzerHz {
    pub mode: Mode,
    pub hz: u16,
    pub time: u16,
}


impl BuzzerHz {
    pub fn new() -> BuzzerHz{
        BuzzerHz {
            mode: Mode::HzInstantly,
            hz: 0,
            time: 0,
        }
    }


    pub const fn size() -> usize { 5 }


    pub fn parse(slice_data: &[u8]) -> Result<BuzzerHz, &'static str> {
        if slice_data.len() == BuzzerHz::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(BuzzerHz{
                mode: Mode::from_u8(ext.get_u8()),
                hz: ext.get_u16(),
                time: ext.get_u16(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for BuzzerHz {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.push(self.mode.into());
        vec_data.extend_from_slice(&self.hz.to_le_bytes());
        vec_data.extend_from_slice(&self.time.to_le_bytes());

        vec_data
    }
}

