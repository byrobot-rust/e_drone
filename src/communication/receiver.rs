use std::time::{SystemTime};
use std::collections::VecDeque;
use std::convert::TryFrom;

use crate::protocol;
use crate::protocol::Serializable;
use crate::protocol::DataType;
use crate::system::DeviceType;
use crate::communication::crc16;
use crate::communication::messaging::{Section, State};


#[derive(Debug)]
pub struct Receiver {
    state: State,
    section_old: Section,
    section: Section,
    index: i32,

    header: protocol::Header,

    time_receive_start: SystemTime,
    time_receive_complete: SystemTime,

    crc16_calculated: u16,
    crc16: u16,

    queue_buffer: VecDeque<u8>,
    vec_data: Vec<u8>,
    vec_data_all: Vec<u8>,

    flag_connected: bool,
    pub flag_show_debug_message: bool,  // 디버깅 정보 표시
}


impl Receiver {
    pub fn new() -> Receiver
    {
        Receiver{
            state: State::Ready,
            section_old: Section::End,
            section: Section::Start,
            index: 0,
        
            header: protocol::Header {
                data_type: DataType::None,
                length: 0,
                from: DeviceType::Base,
                to: DeviceType::Drone,
            },
        
            time_receive_start: SystemTime::now(),
            time_receive_complete: SystemTime::now(),
        
            crc16_calculated: 0,
            crc16: 0,

            queue_buffer: VecDeque::with_capacity(4096),
            vec_data: Vec::new(),
            vec_data_all: Vec::new(),
        
            flag_connected: false,
            flag_show_debug_message: false,
        }
    }


    pub fn set_show_debug_message(&mut self, flag_show_debug_message: bool)
    {
        self.flag_show_debug_message = flag_show_debug_message;
    }


    pub fn is_buffer_cleared(&mut self) -> bool
    {
        self.queue_buffer.len() == 0
    }


    pub fn clear(&mut self)
    {
        self.state = State::Ready;
    }


    pub fn clear_buffer(&mut self)
    {
        self.queue_buffer.clear();
    }


    pub fn clear_all(&mut self)
    {
        self.state = State::Ready;
        self.section_old = Section::End;
        self.section = Section::Start;
        self.index = 0;
    
        self.header.data_type = DataType::None;
        self.header.length = 0;
        self.header.from = DeviceType::Base;
        self.header.to = DeviceType::Drone;
    
        self.time_receive_start = SystemTime::now();
        self.time_receive_complete = SystemTime::now();
    
        self.crc16_calculated = 0;
        self.crc16 = 0;

        self.queue_buffer.clear();
        self.vec_data.clear();
        self.vec_data_all.clear();
    
        self.flag_connected = false;
    }


    pub fn push(&mut self, b: u8) {
        self.queue_buffer.push_back(b);
    }


    pub fn push_slice(&mut self, slice_data: &[u8]) {
        self.queue_buffer.extend(slice_data);
    }


    pub fn check(&mut self) -> &State {
        match self.time_receive_start.elapsed() {
            Ok(elapsed) => {
                if elapsed.as_millis() > 1200 {
                    self.flag_connected = false;
                };
            },
            Err(_e) => {},
        }

        if let State::Loaded = self.state {
            return &self.state
        }

        loop
        {
            match self.queue_buffer.pop_front() {
                None => {
                    break;
                },
                Some(b) => {
                    self.call(b);
                },
            }
            
            if let State::Loaded = self.state {
                break;
            }
        }


        if let State::Loaded = self.state {
            self.flag_connected = true;
        }

        &self.state
    }


