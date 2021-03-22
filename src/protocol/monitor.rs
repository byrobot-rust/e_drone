use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

use crate::protocol::Serializable;
use crate::communication::extractor::Extractor;


// -- HeaderType ----------------------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum HeaderType {
    #[num_enum(default)]
    Monitor0    = 0x00,
    Monitor4    = 0x01,
    Monitor8    = 0x02
}


impl HeaderType {
    pub fn from_u8(data_u8: u8) -> HeaderType {
        match HeaderType::try_from( data_u8 ) {
            Ok(data) => { data },
            _ => { HeaderType::Monitor0 },
        }
    }
}


// -- DataType ----------------------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum DataType {
    #[num_enum(default)]
    U8      = 0x00,
    S8      = 0x01,
    U16     = 0x02,
    S16     = 0x03,
    U32     = 0x04,
    S32     = 0x05,
    U64     = 0x06,
    S64     = 0x07,
    F32     = 0x08,
    F64     = 0x09
}


impl DataType {
    pub fn from_u8(data_u8: u8) -> DataType {
        match DataType::try_from( data_u8 ) {
            Ok(data) => { data },
            _ => { DataType::U8 },
        }
    }
}


// -- MonitorType -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct MonitorType {
    pub header_type: HeaderType,
}


impl MonitorType {
    pub fn new() -> MonitorType{
        MonitorType {
            header_type: HeaderType::Monitor4,
        }
    }


    pub const fn size() -> usize { 1 }


    pub fn parse(slice_data: &[u8]) -> Result<MonitorType, &'static str> {
        if slice_data.len() == MonitorType::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(MonitorType{
                header_type: HeaderType::from_u8(ext.get_u8()),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for MonitorType {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.push(self.header_type.into());

        vec_data
    }
}


// -- Monitor0 -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct Monitor0 {
    pub data_type: DataType,
    pub index: u8,
    pub vec_value: Vec<f64>,
}


impl Monitor0 {
    pub fn new() -> Monitor0{
        Monitor0 {
            data_type: DataType::U8,
            index: 0,
            vec_value: Vec::new(),
        }
    }


    pub const fn size() -> usize { 2 }


    pub fn parse(slice_data: &[u8]) -> Result<Monitor0, &'static str> {
        if slice_data.len() == Monitor0::size() {
            let mut monitor: Monitor0 = Monitor0::new();
            let mut ext: Extractor = Extractor::from_slice(slice_data);

            monitor.data_type = DataType::from_u8(ext.get_u8());
            monitor.index = ext.get_u8();
            monitor.vec_value = Vec::new();

            loop
            {
                if ext.check_remain() > 0
                {
                    let value : f64 = match monitor.data_type {
                        DataType::U8 => { ext.get_u8() as f64 },
                        DataType::S8 => { ext.get_i8() as f64 },
                        DataType::U16 => { ext.get_u16() as f64 },
                        DataType::S16 => { ext.get_i16() as f64 },
                        DataType::U32 => { ext.get_u32() as f64 },
                        DataType::S32 => { ext.get_i32() as f64 },
                        DataType::F32 => { ext.get_f32() as f64 },
                        DataType::F64 => { ext.get_f64() },
                        _ => { 0.0_f64 },
                    };

                    monitor.vec_value.push(value);
                }
                else
                {
                    break;
                }
            }

            Ok(monitor)
        }
        else { Err("Wrong length") }
    }


    pub fn get_length(&self) -> usize { 
        Monitor0::size() + (self.vec_value.len() * 8)
    }
}


impl Serializable for Monitor0 {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.push(self.data_type.into());
        vec_data.push(self.index);

        for v in self.vec_value.iter() {
            vec_data.extend_from_slice(&v.to_le_bytes());
        }

        vec_data
    }
}


// -- Monitor4 -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct Monitor4 {
    pub system_time: u32,
    pub data_type: DataType,
    pub index: u8,
    pub vec_value: Vec<f64>,
}


