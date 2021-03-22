use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

use crate::protocol::Serializable;
use crate::communication::extractor::Extractor;


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
    pub fn from_u8(data_u8: u8) -> Pixel {
        match Pixel::try_from( data_u8 ) {
            Ok(data) => { data },
            _ => { Pixel::Black },
        }
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
    pub fn from_u8(data_u8: u8) -> Font {
        match Font::try_from( data_u8 ) {
            Ok(data) => { data },
            _ => { Font::LM5x8 },
        }
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
    pub fn from_u8(data_u8: u8) -> Align {
        match Align::try_from( data_u8 ) {
            Ok(data) => { data },
            _ => { Align::Left },
        }
    }
}


// -- Line ----------------------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Line {
    #[num_enum(default)]
    Solid = 0x00,
    Dotted = 0x01,
    Dashed = 0x02,
}


impl Line {
    pub fn from_u8(data_u8: u8) -> Line {
        match Line::try_from( data_u8 ) {
            Ok(data) => { data },
            _ => { Line::Solid },
        }
    }
}


// -- ClearAll -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct ClearAll {
    pub pixel: Pixel,
}


impl ClearAll {
    pub fn new() -> ClearAll{
        ClearAll {
            pixel: Pixel::White,
        }
    }


    pub const fn size() -> usize { 1 }


    pub fn parse(slice_data: &[u8]) -> Result<ClearAll, &'static str> {
        if slice_data.len() == ClearAll::size() {
            Ok(ClearAll{ pixel: Pixel::from_u8(slice_data[0]) })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for ClearAll {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.push(self.pixel.into());

        vec_data
    }
}


// -- Clear -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct Clear {
    pub x: i16,
    pub y: i16,
    pub width: i16,
    pub height: i16,
    pub pixel: Pixel,
}


impl Clear {
    pub fn new() -> Clear{
        Clear {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            pixel: Pixel::White,
        }
    }


    pub const fn size() -> usize { 9 }


    pub fn parse(slice_data: &[u8]) -> Result<Clear, &'static str> {
        if slice_data.len() == Clear::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(Clear{
                x: ext.get_i16(),
                y: ext.get_i16(),
                width: ext.get_i16(),
                height: ext.get_i16(),
                pixel: Pixel::from_u8(ext.get_u8()),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for Clear {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.x.to_le_bytes());
        vec_data.extend_from_slice(&self.y.to_le_bytes());
        vec_data.extend_from_slice(&self.width.to_le_bytes());
        vec_data.extend_from_slice(&self.height.to_le_bytes());
        vec_data.push(self.pixel.into());

        vec_data
    }
}


// -- Invert -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct Invert {
    pub x: i16,
    pub y: i16,
    pub width: i16,
    pub height: i16,
}


impl Invert {
    pub fn new() -> Invert{
        Invert {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        }
    }


    pub const fn size() -> usize { 8 }


    pub fn parse(slice_data: &[u8]) -> Result<Invert, &'static str> {
        if slice_data.len() == Invert::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(Invert{
                x: ext.get_i16(),
                y: ext.get_i16(),
                width: ext.get_i16(),
                height: ext.get_i16(),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for Invert {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.x.to_le_bytes());
        vec_data.extend_from_slice(&self.y.to_le_bytes());
        vec_data.extend_from_slice(&self.width.to_le_bytes());
        vec_data.extend_from_slice(&self.height.to_le_bytes());

        vec_data
    }
}


// -- DrawPoint -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct DrawPoint {
    pub x: i16,
    pub y: i16,
    pub pixel: Pixel,
}


impl DrawPoint {
    pub fn new() -> DrawPoint{
        DrawPoint {
            x: 0,
            y: 0,
            pixel: Pixel::White,
        }
    }


    pub const fn size() -> usize { 5 }


    pub fn parse(slice_data: &[u8]) -> Result<DrawPoint, &'static str> {
        if slice_data.len() == DrawPoint::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(DrawPoint{
                x: ext.get_i16(),
                y: ext.get_i16(),
                pixel: Pixel::from_u8(ext.get_u8()),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for DrawPoint {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.x.to_le_bytes());
        vec_data.extend_from_slice(&self.y.to_le_bytes());
        vec_data.push(self.pixel.into());

        vec_data
    }
}


// -- DrawLine -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct DrawLine {
    pub x1: i16,
    pub y1: i16,
    pub x2: i16,
    pub y2: i16,
    pub pixel: Pixel,
    pub line: Line,
}


