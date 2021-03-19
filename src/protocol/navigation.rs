use crate::protocol::{*};
use crate::communication::extractor::Extractor;


// -- ModeNavigation -------------------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum ModeNavigation {
    #[num_enum(default)]
    None = 0x00,

    Ready,      // 준비

    Start,      // 시작
    Cruise,     // 동작
    Pause,      // 일시 정지
    Finish,     // 종료
    
    Error,      // 오류 발생
}


impl ModeNavigation {
    pub fn from_u8(data_u8: u8) -> ModeNavigation {
        match ModeNavigation::try_from( data_u8 ) {
            Ok(data) => { data },
            _ => { ModeNavigation::None },
        }
    }
}


// -- ModeBehavior -------------------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum ModeBehavior {
    #[num_enum(default)]
    None = 0x00,

    Wait,       // 기다림
    
    Takeoff,    // 이륙
    Move,       // 이동
    Landing,    // 착륙
}


impl ModeBehavior {
    pub fn from_u8(data_u8: u8) -> ModeBehavior {
        match ModeBehavior::try_from( data_u8 ) {
            Ok(data) => { data },
            _ => { ModeBehavior::None },
        }
    }
}


// -- NavigationTargetMove -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct NavigationTargetMove {
    pub index: u8,              //  1 명령 번호
    pub latitude: f64,          //  9 위도(Y)
    pub longitude: f64,         // 17 경도(X)
    pub altitude: f32,          // 21 고도(Z)
    pub speed: u8,              // 22 속도(m/s) (0.0 ~ 255 m/s)
    pub heading: i16,           // 24 헤딩(degree, compass 0 ~ 360)
    pub rotational_speed: i8,   // 26 회전 속도(deg/sec) (-120 ~ +120)
}


impl NavigationTargetMove {
    pub fn new() -> NavigationTargetMove{
        NavigationTargetMove {
            index: 0,
            latitude: 0.0_f64,
            longitude: 0.0_f64,
            altitude: 0.0_f32,
            speed: 0,
            heading: 0,
            rotational_speed: 0,
        }
    }
    
