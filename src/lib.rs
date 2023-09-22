use std::collections::HashMap;
use std::i32;
use std::u32;
use unicode_segmentation::UnicodeSegmentation;
use hex;

/*
    WARNING: BEAUTIFUL CODE AHEAD
*/

// takes a string of hex bytes (decodestr), value type (value_type);
// returns readable string (e.g. "484c3244454d4f" into "HL2DEMO");
fn decoder(decode_str: &String, value_type: &str) -> String {
    if value_type == "string" {
        return String::from_utf8_lossy(&hex::decode(&decode_str).unwrap()).to_string();
    } else if value_type == "int" {
        if decode_str.contains("ffffff") {
            let byte_slice = &decode_str
                .as_bytes()
                .chunks(2)
                .map(|chunk| u8::from_str_radix(std::str::from_utf8(chunk).unwrap(), 16).unwrap())
                .collect::<Vec<u8>>();
            let mut le_byte_slice = [0; 4];
            le_byte_slice.copy_from_slice(&byte_slice);
            return i32::from_le_bytes([le_byte_slice[0], le_byte_slice[1], le_byte_slice[2], le_byte_slice[3]]).to_string();
        } else {
            let byte_slice = hex::decode(&decode_str).unwrap();
            return i32::from_le_bytes([byte_slice[0], byte_slice[1], byte_slice[2], byte_slice[3]]).to_string();
        }
        
    } else if value_type == "float" {
        let byte_slice = hex::decode(&decode_str).unwrap();
        return f32::from_bits(u32::from_le_bytes([byte_slice[0], byte_slice[1], byte_slice[2], byte_slice[3]])).to_string();
    }
    return "something went horribly wrong".to_string();
}

// takes file bytes (contents), start index (start), end index (end);
// returns String-ified bytes (every non-null character from contents[start] to contents[end] turned into a String);
fn get_byte_range_into_string(contents: &Vec<u8>, start: usize, length: usize, value_type: &str) -> String  {
    let mut decode_str: String = "".to_string();
    
    for i in start..start + length {
        if format!("{:x}", contents[i]).graphemes(true).count()== 1 {
            decode_str.push_str(&("0".to_owned() + &format!("{:x}", &contents[i])));
        } else if format!("{:x}", contents[i]).graphemes(true).count() == 2 {
            decode_str.push_str(&format!("{:x}", &contents[i]));
        }
    }

    return decoder(&decode_str, value_type).trim_end_matches("\0").to_string();
}

// takes file bytes (contents);
// returns true if the first 7 bytes == "HL2DEMO";
pub fn recognize_demo(contents: &Vec<u8>) -> bool {
    return get_byte_range_into_string(contents, 0, 8, "string") == "HL2DEMO";
}

pub fn get_header_info(contents: &Vec<u8>) -> HashMap<&str, String> {
    let mut header_info: HashMap<&str, String> = HashMap::new();

    header_info.insert("DemoFileStamp", get_byte_range_into_string(contents, 0, 8, "string"));
    header_info.insert("DemoProtocol", get_byte_range_into_string(contents, 8, 4, "int"));
    header_info.insert("NetworkProtocol", get_byte_range_into_string(contents, 12, 4, "int"));
    header_info.insert("ServerName", get_byte_range_into_string(contents, 16, 260, "string"));
    header_info.insert("ClientName", get_byte_range_into_string(contents, 276, 260, "string"));
    header_info.insert("MapName", get_byte_range_into_string(contents, 536, 260, "string"));
    header_info.insert("GameDirectory", get_byte_range_into_string(contents, 796, 260, "string"));
    header_info.insert("PlaybackTime", get_byte_range_into_string(contents, 1056, 4, "float"));
    header_info.insert("PlaybackTicks", get_byte_range_into_string(contents, 1060, 4, "int"));
    header_info.insert("PlaybackFrames", get_byte_range_into_string(contents, 1064, 4, "int"));
    header_info.insert("SignOnLength", get_byte_range_into_string(contents, 1068, 4, "int"));

    return header_info;
}