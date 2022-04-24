// An attribute to hide warnings for unused code.
#![allow(dead_code)]


pub mod communication;
pub mod file;
pub mod protocol;
pub mod system;


use std::{thread};
use std::time::{Duration, Instant};

use communication::{*};
use communication::receiver::{*};
use system::{*};
use protocol::{*};
use protocol::display::{*};
use protocol::command::{*};


pub struct Drone
{
    pub time_start: Instant,        // 인스턴스 시작 시각
    pub time_transfer: Instant,     // 데이터 전송 시각
    pub time_receive: Instant,      // 데이터 수신 시각
    pub receiver: Receiver,         // 데이터 수신 처리기
    pub header: Header,             // 수신 받은 데이터의 헤더
    pub vec_data: Vec<u8>,          // 수신 받은 데이터 배열
    pub data: Data,                 // 수신 받은 데이터 파싱 결과물
    pub flag_show_debug_message: bool,  // 디버깅 정보 표시
}


impl Drone {
    pub fn new() -> Drone{
        Drone{
            time_start: Instant::now(),
            time_transfer: Instant::now(),
            time_receive: Instant::now(),
            receiver: Receiver::new(),
            header: Header::new(),
            vec_data: Vec::new(),
            data: Data::None,
            flag_show_debug_message: false,
        }
    }


    pub fn set_show_debug_message(&mut self, flag_show_debug_message: bool)
    {
        self.flag_show_debug_message = flag_show_debug_message;
        
        self.receiver.set_show_debug_message(flag_show_debug_message);
    }


    pub fn push(&mut self, b: u8)
    {
        if self.flag_show_debug_message 
        {
            println!("RX: {:X?}", b);
        }

        self.receiver.push(b);
    }


    pub fn push_slice(&mut self, slice_data: &[u8])
    {
        if slice_data.len() > 0
        {
            if self.flag_show_debug_message 
            {
                println!("RX: {:X?}", slice_data);
            }

            self.receiver.push_slice(slice_data);
        }

    }


    pub fn check(&mut self) -> bool
    {
        if let messaging::State::Loaded = self.receiver.check()
        {
            self.receiver.clear();
            self.time_receive = Instant::now();

            self.header = self.receiver.get_header().clone();
            self.vec_data = self.receiver.get_data().clone();
            self.data = handler::check(&self.header, &self.vec_data);

            return true;
        }

        return false;
    }


    pub fn get_time_passed_from_start(&self) -> u128
    {
        self.time_start.elapsed().as_millis()
    }

    pub fn get_time_passed_from_last_transfer(&self) -> u128
    {
        self.time_transfer.elapsed().as_millis()
    }

    pub fn get_time_passed_from_last_receive(&self) -> u128
    {
        self.time_receive.elapsed().as_millis()
    }


    pub fn sleep(&self, time_sleep_ms: u64)
    {
        let duration_time_sleep_ms = Duration::from_millis(time_sleep_ms);

        thread::sleep(duration_time_sleep_ms);
    }


    // -- Request ----------------------------------------------------------------------------------------------
    pub fn request(&mut self, target: DeviceType, data_type: DataType) -> Vec<u8>
    {
        transfer::transfer(DataType::Request, DeviceType::Base, target, &Request{data_type}.to_vec())
    }


    // -- Command ----------------------------------------------------------------------------------------------
    pub fn command(&mut self, target: DeviceType, command_type: CommandType, option: u8) -> Vec<u8>
    {
        transfer::transfer(DataType::Command, DeviceType::Base, target, &Command{command_type, option}.to_vec())
    }


    // -- FlightEvent ----------------------------------------------------------------------------------------------
    pub fn flight_event(&mut self, event: FlightEvent) -> Vec<u8>
    {
        transfer::transfer(DataType::Command, DeviceType::Base, DeviceType::Drone, &Command{command_type: CommandType::FlightEvent, option: event.into()}.to_vec())
    }

    pub fn takeoff(&mut self) -> Vec<u8>
    {
        self.flight_event(FlightEvent::Takeoff)
    }

