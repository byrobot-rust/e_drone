use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
use byteorder::{ByteOrder, LittleEndian};


// -- ModelNumber -------------------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u32)]
pub enum ModelNumber {
    None = 0x00000000,

    //                       AAAABBCC, AAAA(Project Number), BB(Device Type), CC(Revision)
    Drone3DroneP1        = 0x00031001,      // Lightrone / GD65 / HW2181 / 3.7v / barometer / RGB LED / Shaking binding
    Drone3DroneP2        = 0x00031002,      // Soccer Drone / HW2181 / 7.4v / barometer / RGB LED / Shaking binding
    Drone3DroneP3        = 0x00031003,      // GD240 / HW2181 / power button / u30 flow / 3.7v / geared motor / barometer
    Drone3DroneP4        = 0x00031004,      // GD50N / HW2181 / power button / 3.7v / barometer
    Drone3DroneP5        = 0x00031005,      // GD30 / HW2181 / 3.7v / normal binding
    Drone3DroneP6        = 0x00031006,      // Soccer Drone 2 / HW2181 / 7.4v / barometer / RGB LED / Shaking binding
    Drone3DroneP7        = 0x00031007,      // SKYKICKV2 / SPI / HW2181 / 7.4v / barometer / RGB LED / Shaking binding
    Drone3DroneP8        = 0x00031008,      // GD65 / SPI / HW2181 / 3.7v / barometer / RGB LED / Shaking binding
    Drone3DroneP9        = 0x00031009,      // GD65 / SPI / HW2181 / 3.7v / barometer / RGB LED / Shaking binding / BladeType Power Connector
    Drone3DroneP10       = 0x0003100A,      // Battle Drone / SPI / HW2181 / 3.7v / barometer / RGB LED / Shaking binding
    
    Drone3ControllerP1   = 0x00032001,      // GD65 Controller / HW2181
    Drone3ControllerP2   = 0x00032002,      // Skykick Controller / HW2181
    Drone3ControllerP3   = 0x00032003,      // GD65 Controller / USB / HW2181
    Drone3ControllerP4   = 0x00032004,      // Battle Drone Controller / USB / HW2181B
    Drone3ControllerP5   = 0x00032005,      // E-Drone 4m Controller / USB / HW2181B
    
    Drone3LinkP0         = 0x00033000,      // 

    Drone3AnalyzerP0     = 0x00034100,      // for soccer drone

    Drone3TesterP4       = 0x0003A004,      // (obsolete)
    Drone3TesterP6       = 0x0003A006,      // Battle Drone, Controller Tester / Drone3DroneP10, Drone3ControllerP4
    Drone3TesterP7       = 0x0003A007,      // Tester / Drone3ControllerP5 테스터
    
    Drone4DroneP5        = 0x00041005,      // HW2000, 2m range sensor
    Drone4DroneP6        = 0x00041006,      // HW2000B, 4m range sensor
    Drone4DroneP7        = 0x00041007,      // HW2000B, 4m range sensor, BLDC Motor
    
    Drone4ControllerP2   = 0x00042002,      // HW2000
    Drone4ControllerP3   = 0x00042003,      // HW2000B
    Drone4ControllerP4   = 0x00042004,      // HW2000B + PA
    Drone4ControllerP5   = 0x00042005,      // HW2000B + PA
    
    Drone4LinkP0         = 0x00043000,      // 
    
    Drone4TesterP6       = 0x0004A006,      // 
    Drone4TesterP7       = 0x0004A007,      // Drone4DroneP6 테스터(E-Drone 4m 버전)
    
    Drone7DroneP2        = 0x00071002,      // Coding Car

    Drone7BleClientP0    = 0x00073200,      // Coding Car Link
    Drone7BleClientP5    = 0x00073205,      // Coding Car Tester BLE

    Drone7BleServerP2    = 0x00073302,      // Coding Car Ble Module

    Drone7TesterP5       = 0x0007A005,      // 
    Drone7TesterP6       = 0x0007A006,      // 
    
    Drone7MonitorP4      = 0x0007A104,      // (obsolete)
    Drone7MonitorP5      = 0x0007A105,      // 
    
    Drone8DroneP0        = 0x00081000,      // (obsolete)
    Drone8DroneP1        = 0x00081001,      // Coding Drone
    
    Drone8TesterP6       = 0x0008A006,      // 

    Drone8MonitorP6      = 0x0008A106,      // 
    
    Drone9DroneP0        = 0x00091000,      // 
    Drone9DroneP1        = 0x00091001,      // 
    Drone9DroneP2        = 0x00091002,      // 
    Drone9DroneP3        = 0x00091003,      // STM32F412RET6, HW2000B + PA, OPT3101, Raspberry PI
    Drone9DroneP4        = 0x00091004,      // 
    Drone9DroneP5        = 0x00091005,      // 
    
