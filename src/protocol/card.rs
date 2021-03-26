use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

use crate::protocol::Serializable;
use crate::communication::extractor::Extractor;


// -- LightLocation ----------------------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum LightLocation {
    #[num_enum(default)]
    Front,
    Rear
}


impl LightLocation {
    pub fn from_u8(data_u8: u8) -> LightLocation {
        match LightLocation::try_from( data_u8 ) {
            Ok(data) => { data },
            _ => { LightLocation::Front },
        }
    }
}


// -- LightColor ----------------------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum LightColor {
    #[num_enum(default)]
    Red,
    Green,
    Blue
}


impl LightColor {
    pub fn from_u8(data_u8: u8) -> LightColor {
        match LightColor::try_from( data_u8 ) {
            Ok(data) => { data },
            _ => { LightColor::Red },
        }
    }
}


// -- CardNameColor ----------------------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum CardNameColor {
    #[num_enum(default)]
    None            = 0x00,

    WhiteWhite      = 0x11,
    WhiteRed        = 0x12,
    WhiteYellow     = 0x13,
    WhiteGreen      = 0x14,
    WhiteCyan       = 0x15,
    WhiteBlue       = 0x16,
    WhiteMagenta    = 0x17,
    WhiteBlack      = 0x18,
    
    RedWhite        = 0x21,
    RedRed          = 0x22,
    RedYellow       = 0x23,
    RedGreen        = 0x24,
    RedCyan         = 0x25,
    RedBlue         = 0x26,
    RedMagenta      = 0x27,
    RedBlack        = 0x28,

    YellowWhite     = 0x31,
    YellowRed       = 0x32,
    YellowYellow    = 0x33,
    YellowGreen     = 0x34,
    YellowCyan      = 0x35,
    YellowBlue      = 0x36,
    YellowMagenta   = 0x37,
    YellowBlack     = 0x38,

    GreenWhite      = 0x41,
    GreenRed        = 0x42,
    GreenYellow     = 0x43,
    GreenGreen      = 0x44,
    GreenCyan       = 0x45,
    GreenBlue       = 0x46,
    GreenMagenta    = 0x47,
    GreenBlack      = 0x48,

    CyanWhite       = 0x51,
    CyanRed         = 0x52,
    CyanYellow      = 0x53,
    CyanGreen       = 0x54,
    CyanCyan        = 0x55,
    CyanBlue        = 0x56,
    CyanMagenta     = 0x57,
    CyanBlack       = 0x58,

    BlueWhite       = 0x61,
    BlueRed         = 0x62,
    BlueYellow      = 0x63,
    BlueGreen       = 0x64,
    BlueCyan        = 0x65,
    BlueBlue        = 0x66,
    BlueMagenta     = 0x67,
    BlueBlack       = 0x68,

    MagentaWhite    = 0x71,
    MagentaRed      = 0x72,
    MagentaYellow   = 0x73,
    MagentaGreen    = 0x74,
    MagentaCyan     = 0x75,
    MagentaBlue     = 0x76,
    MagentaMagenta  = 0x77,
    MagentaBlack    = 0x78,

    BlackWhite      = 0x81,
    BlackRed        = 0x82,
    BlackYellow     = 0x83,
    BlackGreen      = 0x84,
    BlackCyan       = 0x85,
    BlackBlue       = 0x86,
    BlackMagenta    = 0x87,
    BlackBlack      = 0x88,
    
    EndOfType
}


impl CardNameColor {
    pub fn from_u8(data_u8: u8) -> CardNameColor {
        match CardNameColor::try_from( data_u8 ) {
            Ok(data) => { data },
            _ => { CardNameColor::None },
        }
    }
}