    pub fn landing(&mut self) -> Vec<u8>
    {
        self.flight_event(FlightEvent::Landing)
    }

    pub fn stop(&mut self) -> Vec<u8>
    {
        self.flight_event(FlightEvent::Stop)
    }


    // -- Setup ----------------------------------------------------------------------------------------------
    pub fn set_default(&mut self) -> Vec<u8>
    {
        self.command(DeviceType::Drone, CommandType::SetDefault, 0)
    }

    pub fn set_mode_control_flight(&mut self) -> Vec<u8>
    {
        self.command(DeviceType::Drone, CommandType::SetDefault, 0)
    }

    pub fn headless(&mut self, headless: Headless) -> Vec<u8>
    {
        self.command(DeviceType::Drone, CommandType::Headless, headless.into())
    }

    pub fn clear_bias(&mut self) -> Vec<u8>
    {
        self.command(DeviceType::Drone, CommandType::ClearBias, 0)
    }

    pub fn clear_trim(&mut self) -> Vec<u8>
    {
        self.command(DeviceType::Drone, CommandType::ClearTrim, 0)
    }

    pub fn trim(&mut self, roll: i16, pitch: i16, yaw: i16, throttle: i16) -> Vec<u8>
    {
        transfer::transfer(DataType::Trim, DeviceType::Base, DeviceType::Drone, &sensor::Trim{roll, pitch, yaw, throttle}.to_vec())
    }


    // -- Control ----------------------------------------------------------------------------------------------
    pub fn control(&mut self, roll: i8, pitch: i8, yaw: i8, throttle: i8) -> Vec<u8>
    {
        transfer::transfer(DataType::Control, DeviceType::Base, DeviceType::Drone, &control::Quad8{roll, pitch, yaw, throttle}.to_vec())
    }

    pub fn control_request(&mut self, roll: i8, pitch: i8, yaw: i8, throttle: i8, data_type: DataType) -> Vec<u8>
    {
        transfer::transfer(DataType::Control, DeviceType::Base, DeviceType::Drone, &control::Quad8AndRequestData{roll, pitch, yaw, throttle, data_type}.to_vec())
    }

    pub fn control_position(&mut self, x: f32, y: f32, z: f32, velocity: f32, heading: i16, rotational_velocity: i16) -> Vec<u8>
    {
        transfer::transfer(DataType::Control, DeviceType::Base, DeviceType::Drone, &control::Position{x, y, z, velocity, heading, rotational_velocity}.to_vec())
    }


    // -- Battle ----------------------------------------------------------------------------------------------
    pub fn battle_ir_message(&mut self, ir_message: u8) -> Vec<u8>
    {
        transfer::transfer(DataType::Battle, DeviceType::Base, DeviceType::Drone, &battle::IrMessage{ir_message}.to_vec())
    }

    pub fn battle_light_event_command(&mut self, target:DeviceType, event: u8, interval: u16, repeat: u8, r: u8, g: u8, b: u8, command_type: command::CommandType, option: u8) -> Vec<u8>
    {
        transfer::transfer(DataType::Battle, DeviceType::Base, target, &battle::LightEventCommand{event:light::Event{event, interval, repeat}, color: light::Color{r, g, b}, command: command::Command{command_type, option}}.to_vec())
    }

    pub fn battle_ir_message_light_event_command(&mut self, target:DeviceType, ir_message: u8, event: u8, interval: u16, repeat: u8, r: u8, g: u8, b: u8, command_type: command::CommandType, option: u8) -> Vec<u8>
    {
        transfer::transfer(DataType::Battle, DeviceType::Base, target, &battle::IrMessageLightEventCommand{ir_message, event:light::Event{event, interval, repeat}, color: light::Color{r, g, b}, command: command::Command{command_type, option}}.to_vec())
    }


    // -- Light ----------------------------------------------------------------------------------------------
    pub fn light_manual(&mut self, target:DeviceType, flags: u16, brightness: u8) -> Vec<u8>
    {
        transfer::transfer(DataType::LightManual, DeviceType::Base, target, &light::Manual{flags, brightness}.to_vec())
    }

