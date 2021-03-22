use crate::protocol::Serializable;
use crate::communication::extractor::Extractor;


// -- RawMotion -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct RawMotion {
    pub accel_x: i16,
    pub accel_y: i16,
    pub accel_z: i16,
    pub gyro_roll: i16,
    pub gyro_pitch: i16,
    pub gyro_yaw: i16,
}


impl RawMotion {
    pub fn new() -> RawMotion{
        RawMotion {
            accel_x: 0,
            accel_y: 0,
            accel_z: 0,
            gyro_roll: 0,
            gyro_pitch: 0,
            gyro_yaw: 0,
        }
    }


    pub const fn size() -> usize { 12 }


    pub fn parse(slice_data: &[u8]) -> Result<RawMotion, &'static str> {
        if slice_data.len() == RawMotion::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(RawMotion{
                accel_x: ext.get_i16(),
                accel_y: ext.get_i16(),
                accel_z: ext.get_i16(),
        
                gyro_roll: ext.get_i16(),
                gyro_pitch: ext.get_i16(),
                gyro_yaw: ext.get_i16(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for RawMotion {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.accel_x.to_le_bytes());
        vec_data.extend_from_slice(&self.accel_y.to_le_bytes());
        vec_data.extend_from_slice(&self.accel_z.to_le_bytes());

        vec_data.extend_from_slice(&self.gyro_roll.to_le_bytes());
        vec_data.extend_from_slice(&self.gyro_pitch.to_le_bytes());
        vec_data.extend_from_slice(&self.gyro_yaw.to_le_bytes());

        vec_data
    }
}


// -- RawFlow -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct RawFlow {
    pub x: f32,
    pub y: f32,
}


impl RawFlow {
    pub fn new() -> RawFlow{
        RawFlow {
            x: 0.0_f32,
            y: 0.0_f32,
        }
    }


    pub const fn size() -> usize { 8 }


    pub fn parse(slice_data: &[u8]) -> Result<RawFlow, &'static str> {
        if slice_data.len() == RawFlow::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(RawFlow{
                x: ext.get_f32(),
                y: ext.get_f32(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for RawFlow {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.x.to_le_bytes());
        vec_data.extend_from_slice(&self.y.to_le_bytes());

        vec_data
    }
}


// -- Attitude -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct Attitude {
    pub roll: i16,
    pub pitch: i16,
    pub yaw: i16,
}


impl Attitude {
    pub fn new() -> Attitude{
        Attitude {
            roll: 0,
            pitch: 0,
            yaw: 0,
        }
    }


    pub const fn size() -> usize { 6 }


