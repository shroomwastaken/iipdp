use hex;

fn decoder(decodestr: &String) -> String {
    return String::from_utf8(hex::decode(decodestr).unwrap()).unwrap();
}

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

pub fn get_client_name(contents: &Vec<u8>) -> String {
    let mut decodestr: String = "".to_string();

    for i in 276..288 {
        if contents[i] != 0 {
            decodestr.push_str(&format!("{:x}", &contents[i]));
        } else {
            println!("zeroed out");
        }
    }

    return decoder(&decodestr);
}