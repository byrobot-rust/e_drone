use std::time::{SystemTime};
use std::collections::VecDeque;
use std::convert::TryFrom;

use crate::protocol;
use crate::protocol::DataType;
use crate::base::system::DeviceType;
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

    flag_connected: bool,
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
                datatype: DataType::None,
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
        
            flag_connected: false,
        }
    }

    pub fn init(&mut self)
    {
        self.state = State::Ready;
        self.section_old = Section::End;
        self.section = Section::Start;
        self.index = 0;
    
        self.header.datatype = DataType::None;
        self.header.length = 0;
        self.header.from = DeviceType::Base;
        self.header.to = DeviceType::Drone;
    
        self.time_receive_start = SystemTime::now();
        self.time_receive_complete = SystemTime::now();
    
        self.crc16_calculated = 0;
        self.crc16 = 0;

        self.queue_buffer = VecDeque::with_capacity(4096);
        self.vec_data.clear();
    
        self.flag_connected = false;
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


    pub fn push(&mut self, b: u8) {
        self.queue_buffer.push_back(b);
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
                        if elapsed.as_millis() > 30 {
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
                        match DataType::try_from(b){
                            Ok(datatype) => { self.header.datatype = datatype; },
                            _ => { self.state = State::Failure; },
                        }
                        //self.header.datatype = DataType::try_from(b);
                    },
                    _ => {
                        self.state = State::Failure;
                    }
                }
            }
            
            _ => {}
        }


        &self.state
    }
}



