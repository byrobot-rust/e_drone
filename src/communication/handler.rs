use crate::protocol;
use crate::protocol::{DataType};


pub fn check(header: &protocol::Header, vec_data: &Vec<u8>) -> protocol::Data
{
    if header.length != vec_data.len() as u8 {
        return protocol::Data::None;
    }

    let length : usize = header.length as usize;

    match header.data_type {
        DataType::Motion => {
            match protocol::sensor::Motion::parse(vec_data) {
                Ok(data) => return protocol::Data::Motion(data),
                Err(_e) => {},
            }
        },
        DataType::Information => {
            match protocol::Information::parse(vec_data) {
                Ok(data) => return protocol::Data::Information(data),
                Err(_e) => {},
            }

            /*
            // 길이 확인을 먼저 하는 경우
            if length == protocol::Information::size() {
                let mut data: protocol::Information = protocol::Information::new();
                if protocol::Information::parse(&mut data, vec_data) {
                    return protocol::Data::Information (data);
                }
            }
             */
        },
        _ => {},
    }


    protocol::Data::None
}

