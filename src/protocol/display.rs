use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

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
    pub fn from_u8(line_u8: u8) -> Line {
        match Line::try_from( line_u8 ) {
            Ok(line) => { line },
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


    pub fn parse(clear_all: &mut ClearAll, vec_data: &Vec<u8>) -> bool {
        if vec_data.len() != ClearAll::size() {
            return false;
        }

        let mut ext: Extractor = Extractor::from_vec(vec_data);

        clear_all.pixel = Pixel::from_u8(ext.get_u8());

        true
    }


    pub fn from_vec(vec_data: &Vec<u8>) -> ClearAll {
        let mut data = ClearAll::new();
        ClearAll::parse(&mut data, vec_data);
        data
    }
}


impl Serializable for ClearAll {
    fn size() -> usize { 1 }


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


    pub fn parse(clear: &mut Clear, vec_data: &Vec<u8>) -> bool {
        if vec_data.len() != Clear::size() {
            return false;
        }

        let mut ext: Extractor = Extractor::from_vec(vec_data);

        clear.x = ext.get_i16();
        clear.y = ext.get_i16();
        clear.width = ext.get_i16();
        clear.height = ext.get_i16();
        clear.pixel = Pixel::from_u8(ext.get_u8());

        true
    }


    pub fn from_vec(vec_data: &Vec<u8>) -> Clear {
        let mut data = Clear::new();
        Clear::parse(&mut data, vec_data);
        data
    }
}


impl Serializable for Clear {
    fn size() -> usize { 9 }


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


    pub fn parse(invert: &mut Invert, vec_data: &Vec<u8>) -> bool {
        if vec_data.len() != Invert::size() {
            return false;
        }

        let mut ext: Extractor = Extractor::from_vec(vec_data);

        invert.x = ext.get_i16();
        invert.y = ext.get_i16();
        invert.width = ext.get_i16();
        invert.height = ext.get_i16();

        true
    }


    pub fn from_vec(vec_data: &Vec<u8>) -> Invert {
        let mut data = Invert::new();
        Invert::parse(&mut data, vec_data);
        data
    }
}


impl Serializable for Invert {
    fn size() -> usize { 8 }


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


    pub fn parse(draw_point: &mut DrawPoint, vec_data: &Vec<u8>) -> bool {
        if vec_data.len() != DrawPoint::size() {
            return false;
        }

        let mut ext: Extractor = Extractor::from_vec(vec_data);

        draw_point.x = ext.get_i16();
        draw_point.y = ext.get_i16();
        draw_point.pixel = Pixel::from_u8(ext.get_u8());

        true
    }


    pub fn from_vec(vec_data: &Vec<u8>) -> DrawPoint {
        let mut data = DrawPoint::new();
        DrawPoint::parse(&mut data, vec_data);
        data
    }
}


impl Serializable for DrawPoint {
    fn size() -> usize { 5 }


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


    pub fn parse(draw_line: &mut DrawLine, vec_data: &Vec<u8>) -> bool {
        if vec_data.len() != DrawLine::size() {
            return false;
        }

        let mut ext: Extractor = Extractor::from_vec(vec_data);

        draw_line.x1 = ext.get_i16();
        draw_line.y1 = ext.get_i16();
        draw_line.x2 = ext.get_i16();
        draw_line.y2 = ext.get_i16();
        draw_line.pixel = Pixel::from_u8(ext.get_u8());
        draw_line.line = Line::from_u8(ext.get_u8());

        true
    }


    pub fn from_vec(vec_data: &Vec<u8>) -> DrawLine {
        let mut data = DrawLine::new();
        DrawLine::parse(&mut data, vec_data);
        data
    }
}


impl Serializable for DrawLine {
    fn size() -> usize { 10 }


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


    pub fn parse(draw_rect: &mut DrawRect, vec_data: &Vec<u8>) -> bool {
        if vec_data.len() != DrawRect::size() {
            return false;
        }

        let mut ext: Extractor = Extractor::from_vec(vec_data);

        draw_rect.x = ext.get_i16();
        draw_rect.y = ext.get_i16();
        draw_rect.width = ext.get_i16();
        draw_rect.height = ext.get_i16();
        draw_rect.pixel = Pixel::from_u8(ext.get_u8());
        draw_rect.fill = if ext.get_u8() == 0 { false } else { true };
        draw_rect.line = Line::from_u8(ext.get_u8());

        true
    }


    pub fn from_vec(vec_data: &Vec<u8>) -> DrawLine {
        let mut data = DrawLine::new();
        DrawLine::parse(&mut data, vec_data);
        data
    }
}


impl Serializable for DrawRect {
    fn size() -> usize { 11 }


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


    pub fn parse(draw_circle: &mut DrawCircle, vec_data: &Vec<u8>) -> bool {
        if vec_data.len() != DrawCircle::size() {
            return false;
        }

        let mut ext: Extractor = Extractor::from_vec(vec_data);

        draw_circle.x = ext.get_i16();
        draw_circle.y = ext.get_i16();
        draw_circle.radius = ext.get_i16();
        draw_circle.pixel = Pixel::from_u8(ext.get_u8());
        draw_circle.fill = if ext.get_u8() == 0 { false } else { true };

        true
    }


    pub fn from_vec(vec_data: &Vec<u8>) -> DrawCircle {
        let mut data = DrawCircle::new();
        DrawCircle::parse(&mut data, vec_data);
        data
    }
}


impl Serializable for DrawCircle {
    fn size() -> usize { 8 }


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


    pub fn parse(draw_string: &mut DrawString, vec_data: &Vec<u8>) -> bool {
        if vec_data.len() <= DrawString::size() {
            return false;
        }

        let mut vec_string: Vec<u8> = Vec::new();
        vec_string.extend_from_slice(&vec_data[6..]);
        let string: String = match String::from_utf8(vec_string) { 
            Ok(s) => { s },
            Err(_e) => { String::from("") },
        };

        let mut ext: Extractor = Extractor::from_vec(vec_data);

        draw_string.x = ext.get_i16();
        draw_string.y = ext.get_i16();
        draw_string.font = Font::from_u8(ext.get_u8());
        draw_string.pixel = Pixel::from_u8(ext.get_u8());
        draw_string.string = string;

        true
    }


    pub fn from_vec(vec_data: &Vec<u8>) -> DrawString {
        let mut data = DrawString::new();
        DrawString::parse(&mut data, vec_data);
        data
    }


    pub fn get_length(&self) -> usize { 
        DrawString::size() + self.string.clone().into_bytes().len()
    }
}


impl Serializable for DrawString {
    fn size() -> usize { 6 }


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


    pub fn parse(draw_string_align: &mut DrawStringAlign, vec_data: &Vec<u8>) -> bool {
        if vec_data.len() <= DrawStringAlign::size() {
            return false;
        }

        let mut vec_string: Vec<u8> = Vec::new();
        vec_string.extend_from_slice(&vec_data[6..]);
        let string: String = match String::from_utf8(vec_string) { 
            Ok(s) => { s },
            Err(_e) => { String::from("") },
        };

        let mut ext: Extractor = Extractor::from_vec(vec_data);

        draw_string_align.x_start = ext.get_i16();
        draw_string_align.x_end = ext.get_i16();
        draw_string_align.y = ext.get_i16();
        draw_string_align.align = Align::from_u8(ext.get_u8());
        draw_string_align.font = Font::from_u8(ext.get_u8());
        draw_string_align.pixel = Pixel::from_u8(ext.get_u8());
        draw_string_align.string = string;

        true
    }


    pub fn from_vec(vec_data: &Vec<u8>) -> DrawStringAlign {
        let mut data = DrawStringAlign::new();
        DrawStringAlign::parse(&mut data, vec_data);
        data
    }


    pub fn get_length(&self) -> usize { 
        DrawStringAlign::size() + self.string.clone().into_bytes().len()
    }
}


impl Serializable for DrawStringAlign {
    fn size() -> usize { 9 }


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


    pub fn parse(draw_image: &mut DrawImage, vec_data: &Vec<u8>) -> bool {
        if vec_data.len() <= DrawImage::size() {
            return false;
        }

        let mut ext: Extractor = Extractor::from_vec(vec_data);

        draw_image.x = ext.get_i16();
        draw_image.y = ext.get_i16();
        draw_image.width = ext.get_i16();
        draw_image.height = ext.get_i16();
        draw_image.vec_image.clone_from_slice(&vec_data[8..]);

        true
    }


    pub fn from_vec(vec_data: &Vec<u8>) -> DrawImage {
        let mut data = DrawImage::new();
        DrawImage::parse(&mut data, vec_data);
        data
    }


    pub fn get_length(&self) -> usize { 
        DrawImage::size() + self.vec_image.len()
    }
}


impl Serializable for DrawImage {
    fn size() -> usize { 8 }


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