    pub fn light_mode(&mut self, target:DeviceType, mode: u8, interval: u16) -> Vec<u8>
    {
        transfer::transfer(DataType::LightMode, DeviceType::Base, target, &light::Mode{mode, interval}.to_vec())
    }

    pub fn light_event(&mut self, target:DeviceType, event: u8, interval: u16, repeat: u8) -> Vec<u8>
    {
        transfer::transfer(DataType::LightEvent, DeviceType::Base, target, &light::Event{event, interval, repeat}.to_vec())
    }

    pub fn light_mode_color(&mut self, target:DeviceType, mode: u8, interval: u16, r: u8, g: u8, b: u8) -> Vec<u8>
    {
        transfer::transfer(DataType::LightMode, DeviceType::Base, target, &light::ModeColor{mode:light::Mode{mode, interval}, color: light::Color{r, g, b}}.to_vec())
    }

    pub fn light_event_color(&mut self, target:DeviceType, event: u8, interval: u16, repeat: u8, r: u8, g: u8, b: u8) -> Vec<u8>
    {
        transfer::transfer(DataType::LightEvent, DeviceType::Base, target, &light::EventColor{event:light::Event{event, interval, repeat}, color: light::Color{r, g, b}}.to_vec())
    }

    pub fn light_default(&mut self, target:DeviceType, mode: u8, interval: u16, r: u8, g: u8, b: u8) -> Vec<u8>
    {
        transfer::transfer(DataType::LightDefault, DeviceType::Base, target, &light::ModeColor{mode:light::Mode{mode, interval}, color: light::Color{r, g, b}}.to_vec())
    }


    // -- Buzzer ----------------------------------------------------------------------------------------------
    pub fn buzzer_stop(&mut self, target: DeviceType) -> Vec<u8>
    {
        transfer::transfer(DataType::Buzzer, DeviceType::Base, target, &buzzer::BuzzerHz{mode: buzzer::Mode::Stop, hz:0, time:0}.to_vec())
    }

    pub fn buzzer_scale(&mut self, target: DeviceType, scale: buzzer::Scale, time: u16) -> Vec<u8>
    {
        transfer::transfer(DataType::Buzzer, DeviceType::Base, target, &buzzer::BuzzerScale{mode: buzzer::Mode::ScaleInstantly, scale, time}.to_vec())
    }

    pub fn buzzer_scale_reserve(&mut self, target: DeviceType, scale: buzzer::Scale, time: u16) -> Vec<u8>
    {
        transfer::transfer(DataType::Buzzer, DeviceType::Base, target, &buzzer::BuzzerScale{mode: buzzer::Mode::ScaleContinually, scale, time}.to_vec())
    }

    pub fn buzzer_hz(&mut self, target: DeviceType, hz: u16, time: u16) -> Vec<u8>
    {
        transfer::transfer(DataType::Buzzer, DeviceType::Base, target, &buzzer::BuzzerHz{mode: buzzer::Mode::HzInstantly, hz, time}.to_vec())
    }

    pub fn buzzer_hz_reserve(&mut self, target: DeviceType, hz: u16, time: u16) -> Vec<u8>
    {
        transfer::transfer(DataType::Buzzer, DeviceType::Base, target, &buzzer::BuzzerHz{mode: buzzer::Mode::HzContinually, hz, time}.to_vec())
    }

    pub fn buzzer_mute(&mut self, target: DeviceType, time: u16) -> Vec<u8>
    {
        transfer::transfer(DataType::Buzzer, DeviceType::Base, target, &buzzer::BuzzerHz{mode: buzzer::Mode::MuteInstantly, hz: 0, time}.to_vec())
    }

    pub fn buzzer_mute_reserve(&mut self, target: DeviceType, time: u16) -> Vec<u8>
    {
        transfer::transfer(DataType::Buzzer, DeviceType::Base, target, &buzzer::BuzzerHz{mode: buzzer::Mode::MuteContinually, hz: 0, time}.to_vec())
    }


