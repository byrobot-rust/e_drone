use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
use byteorder::{ByteOrder, LittleEndian};

use crate::base::system::{*};
use crate::communication::extractor::Extractor;

use crate::protocol::Serializable;


// -- Pixel ----------------------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Pixel {
    #[num_enum(default)]
    Black = 0x00,       // Black
    White = 0x01,       // White
    Inverse = 0x02,     // Inverse
    Outline = 0x03,     // Outline
}


impl Pixel {
    pub fn from_u8(pixel_u8: u8) -> Pixel {
        match Pixel::try_from( pixel_u8 ) {
            Ok(pixel) => { pixel },
            _ => { Pixel::Black },
        }
    }

    pub fn to_u8(pixel: Pixel) -> u8 {
        pixel.into()
    }
}


// -- Font ----------------------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Font {
    #[num_enum(default)]
    LM5x8 = 0x00,       // LM 5x8
    LM10x16 = 0x01,     // LM 10.X6
}


impl Font {
    pub fn from_u8(font_u8: u8) -> Font {
        match Font::try_from( font_u8 ) {
            Ok(font) => { font },
            _ => { Font::LM5x8 },
        }
    }

    pub fn to_u8(font: Font) -> u8 {
        font.into()
    }
}


// -- Align ----------------------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Align {
    #[num_enum(default)]
    Left = 0x00,
    Center = 0x01,
    Right = 0x02,
}


impl Align {
    pub fn from_u8(align_u8: u8) -> Align {
        match Align::try_from( align_u8 ) {
            Ok(align) => { align },
            _ => { Align::Left },
        }
    }

    pub fn to_u8(align: Align) -> u8 {
        align.into()
    }
}


// -- DrawPoint -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct DrawPoint {
    x: i16,
    y: i16,
    pixel: Pixel,
}


impl DrawPoint {
    pub fn new() -> DrawPoint{
        DrawPoint {
            x: 0,
            y: 0,
            pixel: Pixel::White,
        }
    }


    pub fn parse(draw_point: &mut DrawPoint, vec_data: &Vec<u8>) -> bool {
        if vec_data.len() != DrawPoint::size() {
            return false;
        }

        let mut ext: Extractor = Extractor::new(vec_data);

        draw_point.x = ext.get_i16();
        draw_point.y = ext.get_i16();
        draw_point.pixel = Pixel::from_u8(ext.get_u8());

        true
    }
}


impl Serializable for DrawPoint {

    fn size() -> usize { 5 }


    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.x.to_le_bytes());
        vec_data.extend_from_slice(&self.y.to_le_bytes());
        vec_data.push(Pixel::to_u8(self.pixel));

        vec_data
    }
}