    pub fn parse(slice_data: &[u8]) -> Result<NavigationTargetMove, &'static str> {
        if slice_data.len() == NavigationTargetMove::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(NavigationTargetMove{
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


impl Serializable for NavigationTargetMove {
    fn size() -> usize { 25 }


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


// -- NavigationTargetAction -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct NavigationTargetAction {
    pub index: u8,                      //  1 명령 번호
    pub mode_behavior: ModeBehavior,    //  3 행동
    pub time: u32,                      //  5 실행 시간(ms)
}


impl NavigationTargetAction {
    pub fn new() -> NavigationTargetAction{
        NavigationTargetAction {
            index: 0,
            mode_behavior: ModeBehavior::None,
            time: 0,
        }
    }
    
    pub fn parse(slice_data: &[u8]) -> Result<NavigationTargetAction, &'static str> {
        if slice_data.len() == NavigationTargetAction::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(NavigationTargetAction{
                index: ext.get_u8(),
                mode_behavior: ModeBehavior::from_u8(ext.get_u8()),
                time: ext.get_u32(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for NavigationTargetAction {
    fn size() -> usize { 7 }


    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.index.to_le_bytes());
        vec_data.push(self.mode_behavior.into());
        vec_data.extend_from_slice(&self.time.to_le_bytes());

        vec_data
    }
}


// -- NavigationLocation -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct NavigationLocation {
    pub fix_type: u8,
    pub num_sv: u8,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f32,
}


impl NavigationLocation {
    pub fn new() -> NavigationLocation{
        NavigationLocation {
            fix_type: 0,
            num_sv: 0,
            latitude: 0.0_f64,
            longitude: 0.0_f64,
            altitude: 0.0_f32,
        }
    }
    
    pub fn parse(slice_data: &[u8]) -> Result<NavigationLocation, &'static str> {
        if slice_data.len() == NavigationLocation::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(NavigationLocation{
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


impl Serializable for NavigationLocation {
    fn size() -> usize { 22 }


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


// -- NavigationLocationAdjust -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct NavigationLocationAdjust {
    pub mode: u8,
    pub latitude: f64,
    pub longitude: f64,
}


impl NavigationLocationAdjust {
    pub fn new() -> NavigationLocationAdjust{
        NavigationLocationAdjust {
            mode: 0,
            latitude: 0.0_f64,
            longitude: 0.0_f64,
        }
    }
    
    pub fn parse(slice_data: &[u8]) -> Result<NavigationLocationAdjust, &'static str> {
        if slice_data.len() == NavigationLocationAdjust::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(NavigationLocationAdjust{
                mode: ext.get_u8(),
                latitude: ext.get_f64(),
                longitude: ext.get_f64(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for NavigationLocationAdjust {
    fn size() -> usize { 17 }


    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.mode.to_le_bytes());
        vec_data.extend_from_slice(&self.latitude.to_le_bytes());
        vec_data.extend_from_slice(&self.longitude.to_le_bytes());

        vec_data
    }
}


// -- NavigationMonitor -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct NavigationMonitor {
    pub mode_navigation: ModeNavigation,
    pub distance_to_target: f32,
    pub velocity: f32,
    pub heading: f32,
    pub rotational_velocity: u32,
}


impl NavigationMonitor {
    pub fn new() -> NavigationMonitor{
        NavigationMonitor {
            mode_navigation: ModeNavigation::None,
            distance_to_target: 0.0_f32,
            velocity: 0.0_f32,
            heading: 0.0_f32,
            rotational_velocity: 0,
        }
    }
    
    pub fn parse(slice_data: &[u8]) -> Result<NavigationMonitor, &'static str> {
        if slice_data.len() == NavigationMonitor::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(NavigationMonitor{
                mode_navigation: ModeNavigation::from_u8(ext.get_u8()),
                distance_to_target: ext.get_f32(),
                velocity: ext.get_f32(),
                heading: ext.get_f32(),
                rotational_velocity: ext.get_u32(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for NavigationMonitor {
    fn size() -> usize { 17 }


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



// -- NavigationHeading -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct NavigationHeading {
    pub heading: f32,
    pub heading_path: f32,
    pub heading_to_target: f32,
    pub heading_error: f32,
}


impl NavigationHeading {
    pub fn new() -> NavigationHeading{
        NavigationHeading {
            heading: 0.0_f32,
            heading_path: 0.0_f32,
            heading_to_target: 0.0_f32,
            heading_error: 0.0_f32,
        }
    }
    
    pub fn parse(slice_data: &[u8]) -> Result<NavigationHeading, &'static str> {
        if slice_data.len() == NavigationHeading::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(NavigationHeading{
                heading: ext.get_f32(),
                heading_path: ext.get_f32(),
                heading_to_target: ext.get_f32(),
                heading_error: ext.get_f32(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for NavigationHeading {
    fn size() -> usize { 17 }


    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.heading.to_le_bytes());
        vec_data.extend_from_slice(&self.heading_path.to_le_bytes());
        vec_data.extend_from_slice(&self.heading_to_target.to_le_bytes());
        vec_data.extend_from_slice(&self.heading_error.to_le_bytes());

        vec_data
    }
}


// -- NavigationCounter -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct NavigationCounter {
    pub count_per_sec_rf_receive: u16,
    pub count_per_sec_rf_transfer: u16,
}


impl NavigationCounter {
    pub fn new() -> NavigationCounter{
        NavigationCounter {
            count_per_sec_rf_receive: 0,
            count_per_sec_rf_transfer: 0,
        }
    }
    
    pub fn parse(slice_data: &[u8]) -> Result<NavigationCounter, &'static str> {
        if slice_data.len() == NavigationCounter::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(NavigationCounter{
                count_per_sec_rf_receive: ext.get_u16(),
                count_per_sec_rf_transfer: ext.get_u16(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for NavigationCounter {
    fn size() -> usize { 4 }


    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.count_per_sec_rf_receive.to_le_bytes());
        vec_data.extend_from_slice(&self.count_per_sec_rf_transfer.to_le_bytes());

        vec_data
    }
}



// -- NavigationSatellite -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct NavigationSatellite {
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

impl NavigationSatellite {
    pub fn new() -> NavigationSatellite{
        NavigationSatellite {
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
    
    pub fn parse(slice_data: &[u8]) -> Result<NavigationSatellite, &'static str> {
        if slice_data.len() == NavigationSatellite::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(NavigationSatellite{
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


impl Serializable for NavigationSatellite {
    fn size() -> usize { 21 }


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



// -- NavigationLocationXYZ -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct NavigationLocationXYZ {
    pub x: f64,
    pub y: f64,
    pub z: f32,
}


impl NavigationLocationXYZ {
    pub fn new() -> NavigationLocationXYZ{
        NavigationLocationXYZ {
            x: 0.0_f64,
            y: 0.0_f64,
            z: 0.0_f32,
        }
    }
    
    pub fn parse(slice_data: &[u8]) -> Result<NavigationLocationXYZ, &'static str> {
        if slice_data.len() == NavigationLocationXYZ::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(NavigationLocationXYZ{
                x: ext.get_f64(),
                y: ext.get_f64(),
                z: ext.get_f32(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for NavigationLocationXYZ {
    fn size() -> usize { 20 }


    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.x.to_le_bytes());
        vec_data.extend_from_slice(&self.y.to_le_bytes());
        vec_data.extend_from_slice(&self.z.to_le_bytes());

        vec_data
    }
}