// -- CardNamePiano ----------------------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum CardNamePiano {
    #[num_enum(default)]
    None            = 0x00,

    // Red - Function
    RecordingStart  = 0x21,     // 사용자 정의 멜로디 입력 시작
    RecordingEnd    = 0x22,     // 사용자 정의 멜로디 입력 종료
    Melody1         = 0x23,     // 멜로디 1
    Melody2         = 0x24,     // 멜로디 2
    Melody3         = 0x25,     // 멜로디 3
    Play            = 0x26,     // 저장한 멜로디 플레이
    MuteShort       = 0x27,     // 쉼표 0.5초
    Mute            = 0x28,     // 쉼표 1초
    
    // Yellow - 3 Octave Sharp
    CS3             = 0x31,
    DS3             = 0x32,
    //ES3           = 0x33,
    FS3             = 0x34,
    GS3             = 0x35,
    AS3             = 0x36,
    //BS3           = 0x37,
    //CS4           = 0x38,
    
    // Green - 3 Octave
    C3              = 0x41,
    D3              = 0x42,
    E3              = 0x43,
    F3              = 0x44,
    G3              = 0x45,
    A3              = 0x46,
    B3              = 0x47,
    //C4            = 0x48,
    
    // Cyan - 4 Octave Sharp
    CS4             = 0x51,
    DS4             = 0x52,
    //ES4           = 0x53,
    FS4             = 0x54,
    GS4             = 0x55,
    AS4             = 0x56,
    //BS4           = 0x57,
    //CS5           = 0x58,
    
    // Blue - 4 Octave
    C4              = 0x61,
    D4              = 0x62,
    E4              = 0x63,
    F4              = 0x64,
    G4              = 0x65,
    A4              = 0x66,
    B4              = 0x67,
    //C5            = 0x68,
    
    // Magenta - 5 Octave Sharp
    CS5             = 0x71,
    DS5             = 0x72,
    //ES5           = 0x73,
    FS5             = 0x74,
    GS5             = 0x75,
    AS5             = 0x76,
    //BS5           = 0x77,
    //CS6           = 0x78,
    
    // Black - 5 Octave
    C5              = 0x81,
    D5              = 0x82,
    E5              = 0x83,
    F5              = 0x84,
    G5              = 0x85,
    A5              = 0x86,
    B5              = 0x87,
    //C6            = 0x88,
    
    EndOfType
}


impl CardNamePiano {
    pub fn from_u8(data_u8: u8) -> CardNamePiano {
        match CardNamePiano::try_from( data_u8 ) {
            Ok(data) => { data },
            _ => { CardNamePiano::None },
        }
    }
}


// -- CardNameCardCoding ----------------------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum CardNameCardCoding {
    #[num_enum(default)]
    None                = 0x00,

    // White - Mode
    CalibrationWhite    = 0x11,
    Card                = 0x12, // 카드 코딩
    Motion              = 0x13, // 모션 코딩
    //Maze              = 0x14, // 미로 찾기
    //Random            = 0x15, // 랜덤
    //HandFollowing     = 0x16, // 핸드 팔로잉
    //LineTracer        = 0x17, // 라인 트레이서 시작
    Piano               = 0x18, // 피아노 모드
    
    // Red - Function
    CodingStart         = 0x21, // 카드 입력 시작 - 카드 입력 중 White Dimming
    CodingEnd           = 0x22, // 카드 입력 종료 - 카드 입력 완료 시 White Hold
    FunctionStart       = 0x23, // 함수 입력 시작 - 입력 중 Cyan Dimming
    FunctionEnd         = 0x24, // 함수 입력 종료 - 카드 입력 완료 시 Cyan Hold
    FunctionCall        = 0x25, // 함수 호출
    PlayMelody          = 0x26, // 멜로디 호출
    Speed               = 0x27, // 속도 조절
    Wait1Sec            = 0x28, // 1초 기다림
    
    // Yellow - LightBody
    LightBodyWhite      = 0x31,
    LightBodyRed        = 0x32,
    LightBodyYellow     = 0x33,
    LightBodyGreen      = 0x34,
    LightBodyCyan       = 0x35,
    LightBodyBlue       = 0x36,
    LightBodyMagenta    = 0x37,
    LightBodyBlack      = 0x38,
    
    // Green - 이착륙 및 이동 거리, 회전 각도 설정
    Takeoff             = 0x41, // 이륙
    Landing             = 0x42, // 착륙
    Distance300         = 0x43, // 30cm
    Distance500         = 0x44, // 50cm
    Distance1000        = 0x45, // 1m
    Degree30            = 0x46, // 30도
    Degree45            = 0x47, // 45도
    Degree90            = 0x48, // 90도
    
    // Cyan - Move - Basic
    MoveForward         = 0x51,
    MoveBackward        = 0x52,
    MoveLeft            = 0x53,
    MoveRight           = 0x54,
    MoveUp              = 0x55,
    MoveDown            = 0x56,
    TurnLeft            = 0x57,
    TurnRight           = 0x58,
    
    // Blue - If
    IfFindFrontObstacle = 0x61,
    IfFindGroundRed     = 0x62,
    IfFindGroundYellow  = 0x63,
    IfFindGroundGreen   = 0x64,
    IfFindGroundCyan    = 0x65,
    IfFindGroundBlue    = 0x66,
    IfElse              = 0x67,
    IfEnd               = 0x68,
    
    // Magenta - Loop
    LoopStartInfinite   = 0x71,
    LoopStart2          = 0x72,
    LoopStart3          = 0x73,
    LoopStart4          = 0x74,
    LoopStart5          = 0x75,
    LoopStart10         = 0x76,
    LoopBreak           = 0x77,
    LoopEnd             = 0x78,
    
    // Black - Melody
    C5                  = 0x81,
    D5                  = 0x82,
    E5                  = 0x83,
    F5                  = 0x84,
    G5                  = 0x85,
    A5                  = 0x86,
    B5                  = 0x87,
    C6                  = 0x88,
    
    EndOfType
}


