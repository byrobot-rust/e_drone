use crate::protocol::DataType;
use crate::base::system::DeviceType;
use crate::communication::crc16;


pub fn start() -> Vec<u8>
{
    let mut vec_data: Vec<u8> = Vec::new();

    // start code
    vec_data.push(0x0A);
    vec_data.push(0x55);

    vec_data
}


pub fn add_crc16(vec_data: &mut Vec<u8>)
{
    // crc16
    let crc: u16 = crc16::calc_array(0, &vec_data[2..]);
    vec_data.push((crc & 0xFF) as u8);
    vec_data.push((crc >> 8) as u8);
}


pub fn request(target: DeviceType, data_type: DataType) -> Vec<u8>
{
    let mut vec_data: Vec<u8> = start();

    // header
    vec_data.push(DataType::Request.into());
    vec_data.push(1);
    vec_data.push(DeviceType::Base.into());
    vec_data.push(target.into());

    // data
    vec_data.push(data_type.into());

    // crc16
    add_crc16(&mut vec_data);

    vec_data
}


pub fn control(roll: i8, pitch: i8, yaw: i8, throttle: i8) -> Vec<u8>
{
    let mut vec_data: Vec<u8> = start();

    // header
    vec_data.push(DataType::Control.into());
    vec_data.push(4);
    vec_data.push(DeviceType::Base.into());
    vec_data.push(DeviceType::Drone.into());

    // data
    vec_data.push(roll as u8);
    vec_data.push(pitch as u8);
    vec_data.push(yaw as u8);
    vec_data.push(throttle as u8);

    // crc16
    add_crc16(&mut vec_data);

    vec_data
}

