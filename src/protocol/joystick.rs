use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

use crate::protocol::Serializable;
use crate::communication::extractor::Extractor;


// -- Direction ----------------------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Direction {
    #[num_enum(default)]
    None    = 0x00,     // 정의하지 않은 영역(무시함)
    
    VT      = 0x10,     //   위(세로)
    VM      = 0x20,     // 중앙(세로)
    VB      = 0x40,     // 아래(세로)
    
    HL      = 0x01,     //   왼쪽(가로)
    HM      = 0x02,     //   중앙(가로)
    HR      = 0x04,     // 오른쪽(가로)
    
    TL = 0x11,  TM = 0x12,  TR = 0x14,
    ML = 0x21,  CN = 0x22,  MR = 0x24,
    BL = 0x41,  BM = 0x42,  BR = 0x44,
}


impl Direction {
    pub fn from_u8(data_u8: u8) -> Direction {
        match Direction::try_from( data_u8 ) {
            Ok(data) => { data },
            _ => { Direction::None },
        }
    }
}


// -- Event ----------------------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Event {
    #[num_enum(default)]
    None        = 0x00,     // 이벤트 없음
    
    In          = 0x01,     // 특정 영역에 진입
    Stay        = 0x02,     // 특정 영역에서 상태 유지
    Out         = 0x03,     // 특정 영역에서 벗어남
    
    Calibration = 0x04,     // 캘리브레이션 중
}


impl Event {
    pub fn from_u8(data_u8: u8) -> Event {
        match Event::try_from( data_u8 ) {
            Ok(data) => { data },
            _ => { Event::None },
        }
    }
}


// -- Command ----------------------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Command {
    #[num_enum(default)]
    None                = 0x00,
    UpDownUpDown        = 0x01,     // 위-아래-위-아래
    LeftRightLeftRight  = 0x02,     // 좌-우-좌-우
    RightLeftRightLeft  = 0x03,     // 우-좌-우-좌
    TurnLeft            = 0x04,     // 좌회전
    TurnRight           = 0x05,     // 우회전
}


impl Command {
    pub fn from_u8(data_u8: u8) -> Command {
        match Command::try_from( data_u8 ) {
            Ok(data) => { data },
            _ => { Command::None },
        }
    }
}


// -- JoystickBlock -----------------------------------------------------------------------------------------------
#[derive(Debug, Copy, Clone)]
pub struct JoystickBlock {
    pub x: i8,
    pub y: i8,
    pub direction: Direction,
    pub event: Event,
}


impl JoystickBlock {
    pub fn new() -> JoystickBlock{
        JoystickBlock {
            x: 0,
            y: 0,
            direction: Direction::None,
            event: Event::None,
        }
    }


    pub fn from_slice(slice_data: &[u8]) -> JoystickBlock {
        if slice_data.len() == JoystickBlock::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            JoystickBlock{
                x: ext.get_i8(),
                y: ext.get_i8(),
                direction: Direction::from_u8(ext.get_u8()),
                event: Event::from_u8(ext.get_u8()),
            }
        }
        else { JoystickBlock::new() }
    }


    pub const fn size() -> usize { 4 }


    pub fn parse(slice_data: &[u8]) -> Result<JoystickBlock, &'static str> {
        if slice_data.len() == JoystickBlock::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(JoystickBlock{
                x: ext.get_i8(),
                y: ext.get_i8(),
                direction: Direction::from_u8(ext.get_u8()),
                event: Event::from_u8(ext.get_u8()),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for JoystickBlock {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.push(self.x as u8);
        vec_data.push(self.y as u8);
        vec_data.push(self.direction.into());
        vec_data.push(self.event.into());

        vec_data
    }
}


// -- Joystick -----------------------------------------------------------------------------------------------
#[derive(Debug, Copy, Clone)]
pub struct Joystick {
    pub left: JoystickBlock,
    pub right: JoystickBlock,
}


impl Joystick {
    pub fn new() -> Joystick{
        Joystick {
            left: JoystickBlock::new(),
            right: JoystickBlock::new(),
        }
    }


    pub const fn size() -> usize { JoystickBlock::size() * 2 }


    pub fn parse(slice_data: &[u8]) -> Result<Joystick, &'static str> {
        if slice_data.len() == Joystick::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(Joystick{
                left: JoystickBlock::from_slice(&ext.get_array(JoystickBlock::size())),
                right: JoystickBlock::from_slice(&ext.get_array(JoystickBlock::size())),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for Joystick {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.left.to_vec());
        vec_data.extend_from_slice(&self.right.to_vec());

        vec_data
    }
}

