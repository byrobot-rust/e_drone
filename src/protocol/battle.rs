use crate::protocol::Serializable;
use crate::communication::extractor::Extractor;
use crate::protocol::light::{*};
use crate::protocol::command::{*};


// -- IrMessage -----------------------------------------------------------------------------------------------
#[derive(Debug, Copy, Clone)]
pub struct IrMessage {
    pub ir_message: u8,
}


impl IrMessage {
    pub fn new() -> IrMessage{
        IrMessage {
            ir_message: 0,
        }
    }


    pub const fn size() -> usize { 1 }


    pub fn parse(slice_data: &[u8]) -> Result<IrMessage, &'static str> {
        if slice_data.len() == IrMessage::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(IrMessage{
                ir_message: ext.get_u8(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for IrMessage {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.ir_message.to_le_bytes());

        vec_data
    }
}


// -- LightEventColorCommand -----------------------------------------------------------------------------------------------
#[derive(Debug, Copy, Clone)]
pub struct LightEventCommand {
    pub event: Event,
    pub color: Color,
    pub command: Command,
}


impl LightEventCommand {
    pub fn new() -> LightEventCommand{
        LightEventCommand {
            event: Event::new(),
            color: Color::new(),
            command: Command::new(),
        }
    }


    pub const fn size() -> usize { 9 }


    pub fn parse(slice_data: &[u8]) -> Result<LightEventCommand, &'static str> {
        if slice_data.len() == LightEventCommand::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok( LightEventCommand{
                event: Event {
                    event: ext.get_u8(),
                    interval: ext.get_u16(),
                    repeat: ext.get_u8(),
                },
                color: Color {
                    r: ext.get_u8(),
                    g: ext.get_u8(),
                    b: ext.get_u8(),
                },
                command: Command {
                    command_type: CommandType::None,
                    option: 0,
                }
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for LightEventCommand {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.event.event.to_le_bytes());
        vec_data.extend_from_slice(&self.event.interval.to_le_bytes());
        vec_data.extend_from_slice(&self.event.repeat.to_le_bytes());
        vec_data.extend_from_slice(&self.color.r.to_le_bytes());
        vec_data.extend_from_slice(&self.color.g.to_le_bytes());
        vec_data.extend_from_slice(&self.color.b.to_le_bytes());
        vec_data.push(self.command.command_type.into());
        vec_data.extend_from_slice(&self.command.option.to_le_bytes());

        vec_data
    }
}


// -- IrMessageLightEventCommand -----------------------------------------------------------------------------------------------
#[derive(Debug, Copy, Clone)]
pub struct IrMessageLightEventCommand {
    pub ir_message: u8,
    pub event: Event,
    pub color: Color,
    pub command: Command,
}


impl IrMessageLightEventCommand {
    pub fn new() -> IrMessageLightEventCommand{
        IrMessageLightEventCommand {
            ir_message: 0,
            event: Event::new(),
            color: Color::new(),
            command: Command::new(),
        }
    }


    pub const fn size() -> usize { 10 }


    pub fn parse(slice_data: &[u8]) -> Result<IrMessageLightEventCommand, &'static str> {
        if slice_data.len() == IrMessageLightEventCommand::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok( IrMessageLightEventCommand{
                ir_message: ext.get_u8(),
                event: Event {
                    event: ext.get_u8(),
                    interval: ext.get_u16(),
                    repeat: ext.get_u8(),
                },
                color: Color {
                    r: ext.get_u8(),
                    g: ext.get_u8(),
                    b: ext.get_u8(),
                },
                command: Command {
                    command_type: CommandType::None,
                    option: 0,
                }
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for IrMessageLightEventCommand {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.ir_message.to_le_bytes());
        vec_data.extend_from_slice(&self.event.event.to_le_bytes());
        vec_data.extend_from_slice(&self.event.interval.to_le_bytes());
        vec_data.extend_from_slice(&self.event.repeat.to_le_bytes());
        vec_data.extend_from_slice(&self.color.r.to_le_bytes());
        vec_data.extend_from_slice(&self.color.g.to_le_bytes());
        vec_data.extend_from_slice(&self.color.b.to_le_bytes());
        vec_data.push(self.command.command_type.into());
        vec_data.extend_from_slice(&self.command.option.to_le_bytes());

        vec_data
    }
}


