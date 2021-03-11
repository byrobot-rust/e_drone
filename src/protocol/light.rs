use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
use byteorder::{ByteOrder, LittleEndian};

use crate::communication::extractor::Extractor;
use crate::protocol::Serializable;


// -- ModeLight ----------------------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum ModeLight {
    #[num_enum(default)]
    BodyNone            = 0x20,
    BodyManual          = 0x21,     // 수동 제어
    BodyHold            = 0x22,     // 지정한 색상을 계속 켬
    BodyFlicker         = 0x23,     // 깜빡임
    BodyFlickerDouble   = 0x24,     // 깜빡임(두 번 깜빡이고 깜빡인 시간만큼 꺼짐)
    BodyDimming         = 0x25,     // 밝기 제어하여 천천히 깜빡임
    BodySunrise         = 0x26,     // 꺼진 상태에서 점점 밝아짐
    BodySunset          = 0x27,     // 켜진 상태에서 점점 어두워짐
    BodyRainbow         = 0x28,     // 무지개색
    BodyRainbow2        = 0x29,     // 무지개색
    BodyWarning         = 0x2F,     // 경고
}


impl ModeLight {
    pub fn from_u8(data: u8) -> ModeLight {
        match ModeLight::try_from( data ) {
            Ok(mode_light) => { mode_light },
            _ => { ModeLight::BodyNone },
        }
    }
}


// -- Manual -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct Manual {
    pub flags: u16,
    pub brightness: u8,
}


impl Manual {
    pub fn new() -> Manual{
        Manual {
            flags: 0,
            brightness: 0,
        }
    }

    pub fn parse(slice_data: &[u8]) -> Result<Manual, &'static str> {
        if slice_data.len() == Manual::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(Manual{
                flags: ext.get_u16(),
                brightness: ext.get_u8(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for Manual {
    fn size() -> usize { 3 }


    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.flags.to_le_bytes());
        vec_data.extend_from_slice(&self.brightness.to_le_bytes());

        vec_data
    }
}


// -- Mode -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct Mode {
    pub mode: u8,
    pub brightness: u16,
}


impl Mode {
    pub fn new() -> Mode{
        Mode {
            mode: 0,
            brightness: 0,
        }
    }


    pub fn parse(mode: &mut Mode, vec_data: &Vec<u8>) -> bool {
        if vec_data.len() != Mode::size() {
            return false;
        }

        let mut ext: Extractor = Extractor::from_vec(vec_data);

        mode.mode = ext.get_u8();
        mode.brightness = ext.get_u16();

        true
    }


    pub fn from_vec(vec_data: &Vec<u8>) -> Mode {
        let mut data = Mode::new();
        Mode::parse(&mut data, vec_data);
        data
    }
}


impl Serializable for Mode {
    fn size() -> usize { 3 }


    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.mode.to_le_bytes());
        vec_data.extend_from_slice(&self.brightness.to_le_bytes());

        vec_data
    }
}


// -- Event -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct Event {
    pub event: u8,
    pub brightness: u16,
    pub repeat: u8,
}


impl Event {
    pub fn new() -> Event{
        Event {
            event: 0,
            brightness: 0,
            repeat: 0,
        }
    }


    pub fn parse_slice(event: &mut Event, array_data: &[u8]) -> bool {
        if array_data.len() != Event::size() {
            return false;
        }

        let mut ext: Extractor = Extractor::from_slice(array_data);

        event.event = ext.get_u8();
        event.brightness = ext.get_u16();
        event.repeat = ext.get_u8();

        true
    }


    pub fn parse(event: &mut Event, vec_data: &Vec<u8>) -> bool {
        if vec_data.len() != Event::size() {
            return false;
        }

        let mut ext: Extractor = Extractor::from_vec(vec_data);

        event.event = ext.get_u8();
        event.brightness = ext.get_u16();
        event.repeat = ext.get_u8();

        true
    }


    pub fn from_vec(vec_data: &Vec<u8>) -> Event {
        let mut data = Event::new();
        Event::parse(&mut data, vec_data);
        data
    }
}


impl Serializable for Event {
    fn size() -> usize { 4 }


    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.event.to_le_bytes());
        vec_data.extend_from_slice(&self.brightness.to_le_bytes());
        vec_data.extend_from_slice(&self.repeat.to_le_bytes());

        vec_data
    }
}


// -- Color -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}


impl Color {
    pub fn new() -> Color{
        Color {
            r: 0,
            g: 0,
            b: 0,
        }
    }


    pub fn parse(color: &mut Color, vec_data: &Vec<u8>) -> bool {
        if vec_data.len() != Color::size() {
            return false;
        }

        let mut ext: Extractor = Extractor::from_vec(vec_data);

        color.r = ext.get_u8();
        color.g = ext.get_u8();
        color.b = ext.get_u8();

        true
    }


    pub fn from_vec(vec_data: &Vec<u8>) -> Color {
        let mut data = Color::new();
        Color::parse(&mut data, vec_data);
        data
    }
}


impl Serializable for Color {
    fn size() -> usize { 3 }


    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.r.to_le_bytes());
        vec_data.extend_from_slice(&self.g.to_le_bytes());
        vec_data.extend_from_slice(&self.b.to_le_bytes());

        vec_data
    }
}



