/*
    2021.3.11

    함수 이름과 내부 동작, 반환 형식에 규칙을 정함.

    1.  new() 함수는 입력값 없이 생성하여 빈 인스턴스 반환
    2.  parse(slice_data: &[u8]) 함수는 항상 슬라이스를 인풋으로 하되
        Extractor를 사용하여 데이터를 파싱하고,
        Result<Data, &'static str>를 반환
    3.  size() 함수는 고정된 길이 값을 반환하되 가변길이인 경우에는 고정된 부분에 대한 값만 반환
    4.  get_length() 함수는 가변 길이 데이터를 전송하는 경우 고정된 데이터부분과 가변 데이터 부분을 모두 합한 데이터 길이 반환
    5.  to_vec(&self) 함수는 벡터 데이터를 반환


    * parse 작성 예제 1
        -   아래의 방법을 쓰지 않는 이유는 슬라이스 인덱스를 잘못 지정하는 실수를 할 확률이 높기 때문이다.
            또한 변수의 인덱스가 잘못되어 수정하는 경우 그 아래의 모든 인덱스를 수정해야하는 불편함이 있기 때문.

        pub fn parse(manual: &mut Manual, vec_data: &[u8]) -> bool {
            if vec_data.len() != Manual::size() {
                return false;
            }

            manual.flags = LittleEndian::read_u16(&vec_data[0..2]);
            manual.brightness = vec_data[2];

            true
        }


    * parse 작성 예제 2
        -   아래의 코드를 사용하는 것에서 Result를 반환하는 형식으로 바꾼 이유는 이렇게 작성한 함수를
            단독으로 외부에서 호출하는 일이 거의 없고, 대신 별도로 from_vec(vec_data: &Vec<u8>) 함수를
            통해 벡터에서 바로 데이터를 생성하기 때문.

        pub fn parse(manual: &mut Manual, slice_data: &[u8]) -> bool {
            if slice_data.len() != Manual::size() {
                return false;
            }

            let mut ext: Extractor = Extractor::from_slice(slice_data);

            manual.flags = ext.get_u16();
            manual.brightness = ext.get_u8();

            true
        }


    * parse 작성 예제 3
        -   예제 2와 같이 from_vec(vec_data: &Vec<u8>) 에서 다시 pub fn parse(manual: &mut Manual, slice_data: &[u8]) 를
            호출하는 방식으로 만들었었으나 rust에서 함수를 만드는 스타일과 괴리가 있고, 굳이 두 개의 함수로 분리하지 않아도
            될 것으로 보여 이 방법을 사용하는 것으로 변경함

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

 */


pub mod command;
pub mod control;
pub mod display;
pub mod light;
pub mod sensor;

use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
use byteorder::{ByteOrder, LittleEndian};

use crate::system::{*};
use crate::communication::extractor::Extractor;
use crate::communication::{*};


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


impl DataType {
    // https://crates.io/crates/num_enum
    pub fn from_u8(data: u8) -> DataType {
        match DataType::try_from( data ) {
            Ok(data_type) => { data_type },
            _ => { DataType::None },
        }
    }
}


// -- Data -------------------------------------------------------------------------------------------------
#[derive(Debug)]
pub enum Data {
    None,
    Header (Header),
    Request (Request),
    Information (Information),
    Quad8 (control::Quad8),
    Motion (sensor::Motion),
}


// -- Serializable -----------------------------------------------------------------------------------------
pub trait Serializable
{
    fn size() -> usize;
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
    