impl DrawLine {
    pub fn new() -> DrawLine{
        DrawLine {
            x1: 0,
            y1: 0,
            x2: 0,
            y2: 0,
            pixel: Pixel::White,
            line: Line::Solid,
        }
    }


    pub const fn size() -> usize { 10 }


    pub fn parse(slice_data: &[u8]) -> Result<DrawLine, &'static str> {
        if slice_data.len() == DrawLine::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(DrawLine{
                x1: ext.get_i16(),
                y1: ext.get_i16(),
                x2: ext.get_i16(),
                y2: ext.get_i16(),
                pixel: Pixel::from_u8(ext.get_u8()),
                line: Line::from_u8(ext.get_u8()),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for DrawLine {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.x1.to_le_bytes());
        vec_data.extend_from_slice(&self.y1.to_le_bytes());
        vec_data.extend_from_slice(&self.x2.to_le_bytes());
        vec_data.extend_from_slice(&self.y2.to_le_bytes());
        vec_data.push(self.pixel.into());
        vec_data.push(self.line.into());

        vec_data
    }
}


// -- DrawRect -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct DrawRect {
    pub x: i16,
    pub y: i16,
    pub width: i16,
    pub height: i16,
    pub pixel: Pixel,
    pub fill: bool,
    pub line: Line,
}


impl DrawRect {
    pub fn new() -> DrawRect{
        DrawRect {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            fill: false,
            pixel: Pixel::White,
            line: Line::Solid,
        }
    }


    pub const fn size() -> usize { 11 }


    pub fn parse(slice_data: &[u8]) -> Result<DrawRect, &'static str> {
        if slice_data.len() == DrawRect::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(DrawRect{
                x: ext.get_i16(),
                y: ext.get_i16(),
                width: ext.get_i16(),
                height: ext.get_i16(),
                pixel: Pixel::from_u8(ext.get_u8()),
                fill: if ext.get_u8() == 0 { false } else { true },
                line: Line::from_u8(ext.get_u8()),
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for DrawRect {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.x.to_le_bytes());
        vec_data.extend_from_slice(&self.y.to_le_bytes());
        vec_data.extend_from_slice(&self.width.to_le_bytes());
        vec_data.extend_from_slice(&self.height.to_le_bytes());
        vec_data.push(self.pixel.into());
        vec_data.push(if self.fill { 1 } else { 0 });
        vec_data.push(self.line.into());

        vec_data
    }
}


// -- DrawCircle -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct DrawCircle {
    pub x: i16,
    pub y: i16,
    pub radius: i16,
    pub pixel: Pixel,
    pub fill: bool,
}


impl DrawCircle {
    pub fn new() -> DrawCircle{
        DrawCircle {
            x: 0,
            y: 0,
            radius: 0,
            pixel: Pixel::White,
            fill: false,
        }
    }


    pub const fn size() -> usize { 8 }


    pub fn parse(slice_data: &[u8]) -> Result<DrawCircle, &'static str> {
        if slice_data.len() == DrawCircle::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(DrawCircle{
                x: ext.get_i16(),
                y: ext.get_i16(),
                radius: ext.get_i16(),
                pixel: Pixel::from_u8(ext.get_u8()),
                fill: if ext.get_u8() == 0 { false } else { true },
            })
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for DrawCircle {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.x.to_le_bytes());
        vec_data.extend_from_slice(&self.y.to_le_bytes());
        vec_data.extend_from_slice(&self.radius.to_le_bytes());
        vec_data.push(self.pixel.into());
        vec_data.push(if self.fill { 1 } else { 0 });

        vec_data
    }
}


// -- DrawString -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct DrawString {
    pub x: i16,
    pub y: i16,
    pub font: Font,
    pub pixel: Pixel,
    pub string: String,
}


impl DrawString {
    pub fn new() -> DrawString{
        DrawString {
            x: 0,
            y: 0,
            font: Font::LM5x8,
            pixel: Pixel::White,
            string: String::new(),
        }
    }


    pub const fn size() -> usize { 6 }