    Drone9TesterP6       = 0x0009A006,      // 
    Drone9TesterP7       = 0x0009A007,      // 
}


impl ModelNumber {
    /*
    // 사용하지는 않지만 참고용으로 남겨둠
    pub fn from_slice(data_array: &[u8]) -> ModelNumber {
        if data_array.len() == 4 {
            let model_number_u32: u32 = LittleEndian::read_u32(&data_array[0..4]);
            ModelNumber::from_u32( model_number_u32 )
        }
        else
        {
            ModelNumber::None
        }
    }
    // */

    pub fn from_u32(data_u32: u32) -> ModelNumber {
        match ModelNumber::try_from( data_u32 ) {
            Ok(data) => { data },
            _ => { ModelNumber::None },
        }
    }

    pub fn to_array(&self) -> [u8; 4] {
        let mut buf = [0; 4];
        LittleEndian::write_u32(&mut buf, self.clone().into());
        buf
    }

    pub fn get_device_type(&self) -> DeviceType {
        DeviceType::from_u8(self.to_array()[1])
    }
}


// -- DeviceType -------------------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum DeviceType {
    None            = 0x00,

    Drone           = 0x10, // 드론(Server)

    Controller      = 0x20, // 조종기(Client)

    LinkClient      = 0x30, // 링크 모듈(Client)
    LinkServer      = 0x31, // 링크 모듈(Server)
    BleClient       = 0x32, // BLE 클라이언트
    BleServer       = 0x33, // BLE 서버

    Range           = 0x40, // 거리 센서 모듈
    Analyzer        = 0x41, // 분석 모듈

    Base            = 0x70, // 베이스(응용 프로그램)

    ByScratch       = 0x80, // 바이스크래치
    Scratch         = 0x81, // 스크래치
    Entry           = 0x82, // 네이버 엔트리

    Tester          = 0xA0, // 테스터
    Monitor         = 0xA1, // 모니터
    Updater         = 0xA2, // 펌웨어 업데이트 도구
    Encrypter       = 0xA3, // 암호화 도구

    Whispering      = 0xFE, // 바로 인접한 장치까지만 전달(받은 장치는 자기 자신에게 보낸 것처럼 처리하고 타 장치에 전달하지 않음)
    Broadcasting    = 0xFF, // 연결된 모든 장치에 전달
}


impl DeviceType {
    // https://crates.io/crates/num_enum
    pub fn from_u8(data_u8: u8) -> DeviceType {
        match DeviceType::try_from( data_u8 ) {
            Ok(data) => { data },
            _ => { DeviceType::None },
        }
    }
}



// -- ModeSystem -------------------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum ModeSystem {
    None            = 0x00,    // 
    
    Boot            = 0x10,
    Start           = 0x11,
    Running         = 0x12,
    ReadyToReset    = 0x13,

    Error           = 0xA0,
}


impl ModeSystem {
    pub fn from_u8(data_u8: u8) -> ModeSystem {
        match ModeSystem::try_from( data_u8 ) {
            Ok(data) => { data },
            _ => { ModeSystem::None },
        }
    }
}



// -- ModeControlFlight -------------------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum ModeControlFlight {
    None        = 0x00,    // 
    
    Attitude    = 0x10,     // 자세 - X,Y는 각도(deg)로 입력받음, Z,Yaw는 속도(m/s)로 입력 받음
    Position    = 0x11,     // 위치 - X,Y,Z,Yaw는 속도(m/s)로 입력 받음
    Manual      = 0x12,     // 고도를 수동으로 조종함
    Rate        = 0x13,     // Rate - X,Y는 각속도(deg/s)로 입력받음, Z,Yaw는 속도(m/s)로 입력 받음
    Function    = 0x14,     // 기능

    Error       = 0xA0,
}


impl ModeControlFlight {
    pub fn from_u8(data_u8: u8) -> ModeControlFlight {
        match ModeControlFlight::try_from( data_u8 ) {
            Ok(data) => { data },
            _ => { ModeControlFlight::None },
        }
    }
}



// -- ModeFlight -------------------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum ModeFlight {
    None        = 0x00,
    
    Ready       = 0x10,

    Start       = 0x11,
    Takeoff     = 0x12,
    Flight      = 0x13,
    Landing     = 0x14,
    Flip        = 0x15,
    Reverse     = 0x16,

    Stop        = 0x20,

    Accident    = 0x30,
    Error       = 0x31,

    Test        = 0x40,
}


