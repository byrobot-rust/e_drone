use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
use std::vec;

use crate::protocol::Serializable;
use crate::communication::extractor::Extractor;


// -- LidarData -----------------------------------------------------------------------------------------------
#[derive(Debug, Copy, Clone)]
pub struct LidarData {
    pub angle_radian_x1000: u16,
    pub distance_mm: u16,
}


impl LidarData {
    pub fn new() -> LidarData{
        LidarData {
            //angle_degree_x10: 0,
            angle_radian_x1000: 0,
            distance_mm: 0,
        }
    }


    pub const fn size() -> usize { 4 }


    // LidarData 배열로 변환
    pub fn parse(slice_data: &[u8]) -> Result<Vec<LidarData>, &'static str> {
        if  slice_data.len() == LidarData::size() ||
            (slice_data.len() > 0 && slice_data.len() % LidarData::size() == 0)
        {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            let mut vec_lidar_data: Vec<LidarData> = Vec::new();

            let length = slice_data.len() % LidarData::size();

            for _i in 0..length
            {
                vec_lidar_data.push(LidarData{
                    angle_radian_x1000: ext.get_u16(),
                    distance_mm: ext.get_u16(),
                });
            }

            Ok(vec_lidar_data)
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for LidarData {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.angle_radian_x1000.to_le_bytes());
        vec_data.extend_from_slice(&self.distance_mm.to_le_bytes());

        vec_data
    }
}
