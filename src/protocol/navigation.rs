use crate::protocol::{*};
use crate::communication::extractor::Extractor;


// -- Mode -------------------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Mode {
    #[num_enum(default)]
    None = 0x00,

    Ready,      // 준비

    Start,      // 시작
    Cruise,     // 동작
    Pause,      // 일시 정지
    Finish,     // 종료
    
    Error,      // 오류 발생
}


impl Mode {
    pub fn from_u8(data_u8: u8) -> Mode {
        match Mode::try_from( data_u8 ) {
            Ok(data) => { data },
            _ => { Mode::None },
        }
    }
}


// -- Behavior -------------------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Behavior {
    #[num_enum(default)]
    None = 0x00,

    Wait,       // 기다림
    
    Takeoff,    // 이륙
    Move,       // 이동
    Landing,    // 착륙
}


impl Behavior {
    pub fn from_u8(data_u8: u8) -> Behavior {
        match Behavior::try_from( data_u8 ) {
            Ok(data) => { data },
            _ => { Behavior::None },
        }
    }
}


// -- TargetMove -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct TargetMove {
    pub index: u8,              //  1 명령 번호
    pub latitude: f64,          //  9 위도(Y)
    pub longitude: f64,         // 17 경도(X)
    pub altitude: f32,          // 21 고도(Z)
    pub speed: u8,              // 22 속도(m/s) (0.0 ~ 255 m/s)
    pub heading: i16,           // 24 헤딩(degree, compass 0 ~ 360)
    pub rotational_speed: i8,   // 26 회전 속도(deg/sec) (-120 ~ +120)
}


impl TargetMove {
    pub fn new() -> TargetMove{
        TargetMove {
            index: 0,
            latitude: 0.0_f64,
            longitude: 0.0_f64,
            altitude: 0.0_f32,
            speed: 0,
            heading: 0,
            rotational_speed: 0,
        }
    }


    pub const fn size() -> usize { 25 }


    pub fn parse(slice_data: &[u8]) -> Result<TargetMove, &'static str> {
        if slice_data.len() == TargetMove::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(TargetMove{
                index: ext.get_u8(),
                latitude: ext.get_f64(),
                longitude: ext.get_f64(),
                altitude: ext.get_f32(),
                speed: ext.get_u8(),
                heading: ext.get_i16(),
                rotational_speed: ext.get_i8(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for TargetMove {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.index.to_le_bytes());
        vec_data.extend_from_slice(&self.latitude.to_le_bytes());
        vec_data.extend_from_slice(&self.longitude.to_le_bytes());
        vec_data.extend_from_slice(&self.altitude.to_le_bytes());
        vec_data.extend_from_slice(&self.speed.to_le_bytes());
        vec_data.extend_from_slice(&self.heading.to_le_bytes());
        vec_data.extend_from_slice(&self.rotational_speed.to_le_bytes());

        vec_data
    }
}


// -- TargetAction -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct TargetAction {
    pub index: u8,                      //  1 명령 번호
    pub mode_behavior: Behavior,    //  3 행동
    pub time: u32,                      //  5 실행 시간(ms)
}


impl TargetAction {
    pub fn new() -> TargetAction{
        TargetAction {
            index: 0,
            mode_behavior: Behavior::None,
            time: 0,
        }
    }


    pub const fn size() -> usize { 7 }


    pub fn parse(slice_data: &[u8]) -> Result<TargetAction, &'static str> {
        if slice_data.len() == TargetAction::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(TargetAction{
                index: ext.get_u8(),
                mode_behavior: Behavior::from_u8(ext.get_u8()),
                time: ext.get_u32(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for TargetAction {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.index.to_le_bytes());
        vec_data.push(self.mode_behavior.into());
        vec_data.extend_from_slice(&self.time.to_le_bytes());

        vec_data
    }
}


// -- Location -----------------------------------------------------------------------------------------------
#[derive(Debug)]
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
            fix_type: 0,
            num_sv: 0,
            latitude: 0.0_f64,
            longitude: 0.0_f64,
            altitude: 0.0_f32,
        }
    }


    pub const fn size() -> usize { 22 }