impl CardNameCardCoding {
    pub fn from_u8(data_u8: u8) -> CardNameCardCoding {
        match CardNameCardCoding::try_from( data_u8 ) {
            Ok(data) => { data },
            _ => { CardNameCardCoding::None },
        }
    }
}




// -- CardColor ----------------------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum CardColor {
    #[num_enum(default)]
    Unknown     = 0x00,
    White       = 0x01,
    Red         = 0x02,
    Yellow      = 0x03,
    Green       = 0x04,
    Cyan        = 0x05,
    Blue        = 0x06,
    Magenta     = 0x07,
    Black       = 0x08,
    Grey        = 0x09,
}


impl CardColor {
    pub fn from_u8(data_u8: u8) -> CardColor {
        match CardColor::try_from( data_u8 ) {
            Ok(data) => { data },
            _ => { CardColor::Unknown },
        }
    }
}


// -- Classify -----------------------------------------------------------------------------------------------
#[derive(Debug, Clone)]
pub struct Classify {
    pub index: i8,
    pub cc: Vec<i8>,
    pub l: Vec<i8>,
}


impl Classify {
    const RHM: usize = 0;
    const RHX: usize = 1;
    const RSM: usize = 2;
    const RSX: usize = 3;
    const RLM: usize = 4;
    const RLX: usize = 5;
    
    const YHM: usize = 6;
    const YHX: usize = 7;
    const YSM: usize = 8;
    const YSX: usize = 9;
    const YLM: usize = 10;
    const YLX: usize = 11;
    
    const GHM: usize = 12;
    const GHX: usize = 13;
    const GSM: usize = 14;
    const GSX: usize = 15;
    const GLM: usize = 16;
    const GLX: usize = 17;
    
    const CHM: usize = 18;
    const CHX: usize = 19;
    const CSM: usize = 20;
    const CSX: usize = 21;
    const CLM: usize = 22;
    const CLX: usize = 23;
    
    const BHM: usize = 24;
    const BHX: usize = 25;
    const BSM: usize = 26;
    const BSX: usize = 27;
    const BLM: usize = 28;
    const BLX: usize = 29;
    
    const MHM: usize = 30;
    const MHX: usize = 31;
    const MSM: usize = 32;
    const MSX: usize = 33;
    const MLM: usize = 34;
    const MLX: usize = 35;

    const END: usize = 36;
}

impl Classify {
    pub fn new() -> Classify{
        Classify {
            index: 0,
            cc: Vec::new(),
            l: Vec::new(),
        }
    }


    pub const fn size() -> usize { 1 + 6 * 3 * 2 + 2 }


