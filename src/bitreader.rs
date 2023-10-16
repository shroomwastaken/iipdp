use crate::structs::utils::{Vec3, EHandle};

pub struct BitReader {
    pub contents: Vec<u8>,
    pub cur_bit_index: i32,
}

impl BitReader {
    pub fn read_bits(&mut self, num_bits: i32) -> Vec<u8> {
        let mut result:  Vec<u8> = Vec::new();
        let mut bits_left = num_bits;
        for _ in self.cur_bit_index / 8..self.cur_bit_index / 8 + (num_bits / 8 + if num_bits % 8 == 0 && num_bits >= 8 { 0 } else { 1 }) {
            let mut cur_val: u8 = 0;
            for bit in 0..if bits_left >= 8 { 8 } else { bits_left } {
                cur_val |= ((self.contents[(self.cur_bit_index / 8) as usize] >> ((self.cur_bit_index) % 8)) & 1) << bit;
                self.cur_bit_index += 1;
            }
            result.push(cur_val);
            bits_left -= 8;
        }
        return result;
    }

    pub fn split_and_skip(&mut self, amount: i32) -> Self {
        let new_reader = BitReader { contents: self.contents.clone(), cur_bit_index: self.cur_bit_index };

        self.skip(amount);

        return new_reader;
    }

    pub fn skip(&mut self, amount: i32) {
        self.cur_bit_index += amount;
    }

    pub fn read_int(&mut self, amount: i32) -> i32 {
        let bytes = self.read_bits(amount);
        return match bytes.len() {
            1 => i32::from_le_bytes([bytes[0], 0, 0, 0]),
            2 => i32::from_le_bytes([bytes[0], bytes[1], 0, 0]),
            3 => i32::from_le_bytes([bytes[0], bytes[1], bytes[2], 0]),
            4 => i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
            _ => 0,
        }
    }

    pub fn read_uint_64(&mut self) -> u64 {
        return u64::from_le_bytes(self.read_bits(64).as_slice().try_into().unwrap());
    }

    pub fn read_float(&mut self, amount: i32) -> f32 {
        let bytes = self.read_bits(amount);
        return (match bytes.len() {
            1 => f32::from_le_bytes([bytes[0], 0, 0, 0]),
            2 => f32::from_le_bytes([bytes[0], bytes[1], 0, 0]),
            3 => f32::from_le_bytes([bytes[0], bytes[1], bytes[2], 0]),
            4 => f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
            _ => 0.0,
        } * 1000.0).round() / 1000.0
    }

    pub fn read_ascii_string_nulled(&mut self) -> String {
        let mut char_vec: Vec<u8> = Vec::new();
        let mut byte = self.read_bits(8)[0];

        // while the next character isnt a null terminator ("\0")
        while byte != 0 {
            char_vec.push(byte);
            byte = self.read_bits(8)[0];
        }

        // account for the null terminator
        char_vec.push(byte);

        let res = String::from_utf8(char_vec).unwrap().trim_end_matches("\0").to_string();

        return res;
    }

    pub fn read_ascii_string(&mut self, amount: i32) -> String {
        return String::from_utf8(self.read_bits(amount)).unwrap().trim_end_matches("\0").to_string();
    }

    pub fn read_bytes(&mut self, amount: i32) -> Vec<u8> {
        let mut res: Vec<u8> = Vec::new();
        for _ in 0..amount {
            let cur_byte = self.read_bits(8)[0];
            res.push(cur_byte);
        }

        return res;
    }

    pub fn read_vec3(&mut self) -> Vec3 {
        return Vec3 { x: self.read_float(32), y: self.read_float(32), z: self.read_float(32) };
    }

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

    pub fn read_vector_coords(&mut self) -> Vec<Option<f32>> {
        let (x, y, z) = (self.read_bool(), self.read_bool(), self.read_bool());

        let mut coords_vec: Vec<Option<f32>> = Vec::new();

        if x { coords_vec.push(Some(self.read_vector_coord())) } else { coords_vec.push(None) }
        if y { coords_vec.push(Some(self.read_vector_coord())) } else { coords_vec.push(None) }
        if z { coords_vec.push(Some(self.read_vector_coord())) } else { coords_vec.push(None) }
        
        return coords_vec;
    }

    pub fn read_ehandle(&mut self) -> EHandle { return EHandle { val: self.read_int(32)} }

    pub fn read_bool(&mut self) -> bool { return self.read_bits(1)[0] == 1; }

    pub fn read_int_if_exists(&mut self, amount: i32) -> Option<i32> { return if self.read_bool() { Some(self.read_int(amount)) } else { None } }

    pub fn read_float_if_exists(&mut self, amount: i32) -> Option<f32> { return if self.read_bool() { Some(self.read_float(amount)) } else { None } }
}
