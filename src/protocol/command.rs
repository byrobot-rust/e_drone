use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

use crate::protocol::Serializable;
use crate::communication::extractor::Extractor;


// -- CommandType -------------------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum CommandType {
    #[num_enum(default)]
    None = 0x00, // 이벤트 없음

    Stop = 0x01, // 정지

    ModeControlFlight = 0x02,   // 비행 제어 모드 설정
    Headless = 0x03,            // 헤드리스 모드 설정
    ControlSpeed = 0x04,        // 제어 속도 설정

    ClearBias = 0x05,           // 자이로/엑셀 바이어스 리셋(트림도 같이 초기화 됨)
    ClearTrim = 0x06,           // 트림 초기화

    FlightEvent = 0x07,         // 비행 이벤트 실행

    SetDefault = 0x08,          // 기본 설정으로 초기화
    Backlight = 0x09,           // 조종기 백라이트 설정
    ModeController = 0x0A,      // 조종기 동작 모드(0x10:조종, 0x80:링크)
    Link = 0x0B,                // 링크 제어(0:Client Mode, 1:Server Mode, 2:Pairing Start)
    ClearMagnetometer = 0x0C,   // 지자계 센서 초기화

    // 관리자
    ClearCounter = 0xA0,        // 카운터 클리어(관리자 권한을 획득했을 경우에만 동작)
    JumpToBootloader = 0xA1,    // 부트로더로 이동
    JumpToApplication = 0xA2,   // 앱으로 이동

    // Navigation
    NavigationTargetClear = 0xE0,   // 네비게이션 목표점 초기화
    NavigationStart = 0xE1,         // 네비게이션 시작(처음부터)
    NavigationPause = 0xE2,         // 네비게이션 일시 정지
    NavigationRestart = 0xE3,       // 네비게이션 다시 시작(일시 정지 후 다시 시작할 때 사용)
    NavigationStop = 0xE4,          // 네비게이션 중단
    NavigationNext = 0xE5,          // 네비게이션 목표점을 다음으로 변경
    NavigationReturnToHome = 0xE6,  // 시작 위치로 귀환

    GpsRtkBase = 0xEA,
    GpsRtkRover = 0xEB,

    TestLock = 0xF0,                // 테스트 락(테스트를 완료하기 전까진 사용 불가 / 27:활성화)
}


impl CommandType {
    pub fn from_u8(command_type_u8: u8) -> CommandType {
        match CommandType::try_from( command_type_u8 ) {
            Ok(command_type) => { command_type },
            _ => { CommandType::None },
        }
    }
}



// -- Command -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct Command {
    pub command_type: CommandType,
    pub option: u8,
}


impl Command {
    pub fn new() -> Command{
        Command {
            command_type: CommandType::None,
            option: 0,
        }
    }

    pub fn parse(slice_data: &[u8]) -> Result<Command, &'static str> {
        if slice_data.len() == Command::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(Command{
                command_type: CommandType::from_u8(ext.get_u8()),
                option: ext.get_u8(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for Command {
    fn size() -> usize { 2 }


    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.push(self.command_type.into());
        vec_data.extend_from_slice(&self.option.to_le_bytes());

        vec_data
    }
}
