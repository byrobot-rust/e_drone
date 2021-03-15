use crate::protocol::{*};


pub fn check(header: &Header, vec_data: &Vec<u8>) -> Data
{
    if header.length != vec_data.len() as u8 {
        return Data::None;
    }

    let length : usize = header.length as usize;

    match header.data_type {
        // Information = 0x07
        DataType::Information => {
            match Information::parse(vec_data) {
                Ok(data) => return Data::Information(data),
                Err(_e) => {},
            }
        },
        // Control = 0x10
        DataType::Control => {
            if length == control::Quad8::size() {
                match control::Quad8::parse(vec_data) {
                    Ok(data) => return Data::Quad8(data),
                    Err(_e) => {},
                }
            }
        },
        // Motion = 0x44
        DataType::Motion => {
            match sensor::Motion::parse(vec_data) {
                Ok(data) => return Data::Motion(data),
                Err(_e) => {},
            }
        },
        _ => {},
    }


    Data::None
}