    pub fn call(&mut self, b: u8) -> &State {
        match self.state {
            State::Ready => {
                self.section = Section::Start;
                self.index = 0;
            },

            State::Receiving => {
                match self.time_receive_start.elapsed() {
                    Ok(elapsed) => {
                        if elapsed.as_millis() > 1200 {
                            self.state = State::Ready;
                            self.section = Section::Start;
                            self.index = 0;
                        };
                    },
                    Err(_e) => {},
                }
            },

            State::Loaded => {
                return &self.state
            }

            _ => {},
        }


        if self.section != self.section_old {
            self.index = 0;
            self.section_old = self.section;
        }


        match self.section {
            Section::Start =>
            {
                match self.index {
                    0 => {
                        if b == 0x0A {
                            self.time_receive_start = SystemTime::now();
                            self.state = State::Receiving;
                        }
                        else
                        {
                            self.state = State::Failure;
                        }
                    },
                    1 => {
                        if b == 0x55 {
                            self.section = Section::Header;
                        }
                        else
                        {
                            self.state = State::Failure;
                        }
                    },
                    _ => {
                        self.state = State::Failure;
                    },
                }
            },

            Section::Header =>
            {
                match self.index {
                    0 => {
                        // DataType
                        match DataType::try_from(b){
                            Ok(data_type) => {
                                self.header.data_type = data_type;
                                self.crc16_calculated = crc16::calc_byte(0, b);
                            },
                            _ => {
                                if self.flag_show_debug_message 
                                {
                                    println!("DataType is NOT Matched. - {:X?}", b);
                                } 

                                self.state = State::Failure;
                            },
                        }
                    },
                    1 => {
                        // Length
                        self.header.length = b;
                        self.crc16_calculated = crc16::calc_byte(self.crc16_calculated, b);
                    },
                    2 => {
                        // From
                        match DeviceType::try_from(b){
                            Ok(device_type) => {
                                self.header.from = device_type;
                                self.crc16_calculated = crc16::calc_byte(self.crc16_calculated, b);
                            },
                            _ => { 
                                if self.flag_show_debug_message 
                                {
                                    println!("DeviceType is NOT Matched. - {:X?}", b);
                                }

                                self.state = State::Failure;
                            },
                        }
                    },
                    3 => {
                        // To
                        match DeviceType::try_from(b){
                            Ok(device_type) => {
                                self.header.to = device_type;
                                self.crc16_calculated = crc16::calc_byte(self.crc16_calculated, b);
                                self.vec_data.clear();

                                if self.header.length == 0 {
                                    self.section = Section::End;
                                }
                                else
                                {
                                    self.section = Section::Data;
                                }
                            },
                            _ => { 
                                if self.flag_show_debug_message 
                                {
                                    println!("DeviceType is NOT Matched. - {:X?}", b);
                                }

                                self.state = State::Failure;
                            },
                        }
                    },
                    _ => {
                        self.state = State::Failure;
                    },
                }
            },

            Section::Data =>
            {
                self.vec_data.push(b);
                self.crc16_calculated = crc16::calc_byte(self.crc16_calculated, b);

                if self.index == self.header.length as i32 - 1 {
                    self.section = Section::End;
                }
            },
            
            Section::End =>
            {
                match self.index {
                    0 => {
                        self.crc16 = b as u16;
                    },
                    1 => {
                        self.crc16 = ((b as u16) << 8) + self.crc16;

                        if self.flag_show_debug_message 
                        {
                            println!("CRC16: Received: {:X?}, Calculated: {:X?}", self.crc16, self.crc16_calculated);
                        }
            
                        if self.crc16 == self.crc16_calculated {
                            self.time_receive_complete = SystemTime::now();
                            self.state = State::Loaded;

                            self.vec_data_all.clear();
                            self.vec_data_all.push(0x0A);
                            self.vec_data_all.push(0x55);
                            self.vec_data_all.extend_from_slice(self.header.to_vec().as_slice());
                            self.vec_data_all.extend_from_slice(self.vec_data.as_slice());
                            self.vec_data_all.push((self.crc16 & 0xff) as u8);
                            self.vec_data_all.push(((self.crc16 >> 8) & 0xff) as u8);
                        }
                        else {
                            self.state = State::Failure;
                        }
                    },
                    _ => {
                        self.state = State::Failure;
                    },
                }
            },
        }


        match self.state{
            State::Receiving => { self.index = self.index + 1; },
            State::Failure => { self.state = State::Ready; },
            _ => {}
        }


        &self.state
    }


    pub fn get_header(&self) -> &protocol::Header {
        &self.header
    }


    pub fn get_data(&self) -> &Vec<u8> {
        &self.vec_data
    }


    pub fn get_data_all(&self) -> &Vec<u8> {
        &self.vec_data_all
    }


    pub fn get_index(&self) -> &i32 {
        &self.index
    }


    pub fn get_state(&self) -> &State {
        &self.state
    }


    pub fn is_connected(&self) -> &bool {
        &self.flag_connected
    }
}

