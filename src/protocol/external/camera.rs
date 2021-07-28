use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

use crate::protocol::Serializable;
use crate::communication::extractor::Extractor;


// -- ModeCamera ----------------------------------------------------------------------------------------------
// 카메라 동작 모드
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum ModeCamera {
    #[num_enum(default)]
    None        = 0x00,     // 
    Stop        = 0x01,     // 장치 동작 멈춤
    Error       = 0x02,     // 장치 오류
    
    Camera      = 0x10,     // 카메라 작동
    Recording   = 0x11,     // 영상 저장
}


impl ModeCamera {
    pub fn from_u8(data_u8: u8) -> ModeCamera {
        match ModeCamera::try_from( data_u8 ) {
            Ok(data) => { data },
            _ => { ModeCamera::None },
        }
    }
}


// -- CommandType ----------------------------------------------------------------------------------------------
// 카메라 명령
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum CommandType {
    #[num_enum(default)]
    None                    = 0x00,     // 
    Stop                    = 0x01,     // 장치 동작 멈춤
    
    VideoCapture            = 0x10,     // 영상 촬영 중 한 프레임을 저장
    VideoRecordingStart     = 0x11,     // 영상 저장 시작
    VideoRecordingStop      = 0x12,     // 영상 저장 중단
    
    TakePhoto               = 0x20,     // 영상 촬영을 잠시 중단하고 사진 촬영
}


impl CommandType {
    pub fn from_u8(data_u8: u8) -> CommandType {
        match CommandType::try_from( data_u8 ) {
            Ok(data) => { data },
            _ => { CommandType::None },
        }
    }
}


// -- State -----------------------------------------------------------------------------------------------
#[derive(Debug, Copy, Clone)]
pub struct State {
    pub mode_camera: ModeCamera,
    pub fps: u16,
}


impl State {
    pub fn new() -> State{
        State {
            mode_camera:  ModeCamera::None,
            fps: 0,
        }
    }


    pub const fn size() -> usize { 3 }


    pub fn parse(slice_data: &[u8]) -> Result<State, &'static str> {
        if slice_data.len() == State::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(State{
                mode_camera: ModeCamera::from_u8(ext.get_u8()),
                fps: ext.get_u16(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for State {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.push(self.mode_camera.into());
        vec_data.extend_from_slice(&self.fps.to_le_bytes());

        vec_data
    }
}



// -- Command -----------------------------------------------------------------------------------------------
#[derive(Debug, Copy, Clone)]
pub struct Command {
    pub command_type: CommandType,
}


impl Command {
    pub fn new() -> Command{
        Command {
            command_type:  CommandType::None,
        }
    }


    pub const fn size() -> usize { 1 }


    pub fn parse(slice_data: &[u8]) -> Result<Command, &'static str> {
        if slice_data.len() == Command::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(Command{
                command_type: CommandType::from_u8(ext.get_u8()),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for Command {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.push(self.command_type.into());

        vec_data
    }
}

