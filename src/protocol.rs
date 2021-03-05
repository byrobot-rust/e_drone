use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
use core::convert::TryInto;

pub mod common;

use crate::base::system::{*};


// -- DataType ----------------------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum DataType {
    #[num_enum(default)]
    None = 0x00,                // 없음

    Ping = 0x01,                // 통신 확인
    Ack = 0x02,                 // 데이터 수신에 대한 응답
    Error = 0x03,               // 오류
    Request = 0x04,             // 지정한 타입의 데이터 요청
    Message = 0x05,             // 문자열 데이터
    Address = 0x06,             // 장치 주소(MAC이 있는 경우 MAC) 혹은 고유번호(MAC이 없는 경우 UUID)
    Information = 0x07,         // 펌웨어 및 장치 정보
    Update = 0x08,              // 펌웨어 업데이트
    UpdateLocation = 0x09,      // 펌웨어 업데이트 위치 정정
    Encrypt = 0x0A,             // 펌웨어 암호화
    SystemCount = 0x0B,         // 시스템 카운트
    SystemInformation = 0x0C,   // 시스템 정보
    Registration = 0x0D,        // 제품 등록(암호화 데이터 및 등록 데이터를 데이터 길이로 구분)
    Administrator = 0x0E,       // 관리자 권한 획득
    Monitor = 0x0F,             // 디버깅용 값 배열 전송. 첫번째 바이트에 타입, 두 번째 바이트에 페이지 지정(수신 받는 데이터의 저장 경로 구분)
    Control = 0x10,             // 조종

    Command = 0x11,             // 명령
    Pairing = 0x12,             // 페어링
    Rssi = 0x13,                // RSSI
    TimeSync = 0x14,            // 시간 동기화
    TransmissionPower = 0x15,   // 전송 출력
    Configuration = 0x16,       // 설정
    Echo = 0x17,                // 반향(정상적으로 송수신 되는 데이터 길이 확인용, 받은 데이터를 그대로 반환, RF로 송수신 가능한 데이터 길이를 확인할 목적으로 추가)

    Battle = 0x1F,              // 전투

    // Light
    LightManual = 0x20,         // LED 수동 제어
    LightMode = 0x21,           // LED 모드 지정
    LightEvent = 0x22,          // LED 이벤트
    LightDefault = 0x23,        // LED 기본 색상

    // 센서 RAW 데이터
    RawMotion = 0x30,           // Motion 센서 데이터 RAW 값
    RawFlow = 0x31,             // Flow 센서 데이터 RAW 값

    // 상태,  센서
    State = 0x40,               // 드론의 상태(비행 모드, 방위기준, 배터리량)
    Attitude = 0x41,            // 드론의 자세(Angle)(Attitude)
    Position = 0x42,            // 위치
    Altitude = 0x43,            // 높이, 고도
    Motion = 0x44,              // Motion 센서 데이터 처리한 값(IMU)
    Range = 0x45,               // 거리센서 데이터

    // 설정
    Count = 0x50,               // 카운트
    Bias = 0x51,                // 엑셀, 자이로 바이어스 값
    Trim = 0x52,                // 트림
    Weight = 0x53,              // 무게 설정
    LostConnection = 0x54,      // 연결이 끊긴 후 반응 시간 설정
    MagnetometerOffset = 0x55,  // 지자계 센서 오프셋 조정

    // Device
    Motor = 0x60,               // 모터 제어 및 현재 제어값 확인
    MotorSingle = 0x61,         // 한 개의 모터 제어
    Buzzer = 0x62,              // 버저 제어
    Vibrator = 0x63,            // 진동 제어

    // Input
    Button = 0x70,      // 버튼
    Joystick = 0x71,    // 조이스틱

    // Display
    DisplayClear = 0x80,            // 화면 지우기
    DisplayInvert = 0x81,           // 화면 반전
    DisplayDrawPoint = 0x82,        // 점 그리기
    DisplayDrawLine = 0x83,         // 선 그리기
    DisplayDrawRect = 0x84,         // 사각형 그리기
    DisplayDrawCircle = 0x85,       // 원 그리기
    DisplayDrawString = 0x86,       // 문자열 쓰기
    DisplayDrawStringAlign = 0x87,  // 문자열 쓰기
    DisplayDrawImage = 0x88,        // 그림 그리기

    // Card
    CardClassify = 0x90,            // 카드 색상 분류 기준 설정
    CardRange = 0x91,               // 카드 색 범위(RAW 데이터의 출력 범위)
    CardRaw = 0x92,                 // 카드 데이터 RAW 값(유선으로만 전송)
    CardColor = 0x93,               // 카드 데이터
    CardList = 0x94,                // 카드 리스트 데이터
    CardFunctionList = 0x95,        // 카드 함수 리스트 데이터

    // Information Assembled
    InformationAssembledForController = 0xA0, // 데이터 모음
    InformationAssembledForEntry = 0xA1,      // 데이터 모음
    InformationAssembledForByBlocks = 0xA2,   // 데이터 모음

    // Navigation
    NavigationTarget = 0xD0,   // 네비게이션 목표점
    NavigationLocation = 0xD1, // 네비게이션 드론 위치
    NavigationMonitor = 0xD2,
    NavigationHeading = 0xD3,
    NavigationCounter = 0xD4,
    NavigationSatellite = 0xD5,      // 위성 정보
    NavigationLocationAdjust = 0xD6, // 드론 위치 조정

    NavigationTargetEcef = 0xD8,   // 드론 타겟 위치(ECEF)
    NavigationLocationEcef = 0xD9, // 드론 현재 위치(ECEF)

    GpsRtkNavigationState = 0xDA,            // RTK RAW 데이터 전송
    GpsRtkExtendedRawMeasurementData = 0xDB, // RTK RAW 데이터 전송
}


