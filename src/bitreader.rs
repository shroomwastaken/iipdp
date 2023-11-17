use crate::structs::utils::{Vec3, EHandle};

// the struct and the read_bits, fetch and skip functions are from https://github.com/evanlin96069/sdp-c/blob/master/src/bits.c
// huge thanks to evanlin96069 for giving advice and making readable c code

#[derive(Debug, Clone)]
pub struct BitReader {
    pub bits: Vec<u8>,
    pub offset: u8,
    pub bit_size: usize,
    pub byte_size: usize,
    pub fetch: u64,
    pub current: usize,
}

impl BitReader {
    pub fn new(bits: Vec<u8>) -> BitReader{
        let mut new_reader = BitReader {
            bits: Vec::new(),
            offset: 0,
            bit_size: 0,
            byte_size: 0,
            fetch: 0,
            current: 0,
        };
        new_reader.bits = bits;
        new_reader.bit_size = new_reader.bits.len() * 8;
        new_reader.byte_size = new_reader.bit_size / 8;
        new_reader.fetch();
        
        return new_reader;
    }
    
    pub fn fetch(&mut self) {
        let block = if self.current / 8 + 8 > self.byte_size { self.byte_size - 8 } else { self.current / 8 };
        self.fetch = u64::from_le_bytes(self.bits[block..block + 8].try_into().unwrap());
        self.offset = (self.current - (block * 8)) as u8;
    }
        
    // reads bits
    pub fn read_bits(&mut self, amount: i32) -> u64 {        
        if self.current + amount as usize > self.bit_size {
            panic!("overflowed somehow lmao");
        }
        let mut new_amount = amount.to_owned() as usize; // this is so that i can modify the amount value
        let mut res: u64 = 0;
        let remain: usize = (64 - self.offset).into(); // bits remaining in the current fetch
        let mut shift: usize = 0; // uhhhhh
        

        if remain == 0 { // if we've gone through the entire fetch
            self.fetch();
        } else if new_amount > remain { // if we havent gone trough the entire fetch yet but we need to get the next one
            res |= (self.fetch >> self.offset) & ((1u64 << remain) - 1);
            self.skip(remain as i32); // add remain bits to the current bit index, get new fetch (see skip function)
            new_amount -= remain; // how many bits are left to read
            shift = remain; // what position to put those bits in
        }

        /*
            EXAMPLE:

            fetch = 0b0111000000010100110100000000000000000000010001101001000011000000
            offset = 6
            amount = 8
            remain = 58
            shift: 0

            fetch >> offset                             = 0b0000000111000000010100110100000000000000000000010001101001000011
            (1u64 << amount) - 1                        = 0b0000000000000000000000000000000000000000000000000000000011111111
            (fetch >> offset) & ((1u64 << amount) - 1)  = 0b0000000000000000000000000000000000000000000000000000000001000011
            
            then we right shift it by "shift" to put the bits into their correct place in the number if we fetched above
        */

        res |= ((self.fetch >> self.offset) & ((1u64 << new_amount) - 1)) << shift;
        
        self.current += new_amount;
        self.offset += new_amount as u8;
        return res;
    }
    
    // do i have to explain this one
    pub fn skip(&mut self, amount: i32) {
        self.current += amount as usize;
        if self.current > self.bit_size {
            panic!("overflow while skipping");
        }
        self.fetch();
    }

    // reads character by character until the character is \0
    pub fn read_ascii_string_nulled(&mut self) -> String {
        let mut char_vec: Vec<u8> = Vec::new();
        let mut cur_char = self.read_bits(8);
        while cur_char != 0 {
            char_vec.push(cur_char as u8);
            cur_char = self.read_bits(8);
        }

        return String::from_utf8(char_vec).unwrap().trim_end_matches("\0").to_string();
    }

    // clones the bitreader and skips amount bits in the parent one
    pub fn split_and_skip(&mut self, amount: i32) -> Self {
        let new_reader = self.clone();

        self.skip(amount);

        return new_reader;
    }

    // reads an int ¯\_(ツ)_/¯
    pub fn read_int(&mut self, amount: i32) -> i32 {
        return i32::from_le_bytes((self.read_bits(amount) as u32).to_le_bytes());
    }

    pub fn read_signed_int(&mut self, amount: i32) -> i32 {
        let mut res = self.read_bits(amount) as i32;
        if (res & (1 << (amount - 1))) != 0 {
            res |= i32::MAX << amount;
        }
        return res;
    }

