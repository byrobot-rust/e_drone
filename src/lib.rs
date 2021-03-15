// An attribute to hide warnings for unused code.
#![allow(dead_code)]


pub mod base;
pub mod communication;
pub mod protocol;


extern crate serialport;


use serialport::{*};
use std::time::{Duration, Instant};

use communication::{*};
use communication::receiver::{*};
use base::system::{*};
use protocol::{*};


pub struct Drone
{
    pub time_start: Instant,
    pub time_transfer: Instant,
    pub time_receive: Instant,
    pub receiver: Receiver,
    pub buffer: [u8; 1024],
    pub port: Result<Box<dyn SerialPort>>,
}


impl Drone {
    pub fn new(port_name: &str) -> Drone{
        Drone{
            time_start: Instant::now(),
            time_transfer: Instant::now(),
            time_receive: Instant::now(),
            receiver: Receiver::new(),
            buffer: [0u8; 1024],
            port: serialport::new(port_name, 57_600)
                .timeout(Duration::from_millis(100))
                .open()
        }
    }

    pub fn is_connected(&mut self) -> bool
    {
        match &mut self.port {
            Ok(_port) => { true },
            _ => { false },
        }
    }

    pub fn check(&mut self) -> Data
    {
        match &mut self.port {
            Ok(port) => {
                let length_read = &port.read(&mut self.buffer);
                match length_read {
                    Ok(len) => {
                        if *len > 0 {
                            self.receiver.push_slice(&self.buffer[..*len]);
                        }

                        if let messaging::State::Loaded = self.receiver.check()
                        {
                            self.receiver.clear();
                            self.time_receive = Instant::now();

                            return handler::check(self.receiver.get_header(), self.receiver.get_data())
                        }
                    },
                    _ => {},
                }
            },
            _ => {},
        }

        Data::None
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



    pub fn request(&mut self, device_type: DeviceType, data_type: DataType)
    {
        match &mut self.port {
            Ok(port) => { match port.write(&transfer::request(device_type, data_type)) {
                Ok(_len) => { self.time_transfer = Instant::now(); },
                _ => {},
            } },
            _ => {},
        }
    }

}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