// -- CommandType -------------------------------------------------------------------------------------------
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, TryFromPrimitive)]
pub enum CommandType {
    #[num_enum(default)]
    None = 0x00, // 이벤트 없음

    Stop = 0x01, // 정지

    ModeControlFlight = 0x02, // 비행 제어 모드 설정
    Headless = 0x03,          // 헤드리스 모드 설정
    ControlSpeed = 0x04,      // 제어 속도 설정

    ClearBias = 0x05, // 자이로/엑셀 바이어스 리셋(트림도 같이 초기화 됨)
    ClearTrim = 0x06, // 트림 초기화

    FlightEvent = 0x07, // 비행 이벤트 실행

    SetDefault = 0x08,        // 기본 설정으로 초기화
    Backlight = 0x09,         // 조종기 백라이트 설정
    ModeController = 0x0A,    // 조종기 동작 모드(0x10:조종, 0x80:링크)
    Link = 0x0B,              // 링크 제어(0:Client Mode, 1:Server Mode, 2:Pairing Start)
    ClearMagnetometer = 0x0C, // 지자계 센서 초기화

    // 관리자
    ClearCounter = 0xA0, // 카운터 클리어(관리자 권한을 획득했을 경우에만 동작)
    JumpToBootloader = 0xA1, // 부트로더로 이동
    JumpToApplication = 0xA2, // 앱으로 이동

    // Navigation
    NavigationTargetClear = 0xE0,  // 네비게이션 목표점 초기화
    NavigationStart = 0xE1,        // 네비게이션 시작(처음부터)
    NavigationPause = 0xE2,        // 네비게이션 일시 정지
    NavigationRestart = 0xE3,      // 네비게이션 다시 시작(일시 정지 후 다시 시작할 때 사용)
    NavigationStop = 0xE4,         // 네비게이션 중단
    NavigationNext = 0xE5,         // 네비게이션 목표점을 다음으로 변경
    NavigationReturnToHome = 0xE6, // 시작 위치로 귀환

    GpsRtkBase = 0xEA,
    GpsRtkRover = 0xEB,

    TestLock = 0xF0, // 테스트 락(테스트를 완료하기 전까진 사용 불가 / 27:활성화, 11:해제))
}