    // i dislike steampipe
    // thanks jukspa :)
    pub fn read_var_int32(&mut self) -> i32 {
        let mut res: i32 = 0;

        for i in 0..5 {
            let b: i32 = self.read_int(8);
            res |= (b & 0x7F) << (7 * i);
            if (b & 0x80) == 0 { break; }
        }
        return res as i32;
    }

    // used once, dont remember where
    pub fn read_uint_64(&mut self) -> u64 {
        return u64::from_le_bytes(self.read_bits(64).to_le_bytes());
    }

    // same as read_int function but float :0
    pub fn read_float(&mut self, amount: i32) -> f32 {
        return (f32::from_le_bytes((self.read_bits(amount) as u32).to_le_bytes()) * 1000.0).round() / 1000.0;
    }

    // read ascii string that has a determined length
    // used mostly in the header
    pub fn read_ascii_string(&mut self, amount: i32) -> String {
        let mut char_vec: Vec<u8> = Vec::new();
        for _ in 0..amount / 8 {
            let cur_char = self.read_bits(8);
            char_vec.push(cur_char as u8);
        }

        return String::from_utf8(char_vec).unwrap().trim_end_matches("\0").to_string();
    }

    // also used once
    pub fn read_bytes(&mut self, amount: i32) -> Vec<u8> {
        let mut res: Vec<u8> = Vec::new();
        for _ in 0..amount {
            let cur_byte = self.read_bits(8) as u8;
            res.push(cur_byte);
        }

        return res;
    }

    // used a couple of times
    // rust doesnt have a built in vector3 type (iirc)
    // this just reads a Vec3, for more go to utils.rs line 222
    pub fn read_vec3(&mut self) -> Vec3 {
        return Vec3 { x: self.read_float(32), y: self.read_float(32), z: self.read_float(32) };
    }

    // weird function that i just took from nekz.me
    // seems to work properly so ¯\_(ツ)_/¯
    pub fn read_vector_coord(&mut self) -> f32 {
        const COORD_INTEGER_BITS: i32 = 14;
        const COORD_FRACTIONAL_BITS: i32 = 5;
        const COORD_DENOMINATOR: u8 = 1u8 << COORD_FRACTIONAL_BITS;
        const COORD_RESOLUTION: f32 = 1f32 / (COORD_DENOMINATOR as f32);

        let mut value: f32 = 0f32;
        let integer = self.read_bool();
        let fraction = self.read_bool();

        if integer || fraction {
            let sign = self.read_bool();

            if integer {
                value += self.read_int(COORD_INTEGER_BITS) as f32;
            }

            if fraction {
                value += self.read_float(COORD_FRACTIONAL_BITS) * COORD_RESOLUTION;
            }

            if sign {
                value = -value;
            }
        }

        return value
    }

    // calls the previous function for x, y, and z coords
    pub fn read_vector_coords(&mut self) -> Vec<Option<f32>> {
        let (x, y, z) = (self.read_bool(), self.read_bool(), self.read_bool());

        let mut coords_vec: Vec<Option<f32>> = Vec::new();

        if x { coords_vec.push(Some(self.read_vector_coord())) } else { coords_vec.push(None) }
        if y { coords_vec.push(Some(self.read_vector_coord())) } else { coords_vec.push(None) }
        if z { coords_vec.push(Some(self.read_vector_coord())) } else { coords_vec.push(None) }
        
        return coords_vec;
    }

    // ehandle reader, no clue what ehandles really are yet, ask uncrafted
    // for more info on the Ehandle type go to utils.rs
    pub fn read_ehandle(&mut self) -> EHandle { return EHandle { val: self.read_int(32)} }

    // do i need to explain this
    pub fn read_bool(&mut self) -> bool { return self.read_bits(1) == 1; }

    // the read_x_if_exists functions only read amount bits if the next bit is set
    pub fn read_int_if_exists(&mut self, amount: i32) -> Option<i32> { return if self.read_bool() { Some(self.read_int(amount)) } else { None } }

    pub fn read_float_if_exists(&mut self, amount: i32) -> Option<f32> { return if self.read_bool() { Some(self.read_float(amount)) } else { None } }

    pub fn read_signed_int_if_exists(&mut self, amount: i32) -> Option<i32> { return if self.read_bool() { Some(self.read_signed_int(amount)) } else { None }}
}
