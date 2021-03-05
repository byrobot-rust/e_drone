pub struct Serializer
{
    vec_data: Vec<u8>,
}

impl Serializer
{
    pub fn new() -> Serializer {
        Serializer {
            vec_data: Vec::new()
        }
    }

    pub fn add_u8(&mut self, data: u8)
    {
        self.vec_data.push(data);
    }

    pub fn add_u16(&mut self, data: u16)
    {
        /*
        self.vec_data.push((data & 0xFF) as u8);
        self.vec_data.push((data >> 8) as u8);
        // */
        
        self.vec_data.append(&mut data.to_le_bytes().to_vec());
    }

    pub fn add_u32(&mut self, data: u32)
    {
        /*
        self.vec_data.push((data & 0xFF) as u8);
        self.vec_data.push(((data >> (8 * 1)) & 0xFF) as u8);
        self.vec_data.push(((data >> (8 * 2)) & 0xFF) as u8);
        self.vec_data.push(((data >> (8 * 3)) & 0xFF) as u8);
        // */
        self.vec_data.append(&mut data.to_le_bytes().to_vec());
    }

    pub fn add_u64(&mut self, data: u64)
    {
        /*
        self.vec_data.push((data & 0xFF) as u8);
        self.vec_data.push(((data >> (8 * 1)) & 0xFF) as u8);
        self.vec_data.push(((data >> (8 * 2)) & 0xFF) as u8);
        self.vec_data.push(((data >> (8 * 3)) & 0xFF) as u8);
        self.vec_data.push(((data >> (8 * 4)) & 0xFF) as u8);
        self.vec_data.push(((data >> (8 * 5)) & 0xFF) as u8);
        self.vec_data.push(((data >> (8 * 6)) & 0xFF) as u8);
        self.vec_data.push(((data >> (8 * 7)) & 0xFF) as u8);
        // */
        self.vec_data.append(&mut data.to_le_bytes().to_vec());
    }

    pub fn add_i8(&mut self, data: i8)
    {
        self.vec_data.push(data as u8);
    }

    pub fn add_i16(&mut self, data: i16)
    {
        /*
        let u_data : u16 = data as u16;
        self.vec_data.push((u_data & 0xFF) as u8);
        self.vec_data.push((u_data >> 8) as u8);
        // */
        self.vec_data.append(&mut data.to_le_bytes().to_vec());
    }

    pub fn add_i32(&mut self, data: i32)
    {
        /*
        u_data : u32 = data as u32;
        self.vec_data.push((u_data & 0xFF) as u8);
        self.vec_data.push(((u_data >> (8 * 1)) & 0xFF) as u8);
        self.vec_data.push(((u_data >> (8 * 2)) & 0xFF) as u8);
        self.vec_data.push(((u_data >> (8 * 3)) & 0xFF) as u8);
        // */
        self.vec_data.append(&mut data.to_le_bytes().to_vec());
    }

    pub fn add_i64(&mut self, data: i64)
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