    pub fn parse(slice_data: &[u8]) -> Result<Vec<Classify>, &'static str> {
        if slice_data.len() > 0 && slice_data.len() % Classify::size() == 0 {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            let array_length = slice_data.len() / Classify::size();
            let mut vec_classify: Vec<Classify> = Vec::new();

            for _i in 0..array_length
            {
                let mut cl : Classify = Classify::new();
                
                cl.index = ext.get_i8();

                for _j in 0..Classify::END
                {
                    cl.cc.push(ext.get_i8());
                }

                cl.l.push(ext.get_i8());
                cl.l.push(ext.get_i8());

                vec_classify.push(cl);
            }

            Ok(vec_classify)
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for Classify {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.push(self.index as u8);
        
        for v in self.cc.iter()
        {
            vec_data.push(*v as u8);
        }

        for v in self.l.iter()
        {
            vec_data.push(*v as u8);
        }

        vec_data
    }
}


// -- Range -----------------------------------------------------------------------------------------------
#[derive(Debug, Clone)]
pub struct Range {
    pub range: Vec<i16>,
}


impl Range {
    const FRM: usize = 0;
    const FRX: usize = 1;
    const FGM: usize = 2;
    const FGX: usize = 3;
    const FBM: usize = 4;
    const FBX: usize = 5;
    
    const RRM: usize = 6;
    const RRX: usize = 7;
    const RGM: usize = 8;
    const RGX: usize = 9;
    const RBM: usize = 10;
    const RBX: usize = 11;

    const END: usize = 12;
}

impl Range {
    pub fn new() -> Range{
        Range {
            range: Vec::new(),
        }
    }


    pub const fn size() -> usize { 2 * 12 }


    pub fn parse(slice_data: &[u8]) -> Result<Range, &'static str> {
        if slice_data.len() == Range::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            let mut r : Range = Range::new();

            for _i in 0..Range::END
            {
                r.range.push(ext.get_i16());
            }

            Ok(r)
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for Range {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        for v in self.range.iter()
        {
            vec_data.extend_from_slice(&v.to_le_bytes());
        }

        vec_data
    }
}


// -- Raw -----------------------------------------------------------------------------------------------
#[derive(Debug, Clone)]
pub struct Raw {
    pub raw: Vec<i16>,
    pub rgb: Vec<u8>,
    pub hsvl: Vec<i16>,
    pub color: Vec<u8>,
    pub card: u8,
}

impl Raw {
    const F: usize = 0;
    const R: usize = 1;
}

impl Raw {
    const FR: usize = 0;
    const FG: usize = 1;
    const FB: usize = 2;
    const RR: usize = 3;
    const RG: usize = 4;
    const RB: usize = 5;
}

impl Raw {
    const FH: usize = 0;
    const FS: usize = 1;
    const FV: usize = 2;
    const FL: usize = 3;
    const RH: usize = 4;
    const RS: usize = 5;
    const RV: usize = 6;
    const RL: usize = 7;
}

impl Raw {
    pub fn new() -> Raw{
        Raw {
            raw: Vec::new(),
            rgb: Vec::new(),
            hsvl: Vec::new(),
            color: Vec::new(),
            card: 0,
        }
    }


    pub const fn size() -> usize { (2 * 2 * 3) + (2 * 3) + (2 * 2 * 4) + 2 + 1 }


    pub fn parse(slice_data: &[u8]) -> Result<Raw, &'static str> {
        if slice_data.len() > 0 && slice_data.len() % Raw::size() == 0 {
            let mut ext: Extractor = Extractor::from_slice(slice_data);

            let mut r : Raw = Raw::new();

            for _i in 0..6
            {
                r.raw.push(ext.get_i16());
            }

            for _i in 0..6
            {
                r.rgb.push(ext.get_u8());
            }

            for _i in 0..8
            {
                r.hsvl.push(ext.get_i16());
            }

            for _i in 0..2
            {
                r.color.push(ext.get_u8());
            }

            r.card = ext.get_u8();

            Ok(r)
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for Raw {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        for v in self.raw.iter()
        {
            vec_data.extend_from_slice(&v.to_le_bytes());
        }

        for v in self.rgb.iter()
        {
            vec_data.extend_from_slice(&v.to_le_bytes());
        }

        for v in self.hsvl.iter()
        {
            vec_data.extend_from_slice(&v.to_le_bytes());
        }

        for v in self.color.iter()
        {
            vec_data.extend_from_slice(&v.to_le_bytes());
        }

        vec_data.push(self.card);
        
        vec_data
    }
}


// -- Color -----------------------------------------------------------------------------------------------
#[derive(Debug, Clone)]
pub struct Color {
    pub hsvl: Vec<i16>,
    pub color: Vec<u8>,
    pub card: u8,
}

impl Color {
    const F: usize = 0;
    const R: usize = 1;
}

impl Color {
    const FH: usize = 0;
    const FS: usize = 1;
    const FV: usize = 2;
    const FL: usize = 3;
    const RH: usize = 4;
    const RS: usize = 5;
    const RV: usize = 6;
    const RL: usize = 7;
}

impl Color {
    pub fn new() -> Color{
        Color {
            hsvl: Vec::new(),
            color: Vec::new(),
            card: 0,
        }
    }


