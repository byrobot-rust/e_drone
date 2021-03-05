pub struct Extractor
{
    vec_data: Vec<u8>,
    index : usize,
}

impl Extractor
{
    pub fn new(vec_data_source: &Vec<u8>) -> Extractor {
        Extractor {
            vec_data: vec_data_source.clone(),
            index: 0,
        }
    }

    pub fn get_u8(&mut self) -> u8
    {
        if self.index + 1 <= self.vec_data.len() {
            let value: u8 = self.vec_data[self.index];
            self.index = self.index + 1;
            value
        }
        else { 0 }
    }

    pub fn get_u16(&mut self) -> u16
    {
        if self.index + 2 <= self.vec_data.len() {
            let value: u16 = ((self.vec_data[self.index] as u16) << 8) | (self.vec_data[self.index + 1] as u16);
            self.index = self.index + 2;
            value
        }
        else { 0 }
    }

    pub fn get_u32(&mut self) -> u32
    {
        if self.index + 4 <= self.vec_data.len() {
            let value: u32 =    ((self.vec_data[self.index + 3] as u32) << (8 * 3)) |
                                ((self.vec_data[self.index + 2] as u32) << (8 * 2)) |
                                ((self.vec_data[self.index + 1] as u32) << (8 * 1)) |
                                ( self.vec_data[self.index + 0] as u32);
            self.index = self.index + 4;
            value
        }
        else { 0 }
    }

    pub fn add_u64(&mut self) -> u64
    {
        if self.index + 8 <= self.vec_data.len() {
            let value: u64 =    ((self.vec_data[self.index + 7] as u64) << (8 * 7)) |
                                ((self.vec_data[self.index + 6] as u64) << (8 * 6)) |
                                ((self.vec_data[self.index + 5] as u64) << (8 * 5)) |
                                ((self.vec_data[self.index + 4] as u64) << (8 * 4)) |
                                ((self.vec_data[self.index + 3] as u64) << (8 * 3)) |
                                ((self.vec_data[self.index + 2] as u64) << (8 * 2)) |
                                ((self.vec_data[self.index + 1] as u64) << (8 * 1)) |
                                ( self.vec_data[self.index + 0] as u64);
            self.index = self.index + 8;
            value
        }
        else { 0 }
    }

    pub fn get_i8(&mut self) -> i8
    {
        if self.index + 1 <= self.vec_data.len() {
            let value: i8 = self.vec_data[self.index] as i8;
            self.index = self.index + 1;
            value
        }
        else { 0 }
    }

    pub fn get_i16(&mut self) -> i16
    {
        if self.index + 2 <= self.vec_data.len() {
            let value: i16 = (((self.vec_data[self.index] as u16) << 8) | (self.vec_data[self.index + 1] as u16)) as i16;
            self.index = self.index + 2;
            value
        }
        else { 0 }
    }

    pub fn get_i32(&mut self) -> i32
    {
        if self.index + 4 <= self.vec_data.len() {
            let value: i32 =   (((self.vec_data[self.index + 3] as u32) << (8 * 3)) |
                                ((self.vec_data[self.index + 2] as u32) << (8 * 2)) |
                                ((self.vec_data[self.index + 1] as u32) << (8 * 1)) |
                                ( self.vec_data[self.index + 0] as u32)) as i32;
            self.index = self.index + 4;
            value
        }
        else { 0 }
    }

    pub fn add_i64(&mut self) -> i64
    {
        if self.index + 8 <= self.vec_data.len() {
            /*
            let value: i64 =   (((self.vec_data[self.index + 7] as u64) << (8 * 7)) |
                                ((self.vec_data[self.index + 6] as u64) << (8 * 6)) |
                                ((self.vec_data[self.index + 5] as u64) << (8 * 5)) |
                                ((self.vec_data[self.index + 4] as u64) << (8 * 4)) |
                                ((self.vec_data[self.index + 3] as u64) << (8 * 3)) |
                                ((self.vec_data[self.index + 2] as u64) << (8 * 2)) |
                                ((self.vec_data[self.index + 1] as u64) << (8 * 1)) |
                                ( self.vec_data[self.index + 0] as u64)) as i64;
            // */

            let mut value_array: Vec<u8> = Vec::new();
            value_array.clone_from_slice(&self.vec_data[self.index..self.index + 7]);
            let value: i64 = i64::
            self.index = self.index + 8;
            value
        }
        else { 0 }
    }

    pub fn add_f32(&mut self, data: f32)
    {
        /*
        let u_data : u32 = data as u32;
        self.vec_data.push((u_data & 0xFF) as u8);
        self.vec_data.push(((u_data >> (8 * 1)) & 0xFF) as u8);
        self.vec_data.push(((u_data >> (8 * 2)) & 0xFF) as u8);
        self.vec_data.push(((u_data >> (8 * 3)) & 0xFF) as u8);
        // */
        
        self.vec_data.append(&mut data.to_le_bytes().to_vec());
    }

    pub fn add_f64(&mut self, data: f64)
    {
        /*
        let u_data : u64 = data as u64;
        self.vec_data.push((u_data & 0xFF) as u8);
        self.vec_data.push(((u_data >> (8 * 1)) & 0xFF) as u8);
        self.vec_data.push(((u_data >> (8 * 2)) & 0xFF) as u8);
        self.vec_data.push(((u_data >> (8 * 3)) & 0xFF) as u8);
        self.vec_data.push(((u_data >> (8 * 4)) & 0xFF) as u8);
        self.vec_data.push(((u_data >> (8 * 5)) & 0xFF) as u8);
        self.vec_data.push(((u_data >> (8 * 6)) & 0xFF) as u8);
        self.vec_data.push(((u_data >> (8 * 7)) & 0xFF) as u8);
        // */
        
        self.vec_data.append(&mut data.to_le_bytes().to_vec());
    }

    pub fn get_vec_clone(&self) -> Vec<u8>
    {
        self.vec_data.clone()
    }
}