// -- Data -------------------------------------------------------------------------------------------------
#[derive(Debug)]
pub enum Data {
    None,
    Header {header: Header},
    Motion {motion: Motion},
}


// -- Serializable -----------------------------------------------------------------------------------------
pub trait Serializable
{
    fn size() -> u8;
    fn to_vec(&self) -> Vec<u8>;
}


// -- Header -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct Header {
    pub data_type: DataType,
    pub length: u8,
    pub from: DeviceType,
    pub to: DeviceType,
}


impl Header {
    pub fn new() -> Header{
        Header {
            data_type: DataType::None,
            length: 0,
            from: DeviceType::None,
            to: DeviceType::None,
        }
    }


    pub fn parse(header: &mut Header, vec_data: &Vec<u8>) -> bool {
        if vec_data.len() != Header::size() as usize {
            return false;
        }

        match DataType::try_from(vec_data[0]) {
            Ok(data_type) => { header.data_type = data_type;},
            _ => { return false; },
        };

        header.length = vec_data[1];
        
        match DeviceType::try_from(vec_data[2]) {
            Ok(from) => { header.from = from; },
            _ => { return false; },
        };

        match DeviceType::try_from(vec_data[3]) {
            Ok(to) => { header.to = to; },
            _ => { return false; },
        };

        return true;
    }

    
    pub fn parse_new(vec_data: &Vec<u8>) -> Header {
        let mut data = Header::new();

        Header::parse(&mut data, vec_data);

        data
    }
}


impl Serializable for Header {

    fn size() -> u8 { 4 }


    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.push(self.data_type.into());
        vec_data.push(self.length);
        vec_data.push(self.from.into());
        vec_data.push(self.to.into());

        vec_data
    }
}


// -- Motion -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct Motion {
    accel_x: i16,
    accel_y: i16,
    accel_z: i16,
    gyro_roll: i16,
    gyro_pitch: i16,
    gyro_yaw: i16,
    angle_roll: i16,
    angle_pitch: i16,
    angle_yaw: i16,
}


impl Motion {
    pub fn new() -> Motion{
        Motion {
            accel_x: 0,
            accel_y: 0,
            accel_z: 0,
            gyro_roll: 0,
            gyro_pitch: 0,
            gyro_yaw: 0,
            angle_roll: 0,
            angle_pitch: 0,
            angle_yaw: 0,
        }
    }


    pub fn parse(motion: &mut Motion, vec_data: Vec<u8>) -> bool {
        if vec_data.len() != Motion::size() as usize {
            return false;
        }

        match vec_data[0..1].try_into() {
            Ok(value) => { motion.accel_x = i16::from_le_bytes( value ); },
            _ => { return false; },
        };
        
        match vec_data[2..3].try_into() {
            Ok(value) => { motion.accel_y = i16::from_le_bytes( value ); },
            _ => { return false; },
        };
        
        match vec_data[4..5].try_into() {
            Ok(value) => { motion.accel_z = i16::from_le_bytes( value ); },
            _ => { return false; },
        };
        
        match vec_data[6..7].try_into() {
            Ok(value) => { motion.gyro_roll = i16::from_le_bytes( value ); },
            _ => { return false; },
        };
        
        match vec_data[8..9].try_into() {
            Ok(value) => { motion.gyro_pitch = i16::from_le_bytes( value ); },
            _ => { return false; },
        };
        
        match vec_data[10..11].try_into() {
            Ok(value) => { motion.gyro_yaw = i16::from_le_bytes( value ); },
            _ => { return false; },
        };
        
        match vec_data[12..13].try_into() {
            Ok(value) => { motion.angle_roll = i16::from_le_bytes( value ); },
            _ => { return false; },
        };
        
        match vec_data[14..15].try_into() {
            Ok(value) => { motion.angle_pitch = i16::from_le_bytes( value ); },
            _ => { return false; },
        };
        
        match vec_data[16..17].try_into() {
            Ok(value) => { motion.angle_yaw = i16::from_le_bytes( value ); },
            _ => { return false; },
        };
        
        true
    }
}


impl Serializable for Motion {