    pub const fn size() -> usize { (2 * 2 * 4) + 2 + 1 }


    pub fn parse(slice_data: &[u8]) -> Result<Color, &'static str> {
        if slice_data.len() > 0 && slice_data.len() % Color::size() == 0 {
            let mut ext: Extractor = Extractor::from_slice(slice_data);

            let mut r : Color = Color::new();

            for _i in 0..8
            {
                r.hsvl.push(ext.get_i16());
            }

            for _i in 0..2
            {
                r.color.push(ext.get_u8());
            }

            r.card = ext.get_u8();

            Ok(r)
        }
        else { Err("Wrong length") }
    }
}


impl Serializable for Color {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        for v in self.hsvl.iter()
        {
            vec_data.extend_from_slice(&v.to_le_bytes());
        }

        for v in self.color.iter()
        {
            vec_data.extend_from_slice(&v.to_le_bytes());
        }

        vec_data.push(self.card);
        
        vec_data
    }
}


// -- ListCard -----------------------------------------------------------------------------------------------
#[derive(Debug, Clone)]
pub struct ListCard {
    pub index_run: u8,
    pub total_size: u8,
    pub index: u8,
    pub card: Vec<u8>,
}

impl ListCard {
    pub fn new() -> ListCard{
        ListCard {
            index_run: 0,
            total_size: 0,
            index: 0,
            card: Vec::new(),
        }
    }


    pub const fn size() -> usize { 1 + 1 + 1 }


    pub fn parse(slice_data: &[u8]) -> Result<ListCard, &'static str> {
        if slice_data.len() == ListCard::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            let mut lc : ListCard = ListCard::new();

            lc.index_run = ext.get_u8();
            lc.total_size = ext.get_u8();
            lc.index = ext.get_u8();

            for _i in 0..(slice_data.len() - ListCard::size())
            {
                lc.card.push(ext.get_u8());
            }

            Ok(lc)
        }
        else { Err("Wrong length") }
    }


    pub fn get_length(&self) -> usize { 
        ListCard::size() + self.card.len()
    }
}


impl Serializable for ListCard {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.push(self.index_run);
        vec_data.push(self.total_size);
        vec_data.push(self.index);

        for v in self.card.iter()
        {
            vec_data.extend_from_slice(&v.to_le_bytes());
        }

        vec_data
    }
}


// -- ListFunction -----------------------------------------------------------------------------------------------
#[derive(Debug, Clone)]
pub struct ListFunction {
    pub index_run: u8,
    pub total_size: u8,
    pub index: u8,
    pub card: Vec<u8>,
}

impl ListFunction {
    pub fn new() -> ListFunction{
        ListFunction {
            index_run: 0,
            total_size: 0,
            index: 0,
            card: Vec::new(),
        }
    }


    pub const fn size() -> usize { 1 + 1 + 1 }


    pub fn parse(slice_data: &[u8]) -> Result<ListFunction, &'static str> {
        if slice_data.len() == ListFunction::size() {
            let mut ext: Extractor = Extractor::from_slice(slice_data);
            let mut lc : ListFunction = ListFunction::new();

            lc.index_run = ext.get_u8();
            lc.total_size = ext.get_u8();
            lc.index = ext.get_u8();

            for _i in 0..(slice_data.len() - ListFunction::size())
            {
                lc.card.push(ext.get_u8());
            }

            Ok(lc)
        }
        else { Err("Wrong length") }
    }


    pub fn get_length(&self) -> usize { 
        ListFunction::size() + self.card.len()
    }
}


impl Serializable for ListFunction {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec_data : Vec<u8> = Vec::new();

        vec_data.push(self.index_run);
        vec_data.push(self.total_size);
        vec_data.push(self.index);

        for v in self.card.iter()
        {
            vec_data.extend_from_slice(&v.to_le_bytes());
        }

        vec_data
    }
}



