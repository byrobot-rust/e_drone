use crate::protocol::{*};
use crate::communication::extractor::Extractor;


// -- MotorV -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct MotorV {
    pub value: i16,
}


impl MotorV {
    pub fn new() -> MotorV{
        MotorV {
            value: 0,
        }
    }
    
    pub fn parse(slice_data: &[u8]) -> Result<MotorV, &'static str> {
        if slice_data.len() == MotorV::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(MotorV{
                value: ext.get_i16(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for MotorV {
    fn size() -> usize { 2 }


    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.value.to_le_bytes());

        vec_data
    }
}


// -- MotorRV -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct MotorRV {
    pub rotation: Rotation,
    pub value: i16,
}


impl MotorRV {
    pub fn new() -> MotorRV{
        MotorRV {
            rotation: Rotation::Clockwise,
            value: 0,
        }
    }
    
    pub fn parse(slice_data: &[u8]) -> Result<MotorRV, &'static str> {
        if slice_data.len() == MotorRV::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(MotorRV{
                rotation: Rotation::from_u8(ext.get_u8()),
                value: ext.get_i16(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for MotorRV {
    fn size() -> usize { 3 }


    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.push(self.rotation.into());
        vec_data.extend_from_slice(&self.value.to_le_bytes());

        vec_data
    }
}


// -- MotorVA -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct MotorVA {
    pub value: i16,
    pub adc: i16,
}


impl MotorVA {
    pub fn new() -> MotorVA{
        MotorVA {
            value: 0,
            adc: 0,
        }
    }
    
    pub fn parse(slice_data: &[u8]) -> Result<MotorVA, &'static str> {
        if slice_data.len() == MotorVA::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(MotorVA{
                value: ext.get_i16(),
                adc: ext.get_i16(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for MotorVA {
    fn size() -> usize { 4 }


    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.value.to_le_bytes());
        vec_data.extend_from_slice(&self.adc.to_le_bytes());

        vec_data
    }
}


// -- MotorRVA -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct MotorRVA {
    pub rotation: Rotation,
    pub value: i16,
    pub adc: i16,
}


impl MotorRVA {
    pub fn new() -> MotorRVA{
        MotorRVA {
            rotation: Rotation::Clockwise,
            value: 0,
            adc: 0,
        }
    }
    
    pub fn parse(slice_data: &[u8]) -> Result<MotorRVA, &'static str> {
        if slice_data.len() == MotorRVA::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(MotorRVA{
                rotation: Rotation::from_u8(ext.get_u8()),
                value: ext.get_i16(),
                adc: ext.get_i16(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for MotorRVA {
    fn size() -> usize { 5 }


    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.push(self.rotation.into());
        vec_data.extend_from_slice(&self.value.to_le_bytes());
        vec_data.extend_from_slice(&self.adc.to_le_bytes());

        vec_data
    }
}



// -- MotorSingleV -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct MotorSingleV {
    pub target: u8,
    pub value: i16,
}


impl MotorSingleV {
    pub fn new() -> MotorSingleV{
        MotorSingleV {
            target: 0,
            value: 0,
        }
    }
    
    pub fn parse(slice_data: &[u8]) -> Result<MotorSingleV, &'static str> {
        if slice_data.len() == MotorSingleV::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(MotorSingleV{
                target: ext.get_u8(),
                value: ext.get_i16(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for MotorSingleV {
    fn size() -> usize { 3 }


    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.target.to_le_bytes());
        vec_data.extend_from_slice(&self.value.to_le_bytes());

        vec_data
    }
}


// -- MotorSingleRV -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct MotorSingleRV {
    pub target: u8,
    pub rotation: Rotation,
    pub value: i16,
}


impl MotorSingleRV {
    pub fn new() -> MotorSingleRV{
        MotorSingleRV {
            target: 0,
            rotation: Rotation::Clockwise,
            value: 0,
        }
    }
    
    pub fn parse(slice_data: &[u8]) -> Result<MotorSingleRV, &'static str> {
        if slice_data.len() == MotorSingleRV::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(MotorSingleRV{
                target: ext.get_u8(),
                rotation: Rotation::from_u8(ext.get_u8()),
                value: ext.get_i16(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for MotorSingleRV {
    fn size() -> usize { 4 }


    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.target.to_le_bytes());
        vec_data.push(self.rotation.into());
        vec_data.extend_from_slice(&self.value.to_le_bytes());

        vec_data
    }
}
