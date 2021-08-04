use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

use crate::protocol::Serializable;
use crate::communication::extractor::Extractor;


// -- ModeSystem ----------------------------------------------------------------------------------------------
// 동작 모드
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum ModeSystem {
    #[num_enum(default)]
    None        = 0x00,     // 
    Stop        = 0x01,     // 장치 동작 멈춤
    Error       = 0x02,     // 장치 오류
    
    Run         = 0x10,     // 시스템 작동
}


impl ModeSystem {
    pub fn from_u8(data_u8: u8) -> ModeSystem {
        match ModeSystem::try_from( data_u8 ) {
            Ok(data) => { data },
            _ => { ModeSystem::None },
        }
    }
}


// -- CommandType ----------------------------------------------------------------------------------------------
// 명령
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum CommandType {
    #[num_enum(default)]
    None                    = 0x00,     // 
    Shutdown                = 0x01,     // 장치 종료
    Reboot                  = 0x02,     // 장치 재시작
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
    pub mode_system: ModeSystem,
    pub fps: u16,
}


impl State {
    pub fn new() -> State{
        State {
            mode_system:  ModeSystem::None,
            fps: 0,
        }
    }


    pub const fn size() -> usize { 3 }


    pub fn parse(slice_data: &[u8]) -> Result<State, &'static str> {
        if slice_data.len() == State::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(State{
                mode_system: ModeSystem::from_u8(ext.get_u8()),
                fps: ext.get_u16(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for State {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.push(self.mode_system.into());
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

