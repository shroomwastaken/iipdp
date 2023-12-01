use crate::BitReader;
use crate::structs::send_table::{SendTableProp, PropFlag, FloatParseType};
use crate::structs::utils::{Vec3, Vec2};

/*
    below is prop parsing functions, they dont look very nice
    this is a separate file to keep this less used and more bad-looking code out of the way
    i borrowed these from untitledparser here: https://github.com/UncraftedName/UntitledParser/blob/master/DemoParser/src/Utils/BitStreams/PropDecodeReader.cs
    hopefully i get to this part when making docs and properly explain all of this
*/

// consts taken from the same untitledparser file
const COORD_FRAC_BITS: i32   = 5;
const COORD_DENOM: i32   = 1 << COORD_FRAC_BITS;
const COORD_RES: f32 = 1.0 / COORD_DENOM as f32;
const COORD_INT_BITS_MP: i32   = 11;
const COORD_INT_BITS: i32   = 14;
const COORD_FRAC_BITS_MP_LP: i32 = 3;
const COORD_DENOM_LP: i32 = 1 << COORD_FRAC_BITS_MP_LP;
const COORD_RES_LP: f32 = 1.0 / COORD_DENOM_LP as f32;
const NORM_FRAC_BITS: i32 = 11;
const NORM_DENOM: i32 = (1 << NORM_FRAC_BITS) - 1;
const NORM_RES: f32 = 1.0 / NORM_DENOM as f32;
const DT_MAX_STRING_BITS: i32 = 9; // read this many bits to get the length of a string prop
const MAX_VAR_INT32_BYTES: i32 = 5;

impl BitReader {
    pub fn read_u_bit_var(&mut self) -> i32 {
        match self.read_int(2) {
            0 => self.read_int(4),
            1 => self.read_int(8),
            2 => self.read_int(12),
            3 => self.read_int(32),
            _ => panic!("something went wrong in read_u_bit_var"),
        }
    }

    pub fn read_u_bit_int(&mut self) -> i32 {
        let res = self.read_int(4);
        match res {
            0 => res,
            1 => res | (self.read_int(4) << 4),
            2 => res | (self.read_int(8) << 4),
            3 => res | (self.read_int(28) << 4),
            _ => panic!("something went wrong in read_u_bit_int"),
        }
    }

    pub fn read_bit_coord(&mut self) -> f32 {
        let mut val: f32 = 0.0;
        let has_int = self.read_bool();
        let has_frac = self.read_bool();
        if has_int || has_frac {
            let sign = self.read_bool();
            if has_int { val += (self.read_int(COORD_INT_BITS) + 1) as f32; }
            if has_frac { val += self.read_int(COORD_FRAC_BITS) as f32 * COORD_RES; }
            if sign { val = -val; }
        }
        return val;
    }

    pub fn read_bit_coord_mp(&mut self, prop: &SendTableProp) -> f32 {
        let mut sign = false;
        let mut val: f32 = 0.0;
        let b_inbounds = self.read_bool();
        if prop.float_parse_type == FloatParseType::BitCoordMpInt {
            if self.read_bool() {
                sign = self.read_bool();
                if b_inbounds {
                    val = (self.read_int(COORD_INT_BITS_MP) + 1) as f32; 
                } else {
                    val = (self.read_int(COORD_INT_BITS) + 1) as f32;
                }
            }
        } else {
            let mut int_val = self.read_int(1);
            sign = self.read_bool();
            if int_val != 0 {
                if b_inbounds {
                    int_val = self.read_int(COORD_INT_BITS_MP) + 1;
                } else {
                    int_val = self.read_int(COORD_INT_BITS) + 1;
                }
            }
            let lp = prop.float_parse_type == FloatParseType::BitCoordMpLp;
            let fract_val = self.read_int(if lp { COORD_FRAC_BITS_MP_LP } else { COORD_FRAC_BITS });
            val = int_val as f32 + fract_val as f32 * (if lp { COORD_RES_LP } else { COORD_RES });
        }
        if sign { val = -val; }
        return val;
    }

    pub fn read_field_index(&mut self, last_index: i32, b_new_way: bool) -> i32 {
        if b_new_way && self.read_bool() {
            return last_index + 1;
        } // short circuit
			let mut ret: i32;
			if b_new_way && self.read_bool() {
				ret = self.read_int(3);
			} else {
				ret = self.read_int(5);
				ret = match self.read_int(2) {
					0 => ret,
					1 => ret | (self.read_int(2) << 5),
					2 => ret | (self.read_int(4) << 5),
					3 => ret | (self.read_int(7) << 5),
                    _ => panic!("something went very wrong in read_field_index")
				};
			}
			if ret == 0xFFF {
				return -1;
            } // end marker
			return last_index + 1 + ret;
    }

