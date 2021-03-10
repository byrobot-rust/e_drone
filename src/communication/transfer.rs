use crate::protocol::Serializable;
use crate::protocol::DataType;
use crate::base::system::DeviceType;
use crate::communication::crc16;
use crate::protocol::sensor::{*};
use crate::protocol::display::{*};


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


// -- Display ----------------------------------------------------------------------------------------------
pub fn draw_clear_all(pixel: Pixel) -> Vec<u8>
{
    let mut data: ClearAll = ClearAll::new();
    data.pixel = pixel;

    // start
    let mut vec_data: Vec<u8> = start();

    // header
    vec_data.push(DataType::DisplayClear.into());
    vec_data.push(ClearAll::size() as u8);
    vec_data.push(DeviceType::Base.into());
    vec_data.push(DeviceType::Controller.into());

    // data
    vec_data.append(&mut data.to_vec());

    // crc16
    add_crc16(&mut vec_data);

    vec_data
}


pub fn draw_clear(x: i16, y: i16, width: i16, height: i16, pixel: Pixel) -> Vec<u8>
{
    let mut data: Clear = Clear::new();
    data.x = x;
    data.y = y;
    data.width = width;
    data.height = height;
    data.pixel = pixel;

    // start
    let mut vec_data: Vec<u8> = start();

    // header
    vec_data.push(DataType::DisplayClear.into());
    vec_data.push(Clear::size() as u8);
    vec_data.push(DeviceType::Base.into());
    vec_data.push(DeviceType::Controller.into());

    // data
    vec_data.append(&mut data.to_vec());

    // crc16
    add_crc16(&mut vec_data);

    vec_data
}


pub fn draw_invert(x: i16, y: i16, width: i16, height: i16) -> Vec<u8>
{
    let mut data: Invert = Invert::new();
    data.x = x;
    data.y = y;
    data.width = width;
    data.height = height;

    // start
    let mut vec_data: Vec<u8> = start();

    // header
    vec_data.push(DataType::DisplayInvert.into());
    vec_data.push(Invert::size() as u8);
    vec_data.push(DeviceType::Base.into());
    vec_data.push(DeviceType::Controller.into());

    // data
    vec_data.append(&mut data.to_vec());

    // crc16
    add_crc16(&mut vec_data);

    vec_data
}


pub fn draw_point(x: i16, y: i16, pixel: Pixel) -> Vec<u8>
{
    let mut data: DrawPoint = DrawPoint::new();
    data.x = x;
    data.y = y;
    data.pixel = pixel;

    // start
    let mut vec_data: Vec<u8> = start();

    // header
    vec_data.push(DataType::DisplayDrawPoint.into());
    vec_data.push(DrawPoint::size() as u8);
    vec_data.push(DeviceType::Base.into());
    vec_data.push(DeviceType::Controller.into());

    // data
    vec_data.append(&mut data.to_vec());

    // crc16
    add_crc16(&mut vec_data);

    vec_data
}


pub fn draw_line(x1: i16, y1: i16, x2: i16, y2: i16, pixel: Pixel, line: Line) -> Vec<u8>
{
    let mut data: DrawLine = DrawLine::new();
    data.x1 = x1;
    data.y1 = y1;
    data.x2 = x2;
    data.y2 = y2;
    data.pixel = pixel;
    data.line = line;

    // start
    let mut vec_data: Vec<u8> = start();

    // header
    vec_data.push(DataType::DisplayDrawLine.into());
    vec_data.push(DrawLine::size() as u8);
    vec_data.push(DeviceType::Base.into());
    vec_data.push(DeviceType::Controller.into());

    // data
    vec_data.append(&mut data.to_vec());

    // crc16
    add_crc16(&mut vec_data);

    vec_data
}


pub fn draw_rect(x: i16, y: i16, width: i16, height: i16, pixel: Pixel, fill: bool, line: Line) -> Vec<u8>
{
    let mut data: DrawRect = DrawRect::new();
    data.x = x;
    data.y = y;
    data.width = width;
    data.height = height;
    data.pixel = pixel;
    data.fill = fill;
    data.line = line;

    // start
    let mut vec_data: Vec<u8> = start();

    // header
    vec_data.push(DataType::DisplayDrawRect.into());
    vec_data.push(DrawRect::size() as u8);
    vec_data.push(DeviceType::Base.into());
    vec_data.push(DeviceType::Controller.into());

    // data
    vec_data.append(&mut data.to_vec());

    // crc16
    add_crc16(&mut vec_data);

    vec_data
}


pub fn draw_circle(x: i16, y: i16, radius: i16, pixel: Pixel, fill: bool) -> Vec<u8>
{
    let mut data: DrawCircle = DrawCircle::new();
    data.x = x;
    data.y = y;
    data.radius = radius;
    data.pixel = pixel;
    data.fill = fill;

    // start
    let mut vec_data: Vec<u8> = start();

    // header
    vec_data.push(DataType::DisplayDrawCircle.into());
    vec_data.push(DrawCircle::size() as u8);
    vec_data.push(DeviceType::Base.into());
    vec_data.push(DeviceType::Controller.into());

    // data
    vec_data.append(&mut data.to_vec());

    // crc16
    add_crc16(&mut vec_data);

    vec_data
}


pub fn draw_string(x: i16, y: i16, font: Font, pixel: Pixel, string: String) -> Vec<u8>
{
    let mut data: DrawString = DrawString::new();
    data.x = x;
    data.y = y;
    data.font = font;
    data.pixel = pixel;
    data.string = string;

    // start
    let mut vec_data: Vec<u8> = start();

    // header
    vec_data.push(DataType::DisplayDrawString.into());
    vec_data.push(data.get_length() as u8);
    vec_data.push(DeviceType::Base.into());
    vec_data.push(DeviceType::Controller.into());

    // data
    vec_data.append(&mut data.to_vec());

    // crc16
    add_crc16(&mut vec_data);

    vec_data
}


pub fn draw_string_align(x_start: i16, x_end: i16, y: i16, align: Align, font: Font, pixel: Pixel, string: String) -> Vec<u8>
{
    let mut data: DrawStringAlign = DrawStringAlign::new();
    data.x_start = x_start;
    data.x_end = x_end;
    data.y = y;
    data.align = align;
    data.font = font;
    data.pixel = pixel;
    data.string = string;

    // start
    let mut vec_data: Vec<u8> = start();

    // header
    vec_data.push(DataType::DisplayDrawStringAlign.into());
    vec_data.push(data.get_length() as u8);
    vec_data.push(DeviceType::Base.into());
    vec_data.push(DeviceType::Controller.into());

    // data
    vec_data.append(&mut data.to_vec());

    // crc16
    add_crc16(&mut vec_data);

    vec_data
}


pub fn draw_image(x: i16, y: i16, width: i16, height: i16, vec_image: Vec<u8>) -> Vec<u8>
{
    let mut data: DrawImage = DrawImage::new();
    data.x = x;
    data.y = y;
    data.width = width;
    data.height = height;
    data.vec_image = vec_image;

    // start
    let mut vec_data: Vec<u8> = start();

    // header
    vec_data.push(DataType::DisplayDrawImage.into());
    vec_data.push(data.get_length() as u8);
    vec_data.push(DeviceType::Base.into());
    vec_data.push(DeviceType::Controller.into());

    // data
    vec_data.append(&mut data.to_vec());

    // crc16
    add_crc16(&mut vec_data);

    vec_data
}