    pub fn parse(slice_data: &[u8]) -> Result<Header, &'static str> {
        if slice_data.len() == Header::size() {
            Ok(Header{
                data_type: DataType::from_u8(slice_data[0]),
                length: slice_data[1],
                from: DeviceType::from_u8(slice_data[2]),
                to: DeviceType::from_u8(slice_data[3]),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for Header {
    fn size() -> usize { 4 }

    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        //serializer::add_u8(&mut vec_data, self.data_type.into());
        vec_data.push(self.data_type.into());
        vec_data.push(self.length);
        vec_data.push(self.from.into());
        vec_data.push(self.to.into());

        vec_data
    }
}


// -- Ping -------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct Ping {
    pub system_time: u64,
}


impl Ping {
    pub fn new() -> Ping{
        Ping {
            system_time: 0,
        }
    }
    
    pub fn parse(slice_data: &[u8]) -> Result<Ping, &'static str> {
        if slice_data.len() == Ping::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(Ping{
                system_time: ext.get_u64(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for Ping {
    fn size() -> usize { 8 }


    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        serializer::add_u64(&mut vec_data, self.system_time);

        vec_data
    }
}


// -- Ack -------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct Ack {
    pub system_time: u64,
    pub data_type: DataType,
    pub crc16: u16,
}


impl Ack {
    pub fn new() -> Ack{
        Ack {
            system_time: 0,
            data_type: DataType::None,
            crc16: 1,
        }
    }
    
    pub fn parse(slice_data: &[u8]) -> Result<Ack, &'static str> {
        if slice_data.len() == Ack::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(Ack{
                system_time: ext.get_u64(),
                data_type: DataType::from_u8(ext.get_u8()),
                crc16: ext.get_u16(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for Ack {
    fn size() -> usize { 11 }


    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        /*
        let mut system_time_array = [0; 8];
        LittleEndian::write_u64(&mut system_time_array, self.system_time);

        let mut crc16_array = [0; 2];
        LittleEndian::write_u16(&mut crc16_array, self.crc16);
        
        vec_data.extend_from_slice(&system_time_array);
        vec_data.push(self.data_type.into());
        vec_data.extend_from_slice(&crc16_array);
        // */
        
        serializer::add_u64(&mut vec_data, self.system_time);
        serializer::add_u8(&mut vec_data, self.data_type.into());
        serializer::add_u16(&mut vec_data, self.crc16);

        vec_data
    }
}


// -- Error -------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct Error {
    pub system_time: u64,
    pub error_flags_for_sensor: u32,
    pub error_flags_for_state: u32,
}


impl Error {
    pub fn new() -> Error{
        Error {
            system_time: 0,
            error_flags_for_sensor: 0,
            error_flags_for_state: 0,
        }
    }
    
    pub fn parse(slice_data: &[u8]) -> Result<Error, &'static str> {
        if slice_data.len() == Error::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(Error{
                system_time: ext.get_u64(),
                error_flags_for_sensor: ext.get_u32(),
                error_flags_for_state: ext.get_u32(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for Error {
    fn size() -> usize { 16 }


    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        /*
        let mut system_time_array = [0; 8];
        LittleEndian::write_u64(&mut system_time_array, self.system_time);

        let mut error_flags_for_sensor = [0; 4];
        LittleEndian::write_u32(&mut error_flags_for_sensor, self.error_flags_for_sensor);
        
        let mut error_flags_for_state = [0; 4];
        LittleEndian::write_u32(&mut error_flags_for_state, self.error_flags_for_state);
        
        vec_data.extend_from_slice(&system_time_array);
        vec_data.extend_from_slice(&error_flags_for_sensor);
        vec_data.extend_from_slice(&error_flags_for_state);
        // */
        
        serializer::add_u64(&mut vec_data, self.system_time);
        serializer::add_u32(&mut vec_data, self.error_flags_for_sensor);
        serializer::add_u32(&mut vec_data, self.error_flags_for_state);

        vec_data
    }
}


// -- Request -------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct Request {
    pub data_type: DataType,
}


impl Request {
    pub fn new() -> Request{
        Request {
            data_type: DataType::None,
        }
    }
    
    pub fn parse(slice_data: &[u8]) -> Result<Request, &'static str> {
        if slice_data.len() == Request::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(Request{
                data_type: DataType::from_u8(ext.get_u8()),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for Request {
    fn size() -> usize { 1 }


    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.push(self.data_type.into());

        vec_data
    }
}


// -- RequestOption -------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct RequestOption {
    pub data_type: DataType,
    pub option: u32,
}


impl RequestOption {
    pub fn new() -> RequestOption{
        RequestOption {
            data_type: DataType::None,
            option: 0,
        }
    }
    
    pub fn parse(slice_data: &[u8]) -> Result<RequestOption, &'static str> {
        if slice_data.len() == RequestOption::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(RequestOption{
                data_type: DataType::from_u8(ext.get_u8()),
                option: ext.get_u32(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for RequestOption {
    fn size() -> usize { 5 }


    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        /*
        let mut option_array = [0; 4];
        LittleEndian::write_u32(&mut option_array, self.option);

        vec_data.push(self.data_type.into());
        vec_data.extend_from_slice(&option_array);
        // */

        serializer::add_u8(&mut vec_data, self.data_type.into());
        serializer::add_u32(&mut vec_data, self.option);

        vec_data
    }
}


// -- SystemInformation -------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct SystemInformation {
    pub crc32_bootloader: u32,
    pub crc32_application: u32,
}


impl SystemInformation {
    pub fn new() -> SystemInformation{
        SystemInformation {
            crc32_bootloader: 0,
            crc32_application: 0,
        }
    }
    
    pub fn parse(slice_data: &[u8]) -> Result<SystemInformation, &'static str> {
        if slice_data.len() == SystemInformation::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(SystemInformation{
                crc32_bootloader: ext.get_u32(),
                crc32_application: ext.get_u32(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for SystemInformation {
    fn size() -> usize { 8 }


    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        /*
        let mut crc32_bootloader_array = [0; 4];
        LittleEndian::write_u32(&mut crc32_bootloader_array, self.crc32_bootloader);
        
        let mut crc32_application_array = [0; 4];
        LittleEndian::write_u32(&mut crc32_application_array, self.crc32_application);

        vec_data.extend_from_slice(&crc32_bootloader_array);
        vec_data.extend_from_slice(&crc32_application_array);
        // */
        
        serializer::add_u32(&mut vec_data, self.crc32_bootloader);
        serializer::add_u32(&mut vec_data, self.crc32_application);

        vec_data
    }
}


// -- Information -------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct Information {
    pub mode_update: ModeUpdate,
    pub model_number: ModelNumber,
    pub version: Version,
    pub year: u16,
    pub month: u8,
    pub day: u8,
}


impl Information {
    pub fn new() -> Information{
        Information {
            mode_update: ModeUpdate::Ready,
            model_number: ModelNumber::None,
            version: Version{ major:21, minor:1, build:1 },
            year: 2021,
            month: 1,
            day: 1,
        }
    }
    
    pub fn parse(slice_data: &[u8]) -> Result<Information, &'static str> {
        if slice_data.len() == Information::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(Information{
                mode_update: ModeUpdate::from_u8(ext.get_u8()),
                model_number: ModelNumber::from_u32(ext.get_u32()),
                version: Version::from_u32(ext.get_u32()),
                year: ext.get_u16(),
                month: ext.get_u8(),
                day: ext.get_u8(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for Information {
    fn size() -> usize { 13 }


    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        /*
        let mut year_array = [0; 2];
        LittleEndian::write_u16(&mut year_array, self.year);

        vec_data.push(self.mode_update.into());
        vec_data.extend_from_slice(&self.model_number.to_array());
        vec_data.extend_from_slice(&self.version.to_array());
        vec_data.extend_from_slice(&year_array);
        vec_data.push(self.month);
        vec_data.push(self.day);
        // */
        
        serializer::add_u8(&mut vec_data, self.mode_update.into());
        serializer::add_slice(&mut vec_data, &self.model_number.to_array());
        serializer::add_slice(&mut vec_data, &self.version.to_array());
        serializer::add_u16(&mut vec_data, self.year);
        serializer::add_u8(&mut vec_data, self.month);
        serializer::add_u8(&mut vec_data, self.day);

        vec_data
    }
}


// -- UpdateLocation -------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct UpdateLocation {
    pub index_block_next: u16,
}


impl UpdateLocation {
    pub fn new() -> UpdateLocation{
        UpdateLocation {
            index_block_next: 0,
        }
    }
    
    pub fn parse(slice_data: &[u8]) -> Result<UpdateLocation, &'static str> {
        if slice_data.len() == UpdateLocation::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(UpdateLocation{
                index_block_next: ext.get_u16(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for UpdateLocation {
    fn size() -> usize { 2 }


    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        /*
        let mut index_block_next_array = [0; 2];
        LittleEndian::write_u16(&mut index_block_next_array, self.index_block_next);

        vec_data.extend_from_slice(&index_block_next_array);
        // */
        
        serializer::add_u16(&mut vec_data, self.index_block_next);

        vec_data
    }
}

