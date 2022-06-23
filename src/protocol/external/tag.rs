use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
use std::vec;

use crate::protocol::Serializable;
use crate::communication::extractor::Extractor;


// -- TagData -----------------------------------------------------------------------------------------------
#[derive(Debug, Copy, Clone)]
pub struct TagData {
    pub x: i16,
    pub y: i16,
    pub width: i16,
    pub height: i16,
    pub id: u16,
}


impl TagData {
    pub fn new() -> TagData{
        TagData {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            id: 0,
        }
    }


    pub const fn size() -> usize { 10 }


    // TagData 배열로 변환
    pub fn parse(slice_data: &[u8]) -> Result<Vec<TagData>, &'static str> {
        if  slice_data.len() == TagData::size() ||
            (slice_data.len() > 0 && slice_data.len() % TagData::size() == 0)
        {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            let mut vec_tag_data: Vec<TagData> = Vec::new();

            let length = slice_data.len() % TagData::size();

            for _i in 0..length
            {
                vec_tag_data.push(TagData{
                    x: ext.get_i16(),
                    y: ext.get_i16(),
                    width: ext.get_i16(),
                    height: ext.get_i16(),
                    id: ext.get_u16(),
                });
            }

            Ok(vec_tag_data)
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for TagData {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.x.to_le_bytes());
        vec_data.extend_from_slice(&self.y.to_le_bytes());
        vec_data.extend_from_slice(&self.width.to_le_bytes());

        vec_data.extend_from_slice(&self.height.to_le_bytes());
        vec_data.extend_from_slice(&self.id.to_le_bytes());

        vec_data
    }
}
