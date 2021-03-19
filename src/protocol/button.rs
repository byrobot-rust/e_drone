use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

use crate::protocol::Serializable;
use crate::communication::extractor::Extractor;


// -- Event ----------------------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Event {
    #[num_enum(default)]
    None,
    
    Down,               // 누르기 시작
    Press,              // 누르는 중
    Up,                 // 뗌
    
    EndContinuePress    // 연속 입력 종료
}


impl Event {
    pub fn from_u8(data_u8: u8) -> Event {
        match Event::try_from( data_u8 ) {
            Ok(data) => { data },
            _ => { Event::None },
        }
    }
}


// -- Button -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct Button {
    pub button: u16,
    pub event: Event,
}


impl Button {
    pub fn new() -> Button{
        Button {
            button: 0,
            event: Event::None,
        }
    }


    pub fn parse(slice_data: &[u8]) -> Result<Button, &'static str> {
        if slice_data.len() == Button::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(Button{
                button: ext.get_u16(),
                event: Event::from_u8(ext.get_u8()),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for Button {
    fn size() -> usize { 3 }


    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.button.to_le_bytes());
        vec_data.push(self.event.into());

        vec_data
    }
}


