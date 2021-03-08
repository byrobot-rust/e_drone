use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
use byteorder::{ByteOrder, LittleEndian};


#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u32)]
pub enum ModelNumber {
    None = 0x00000000,

    Drone3DroneP1 = 0x00031001,
    Drone3DroneP10 = 0x0003100A,

    Drone4DroneP4 = 0x00041004,
    Drone4DroneP5 = 0x00041005,
    Drone4DroneP6 = 0x00041006,
    Drone4DroneP7 = 0x00041007,

    Drone4ControllerP2 = 0x00042002,
    Drone4ControllerP3 = 0x00042003,
    Drone4ControllerP4 = 0x00042004,

    Drone8DroneP1 = 0x00081004,

    Drone9DroneP2 = 0x00091002,
}


impl ModelNumber {
    pub fn from_slice(data_array: &[u8]) -> ModelNumber {
        if data_array.len() == 4 {
            let model_number_u32: u32 = LittleEndian::read_u32(&data_array[0..3]);
            ModelNumber::from_u32( model_number_u32 )
        }
        else
        {
            ModelNumber::None
        }
    }

    pub fn from_u32(model_number_u32: u32) -> ModelNumber {
        match ModelNumber::try_from( model_number_u32 ) {
            Ok(model_number) => { model_number },
            _ => { ModelNumber::None },
        }
    }

    pub fn to_u32(data_type: ModelNumber) -> u32 {
        data_type.into()
    }

    pub fn to_array(data_type: ModelNumber) -> [u8; 4] {
        let mut buf = [0; 4];
        LittleEndian::write_u32(&mut buf, data_type.into());
        buf
    }
}


#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum DeviceType {
    None = 0x00,

    Drone = 0x10, // 드론(Server)

    Controller = 0x20, // 조종기(Client)

    LinkClient = 0x30, // 링크 모듈(Client)
    LinkServer = 0x31, // 링크 모듈(Server)
    BleClient = 0x32,  // BLE 클라이언트
    BleServer = 0x33,  // BLE 서버

    Range = 0x40, // 거리 센서 모듈

    Base = 0x70, // 베이스(응용 프로그램)

    ByScratch = 0x80, // 바이스크래치
    Scratch = 0x81,   // 스크래치
    Entry = 0x82,     // 네이버 엔트리

    Tester = 0xA0,    // 테스터
    Monitor = 0xA1,   // 모니터
    Updater = 0xA2,   // 펌웨어 업데이트 도구
    Encrypter = 0xA3, // 암호화 도구

    Whispering = 0xFE, // 바로 인접한 장치까지만 전달(받은 장치는 자기 자신에게 보낸 것처럼 처리하고 타 장치에 전달하지 않음)
    Broadcasting = 0xFF, // 연결된 모든 장치에 전달
}


impl DeviceType {
    // https://crates.io/crates/num_enum
    pub fn from_u8(device_type_u8: u8) -> DeviceType {
        match DeviceType::try_from( device_type_u8 ) {
            Ok(device_type) => { device_type },
            _ => { DeviceType::None },
        }
    }

    pub fn to_u8(device_type: DeviceType) -> u8 {
        device_type.into()
    }
}



#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum ModeUpdate {
    None = 0x00,    // 
    
    Ready = 0x01,           // 업데이트 가능 상태
    Update = 0x02,          // 업데이트 중
    Complete = 0x03,        // 업데이트 완료
        
    Failed = 0x04,          // 업데이트 실패(업데이트 완료까지 갔으나 body의 CRC16이 일치하지 않는 경우 등)
    
    NotAvailable = 0x05,    // 업데이트 불가능 상태(Debug 모드 등)
    RunApplication = 0x06,  // 어플리케이션 동작 중
    NotRegistered = 0x07,   // 등록되지 않음
    
    EndOfType = 0x08,
}


impl ModeUpdate {
    pub fn from_u8(mode_update_u8: u8) -> ModeUpdate {
        match ModeUpdate::try_from( mode_update_u8 ) {
            Ok(mode_update) => { mode_update },
            _ => { ModeUpdate::None },
        }
    }

    pub fn to_u8(mode_update: ModeUpdate) -> u8 {
        mode_update.into()
    }
}


// -- Version -------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct Version {
    pub build: u16,
    pub minor: u8,
    pub major: u8,
}


impl Version {
    pub fn from_slice(data_array: &[u8]) -> Version {
        if data_array.len() == 4 {
            Version {
                build: ((data_array[1] as u16) << 8) | data_array[0] as u16,
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

    pub fn to_vec(version: &Version) -> Vec<u8> {
        let mut vec_data: Vec<u8> = Vec::new();
        vec_data.push((version.build & 0xFF) as u8);
        vec_data.push((version.build >> 8) as u8);
        vec_data.push(version.minor);
        vec_data.push(version.major);
        vec_data
    }

    pub fn to_array(version: &Version) -> [u8; 4] {
        let mut buf = [0; 4];
        LittleEndian::write_u32(&mut buf, Version::to_u32(&version));
        buf
    }
    
    pub fn to_u32(version: &Version) -> u32 {
        ((version.major as u32) << 24) | ((version.minor as u32) << 16) | version.build as u32
    }
}


