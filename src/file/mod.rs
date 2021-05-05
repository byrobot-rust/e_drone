use std::fs::File;
use std::io::prelude::{*};

use crate::system::{*};
use crate::communication::extractor::Extractor;


#[derive(Debug, Copy, Clone)]
pub struct EncryptedBinaryHeader {
    pub model_number: ModelNumber,  // 4
    pub version: Version,           // 4
    pub length: u32,                // 4
    pub year: u16,                  // 2
    pub month: u8,                  // 1
    pub day: u8,                    // 1
}


impl EncryptedBinaryHeader {
    pub fn new() -> EncryptedBinaryHeader{
        EncryptedBinaryHeader {
            model_number: ModelNumber::None,
            version: Version{ major:21, minor:1, build:1 },
            length: 0,
            year: 2021,
            month: 1,
            day: 1,
        }
    }


    pub const fn size() -> usize { 16 }


    pub fn parse(slice_data: &[u8]) -> Result<EncryptedBinaryHeader, &'static str> {
        if slice_data.len() == EncryptedBinaryHeader::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(EncryptedBinaryHeader{
                model_number: ModelNumber::from_u32(ext.get_u32()),
                version: Version::from_u32(ext.get_u32()),
                length: ext.get_u32(),
                year: ext.get_u16(),
                month: ext.get_u8(),
                day: ext.get_u8(),
            })
        }
        else { Err("Wrong length") }
    }

    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.model_number.to_array());
        vec_data.extend_from_slice(&self.version.to_array());
        vec_data.extend_from_slice(&self.length.to_le_bytes());
        vec_data.extend_from_slice(&self.year.to_le_bytes());
        vec_data.extend_from_slice(&self.month.to_le_bytes());
        vec_data.extend_from_slice(&self.day.to_le_bytes());

        vec_data
    }
}


#[derive(Debug)]
pub struct EncryptedBinary {
    pub file_name: String,
    pub header: EncryptedBinaryHeader,
    pub data_array: Vec<u8>,
    pub flag_open: bool
}

impl EncryptedBinary {
    pub fn new() -> EncryptedBinary {
        EncryptedBinary {
            file_name: String::from(""),
            header: EncryptedBinaryHeader::new(),
            data_array: Vec::new(),
            flag_open: false,
        }
    }


    pub fn read(&mut self, file_name: &str) -> bool {
        // open target file
        let file = File::open(&file_name);

        match file {
            Ok(mut f) => {
                // 파일 전체 읽기
                match f.read_to_end(&mut self.data_array)
                {
                    Ok(length) => {
                        if length > 16 {
                            match EncryptedBinaryHeader::parse(&self.data_array[0..16]) {
                                Ok(header) => {
                                    self.header = header;
                                    self.flag_open = true;
                                    return true;
                                },
                                _ => {}
                            }
                        }
                    },
                    _ => { },
                }
            },
            _ => {}
        }

        self.flag_open = false;
        false
    }

    /*
    pub fn get_data_block(&self, index_block: u16) -> Result<[u8; 16], &'static str>
    {
        let mut data_block: [u8; 16] = [0; 16];

        let index_array: usize = (index_block as usize) << 4;
        if index_array + 16 <= self.data_array.len()
        {
            data_block.clone_from_slice(&self.data_array[index_array..index_array + 16]);
            return Ok(data_block);
        }

        if self.data_array.len() == 0 {
            Err("Data array length is zero.")
        }
        else
        {
            Err("index over")
        }
    }
    // */


    pub fn get_data_block(&self, index_block: u16, count_block: u16) -> Option<Vec<u8>>
    {
        let index_array: usize = (index_block as usize) << 4;
        let index_end: usize = std::cmp::min(index_array + (16 * count_block as usize), self.data_array.len());
        if  index_array < self.data_array.len() &&
            index_end <= self.data_array.len() {
            return Some(self.data_array[index_array..index_end].to_vec());
        }
        
        None
    }


    pub fn get_length(&self) -> usize
    {
        self.data_array.len()
    }
}

