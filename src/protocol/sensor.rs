use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
use byteorder::{ByteOrder, LittleEndian};

use crate::base::system::{*};
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

        /*
        motion.accel_x = LittleEndian::read_i16(&vec_data[0..1]);
        motion.accel_y = LittleEndian::read_i16(&vec_data[2..3]);
        motion.accel_z = LittleEndian::read_i16(&vec_data[4..5]);

        motion.gyro_roll = LittleEndian::read_i16(&vec_data[6..7]);
        motion.gyro_pitch = LittleEndian::read_i16(&vec_data[8..9]);
        motion.gyro_yaw = LittleEndian::read_i16(&vec_data[10..11]);

        motion.angle_roll = LittleEndian::read_i16(&vec_data[12..13]);
        motion.angle_pitch = LittleEndian::read_i16(&vec_data[14..15]);
        motion.angle_yaw = LittleEndian::read_i16(&vec_data[16..17]);
        // */

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
