use crate::system::{*};
use crate::communication::crc16;
use crate::protocol::{*};
use crate::protocol::display::{*};
use crate::protocol::command::{*};

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


pub fn command(target: DeviceType, command_type: CommandType, option: u8) -> Vec<u8>
{
    transfer(DataType::Command, DeviceType::Base, target, &Command{command_type, option}.to_vec())
}


// -- FlightEvent ----------------------------------------------------------------------------------------------
pub fn flight_event(event: FlightEvent) -> Vec<u8>
{
    transfer(DataType::Command, DeviceType::Base, DeviceType::Drone, &Command{command_type: CommandType::FlightEvent, option: event.into()}.to_vec())
}

pub fn takeoff() -> Vec<u8>
{
    flight_event(FlightEvent::Takeoff)
}

pub fn landing() -> Vec<u8>
{
    flight_event(FlightEvent::Landing)
}

pub fn stop() -> Vec<u8>
{
    flight_event(FlightEvent::Stop)
}


// -- Setup ----------------------------------------------------------------------------------------------
pub fn set_default() -> Vec<u8>
{
    command(DeviceType::Drone, CommandType::SetDefault, 0)
}

pub fn set_mode_control_flight() -> Vec<u8>
{
    command(DeviceType::Drone, CommandType::SetDefault, 0)
}

pub fn headless(headless: Headless) -> Vec<u8>
{
    command(DeviceType::Drone, CommandType::Headless, headless.into())
}

pub fn clear_bias() -> Vec<u8>
{
    command(DeviceType::Drone, CommandType::ClearBias, 0)
}

pub fn clear_trim() -> Vec<u8>
{
    command(DeviceType::Drone, CommandType::ClearTrim, 0)
}

pub fn trim(roll: i16, pitch: i16, yaw: i16, throttle: i16) -> Vec<u8>
{
    transfer(DataType::Trim, DeviceType::Base, DeviceType::Drone, &sensor::Trim{roll, pitch, yaw, throttle}.to_vec())
}


// -- Control ----------------------------------------------------------------------------------------------
pub fn control(roll: i8, pitch: i8, yaw: i8, throttle: i8) -> Vec<u8>
{
    transfer(DataType::Control, DeviceType::Base, DeviceType::Drone, &control::Quad8{roll, pitch, yaw, throttle}.to_vec())
}

pub fn control_request(roll: i8, pitch: i8, yaw: i8, throttle: i8, data_type: DataType) -> Vec<u8>
{
    transfer(DataType::Control, DeviceType::Base, DeviceType::Drone, &control::Quad8AndRequestData{roll, pitch, yaw, throttle, data_type}.to_vec())
}

pub fn control_position(x: f32, y: f32, z: f32, velocity: f32, heading: i16, rotational_velocity: i16) -> Vec<u8>
{
    transfer(DataType::Control, DeviceType::Base, DeviceType::Drone, &control::Position{x, y, z, velocity, heading, rotational_velocity}.to_vec())
}


// -- Buzzer ----------------------------------------------------------------------------------------------
pub fn buzzer_scale(target: DeviceType, scale: buzzer::Scale, time: u16) -> Vec<u8>
{
    let mode = buzzer::Mode::ScaleInstantly;
    transfer(DataType::Buzzer, DeviceType::Base, target, &buzzer::BuzzerScale{mode, scale, time}.to_vec())
}

pub fn buzzer_scale_reserve(target: DeviceType, scale: buzzer::Scale, time: u16) -> Vec<u8>
{
    let mode = buzzer::Mode::ScaleContinually;
    transfer(DataType::Buzzer, DeviceType::Base, target, &buzzer::BuzzerScale{mode, scale, time}.to_vec())
}

pub fn buzzer_hz(target: DeviceType, hz: u16, time: u16) -> Vec<u8>
{
    let mode = buzzer::Mode::HzInstantly;
    transfer(DataType::Buzzer, DeviceType::Base, target, &buzzer::BuzzerHz{mode, hz, time}.to_vec())
}

pub fn buzzer_hz_reserve(target: DeviceType, hz: u16, time: u16) -> Vec<u8>
{
    let mode = buzzer::Mode::HzContinually;
    transfer(DataType::Buzzer, DeviceType::Base, target, &buzzer::BuzzerHz{mode, hz, time}.to_vec())
}

pub fn buzzer_mute(target: DeviceType, time: u16) -> Vec<u8>
{
    let mode = buzzer::Mode::MuteInstantly;
    let hz: u16 = 0;
    transfer(DataType::Buzzer, DeviceType::Base, target, &buzzer::BuzzerHz{mode, hz, time}.to_vec())
}

pub fn buzzer_mute_reserve(target: DeviceType, time: u16) -> Vec<u8>
{
    let mode = buzzer::Mode::MuteContinually;
    let hz: u16 = 0;
    transfer(DataType::Buzzer, DeviceType::Base, target, &buzzer::BuzzerHz{mode, hz, time}.to_vec())
}


// -- Vibrator ----------------------------------------------------------------------------------------------
pub fn vibrator(on: u16, off: u16, time: u16) -> Vec<u8>
{
    let mode = vibrator::Mode::Instantly;
    transfer(DataType::Vibrator, DeviceType::Base, DeviceType::Controller, &vibrator::Vibrator{mode, on, off, time}.to_vec())
}

pub fn vibrator_reserve(on: u16, off: u16, time: u16) -> Vec<u8>
{
    let mode = vibrator::Mode::Continually;
    transfer(DataType::Vibrator, DeviceType::Base, DeviceType::Controller, &vibrator::Vibrator{mode, on, off, time}.to_vec())
}


// -- Display ----------------------------------------------------------------------------------------------
pub fn draw_clear_all(pixel: Pixel) -> Vec<u8>
{
    transfer(DataType::DisplayClear, DeviceType::Base, DeviceType::Controller, &ClearAll{pixel}.to_vec())
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