    pub fn parse(slice_data: &[u8]) -> Result<Location, &'static str> {
        if slice_data.len() == Location::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(Location{
                fix_type: ext.get_u8(),
                num_sv: ext.get_u8(),
                latitude: ext.get_f64(),
                longitude: ext.get_f64(),
                altitude: ext.get_f32(),
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
#[derive(Debug)]
pub struct LocationAdjust {
    pub mode: u8,
    pub latitude: f64,
    pub longitude: f64,
}


impl LocationAdjust {
    pub fn new() -> LocationAdjust{
        LocationAdjust {
            mode: 0,
            latitude: 0.0_f64,
            longitude: 0.0_f64,
        }
    }


    pub const fn size() -> usize { 17 }


    pub fn parse(slice_data: &[u8]) -> Result<LocationAdjust, &'static str> {
        if slice_data.len() == LocationAdjust::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(LocationAdjust{
                mode: ext.get_u8(),
                latitude: ext.get_f64(),
                longitude: ext.get_f64(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for LocationAdjust {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.mode.to_le_bytes());
        vec_data.extend_from_slice(&self.latitude.to_le_bytes());
        vec_data.extend_from_slice(&self.longitude.to_le_bytes());

        vec_data
    }
}


// -- Monitor -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct Monitor {
    pub mode_navigation: Mode,
    pub distance_to_target: f32,
    pub velocity: f32,
    pub heading: f32,
    pub rotational_velocity: u32,
}


impl Monitor {
    pub fn new() -> Monitor{
        Monitor {
            mode_navigation: Mode::None,
            distance_to_target: 0.0_f32,
            velocity: 0.0_f32,
            heading: 0.0_f32,
            rotational_velocity: 0,
        }
    }


    pub const fn size() -> usize { 17 }


    pub fn parse(slice_data: &[u8]) -> Result<Monitor, &'static str> {
        if slice_data.len() == Monitor::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(Monitor{
                mode_navigation: Mode::from_u8(ext.get_u8()),
                distance_to_target: ext.get_f32(),
                velocity: ext.get_f32(),
                heading: ext.get_f32(),
                rotational_velocity: ext.get_u32(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for Monitor {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.push(self.mode_navigation.into());
        vec_data.extend_from_slice(&self.distance_to_target.to_le_bytes());
        vec_data.extend_from_slice(&self.velocity.to_le_bytes());
        vec_data.extend_from_slice(&self.heading.to_le_bytes());
        vec_data.extend_from_slice(&self.rotational_velocity.to_le_bytes());

        vec_data
    }
}



// -- Heading -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct Heading {
    pub heading: f32,
    pub heading_path: f32,
    pub heading_to_target: f32,
    pub heading_error: f32,
}


impl Heading {
    pub fn new() -> Heading{
        Heading {
            heading: 0.0_f32,
            heading_path: 0.0_f32,
            heading_to_target: 0.0_f32,
            heading_error: 0.0_f32,
        }
    }


    pub const fn size() -> usize { 17 }


    pub fn parse(slice_data: &[u8]) -> Result<Heading, &'static str> {
        if slice_data.len() == Heading::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(Heading{
                heading: ext.get_f32(),
                heading_path: ext.get_f32(),
                heading_to_target: ext.get_f32(),
                heading_error: ext.get_f32(),
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
#[derive(Debug)]
pub struct Counter {
    pub count_per_sec_rf_receive: u16,
    pub count_per_sec_rf_transfer: u16,
}


impl Counter {
    pub fn new() -> Counter{
        Counter {
            count_per_sec_rf_receive: 0,
            count_per_sec_rf_transfer: 0,
        }
    }


    pub const fn size() -> usize { 4 }


    pub fn parse(slice_data: &[u8]) -> Result<Counter, &'static str> {
        if slice_data.len() == Counter::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(Counter{
                count_per_sec_rf_receive: ext.get_u16(),
                count_per_sec_rf_transfer: ext.get_u16(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for Counter {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.count_per_sec_rf_receive.to_le_bytes());
        vec_data.extend_from_slice(&self.count_per_sec_rf_transfer.to_le_bytes());

        vec_data
    }
}



// -- Satellite -----------------------------------------------------------------------------------------------
#[derive(Debug)]
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
            i_tow: 0,
            year: 0,
            month: 0,
            day: 0,
            hour: 0,
            min: 0,
            sec: 0,
            valid: 0,
            flags: 0,
            flags2: 0,
            g_speed: 0,
            p_dop: 0,
        }
    }


    pub const fn size() -> usize { 21 }


    pub fn parse(slice_data: &[u8]) -> Result<Satellite, &'static str> {
        if slice_data.len() == Satellite::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(Satellite{
                i_tow: ext.get_u32(),
                year: ext.get_u16(),
                month: ext.get_u8(),
                day: ext.get_u8(),
                hour: ext.get_u8(),
                min: ext.get_u8(),
                sec: ext.get_u8(),
                valid: ext.get_u8(),
                flags: ext.get_u8(),
                flags2: ext.get_u8(),
                g_speed: ext.get_i32(),
                p_dop: ext.get_u16(),
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
#[derive(Debug)]
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


