use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

use crate::protocol::Serializable;
use crate::communication::extractor::Extractor;


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
    pub fn from_u8(data_u8: u8) -> ModeLight {
        match ModeLight::try_from( data_u8 ) {
            Ok(data) => { data },
            _ => { ModeLight::BodyNone },
        }
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


    pub fn parse(slice_data: &[u8]) -> Result<Color, &'static str> {
        if slice_data.len() == Color::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(Color{
                r: ext.get_u8(),
                g: ext.get_u8(),
                b: ext.get_u8(),
            })
        }
        else { Err("Wrong length") }
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
    pub interval: u16,
}


impl Mode {
    pub fn new() -> Mode{
        Mode {
            mode: 0,
            interval: 0,
        }
    }


    pub fn parse(slice_data: &[u8]) -> Result<Mode, &'static str> {
        if slice_data.len() == Mode::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(Mode{
                mode: ext.get_u8(),
                interval: ext.get_u16(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for Mode {
    fn size() -> usize { 3 }


    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.mode.to_le_bytes());
        vec_data.extend_from_slice(&self.interval.to_le_bytes());

        vec_data
    }
}


// -- Event -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct Event {
    pub event: u8,
    pub interval: u16,
    pub repeat: u8,
}


impl Event {
    pub fn new() -> Event{
        Event {
            event: 0,
            interval: 0,
            repeat: 0,
        }
    }


    pub fn parse(slice_data: &[u8]) -> Result<Event, &'static str> {
        if slice_data.len() == Event::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(Event{
                event: ext.get_u8(),
                interval: ext.get_u16(),
                repeat: ext.get_u8(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for Event {
    fn size() -> usize { 4 }


    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.event.to_le_bytes());
        vec_data.extend_from_slice(&self.interval.to_le_bytes());
        vec_data.extend_from_slice(&self.repeat.to_le_bytes());

        vec_data
    }
}


// -- ModeColor -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct ModeColor {
    pub mode: Mode,
    pub color: Color,
}


impl ModeColor {
    pub fn new() -> ModeColor{
        ModeColor {
            mode: Mode::new(),
            color: Color::new(),
        }
    }


    pub fn parse(slice_data: &[u8]) -> Result<ModeColor, &'static str> {
        if slice_data.len() == ModeColor::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(ModeColor{
                mode: Mode{
                    mode: ext.get_u8(),
                    interval: ext.get_u16(),
                },
                color: Color {
                    r: ext.get_u8(),
                    g: ext.get_u8(),
                    b: ext.get_u8(),
                },
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for ModeColor {
    fn size() -> usize { 6 }


    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.mode.mode.to_le_bytes());
        vec_data.extend_from_slice(&self.mode.interval.to_le_bytes());
        vec_data.extend_from_slice(&self.color.r.to_le_bytes());
        vec_data.extend_from_slice(&self.color.g.to_le_bytes());
        vec_data.extend_from_slice(&self.color.b.to_le_bytes());

        vec_data
    }
}


// -- EventColor -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct EventColor {
    pub event: Event,
    pub color: Color,
}


impl EventColor {
    pub fn new() -> EventColor{
        EventColor {
            event: Event::new(),
            color: Color::new(),
        }
    }


    pub fn parse(slice_data: &[u8]) -> Result<EventColor, &'static str> {
        if slice_data.len() == EventColor::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok( EventColor{
                event: Event {
                    event: ext.get_u8(),
                    interval: ext.get_u16(),
                    repeat: ext.get_u8(),
                },
                color: Color {
                    r: ext.get_u8(),
                    g: ext.get_u8(),
                    b: ext.get_u8(),
                },
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for EventColor {
    fn size() -> usize { 7 }


    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.event.event.to_le_bytes());
        vec_data.extend_from_slice(&self.event.interval.to_le_bytes());
        vec_data.extend_from_slice(&self.event.repeat.to_le_bytes());
        vec_data.extend_from_slice(&self.color.r.to_le_bytes());
        vec_data.extend_from_slice(&self.color.g.to_le_bytes());
        vec_data.extend_from_slice(&self.color.b.to_le_bytes());

        vec_data
    }
}