impl ModeFlight {
    pub fn from_u8(data_u8: u8) -> ModeFlight {
        match ModeFlight::try_from( data_u8 ) {
            Ok(data) => { data },
            _ => { ModeFlight::None },
        }
    }
}


// -- ModeMovement -------------------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum ModeMovement {
    None        = 0x00,
    
    Ready       = 0x01,
    Hovering    = 0x02,
    Moving      = 0x03,
    ReturnHome  = 0x04
}


impl ModeMovement {
    pub fn from_u8(data_u8: u8) -> ModeMovement {
        match ModeMovement::try_from( data_u8 ) {
            Ok(data) => { data },
            _ => { ModeMovement::None },
        }
    }
}


// -- ModeUpdate -------------------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum ModeUpdate {
    None            = 0x00, // 
    
    Ready           = 0x01, // 업데이트 가능 상태
    Update          = 0x02, // 업데이트 중
    Complete        = 0x03, // 업데이트 완료
        
    Failed          = 0x04, // 업데이트 실패(업데이트 완료까지 갔으나 body의 CRC16이 일치하지 않는 경우 등)
    
    NotAvailable    = 0x05, // 업데이트 불가능 상태(Debug 모드 등)
    RunApplication  = 0x06, // 어플리케이션 동작 중
    NotRegistered   = 0x07, // 등록되지 않음
    
    EndOfType = 0x08,
}


impl ModeUpdate {
    pub fn from_u8(data_u8: u8) -> ModeUpdate {
        match ModeUpdate::try_from( data_u8 ) {
            Ok(data) => { data },
            _ => { ModeUpdate::None },
        }
    }
}


// -- Version -------------------------------------------------------------------------------------------
#[derive(Debug, Copy, Clone)]
pub struct Version {
    pub build: u16,
    pub minor: u8,
    pub major: u8,
}


impl Version {
    pub fn from_slice(data_array: &[u8]) -> Version {
        if data_array.len() == 4 {
            Version {
                build: LittleEndian::read_u16(&data_array[0..2]),
                minor: data_array[2],
                major: data_array[3],
            }
        }
        else {
            Version {
                build: 1,
                minor: 1,
                major: 21,
            }
        }
    }

    pub fn from_u32(version: u32) -> Version {
        Version {
            build: (version & 0xFFFF) as u16,
            minor: ((version >> 16) & 0xFF) as u8,
            major: ((version >> 24) & 0xFF) as u8,
        }
    }

    pub fn to_array(&self) -> [u8; 4] {
        let mut buf = [0; 4];
        LittleEndian::write_u32(&mut buf, self.to_u32());
        buf
    }
    
    pub fn to_u32(&self) -> u32 {
        ((self.major as u32) << 24) | ((self.minor as u32) << 16) | self.build as u32
    }
}


// -- ErrorFlagsForSensor -------------------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u32)]
pub enum ErrorFlagsForSensor {
    None                                = 0x00000000,

    MotionNoAnswer                      = 0x00000001,    // Motion 응답 없음
    MotionWrongValue                    = 0x00000002,    // Motion 잘못된 값
    MotionNotCalibrated                 = 0x00000004,    // Gyro Bias 보정이 완료되지 않음
    MotionCalibrating                   = 0x00000008,    // Gyro Bias 보정 중

    PressureNoAnswer                    = 0x00000010,    // 압력 센서 응답 없음
    PressureWrongValue                  = 0x00000020,    // 압력 센서 잘못된 값

    RangeGroundNoAnswer                 = 0x00000100,    // 바닥 거리 센서 응답 없음
    RangeGroundWrongValue               = 0x00000200,    // 바닥 거리 센서 잘못된 값
    RangeFrontNoAnswer                  = 0x00000400,    // 정면 거리 센서 응답 없음
    RangeFrontWrongValue                = 0x00000800,    // 정면 거리 센서 잘못된 값

    FlowNoAnswer                        = 0x00001000,    // Flow 응답 없음
    FlowWrongValue                      = 0x00002000,    // Flow 잘못된 값
    FlowCannotRecognizeGroundImage      = 0x00004000,    // 바닥 이미지를 인식할 수 없음

    RfNoAnswer                          = 0x10000000,    // RF 응답 없음
    RfPaired                            = 0x20000000,    // RF 응답 없음
    RfConnected                         = 0x40000000,    // RF 응답 없음
}


impl ErrorFlagsForSensor {
    pub fn from_u8(data_u32: u32) -> ErrorFlagsForSensor {
        match ErrorFlagsForSensor::try_from( data_u32 ) {
            Ok(data) => { data },
            _ => { ErrorFlagsForSensor::None },
        }
    }
}


// -- ErrorFlagsForState -------------------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u32)]
pub enum ErrorFlagsForState {
    None                                    = 0x00000000,

