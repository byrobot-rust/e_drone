use crate::protocol::Serializable;
use crate::protocol::DataType;
use crate::base::system::DeviceType;
use crate::communication::crc16;
use crate::protocol::display::{*};
use crate::protocol::{*};

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


pub fn transfer_header_data(header: &Header, data: &Vec<u8>) -> Vec<u8>
{
    // start
    let mut vec_data: Vec<u8> = start();

    // header
    vec_data.append(&mut header.to_vec());
    
    // data
    vec_data.append(&mut data.to_vec());

    // crc16
    add_crc16(&mut vec_data);

    vec_data
}


pub fn transfer(data_type: DataType, from: DeviceType, to: DeviceType, data: &Vec<u8>) -> Vec<u8>
{
    // start
    let mut vec_data: Vec<u8> = start();

    // header
    vec_data.push(data_type.into());
    vec_data.push(data.len() as u8);
    vec_data.push(from.into());
    vec_data.push(to.into());

    // data
    vec_data.append(&mut data.to_vec());

    // crc16
    add_crc16(&mut vec_data);

    vec_data
}


pub fn request(target: DeviceType, data_type: DataType) -> Vec<u8>
{
    transfer(DataType::Request, DeviceType::Base, target, &Request{data_type}.to_vec())
}


pub fn control(roll: i8, pitch: i8, yaw: i8, throttle: i8) -> Vec<u8>
{
    transfer(DataType::Request, DeviceType::Base, DeviceType::Drone, &control::Quad8{roll, pitch, yaw, throttle}.to_vec())
}


// -- Display ----------------------------------------------------------------------------------------------
pub fn draw_clear_all(pixel: Pixel) -> Vec<u8>
{
    transfer(DataType::DisplayClear, DeviceType::Base, DeviceType::Drone, &ClearAll{pixel}.to_vec())
}


pub fn draw_clear(x: i16, y: i16, width: i16, height: i16, pixel: Pixel) -> Vec<u8>
{
    transfer(DataType::DisplayClear, DeviceType::Base, DeviceType::Controller, &Clear{x, y, width, height, pixel}.to_vec())
}


pub fn draw_invert(x: i16, y: i16, width: i16, height: i16) -> Vec<u8>
{
    transfer(DataType::DisplayInvert, DeviceType::Base, DeviceType::Controller, &Invert{x, y, width, height}.to_vec())
}


pub fn draw_point(x: i16, y: i16, pixel: Pixel) -> Vec<u8>
{
    transfer(DataType::DisplayDrawPoint, DeviceType::Base, DeviceType::Controller, &DrawPoint{x, y, pixel}.to_vec())
}


pub fn draw_line(x1: i16, y1: i16, x2: i16, y2: i16, pixel: Pixel, line: Line) -> Vec<u8>
{
    transfer(DataType::DisplayDrawLine, DeviceType::Base, DeviceType::Controller, &DrawLine{x1, y1, x2, y2, pixel, line}.to_vec())
}


pub fn draw_rect(x: i16, y: i16, width: i16, height: i16, pixel: Pixel, fill: bool, line: Line) -> Vec<u8>
{
    transfer(DataType::DisplayDrawRect, DeviceType::Base, DeviceType::Controller, &DrawRect{x, y, width, height, pixel, fill, line}.to_vec())
}


pub fn draw_circle(x: i16, y: i16, radius: i16, pixel: Pixel, fill: bool) -> Vec<u8>
{
    transfer(DataType::DisplayDrawCircle, DeviceType::Base, DeviceType::Controller, &DrawCircle{x, y, radius, pixel, fill}.to_vec())
}


pub fn draw_string(x: i16, y: i16, font: Font, pixel: Pixel, string: String) -> Vec<u8>
{
    transfer(DataType::DisplayDrawString, DeviceType::Base, DeviceType::Controller, &DrawString{x, y, font, pixel, string}.to_vec())
}


pub fn draw_string_align(x_start: i16, x_end: i16, y: i16, align: Align, font: Font, pixel: Pixel, string: String) -> Vec<u8>
{
    transfer(DataType::DisplayDrawStringAlign, DeviceType::Base, DeviceType::Controller, &DrawStringAlign{x_start, x_end, y, align, font, pixel, string}.to_vec())
}


pub fn draw_image(x: i16, y: i16, width: i16, height: i16, vec_image: Vec<u8>) -> Vec<u8>
{
    transfer(DataType::DisplayDrawImage, DeviceType::Base, DeviceType::Controller, &DrawImage{x, y, width, height, vec_image}.to_vec())
}


