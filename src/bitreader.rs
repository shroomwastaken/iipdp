pub struct BitReader {
    pub bit_str: String,
}

impl BitReader {
    pub fn from_least_to_most_significant(&self, byte: &String) -> String {
        let mut bits = [false; 8];

        let mut char_vec_bits: Vec<char> = Vec::new();

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

    pub fn init(&mut self) {
        let mut chunks: Vec<String> = self.bit_str
                .chars()
                .collect::<Vec<char>>()
                .chunks(8)
                .map(|chunk| chunk.iter().collect())
                .collect();

        for i in 0..chunks.len() {
            chunks[i] = self.from_least_to_most_significant(&chunks[i])
        }

        self.bit_str = chunks.into_iter().collect();
    }

    pub fn read_bits(&mut self, amount: usize) -> String {
        let mut res: String = "".to_string();

        for _i in 0..amount {
            res.push_str(self.bit_str.get(..1).unwrap());
            self.bit_str = self.bit_str[1..].to_string();
        }

        return res;
    }

    pub fn read_x_if_exists(&mut self, x:usize) -> String{
        let mut res: String;

        if self.read_bits(1) == "1" {
            res = self.read_bits(x);
            res = res.chars().rev().collect::<String>();
        } else {
            res = "Null".to_string();
        }

        return res;
    }
}