    // -- Vibrator ----------------------------------------------------------------------------------------------
    pub fn vibrator(&mut self, on: u16, off: u16, time: u16) -> Vec<u8>
    {
        transfer::transfer(DataType::Vibrator, DeviceType::Base, DeviceType::Controller, &vibrator::Vibrator{mode: vibrator::Mode::Instantly, on, off, time}.to_vec())
    }

    pub fn vibrator_reserve(&mut self, on: u16, off: u16, time: u16) -> Vec<u8>
    {
        transfer::transfer(DataType::Vibrator, DeviceType::Base, DeviceType::Controller, &vibrator::Vibrator{mode: vibrator::Mode::Continually, on, off, time}.to_vec())
    }


    // -- Display ----------------------------------------------------------------------------------------------
    pub fn draw_clear_all(&mut self, pixel: Pixel) -> Vec<u8>
    {
        transfer::transfer(DataType::DisplayClear, DeviceType::Base, DeviceType::Controller, &ClearAll{pixel}.to_vec())
    }

    pub fn draw_clear(&mut self, x: i16, y: i16, width: i16, height: i16, pixel: Pixel) -> Vec<u8>
    {
        transfer::transfer(DataType::DisplayClear, DeviceType::Base, DeviceType::Controller, &Clear{x, y, width, height, pixel}.to_vec())
    }

    pub fn draw_invert(&mut self, x: i16, y: i16, width: i16, height: i16) -> Vec<u8>
    {
        transfer::transfer(DataType::DisplayInvert, DeviceType::Base, DeviceType::Controller, &Invert{x, y, width, height}.to_vec())
    }

    pub fn draw_point(&mut self, x: i16, y: i16, pixel: Pixel) -> Vec<u8>
    {
        transfer::transfer(DataType::DisplayDrawPoint, DeviceType::Base, DeviceType::Controller, &DrawPoint{x, y, pixel}.to_vec())
    }

    pub fn draw_line(&mut self, x1: i16, y1: i16, x2: i16, y2: i16, pixel: Pixel, line: Line) -> Vec<u8>
    {
        transfer::transfer(DataType::DisplayDrawLine, DeviceType::Base, DeviceType::Controller, &DrawLine{x1, y1, x2, y2, pixel, line}.to_vec())
    }

    pub fn draw_rect(&mut self, x: i16, y: i16, width: i16, height: i16, pixel: Pixel, fill: bool, line: Line) -> Vec<u8>
    {
        transfer::transfer(DataType::DisplayDrawRect, DeviceType::Base, DeviceType::Controller, &DrawRect{x, y, width, height, pixel, fill, line}.to_vec())
    }

    pub fn draw_circle(&mut self, x: i16, y: i16, radius: i16, pixel: Pixel, fill: bool) -> Vec<u8>
    {
        transfer::transfer(DataType::DisplayDrawCircle, DeviceType::Base, DeviceType::Controller, &DrawCircle{x, y, radius, pixel, fill}.to_vec())
    }

    pub fn draw_string(&mut self, x: i16, y: i16, font: Font, pixel: Pixel, string: String) -> Vec<u8>
    {
        transfer::transfer(DataType::DisplayDrawString, DeviceType::Base, DeviceType::Controller, &DrawString{x, y, font, pixel, string}.to_vec())
    }

    pub fn draw_string_align(&mut self, x_start: i16, x_end: i16, y: i16, align: Align, font: Font, pixel: Pixel, string: String) -> Vec<u8>
    {
        transfer::transfer(DataType::DisplayDrawStringAlign, DeviceType::Base, DeviceType::Controller, &DrawStringAlign{x_start, x_end, y, align, font, pixel, string}.to_vec())
    }

    pub fn draw_image(&mut self, x: i16, y: i16, width: i16, height: i16, vec_image: Vec<u8>) -> Vec<u8>
    {
        transfer::transfer(DataType::DisplayDrawImage, DeviceType::Base, DeviceType::Controller, &DrawImage{x, y, width, height, vec_image}.to_vec())
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
