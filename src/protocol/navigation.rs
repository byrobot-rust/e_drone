use crate::protocol::{*};
use crate::communication::extractor::Extractor;

mod mode
{
    use num_enum::IntoPrimitive;
    use num_enum::TryFromPrimitive;
    use std::convert::TryFrom;
    use byteorder::{ByteOrder, LittleEndian};

    
    // -- Navigation -------------------------------------------------------------------------------------------
    #[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
    #[repr(u32)]
    pub enum Navigation {
        None        = 0x00,

        Ready       = 0x01,   // 준비

        Start       = 0x02,   // 시작
        Cruise      = 0x03,   // 동작
        Pause       = 0x04,   // 일시 정지
        Finish      = 0x05,   // 종료
        
        Error       = 0x06,   // 오류 발생
    }


    impl Navigation {
        pub fn from_u32(data_u32: u32) -> Navigation {
            match Navigation::try_from( data_u32 ) {
                Ok(data) => { data },
                _ => { Navigation::None },
            }
        }
        
        pub fn to_array(&self) -> [u8; 4] {
            let mut buf = [0; 4];
            LittleEndian::write_u32(&mut buf, self.clone().into());
            buf
        }
    }


    // -- Action -------------------------------------------------------------------------------------------
    #[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
    #[repr(u32)]
    pub enum Action {
        None        = 0x00,

        Wait        = 0x01,   // 기다림
        
        Takeoff     = 0x02,   // 이륙
        Move        = 0x03,   // 이동
        Landing     = 0x04,   // 착륙
    }


    impl Action {
        pub fn from_u32(data_u32: u32) -> Action {
            match Action::try_from( data_u32 ) {
                Ok(data) => { data },
                _ => { Action::None },
            }
        }
        
        pub fn to_array(&self) -> [u8; 4] {
            let mut buf = [0; 4];
            LittleEndian::write_u32(&mut buf, self.clone().into());
            buf
        }
    }


    // -- Option -------------------------------------------------------------------------------------------
    #[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
    #[repr(u32)]
    pub enum Option {
        None            = 0x00,

        VideoCapture    = 0x01,   // 비디오 캡쳐
        TakePhoto       = 0x02,   // 사진 촬영
    }


    impl Option {
        pub fn from_u32(data_u32: u32) -> Option {
            match Option::try_from( data_u32 ) {
                Ok(data) => { data },
                _ => { Option::None },
            }
        }
        
        pub fn to_array(&self) -> [u8; 4] {
            let mut buf = [0; 4];
            LittleEndian::write_u32(&mut buf, self.clone().into());
            buf
        }
    }
}


// -- Target -----------------------------------------------------------------------------------------------
#[derive(Debug, Copy, Clone)]
pub struct Target {
    pub index: u32,             //  4 명령 번호

    pub mode_action: mode::Action,   //  8 행동
    pub mode_option: mode::Option,   // 12 옵션

    pub time: u32,              // 16 실행 시간(ms)

    pub latitude: f64,          // 24 위도(Y)
    pub longitude: f64,         // 32 경도(X)
    pub altitude: f32,          // 36 고도(Z)
    pub speed: f32,             // 40 속도(m/s)

    pub heading: f32,           // 44 헤딩(degree, compass 0.0 ~ 360.0)
    pub rotational_speed: f32,  // 48 회전 속도(deg/sec) (-180.0(CW) ~ +180.0(CCW))
}


impl Target {
    pub fn new() -> Target{
        Target {
            index:              0,
            mode_action:        mode::Action::None,
            mode_option:        mode::Option::None,
            time:               0,
            latitude:           0.0_f64,
            longitude:          0.0_f64,
            altitude:           0.0_f32,
            speed:              0.0_f32,
            heading:            0.0_f32,
            rotational_speed:   0.0_f32,
        }
    }


    pub const fn size() -> usize { 48 }


