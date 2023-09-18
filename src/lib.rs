use hex;

// takes a string of hex bytes (decodestr);
// returns readable string (e.g. "484c3244454d4f" into "HL2DEMO");
fn decoder(decodestr: &String) -> String {
    return String::from_utf8(hex::decode(decodestr).unwrap()).unwrap();
}

// takes file bytes (contents), start index (start), end index (end);
// returns String-ified bytes (every non-null character from contents[start] to contents[end] turned into a String);
fn get_byte_range_into_string(contents: &Vec<u8>, start: usize, end: usize) -> String  {
    let mut decode_str: String = "".to_string();
    
    for i in  start..end + 1 {
        if contents[i] != 0 {
            decode_str.push_str(&format!("{:x}", &contents[i]));
        } 
    }

    return decoder(&decode_str);
}

// takes file bytes (contents);
// returns true if the first 7 bytes == "HL2DEMO";
pub fn recognize_demo(contents: &Vec<u8>) -> bool {
    return get_byte_range_into_string(contents, 0, 6) == "HL2DEMO";
}

// takes file bytes (contents);
// returns client name (assuming client name length <= 50 characters);
pub fn get_client_name(contents: &Vec<u8>) -> String {
    return get_byte_range_into_string(contents, 276, 326);
}

// takes file bytes (contents);
// returns map name (maximum map name length is 13 characters (testchmb_a_XX));
pub fn get_map_name(contents: &Vec<u8>) -> String {
    return get_byte_range_into_string(contents, 536, 548);
}