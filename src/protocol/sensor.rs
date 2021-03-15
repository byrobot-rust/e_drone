use crate::protocol::Serializable;
use crate::communication::extractor::Extractor;


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
    fn size() -> usize { 18 }


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
