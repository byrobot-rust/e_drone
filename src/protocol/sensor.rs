use crate::communication::extractor::Extractor;

use crate::protocol::Serializable;


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


    pub fn parse(motion: &mut Motion, vec_data: &Vec<u8>) -> bool {
        if vec_data.len() != Motion::size() {
            return false;
        }

        let mut ext: Extractor = Extractor::new(vec_data);

        motion.accel_x = ext.get_i16();
        motion.accel_y = ext.get_i16();
        motion.accel_z = ext.get_i16();

        motion.gyro_roll = ext.get_i16();
        motion.gyro_pitch = ext.get_i16();
        motion.gyro_yaw = ext.get_i16();

        motion.angle_roll = ext.get_i16();
        motion.angle_pitch = ext.get_i16();
        motion.angle_yaw = ext.get_i16();

        true
    }


    pub fn from_vec(vec_data: &Vec<u8>) -> Motion {
        let mut data = Motion::new();
        Motion::parse(&mut data, vec_data);
        data
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