    NotRegistered                           = 0x00000001,    // 장치 등록이 안됨
    FlashReadLockUnLocked                   = 0x00000002,    // 플래시 메모리 읽기 Lock이 안 걸림
    BootloaderWriteLockUnLocked             = 0x00000004,    // 부트로더 영역 쓰기 Lock이 안 걸림
    LowBattery                              = 0x00000008,    // Low Battery
    
    TakeoffFailureCheckPropellerAndMotor    = 0x00000010,    // 이륙 실패
    CheckPropellerVibration                 = 0x00000020,    // 프로펠러 진동발생
    AttitudeNotStable                       = 0x00000040,    // 자세가 많이 기울어져 있거나 뒤집어져 있을때
    
    CanNotFlipLowBattery                    = 0x00000100,    // 배터리가 30이하
    CanNotFlipTooHeavy                      = 0x00000200,    // 기체가 무거움
}


impl ErrorFlagsForState {
    pub fn from_u8(data_u32: u32) -> ErrorFlagsForState {
        match ErrorFlagsForState::try_from( data_u32 ) {
            Ok(data) => { data },
            _ => { ErrorFlagsForState::None },
        }
    }
}



// -- Direction -------------------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Direction {
    None = 0,

    Left,
    Front,
    Right,
    Rear,
    
    Top,
    Bottom,
    
    Center,
}


impl Direction {
    pub fn from_u8(data_u8: u8) -> Direction {
        match Direction::try_from( data_u8 ) {
            Ok(data) => { data },
            _ => { Direction::None },
        }
    }
}



// -- Rotation -------------------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Rotation {
    None                = 0x00,
    
    Clockwise           = 0x01, // 시계 방향
    Counterclockwise    = 0x02, // 반시계 방향
}


impl Rotation {
    pub fn from_u8(data_u8: u8) -> Rotation {
        match Rotation::try_from( data_u8 ) {
            Ok(data) => { data },
            _ => { Rotation::None },
        }
    }
}


// -- FlightEvent -------------------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum FlightEvent {
    None            = 0x00,     // 없음
            
    Stop            = 0x10,     // 정지
    Takeoff         = 0x11,     // 이륙
    Landing         = 0x12,     // 착륙
    
    Reverse         = 0x14,     // 뒤집기
    
    FlipFront       = 0x15,     // 회전
    FlipRear        = 0x16,     // 회전
    FlipLeft        = 0x17,     // 회전
    FlipRight       = 0x18,     // 회전
    
    ReturnHone      = 0x19,     // Return Home
    
    Shot            = 0x90,     // Shot
    UnderAttack     = 0x91,     // UnderAttack
    
    ResetHeading    = 0xA0,     // 헤딩 리셋(Headless 모드 일 때 현재 heading을 0도로 변경)
}


impl FlightEvent {
    pub fn from_u8(data_u8: u8) -> FlightEvent {
        match FlightEvent::try_from( data_u8 ) {
            Ok(data) => { data },
            _ => { FlightEvent::None },
        }
    }
}


// -- SensorOrientation -------------------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum SensorOrientation {
    None = 0,       // 없음
    
    Normal,         // 정상
    ReverseStart,   // 뒤집히기 시작
    Reversed,       // 뒤집힘
}


impl SensorOrientation {
    pub fn from_u8(data_u8: u8) -> SensorOrientation {
        match SensorOrientation::try_from( data_u8 ) {
            Ok(data) => { data },
            _ => { SensorOrientation::None },
        }
    }
}


// -- Headless -------------------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Headless {
    None = 0,       // 없음
    
    Headless,       // 헤드리스(앱솔루트)
    Normal,         // 일반
}


impl Headless {
    pub fn from_u8(data_u8: u8) -> Headless {
        match Headless::try_from( data_u8 ) {
            Ok(data) => { data },
            _ => { Headless::None },
        }
    }
}


// -- TrimDirection -------------------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum TrimDirection {
    None = 0,           // 없음

    RollIncrease,       // Roll 증가
    RollDecrease,       // Roll 감소
    PitchIncrease,      // Pitch 증가
    PitchDecrease,      // Pitch 감소
    YawIncrease,        // Yaw 증가
    YawDecrease,        // Yaw 감소
    ThrottleIncrease,   // Throttle 증가
    ThrottleDecrease,   // Throttle 감소
    
    Reset,              // 전체 트림 리셋
}


impl TrimDirection {
    pub fn from_u8(data_u8: u8) -> TrimDirection {
        match TrimDirection::try_from( data_u8 ) {
            Ok(data) => { data },
            _ => { TrimDirection::None },
        }
    }
}


