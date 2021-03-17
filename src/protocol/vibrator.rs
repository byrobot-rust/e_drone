use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

use crate::protocol::Serializable;
use crate::communication::extractor::Extractor;


// -- Mode ----------------------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Mode {
    #[num_enum(default)]
    Stop            = 0,    // 정지
    
    Instantly       = 1,    // 즉시 적용
    Continually     = 2,    // 예약
}


impl Mode {
    pub fn from_u8(data: u8) -> Mode {
        match Mode::try_from( data ) {
            Ok(mode) => { mode },
            _ => { Mode::Stop },
        }
    }
}



// -- Vibrator -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct Vibrator {
    pub mode: Mode,
    pub on: u16,
    pub off: u16,
    pub time: u16,
}


impl Vibrator {
    pub fn new() -> Vibrator{
        Vibrator {
            mode: Mode::Instantly,
            on: 0,
            off: 0,
            time: 0,
        }
    }

    pub fn parse(slice_data: &[u8]) -> Result<Vibrator, &'static str> {
        if slice_data.len() == Vibrator::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(Vibrator{
                mode: Mode::from_u8(ext.get_u8()),
                on: ext.get_u16(),
                off: ext.get_u16(),
                time: ext.get_u16(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for Vibrator {
    fn size() -> usize { 7 }


    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.push(self.mode.into());
        vec_data.extend_from_slice(&self.on.to_le_bytes());
        vec_data.extend_from_slice(&self.off.to_le_bytes());
        vec_data.extend_from_slice(&self.time.to_le_bytes());

        vec_data
    }
}

