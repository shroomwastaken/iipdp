use hex;

// takes a string of hex bytes (decodestr); returns readable string (e.g. "484c3244454d4f" into "HL2DEMO")
fn decoder(decodestr: &String) -> String {
    return String::from_utf8(hex::decode(decodestr).unwrap()).unwrap();
}

// takes file bytes (contents); returns true if the first 7 bytes == "HL2DEMO"
pub fn recognize_demo(contents: &Vec<u8>) -> bool {
    let mut decodestr: String = "".to_string();
    
    for i in  0..7 {
        decodestr.push_str(&format!("{:x}", &contents[i]));
    }

    if decoder(&decodestr) == "HL2DEMO" {
        return true;
    }
    
    return false;
}

// takes file bytes (contents); returns client name (every non-null character from 276 to 349 turned into a String)
pub fn get_client_name(contents: &Vec<u8>) -> String {
    let mut decodestr: String = "".to_string();

    for i in 276..350 { // assuming steam name is less than 74 characters long
        if contents[i] != 0 {
            decodestr.push_str(&format!("{:x}", &contents[i]));
        }
    }

    return decoder(&decodestr);
}