    fn size() -> u8 { 18 }


    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.accel_x.to_le_bytes());
        vec_data.extend_from_slice(&self.accel_y.to_le_bytes());
        vec_data.extend_from_slice(&self.accel_z.to_le_bytes());
        vec_data.extend_from_slice(&self.gyro_roll.to_le_bytes());
        vec_data.extend_from_slice(&self.gyro_pitch.to_le_bytes());
        vec_data.extend_from_slice(&self.gyro_yaw.to_le_bytes());
        vec_data.extend_from_slice(&self.angle_roll.to_le_bytes());
        vec_data.extend_from_slice(&self.angle_pitch.to_le_bytes());
        vec_data.extend_from_slice(&self.angle_yaw.to_le_bytes());

        vec_data
    }
}


// -- Information -------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct Version {
    build: u16,
    minor: u8,
    major: u8,
}


impl Version {
    pub fn from_slice(data_array: &[u8]) -> Version {
        Version {
            build: (data_array[0] as u16) | data_array[1] as u16,
            minor: data_array[2],
            major: data_array[3],
        }
    }

    pub fn to_vec(version: Version) -> Vec<u8> {
        let mut vec_data: Vec<u8> = Vec::new();
        vec_data.push((version.build >> 8) as u8);
        vec_data.push((version.build & 0xFF) as u8);
        vec_data.push(version.minor);
        vec_data.push(version.major);
        vec_data
    }

    pub fn from_u32(&mut self, version: u32) {
        self.build = (version & 0xFFFF) as u16;
        self.minor = ((version >> 16) & 0xFF) as u8;
        self.major = ((version >> 24) & 0xFF) as u8;
    }
    
    pub fn to_u32(&self) -> u32 {
        ((self.major as u32) << 24) | ((self.minor as u32) << 16) | self.build as u32
    }
}




// -- Information -------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct Information {
    mode_update: ModeUpdate,
    model_number: ModelNumber,
    version: Version,
    year: u16,
    month: u8,
    day: u8,
}


impl Information {
    pub fn new() -> Information{
        Information {
            mode_update: ModeUpdate::Ready,
            model_number: ModelNumber::None,
            version: Version{ major:21, minor:3, build:5 },
            year: 2021,
            month: 3,
            day: 5,
        }
    }


    pub fn parse(information: &mut Information, vec_data: Vec<u8>) -> bool {
        if vec_data.len() != Information::size() as usize {
            return false;
        }

        match ModeUpdate::try_from(vec_data[0]) {
            Ok(mode_update) => { information.mode_update = mode_update;},
            _ => { return false; },
        };
        
        match vec_data[1..4].try_into() {
            Ok(value) => {
                match ModelNumber::try_from( u32::from_le_bytes( value ) ) {
                    Ok(model_number) => { information.model_number = model_number;},
                    _ => { return false; },
                }
            },
            _ => { return false; },
        };

        /*
        match vec_data[5..8].try_into() {
            Ok(value) => { information.version.from_u32(u32::from_le_bytes( value )); },
            _ => { return false; },
        };
         */
        information.version = Version::from_slice(&vec_data[5..8]);

        information.year = ((vec_data[9] as u16) << 8) | vec_data[10] as u16;
        information.month = vec_data[11];
        information.day = vec_data[12];

        true
    }
}


impl Serializable for Information {

    fn size() -> u8 { 13 }


    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.accel_x.to_le_bytes());
        vec_data.extend_from_slice(&self.accel_y.to_le_bytes());
        vec_data.extend_from_slice(&self.accel_z.to_le_bytes());
        vec_data.extend_from_slice(&self.gyro_roll.to_le_bytes());
        vec_data.extend_from_slice(&self.gyro_pitch.to_le_bytes());
        vec_data.extend_from_slice(&self.gyro_yaw.to_le_bytes());
        vec_data.extend_from_slice(&self.angle_roll.to_le_bytes());
        vec_data.extend_from_slice(&self.angle_pitch.to_le_bytes());
        vec_data.extend_from_slice(&self.angle_yaw.to_le_bytes());

        vec_data
    }
}