    pub fn parse(slice_data: &[u8]) -> Result<DrawString, &'static str> {
        if slice_data.len() > DrawString::size() {
            let mut vec_string: Vec<u8> = Vec::new();
            vec_string.extend_from_slice(&slice_data[DrawString::size()..]);
            let string: String = match String::from_utf8(vec_string) { 
                Ok(s) => { s },
                Err(_e) => { String::from("") },
            };
    
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(DrawString{
                x: ext.get_i16(),
                y: ext.get_i16(),
                font: Font::from_u8(ext.get_u8()),
                pixel: Pixel::from_u8(ext.get_u8()),
                string: string,
            })
        }
        else { Err("Wrong length") }
    }


    pub fn get_length(&self) -> usize { 
        DrawString::size() + self.string.clone().into_bytes().len()
    }
}


impl Serializable for DrawString {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.x.to_le_bytes());
        vec_data.extend_from_slice(&self.y.to_le_bytes());
        vec_data.push(self.font.into());
        vec_data.push(self.pixel.into());
        vec_data.extend_from_slice(&self.string.clone().into_bytes());

        vec_data
    }
}


// -- DrawStringAlign -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct DrawStringAlign {
    pub x_start: i16,
    pub x_end: i16,
    pub y: i16,
    pub align: Align,
    pub font: Font,
    pub pixel: Pixel,
    pub string: String,
}


impl DrawStringAlign {
    pub fn new() -> DrawStringAlign{
        DrawStringAlign {
            x_start: 0,
            x_end: 0,
            y: 0,
            align: Align::Left,
            font: Font::LM5x8,
            pixel: Pixel::White,
            string: String::new(),
        }
    }


    pub const fn size() -> usize { 9 }


    pub fn parse(slice_data: &[u8]) -> Result<DrawStringAlign, &'static str> {
        if slice_data.len() > DrawStringAlign::size() {
            let mut vec_string: Vec<u8> = Vec::new();
            vec_string.extend_from_slice(&slice_data[DrawStringAlign::size()..]);
            let string: String = match String::from_utf8(vec_string) { 
                Ok(s) => { s },
                Err(_e) => { String::from("") },
            };
    
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(DrawStringAlign{
                x_start: ext.get_i16(),
                x_end: ext.get_i16(),
                y: ext.get_i16(),
                align: Align::from_u8(ext.get_u8()),
                font: Font::from_u8(ext.get_u8()),
                pixel: Pixel::from_u8(ext.get_u8()),
                string: string,
            })
        }
        else { Err("Wrong length") }
    }


    pub fn get_length(&self) -> usize { 
        DrawStringAlign::size() + self.string.clone().into_bytes().len()
    }
}


impl Serializable for DrawStringAlign {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.x_start.to_le_bytes());
        vec_data.extend_from_slice(&self.x_end.to_le_bytes());
        vec_data.extend_from_slice(&self.y.to_le_bytes());
        vec_data.push(self.align.into());
        vec_data.push(self.font.into());
        vec_data.push(self.pixel.into());
        vec_data.extend_from_slice(&self.string.clone().into_bytes());

        vec_data
    }
}


// -- DrawImage -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct DrawImage {
    pub x: i16,
    pub y: i16,
    pub width: i16,
    pub height: i16,
    pub vec_image: Vec<u8>,
}


impl DrawImage {
    pub fn new() -> DrawImage{
        DrawImage {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            vec_image: Vec::new(),
        }
    }


    pub const fn size() -> usize { 8 }


    pub fn parse(slice_data: &[u8]) -> Result<DrawImage, &'static str> {
        if slice_data.len() > DrawImage::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            Ok(DrawImage{
                x: ext.get_i16(),
                y: ext.get_i16(),
                width: ext.get_i16(),
                height: ext.get_i16(),
                vec_image: slice_data[DrawImage::size()..].to_vec(),
            })
        }
        else { Err("Wrong length") }
    }


    pub fn get_length(&self) -> usize { 
        DrawImage::size() + self.vec_image.len()
    }
}


impl Serializable for DrawImage {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.extend_from_slice(&self.x.to_le_bytes());
        vec_data.extend_from_slice(&self.y.to_le_bytes());
        vec_data.extend_from_slice(&self.width.to_le_bytes());
        vec_data.extend_from_slice(&self.height.to_le_bytes());
        vec_data.extend_from_slice(&self.vec_image[..]);

        vec_data
    }
}