    pub fn parse(slice_data: &[u8]) -> Result<Attitude, &'static str> {
        if slice_data.len() == Attitude::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(Attitude{
                roll: ext.get_i16(),
                pitch: ext.get_i16(),
                yaw: ext.get_i16(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for Attitude {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.roll.to_le_bytes());
        vec_data.extend_from_slice(&self.pitch.to_le_bytes());
        vec_data.extend_from_slice(&self.yaw.to_le_bytes());

        vec_data
    }
}



// -- Motion -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct Motion {
    pub accel_x: i16,
    pub accel_y: i16,
    pub accel_z: i16,
    pub gyro_roll: i16,
    pub gyro_pitch: i16,
    pub gyro_yaw: i16,
    pub angle_roll: i16,
    pub angle_pitch: i16,
    pub angle_yaw: i16,
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


    pub const fn size() -> usize { 18 }


    pub fn parse(slice_data: &[u8]) -> Result<Motion, &'static str> {
        if slice_data.len() == Motion::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(Motion{
                accel_x: ext.get_i16(),
                accel_y: ext.get_i16(),
                accel_z: ext.get_i16(),
        
                gyro_roll: ext.get_i16(),
                gyro_pitch: ext.get_i16(),
                gyro_yaw: ext.get_i16(),
        
                angle_roll: ext.get_i16(),
                angle_pitch: ext.get_i16(),
                angle_yaw: ext.get_i16(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for Motion {
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



// -- Range -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct Range {
    pub left: i16,
    pub front: i16,
    pub right: i16,
    pub rear: i16,
    pub top: i16,
    pub bottom: i16,
}


impl Range {
    pub fn new() -> Range{
        Range {
            left: 0,
            front: 0,
            right: 0,
            rear: 0,
            top: 0,
            bottom: 0,
        }
    }


    pub const fn size() -> usize { 12 }


    pub fn parse(slice_data: &[u8]) -> Result<Range, &'static str> {
        if slice_data.len() == Range::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(Range{
                left: ext.get_i16(),
                front: ext.get_i16(),
                right: ext.get_i16(),
        
                rear: ext.get_i16(),
                top: ext.get_i16(),
                bottom: ext.get_i16(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for Range {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.left.to_le_bytes());
        vec_data.extend_from_slice(&self.front.to_le_bytes());
        vec_data.extend_from_slice(&self.right.to_le_bytes());

        vec_data.extend_from_slice(&self.rear.to_le_bytes());
        vec_data.extend_from_slice(&self.top.to_le_bytes());
        vec_data.extend_from_slice(&self.bottom.to_le_bytes());

        vec_data
    }
}


// -- Position -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}


impl Position {
    pub fn new() -> Position{
        Position {
            x: 0.0_f32,
            y: 0.0_f32,
            z: 0.0_f32,
        }
    }


    pub const fn size() -> usize { 12 }


    pub fn parse(slice_data: &[u8]) -> Result<Position, &'static str> {
        if slice_data.len() == Position::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(Position{
                x: ext.get_f32(),
                y: ext.get_f32(),
                z: ext.get_f32(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for Position {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.x.to_le_bytes());
        vec_data.extend_from_slice(&self.y.to_le_bytes());
        vec_data.extend_from_slice(&self.z.to_le_bytes());

        vec_data
    }
}


// -- PositionVelocity -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct PositionVelocity {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub vx: f32,
    pub vy: f32,
    pub vz: f32,
}


impl PositionVelocity {
    pub fn new() -> PositionVelocity{
        PositionVelocity {
            x: 0.0_f32,
            y: 0.0_f32,
            z: 0.0_f32,
            vx: 0.0_f32,
            vy: 0.0_f32,
            vz: 0.0_f32,
        }
    }


    pub const fn size() -> usize { 24 }


    pub fn parse(slice_data: &[u8]) -> Result<PositionVelocity, &'static str> {
        if slice_data.len() == PositionVelocity::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(PositionVelocity{
                x: ext.get_f32(),
                y: ext.get_f32(),
                z: ext.get_f32(),
                vx: ext.get_f32(),
                vy: ext.get_f32(),
                vz: ext.get_f32(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for PositionVelocity {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.x.to_le_bytes());
        vec_data.extend_from_slice(&self.y.to_le_bytes());
        vec_data.extend_from_slice(&self.z.to_le_bytes());
        vec_data.extend_from_slice(&self.vx.to_le_bytes());
        vec_data.extend_from_slice(&self.vy.to_le_bytes());
        vec_data.extend_from_slice(&self.vz.to_le_bytes());

        vec_data
    }
}



// -- Bias -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct Bias {
    pub accel_x: i16,
    pub accel_y: i16,
    pub accel_z: i16,
    pub gyro_roll: i16,
    pub gyro_pitch: i16,
    pub gyro_yaw: i16,
}


impl Bias {
    pub fn new() -> Bias{
        Bias {
            accel_x: 0,
            accel_y: 0,
            accel_z: 0,
            gyro_roll: 0,
            gyro_pitch: 0,
            gyro_yaw: 0,
        }
    }


    pub const fn size() -> usize { 12 }


    pub fn parse(slice_data: &[u8]) -> Result<Bias, &'static str> {
        if slice_data.len() == Bias::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(Bias{
                accel_x: ext.get_i16(),
                accel_y: ext.get_i16(),
                accel_z: ext.get_i16(),
        
                gyro_roll: ext.get_i16(),
                gyro_pitch: ext.get_i16(),
                gyro_yaw: ext.get_i16(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for Bias {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.accel_x.to_le_bytes());
        vec_data.extend_from_slice(&self.accel_y.to_le_bytes());
        vec_data.extend_from_slice(&self.accel_z.to_le_bytes());

        vec_data.extend_from_slice(&self.gyro_roll.to_le_bytes());
        vec_data.extend_from_slice(&self.gyro_pitch.to_le_bytes());
        vec_data.extend_from_slice(&self.gyro_yaw.to_le_bytes());

        vec_data
    }
}



// -- Trim -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct Trim {
    pub roll: i16,
    pub pitch: i16,
    pub yaw: i16,
    pub throttle: i16,
}


impl Trim {
    pub fn new() -> Trim{
        Trim {
            roll: 0,
            pitch: 0,
            yaw: 0,
            throttle: 0,
        }
    }


    pub const fn size() -> usize { 8 }


    pub fn parse(slice_data: &[u8]) -> Result<Trim, &'static str> {
        if slice_data.len() == Trim::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(Trim{
                roll: ext.get_i16(),
                pitch: ext.get_i16(),
                yaw: ext.get_i16(),
                throttle: ext.get_i16(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for Trim {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.roll.to_le_bytes());
        vec_data.extend_from_slice(&self.pitch.to_le_bytes());
        vec_data.extend_from_slice(&self.yaw.to_le_bytes());
        vec_data.extend_from_slice(&self.throttle.to_le_bytes());

        vec_data
    }
}



// -- MagnetometerOffset -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct MagnetometerOffset {
    pub offset: i16,
}


impl MagnetometerOffset {
    pub fn new() -> MagnetometerOffset{
        MagnetometerOffset {
            offset: 0,
        }
    }


    pub const fn size() -> usize { 2 }


    pub fn parse(slice_data: &[u8]) -> Result<MagnetometerOffset, &'static str> {
        if slice_data.len() == MagnetometerOffset::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(MagnetometerOffset{
                offset: ext.get_i16(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for MagnetometerOffset {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.offset.to_le_bytes());

        vec_data
    }
}


