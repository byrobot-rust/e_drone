use std::fs::File;
use std::io::prelude::{*};
use std::convert::TryFrom;

use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;
use byteorder::{ByteOrder, LittleEndian};

use crate::system::{*};
use crate::communication::extractor::Extractor;


/*
// https://dev.to/dandyvica/different-ways-of-reading-files-in-rust-2n30
// Read PNG file image width and height
fn read_png(file_name: &str) -> Result<(), std::io::Error> {
    const BUFFER_SIZE: usize = 256;

    // open target file
    let mut file = File::open(&file_name)?;

    // we'll use this buffer to get data
    let mut buffer = [0; BUFFER_SIZE];

    // reader PNG header of 8 bytes
    let _ = file.by_ref().take(8).read(&mut buffer)?;
    assert_eq!(&buffer[1..4], "PNG".as_bytes());

    // read IHDR chunk
    let chunk_size = file.read_u32::<BigEndian>().unwrap();
    let _ = file.by_ref().take(4).read(&mut buffer)?;
    assert_eq!(&buffer[0..4], "IHDR".as_bytes());

    let image_width = file.read_u32::<BigEndian>().unwrap();
    let image_height = file.read_u32::<BigEndian>().unwrap();
    println!("image is W={} x H={}", image_width, image_height);

    Ok(())
}
// */


#[derive(Debug)]
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
}

impl EncryptedBinary {
    pub fn new() -> EncryptedBinary {
        EncryptedBinary {
            file_name: String::from(""),
            header: EncryptedBinaryHeader::new(),
            data_array: Vec::new(),
        }
    }


    pub fn read(&mut self, file_name: &str) -> bool {
        // open target file
        let file = File::open(&file_name);

        match file {
            Ok(mut f) => {
                // 파일 전체 읽기
                f.read_to_end(&mut self.data_array);

                match EncryptedBinaryHeader::parse(&self.data_array[0..16]) {
                    Ok(header) => { self.header = header; },
                    _ => { return false; }
                }
            },
            _ => {}
        }

        true
    }
}