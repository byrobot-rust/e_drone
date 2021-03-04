use crate::protocol;
use crate::protocol::{DataType, Serializable};


pub fn check(header: protocol::Header, vec_data: Vec<u8>) -> protocol::Data
{
    if header.length != vec_data.len() as u8 {
        return protocol::Data::None;
    }


    match header.data_type {
        DataType::Motion => {
            if header.length == protocol::Motion::size() {
                let mut data: protocol::Motion = protocol::Motion::new();
                if protocol::Motion::parse(&mut data, vec_data) {
                    return protocol::Data::Motion { motion: data };
                }
            }
        },
        _ => {},
    }


    return protocol::Data::None;
}