impl Monitor4 {
    pub fn new() -> Monitor4{
        Monitor4 {
            system_time: 0,
            data_type: DataType::U8,
            index: 0,
            vec_value: Vec::new(),
        }
    }


    pub const fn size() -> usize { 6 }


    pub fn parse(slice_data: &[u8]) -> Result<Monitor4, &'static str> {
        if slice_data.len() == Monitor4::size() {
            let mut monitor: Monitor4 = Monitor4::new();
            let mut ext: Extractor = Extractor::from_slice(slice_data);

            monitor.system_time = ext.get_u32();
            monitor.data_type = DataType::from_u8(ext.get_u8());
            monitor.index = ext.get_u8();
            monitor.vec_value = Vec::new();

            loop
            {
                if ext.check_remain() > 0
                {
                    let value : f64 = match monitor.data_type {
                        DataType::U8 => { ext.get_u8() as f64 },
                        DataType::S8 => { ext.get_i8() as f64 },
                        DataType::U16 => { ext.get_u16() as f64 },
                        DataType::S16 => { ext.get_i16() as f64 },
                        DataType::U32 => { ext.get_u32() as f64 },
                        DataType::S32 => { ext.get_i32() as f64 },
                        DataType::F32 => { ext.get_f32() as f64 },
                        DataType::F64 => { ext.get_f64() },
                        _ => { 0.0_f64 },
                    };

                    monitor.vec_value.push(value);
                }
                else
                {
                    break;
                }
            }

            Ok(monitor)
        }
        else { Err("Wrong length") }
    }


    pub fn get_length(&self) -> usize { 
        Monitor4::size() + (self.vec_value.len() * 8)
    }
}


impl Serializable for Monitor4 {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.system_time.to_le_bytes());
        vec_data.push(self.data_type.into());
        vec_data.push(self.index);

        for v in self.vec_value.iter() {
            vec_data.extend_from_slice(&v.to_le_bytes());
        }

        vec_data
    }
}


// -- Monitor8 -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct Monitor8 {
    pub system_time: u64,
    pub data_type: DataType,
    pub index: u8,
    pub vec_value: Vec<f64>,
}


impl Monitor8 {
    pub fn new() -> Monitor8{
        Monitor8 {
            system_time: 0,
            data_type: DataType::U8,
            index: 0,
            vec_value: Vec::new(),
        }
    }


    pub const fn size() -> usize { 10 }


    pub fn parse(slice_data: &[u8]) -> Result<Monitor8, &'static str> {
        if slice_data.len() == Monitor8::size() {
            let mut monitor: Monitor8 = Monitor8::new();
            let mut ext: Extractor = Extractor::from_slice(slice_data);

            monitor.system_time = ext.get_u64();
            monitor.data_type = DataType::from_u8(ext.get_u8());
            monitor.index = ext.get_u8();
            monitor.vec_value = Vec::new();

            loop
            {
                if ext.check_remain() > 0
                {
                    let value : f64 = match monitor.data_type {
                        DataType::U8 => { ext.get_u8() as f64 },
                        DataType::S8 => { ext.get_i8() as f64 },
                        DataType::U16 => { ext.get_u16() as f64 },
                        DataType::S16 => { ext.get_i16() as f64 },
                        DataType::U32 => { ext.get_u32() as f64 },
                        DataType::S32 => { ext.get_i32() as f64 },
                        DataType::F32 => { ext.get_f32() as f64 },
                        DataType::F64 => { ext.get_f64() },
                        _ => { 0.0_f64 },
                    };

                    monitor.vec_value.push(value);
                }
                else
                {
                    break;
                }
            }

            Ok(monitor)
        }
        else { Err("Wrong length") }
    }


    pub fn get_length(&self) -> usize { 
        Monitor8::size() + (self.vec_value.len() * 8)
    }
}


impl Serializable for Monitor8 {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.system_time.to_le_bytes());
        vec_data.push(self.data_type.into());
        vec_data.push(self.index);

        for v in self.vec_value.iter() {
            vec_data.extend_from_slice(&v.to_le_bytes());
        }

        vec_data
    }
}