    pub fn parse(slice_data: &[u8]) -> Result<Target, &'static str> {
        if slice_data.len() == Target::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(Target{
                index:          ext.get_u32(),
                mode_action:    mode::Action::from_u32(ext.get_u32()),
                mode_option:    mode::Option::from_u32(ext.get_u32()),
                time:           ext.get_u32(),
                latitude:       ext.get_f64(),
                longitude:      ext.get_f64(),
                altitude:       ext.get_f32(),
                speed:          ext.get_f32(),
                heading:        ext.get_f32(),
                rotational_speed: ext.get_f32(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for Target {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.index.to_le_bytes());
        vec_data.extend_from_slice(&self.mode_action.to_array());
        vec_data.extend_from_slice(&self.mode_option.to_array());
        vec_data.extend_from_slice(&self.time.to_le_bytes());
        vec_data.extend_from_slice(&self.latitude.to_le_bytes());
        vec_data.extend_from_slice(&self.longitude.to_le_bytes());
        vec_data.extend_from_slice(&self.altitude.to_le_bytes());
        vec_data.extend_from_slice(&self.speed.to_le_bytes());
        vec_data.extend_from_slice(&self.heading.to_le_bytes());
        vec_data.extend_from_slice(&self.rotational_speed.to_le_bytes());

        vec_data
    }
}


// -- TargetLight -----------------------------------------------------------------------------------------------
#[derive(Debug, Copy, Clone)]
pub struct TargetLight {
    pub index: u16,             //  2 명령 번호

    pub mode_action: mode::Action,   //  4 행동
    pub mode_option: mode::Option,   //  6 옵션

    pub time: u16,              //  8 실행 시간(sec)	/ 1000

    pub latitude: i32,          // 12 위도(Y)			x 10,000,000	// int range (-2,147,483,647 to 2,147,483,647)
    pub longitude: i32,         // 16 경도(X)			x 10,000,000
    pub altitude: i16,          // 18 고도(Z)(m)		x 100
    pub speed: i16,             // 20 이동속도(m/s)		x 100

    pub heading: i16,           // 22 헤딩(degree, compass 0 ~ 360)	x 10
    pub rotational_speed: i16,  // 24 헤딩 회전 속도(0 ~ 360)		x 10
}


impl TargetLight {
    pub fn new() -> TargetLight{
        TargetLight {
            index:              0,
            mode_action:        mode::Action::None,
            mode_option:        mode::Option::None,
            time:               0,
            latitude:           0,
            longitude:          0,
            altitude:           0,
            speed:              0,
            heading:            0,
            rotational_speed:   0,
        }
    }


    pub const fn size() -> usize { 24 }


    pub fn parse(slice_data: &[u8]) -> Result<TargetLight, &'static str> {
        if slice_data.len() == TargetLight::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(TargetLight{
                index:          ext.get_u16(),
                mode_action:    mode::Action::from_u32(ext.get_u16() as u32),
                mode_option:    mode::Option::from_u32(ext.get_u16() as u32),
                time:           ext.get_u16(),
                latitude:       ext.get_i32(),
                longitude:      ext.get_i32(),
                altitude:       ext.get_i16(),
                speed:          ext.get_i16(),
                heading:        ext.get_i16(),
                rotational_speed: ext.get_i16(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for TargetLight {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.index.to_le_bytes());
        vec_data.extend_from_slice(&self.mode_action.to_array()[0..2]);
        vec_data.extend_from_slice(&self.mode_option.to_array()[0..2]);
        vec_data.extend_from_slice(&self.time.to_le_bytes());
        vec_data.extend_from_slice(&self.latitude.to_le_bytes());
        vec_data.extend_from_slice(&self.longitude.to_le_bytes());
        vec_data.extend_from_slice(&self.altitude.to_le_bytes());
        vec_data.extend_from_slice(&self.speed.to_le_bytes());
        vec_data.extend_from_slice(&self.heading.to_le_bytes());
        vec_data.extend_from_slice(&self.rotational_speed.to_le_bytes());

        vec_data
    }
}


// -- Location -----------------------------------------------------------------------------------------------
#[derive(Debug, Copy, Clone)]
pub struct Location {
    pub fix_type: u8,
    pub num_sv: u8,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f32,
}


impl Location {
    pub fn new() -> Location{
        Location {
            fix_type:   0,
            num_sv:     0,
            latitude:   0.0_f64,
            longitude:  0.0_f64,
            altitude:   0.0_f32,
        }
    }


    pub const fn size() -> usize { 22 }


    pub fn parse(slice_data: &[u8]) -> Result<Location, &'static str> {
        if slice_data.len() == Location::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(Location{
                fix_type:   ext.get_u8(),
                num_sv:     ext.get_u8(),
                latitude:   ext.get_f64(),
                longitude:  ext.get_f64(),
                altitude:   ext.get_f32(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for Location {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.fix_type.to_le_bytes());
        vec_data.extend_from_slice(&self.num_sv.to_le_bytes());
        vec_data.extend_from_slice(&self.latitude.to_le_bytes());
        vec_data.extend_from_slice(&self.longitude.to_le_bytes());
        vec_data.extend_from_slice(&self.altitude.to_le_bytes());

        vec_data
    }
}


// -- LocationAdjust -----------------------------------------------------------------------------------------------
#[derive(Debug, Copy, Clone)]
pub struct LocationAdjust {
    pub mode_navigation: u8,
    pub latitude: f64,
    pub longitude: f64,
}


impl LocationAdjust {
    pub fn new() -> LocationAdjust{
        LocationAdjust {
            mode_navigation:    0,
            latitude:           0.0_f64,
            longitude:          0.0_f64,
        }
    }


    pub const fn size() -> usize { 17 }


    pub fn parse(slice_data: &[u8]) -> Result<LocationAdjust, &'static str> {
        if slice_data.len() == LocationAdjust::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(LocationAdjust{
                mode_navigation:    ext.get_u8(),
                latitude:           ext.get_f64(),
                longitude:          ext.get_f64(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for LocationAdjust {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.mode_navigation.to_le_bytes());
        vec_data.extend_from_slice(&self.latitude.to_le_bytes());
        vec_data.extend_from_slice(&self.longitude.to_le_bytes());

        vec_data
    }
}


// -- Monitor -----------------------------------------------------------------------------------------------
#[derive(Debug, Copy, Clone)]
pub struct Monitor {
    pub mode_navigation: mode::Navigation,
    pub distance_to_target: f32,
    pub velocity: f32,
    pub heading: f32,
    pub time_remain: u32,
}


impl Monitor {
    pub fn new() -> Monitor{
        Monitor {
            mode_navigation: mode::Navigation::None,
            distance_to_target:     0.0_f32,
            velocity:               0.0_f32,
            heading:                0.0_f32,
            time_remain:            0,
        }
    }


    pub const fn size() -> usize { 17 }


    pub fn parse(slice_data: &[u8]) -> Result<Monitor, &'static str> {
        if slice_data.len() == Monitor::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(Monitor{
                mode_navigation:        mode::Navigation::from_u32(ext.get_u32()),
                distance_to_target:     ext.get_f32(),
                velocity:               ext.get_f32(),
                heading:                ext.get_f32(),
                time_remain:            ext.get_u32(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for Monitor {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.mode_navigation.to_array());
        vec_data.extend_from_slice(&self.distance_to_target.to_le_bytes());
        vec_data.extend_from_slice(&self.velocity.to_le_bytes());
        vec_data.extend_from_slice(&self.heading.to_le_bytes());
        vec_data.extend_from_slice(&self.time_remain.to_le_bytes());

        vec_data
    }
}



// -- Heading -----------------------------------------------------------------------------------------------
#[derive(Debug, Copy, Clone)]
pub struct Heading {
    pub heading: f32,
    pub heading_path: f32,
    pub heading_to_target: f32,
    pub heading_error: f32,
}


impl Heading {
    pub fn new() -> Heading{
        Heading {
            heading:            0.0_f32,
            heading_path:       0.0_f32,
            heading_to_target:  0.0_f32,
            heading_error:      0.0_f32,
        }
    }


    pub const fn size() -> usize { 17 }


    pub fn parse(slice_data: &[u8]) -> Result<Heading, &'static str> {
        if slice_data.len() == Heading::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(Heading{
                heading:            ext.get_f32(),
                heading_path:       ext.get_f32(),
                heading_to_target:  ext.get_f32(),
                heading_error:      ext.get_f32(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for Heading {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.heading.to_le_bytes());
        vec_data.extend_from_slice(&self.heading_path.to_le_bytes());
        vec_data.extend_from_slice(&self.heading_to_target.to_le_bytes());
        vec_data.extend_from_slice(&self.heading_error.to_le_bytes());

        vec_data
    }
}


// -- Counter -----------------------------------------------------------------------------------------------
#[derive(Debug, Copy, Clone)]
pub struct Counter {
    pub count_per_sec_receive: u16,
    pub count_per_sec_transfer: u16,
}


impl Counter {
    pub fn new() -> Counter{
        Counter {
            count_per_sec_receive:   0,
            count_per_sec_transfer:  0,
        }
    }


    pub const fn size() -> usize { 4 }


    pub fn parse(slice_data: &[u8]) -> Result<Counter, &'static str> {
        if slice_data.len() == Counter::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(Counter{
                count_per_sec_receive:   ext.get_u16(),
                count_per_sec_transfer:  ext.get_u16(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for Counter {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.count_per_sec_receive.to_le_bytes());
        vec_data.extend_from_slice(&self.count_per_sec_transfer.to_le_bytes());

        vec_data
    }
}



// -- Satellite -----------------------------------------------------------------------------------------------
#[derive(Debug, Copy, Clone)]
pub struct Satellite {
    pub i_tow: u32,
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub min: u8,
    pub sec: u8,
    pub valid: u8,
    pub flags: u8,
    pub flags2: u8,
    pub g_speed: i32,
    pub p_dop: u16,
}

impl Satellite {
    pub fn new() -> Satellite{
        Satellite {
            i_tow:      0,
            year:       0,
            month:      0,
            day:        0,
            hour:       0,
            min:        0,
            sec:        0,
            valid:      0,
            flags:      0,
            flags2:     0,
            g_speed:    0,
            p_dop:      0,
        }
    }


    pub const fn size() -> usize { 21 }


    pub fn parse(slice_data: &[u8]) -> Result<Satellite, &'static str> {
        if slice_data.len() == Satellite::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(Satellite{
                i_tow:      ext.get_u32(),
                year:       ext.get_u16(),
                month:      ext.get_u8(),
                day:        ext.get_u8(),
                hour:       ext.get_u8(),
                min:        ext.get_u8(),
                sec:        ext.get_u8(),
                valid:      ext.get_u8(),
                flags:      ext.get_u8(),
                flags2:     ext.get_u8(),
                g_speed:    ext.get_i32(),
                p_dop:      ext.get_u16(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for Satellite {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.i_tow.to_le_bytes());
        vec_data.extend_from_slice(&self.year.to_le_bytes());
        vec_data.extend_from_slice(&self.month.to_le_bytes());
        vec_data.extend_from_slice(&self.day.to_le_bytes());
        vec_data.extend_from_slice(&self.hour.to_le_bytes());
        vec_data.extend_from_slice(&self.min.to_le_bytes());
        vec_data.extend_from_slice(&self.sec.to_le_bytes());
        vec_data.extend_from_slice(&self.valid.to_le_bytes());
        vec_data.extend_from_slice(&self.flags.to_le_bytes());
        vec_data.extend_from_slice(&self.flags2.to_le_bytes());
        vec_data.extend_from_slice(&self.g_speed.to_le_bytes());
        vec_data.extend_from_slice(&self.p_dop.to_le_bytes());

        vec_data
    }
}



// -- LocationXYZ -----------------------------------------------------------------------------------------------
#[derive(Debug, Copy, Clone)]
pub struct LocationXYZ {
    pub x: f64,
    pub y: f64,
    pub z: f32,
}


impl LocationXYZ {
    pub fn new() -> LocationXYZ{
        LocationXYZ {
            x: 0.0_f64,
            y: 0.0_f64,
            z: 0.0_f32,
        }
    }


    pub const fn size() -> usize { 20 }


    pub fn parse(slice_data: &[u8]) -> Result<LocationXYZ, &'static str> {
        if slice_data.len() == LocationXYZ::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(LocationXYZ{
                x: ext.get_f64(),
                y: ext.get_f64(),
                z: ext.get_f32(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for LocationXYZ {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.x.to_le_bytes());
        vec_data.extend_from_slice(&self.y.to_le_bytes());
        vec_data.extend_from_slice(&self.z.to_le_bytes());

        vec_data
    }
}