    pub fn read_standard_float(&mut self, prop: &SendTableProp) -> f32 {
        let bits = prop.num_bits.unwrap();
        let dw_interp = self.read_int(bits);
        let val: f32 = dw_interp as f32 / ((1 << bits) - 1) as f32;
        return prop.low_value.unwrap() + (prop.high_value.unwrap() - prop.low_value.unwrap()) * val;
    }

    pub fn read_bit_normal(&mut self) -> f32 {
        let sign = self.read_bool();
        let mut val: f32 = self.read_int(NORM_FRAC_BITS) as f32 * NORM_RES;
        if sign { val = -val }
        return val;
    }

    pub fn read_bit_cell_coord(&mut self, prop: &SendTableProp) -> f32 {
        let int_val = self.read_int(prop.num_bits.unwrap());
        if prop.float_parse_type == FloatParseType::BitCellChordInt {
            return int_val as f32;
        } else {
            let lp = prop.float_parse_type == FloatParseType::BitCellChordLp;
            let fract_val = self.read_int(if lp { COORD_FRAC_BITS_MP_LP } else { COORD_FRAC_BITS });
            return int_val as f32 + fract_val as f32 * (if lp { COORD_RES_LP } else { COORD_RES });
        }
    }

    pub fn set_float_parse_type(&mut self, prop: &mut SendTableProp) {
        let mut ptype = FloatParseType::None;
        if prop.flags.contains(PropFlag::Coord) {
            ptype = FloatParseType::Coord;
        } else if prop.flags.contains(PropFlag::CoordMp) {
            ptype = FloatParseType::BitCoordMp;
        } else if prop.flags.contains(PropFlag::CoordMpInt) {
            ptype = FloatParseType::BitCoordMpInt;
        } else if prop.flags.contains(PropFlag::NoScale) {
            ptype = FloatParseType::NoScale;
        } else if prop.flags.contains(PropFlag::Normal) {
            ptype = FloatParseType::Normal;
        } // theres more checks here but those are for protocol 4 so ill do them later
        prop.float_parse_type = ptype;
    }

    // decoding values of different types

    pub fn decode_int(&mut self, prop: &SendTableProp) -> i32 {
        return if prop.flags.contains(PropFlag::Unsigned) {
            self.read_int(prop.num_bits.unwrap() )
        } else {
            self.read_signed_int(prop.num_bits.unwrap())
        }
    }

    pub fn decode_string(&mut self) -> String {
        let x = self.read_int(DT_MAX_STRING_BITS);
        return self.read_ascii_string(x);
    }

    pub fn decode_float(&mut self, prop: &mut SendTableProp) -> f32 {
        if prop.float_parse_type == FloatParseType::None {
            self.set_float_parse_type(prop);
        }

        return match prop.float_parse_type {
            FloatParseType::Standard        => self.read_standard_float(prop),
            FloatParseType::Coord           => self.read_bit_coord(),
            FloatParseType::BitCoordMp      => self.read_bit_coord_mp(prop),
            FloatParseType::BitCoordMpLp    => self.read_bit_coord_mp(prop),
            FloatParseType::BitCoordMpInt   => self.read_bit_coord_mp(prop),
            FloatParseType::NoScale         => self.read_float(32),
            FloatParseType::Normal          => self.read_bit_normal(),
            FloatParseType::BitCellChord    => self.read_bit_cell_coord(prop),
            FloatParseType::BitCellChordLp  => self.read_bit_cell_coord(prop),
            FloatParseType::BitCellChordInt => self.read_bit_cell_coord(prop),
            _ => panic!("error when matching float parse type")
        };
    }

    pub fn decode_vector_3(&mut self, prop: &mut SendTableProp) -> Vec3 {
        let mut ret_vec = Vec3::new();
        ret_vec.x = self.decode_float(prop);
        ret_vec.y = self.decode_float(prop);
        if prop.float_parse_type == FloatParseType::Normal {
            let sign = self.read_bool();
            let dist_sqr: f32 = ret_vec.x * ret_vec.x + ret_vec.y * ret_vec.y;
            if dist_sqr < 1.0 {
                ret_vec.z = (1.0 - dist_sqr).sqrt()
            } else {
                ret_vec.z = 0.0;
            }
            if sign { ret_vec.z = -ret_vec.z }
        } else {
            ret_vec.z = self.decode_float(prop);
        }

        return ret_vec;
    }

    pub fn decode_vector_2(&mut self, prop: &mut SendTableProp) -> Vec2 {
        return Vec2 { x: self.decode_float(prop), y: self.decode_float(prop) }
    }
}