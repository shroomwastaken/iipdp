pub struct BitReader {
    pub bit_str: String,
}

impl BitReader {
    pub fn new(bits: String) -> Self {
        let mut chunks: Vec<String> = bits
                .chars()
                .collect::<Vec<char>>()
                .chunks(8)
                .map(|chunk| chunk.iter().collect())
                .collect();

        for i in 0..chunks.len() {
            chunks[i] = from_least_to_most_significant(&chunks[i])
        }

        let res_bits: String = chunks.into_iter().collect();
        Self { bit_str: res_bits }
    }    

    // takes amount of bits to read (amount);
    // returns read bits and removes read bits from bit string;
    pub fn read_bits(&mut self, amount: usize) -> String {
        let mut res: String = String::new();

        res = self.bit_str[..amount].to_string();
        self.bit_str = self.bit_str[amount..].to_string();
        // reverses bits back, so that decoder parses properly
        return res.chars().into_iter().rev().collect();
    }

    pub fn split_and_skip(&mut self, size: usize) -> Self {
        let res = self.bit_str.get(..(size as usize)).unwrap().to_string();
        self.skip((size) as usize);

        Self { bit_str: res }
    }

    pub fn skip(&mut self, amount: usize) {
        self.bit_str = self.bit_str[amount..].to_string()
    }

    pub fn read_int(&mut self, amount: usize) -> i32 {
        let decode_str = self.read_bits(amount);
        return i32::from_str_radix(&decode_str, 2).unwrap_or_else(|_| {
            u32::from_str_radix(&decode_str, 2).unwrap() as i32
        })
    }

    pub fn read_uint_64(&mut self) -> u64 {
        let decode_str = self.read_bits(64);
        return u64::from_str_radix(&decode_str, 2).unwrap();
    }

    pub fn read_float(&mut self, amount: usize) -> f32 {
        return (f32::from_bits(u32::from_str_radix(&self.read_bits(amount), 2).unwrap()) * 1000.0).round() / 1000.0;
    }

    pub fn read_ascii_string_nulled(&mut self) -> String {
        let mut char_vec: Vec<u8> = Vec::new();

        // while the next character isnt a null terminator ("\0")
        while self.peek_bits(8) != "00000000" {
            char_vec.push(u8::from_str_radix(&self.read_bits(8), 2).unwrap());
        }

        // account for the null terminator
        char_vec.push(u8::from_str_radix(&self.read_bits(8), 2).unwrap());

        return String::from_utf8(char_vec).unwrap().trim_end_matches("\0").to_string();
    }

    pub fn read_ascii_string(&mut self, amount: usize) -> String {
        let mut char_vec: Vec<u8> = Vec::new();

        for _ in 0..amount / 8 {
            char_vec.push(u8::from_str_radix(&self.read_bits(8), 2).unwrap());
        }

        return String::from_utf8(char_vec).unwrap().trim_end_matches("\0").to_string();
    }

    pub fn read_bool(&mut self) -> bool { return self.read_bits(1) == "1"; }

    pub fn read_int_if_exists(&mut self, amount: usize) -> Option<i32> { return if self.read_bool() { Some(self.read_int(amount)) } else { None } }

    pub fn read_float_if_exists(&mut self, amount: usize) -> Option<f32> { return if self.read_bool() { Some(self.read_float(amount)) } else { None } }

    pub fn peek_bits(&self, amount: usize) -> &str {
        return self.bit_str.get(..amount).unwrap();
    }
}

// takes byte string;
// returns byte string but the bits are from least significant to most significant;
// e. g. "00100001" turns into "10000100"
pub fn from_least_to_most_significant(byte: &String) -> String {
    let mut bits = [false; 8];

    let mut char_vec_bits: Vec<char> = Vec::new();

    // i have no idea how exactly this works but it works so im not gonna touch it :)
    for i in 0..8 {
        bits[i] = (u8::from_str_radix(&byte, 2).unwrap() & (1 << i)) != 0;
    }

    for i in 0..bits.len() {
        if bits[i] {
            char_vec_bits.push('1');
        } else {
            char_vec_bits.push('0');
        }
    }

    return char_vec_bits.into_iter().collect();
}