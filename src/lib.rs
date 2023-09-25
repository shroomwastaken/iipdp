mod bitreader;
use std::any::Any;
use std::collections::HashMap;
use std::i32;
use bitreader::BitReader;
use hex;

// all information about the .dem file structure was taken from https://nekz.me/dem/demo.html

/*
    WARNING: BEAUTIFUL CODE AHEAD
*/

// takes a string of hex bytes (decodestr), value type (value_type);
// returns readable string (e.g. "484c3244454d4f" into "HL2DEMO");
fn decoder(decode_str: &String, value_type: &str) -> String {
    if decode_str != "Null" {
        if value_type == "string" {
            return String::from_utf8_lossy(&hex::decode(&decode_str).unwrap()).to_string();
        } else if value_type == "int4" {
            // this is for negative ints, dunno why this works ill be honest 
            if decode_str.contains("ffffff") {
                let byte_slice: &Vec<u8> = &decode_str
                    .as_bytes()
                    .chunks(2)
                    .map(|chunk| u8::from_str_radix(std::str::from_utf8(chunk).unwrap(), 16).unwrap())
                    .collect::<Vec<u8>>();
                let mut le_byte_slice = [0; 4];
                le_byte_slice.copy_from_slice(&byte_slice);
                return i32::from_le_bytes([le_byte_slice[0], le_byte_slice[1], le_byte_slice[2], le_byte_slice[3]]).to_string();
            } else {
                let byte_slice: Vec<u8> = hex::decode(&decode_str).unwrap();
                return i32::from_le_bytes([byte_slice[0], byte_slice[1], byte_slice[2], byte_slice[3]]).to_string();
            } 
        } else if value_type == "float" {
            let byte_slice: Vec<u8> = hex::decode(&decode_str).unwrap();
            let mut res_float: f32 = f32::from_bits(u32::from_le_bytes([byte_slice[0], byte_slice[1], byte_slice[2], byte_slice[3]]));

            // also adjusting for floating point stuff? it works so im not gonna touch it
            if res_float > 0.0 && res_float < 0.001 {
                res_float = 0.00;
            } else if res_float > 10000.0 || res_float < -10000.0 {
                res_float = 0.00;
            }

            return format!("{:.3}", res_float);
        } else if value_type == "byte" {
            return i8::from_le_bytes([hex::decode(&decode_str).unwrap()[0]]).to_string();
        } else if value_type == "end int" {
            // the last integer value in the file is 3 bytes instead of 4 for some reason ¯\_(ツ)_/¯
            let byte_slice: Vec<u8> = hex::decode(&decode_str).unwrap();
            return i32::from_le_bytes([byte_slice[0], byte_slice[1], byte_slice[2], 0]).to_string();
        } else if value_type == "bits_int" {
            return i32::from_str_radix(&decode_str, 2).unwrap().to_string();
        } else if value_type == "bits_short" {
            return (u16::from_str_radix(&decode_str, 2).unwrap() as i16).to_string();
        } else if value_type == "bits_float" {
            return format!("{:.3}", f32::from_bits(u32::from_str_radix(&decode_str, 2).unwrap()));
        } else if value_type == "bits_byte" {
            return i8::from_str_radix(&decode_str, 2).unwrap().to_string();
        }
        return "something went horribly wrong".to_string();
    }

    return "Null".to_string();
}

// takes file bytes (contents), start index (start), end index (end);
// returns String-ified bytes (every non-null character from contents[start] to contents[start + length] turned into a String);
fn get_byte_range_into_string(contents: &Vec<u8>, start: usize, length: usize, value_type: &str) -> String  {
    let mut decode_str: String = "".to_string();
    
    for i in start..start + length {
        if format!("{:x}", contents[i]).char_indices().count() == 1 {
            decode_str.push_str(&("0".to_owned() + &format!("{:x}", &contents[i])));
        } else if format!("{:x}", contents[i]).char_indices().count() == 2 {
            decode_str.push_str(&format!("{:x}", &contents[i]));
        }
    }

    let res: String = decoder(&decode_str, value_type).trim_end_matches("\0").to_string();

    // adjusts for weird floating point stuff
    if res == "0" && value_type == "float" {
        return "0.00".to_string();
    } else {
        return res;
    }
}

fn get_byte_range_into_binary(contents: &Vec<u8>, start: usize, length: usize) -> String {
    let mut res: String = "".to_string();

    for i in start..start + length {
        res.push_str(&format!("{:08b}", &contents[i]));
    }

    return res;
}

// takes file bytes (contents);
// returns HashMap<field name: &str, value: String> with header info;
pub fn get_header_info(contents: &Vec<u8>) -> HashMap<&str, String> {
    let mut header_info: HashMap<&str, String> = HashMap::new();

    header_info.insert("DemoFileStamp", get_byte_range_into_string(contents, 0, 8, "string"));
    header_info.insert("DemoProtocol", get_byte_range_into_string(contents, 8, 4, "int4"));
    header_info.insert("NetworkProtocol", get_byte_range_into_string(contents, 12, 4, "int4"));
    header_info.insert("ServerName", get_byte_range_into_string(contents, 16, 260, "string"));
    header_info.insert("ClientName", get_byte_range_into_string(contents, 276, 260, "string"));
    header_info.insert("MapName", get_byte_range_into_string(contents, 536, 260, "string"));
    header_info.insert("GameDirectory", get_byte_range_into_string(contents, 796, 260, "string"));
    header_info.insert("PlaybackTime", get_byte_range_into_string(contents, 1056, 4, "float"));
    header_info.insert("PlaybackTicks", get_byte_range_into_string(contents, 1060, 4, "int4"));
    header_info.insert("PlaybackFrames", get_byte_range_into_string(contents, 1064, 4, "int4"));
    header_info.insert("SignOnLength", get_byte_range_into_string(contents, 1068, 4, "int4"));

    return header_info;
}

// takes file bytes (contents);
// returns HashMap<msg_tick: i32, msg_data: Vec<field name: &str, value: Box<dyn Any>>>;
pub fn get_messages(contents: &Vec<u8>) -> HashMap<i32, Vec<HashMap<&str, Box<dyn Any>>>>{
    let mut cur_index: usize = 1072;

    let mut messages: HashMap<i32, Vec<HashMap<&str, Box<dyn Any>>>> = HashMap::new();

    loop {
        let cur_msg_type: usize = get_byte_range_into_string(contents, cur_index, 1, "byte").parse::<usize>().unwrap();
        cur_index += 1;

        if cur_msg_type == 7 {
            let cur_msg_tick: i32 = get_byte_range_into_string(contents, cur_index, 3, "end int").parse::<i32>().unwrap();
            let mut stop: HashMap<&str, Box<dyn Any>> = HashMap::new();
            stop.insert("Stop", Box::new("7"));
            stop.insert("Type", Box::new(cur_msg_type));
            messages.entry(cur_msg_tick).or_insert(Vec::new()).push(stop);
            break;
        }

        let cur_msg_tick: i32 = get_byte_range_into_string(contents, cur_index, 4, "int4").parse::<i32>().unwrap();
        cur_index += 4;

        if cur_msg_type == 1 {
            let packet_info: HashMap<&str, Box<dyn Any>> = read_packet_data(contents, cur_index, 1);
            cur_index += 88 + packet_info["Size"].downcast_ref::<usize>().unwrap();
            messages.entry(cur_msg_tick).or_insert(Vec::new()).push(packet_info);
        } else if cur_msg_type == 2 {
            let packet_info: HashMap<&str, Box<dyn Any>> = read_packet_data(contents, cur_index, 2);
            cur_index += 88 + packet_info["Size"].downcast_ref::<usize>().unwrap();
            messages.entry(cur_msg_tick).or_insert(Vec::new()).push(packet_info);
        } 
        else if cur_msg_type == 3 {
            let mut synctick: HashMap<&str, Box<dyn Any>> = HashMap::new();
            synctick.insert("Type", Box::new(cur_msg_type));
            synctick.insert("SyncTick", Box::new("3"));
            messages.entry(cur_msg_tick).or_insert(Vec::new()).push(synctick);

        } else if cur_msg_type == 4 {
            let packet_info: HashMap<&str, Box<dyn Any>> = read_packet_data(contents, cur_index, 4);
            cur_index += 4 + packet_info["Size"].downcast_ref::<usize>().unwrap();
            messages.entry(cur_msg_tick).or_insert(Vec::new()).push(packet_info);
        } else if cur_msg_type == 5 {
            let packet_info: HashMap<&str, Box<dyn Any>> = read_packet_data(contents, cur_index, 5);
            cur_index += 8 + packet_info["Size"].downcast_ref::<usize>().unwrap();
            messages.entry(cur_msg_tick).or_insert(Vec::new()).push(packet_info);
        } else if cur_msg_type == 6 {
            let packet_info: HashMap<&str, Box<dyn Any>> = read_packet_data(contents, cur_index, 6);
            cur_index += 4 + packet_info["Size"].downcast_ref::<usize>().unwrap();
            messages.entry(cur_msg_tick).or_insert(Vec::new()).push(packet_info);
        } else if cur_msg_type == 8 {
            let packet_info: HashMap<&str, Box<dyn Any>> = read_packet_data(contents, cur_index, 8);
            cur_index += 4 + packet_info["Size"].downcast_ref::<usize>().unwrap();
            messages.entry(cur_msg_tick).or_insert(Vec::new()).push(packet_info);
        }
    }

    return messages;
}

// takes file bytes (contents), start index (start), message type (msg_type);
// returns HashMap<field name: &str, value: Box<dyn Any>>;
fn read_packet_data(contents: &Vec<u8>, start: usize, msg_type: usize) -> HashMap<&str, Box<dyn Any>> {
    let mut msg_data: HashMap<&str, Box<dyn Any>> = HashMap::new();
    let mut cur_index: usize = start;

    msg_data.insert("Type", Box::new(msg_type));

    if msg_type == 1 || msg_type == 2 {
        msg_data.insert("CmdInfo", Box::new(read_cmd_info(contents, cur_index)));
        cur_index += 76;

        msg_data.insert("InSequence",Box::new(get_byte_range_into_string(contents, cur_index, 4, "int4")));
        cur_index += 4;

        msg_data.insert("OutSequence", Box::new(get_byte_range_into_string(contents, cur_index, 4, "int4")));
        cur_index += 4;

        let size: usize = get_byte_range_into_string(contents, cur_index, 4, "int4").parse().unwrap();
        msg_data.insert("Size", Box::new(size));
    } else if msg_type == 4 {
        let size: usize = get_byte_range_into_string(contents, cur_index, 4, "int4").parse().unwrap();
        msg_data.insert("Size", Box::new(size));
        cur_index += 4;
        
        msg_data.insert("Data", Box::new(get_byte_range_into_string(contents, cur_index, size, "string")));
    } else if msg_type == 5 {
        msg_data.insert("Cmd", Box::new(get_byte_range_into_string(contents, cur_index, 4, "int4")));
        cur_index += 4;

        let size: usize = get_byte_range_into_string(contents, cur_index, 4, "int4").parse().unwrap();
        msg_data.insert("Size", Box::new(size));
        cur_index += 4;

        msg_data.insert("UserCmdInfo", Box::new(read_user_cmd_info(contents, cur_index, size)));
    } else if msg_type == 6 {
        let size: usize = get_byte_range_into_string(contents, cur_index, 4, "int4").parse().unwrap();
        msg_data.insert("Size", Box::new(size));

        // TODO: add SendTable[] and ServerClassInfo[] readers, requires bitstream reader.
    } else if msg_type == 8 {
        let size: usize = get_byte_range_into_string(contents, cur_index, 4, "int4").parse().unwrap();
        msg_data.insert("Size", Box::new(size));

        // TODO: figure out the difference between CustomData and StringTables, for now this assumes that this is the StringTables
        // message type because parsing only works for old engine demos (i think). Also add a bitstream reader.
    }

    return msg_data;
}

// takes file bytes (contents), start index (start);
// returns HashMap<field name: String, value: Box<dyn Any>>;
fn read_cmd_info(contents: &Vec<u8>, start: usize) -> HashMap<String, Box<dyn Any>> {
    let mut cmd_info: HashMap<String, Box<dyn Any>> = HashMap::new();
    let mut cur_index: usize = start;

    let names: [&str; 6] = ["ViewOrigin", "ViewAngles", "LocalViewAngles", "ViewOrigin2", "ViewAngles2", "LocalViewAngles2"];

    let flags: String = get_byte_range_into_string(contents, cur_index, 4, "int4");
    cmd_info.insert("Flags".to_string().to_owned(), Box::new(flags));
    cur_index += 4;

    for i in 0..6 {
        let mut cur_arr: Vec<String> = Vec::new();
        for _j in 0..3 {
            cur_arr.push(get_byte_range_into_string(contents, cur_index, 4, "float"));
            cur_index += 4;
        }
        cmd_info.insert(names[i].to_string(), Box::new(cur_arr.to_owned()));
    }

    return cmd_info;
}

fn read_user_cmd_info(contents: &Vec<u8>, start: usize, length: usize) -> HashMap<String, Box<dyn Any>>{
    let mut user_cmd_info: HashMap<String, Box<dyn Any>> = HashMap::new();
    let binary_data: String = get_byte_range_into_binary(contents, start, length);

    let mut reader = BitReader { bit_str: binary_data };
    reader.init();

    user_cmd_info.insert("CommandNumber".to_string(), Box::new(decoder(&reader.read_x_if_exists(32), "bits_int")));
    user_cmd_info.insert("TickCount".to_string(), Box::new(decoder(&reader.read_x_if_exists(32), "bits_int")));
    
    user_cmd_info.insert("ViewAnglesX".to_string(), Box::new(decoder(&reader.read_x_if_exists(32), "bits_float")));
    user_cmd_info.insert("ViewAnglesY".to_string(), Box::new(decoder(&reader.read_x_if_exists(32), "bits_float")));
    user_cmd_info.insert("ViewAnglesZ".to_string(), Box::new(decoder(&reader.read_x_if_exists(32), "bits_float")));
    
    user_cmd_info.insert("ForwardMove".to_string(), Box::new(decoder(&reader.read_x_if_exists(32), "bits_float")));
    user_cmd_info.insert("SideMove".to_string(), Box::new(decoder(&reader.read_x_if_exists(32), "bits_float")));
    user_cmd_info.insert("UpMove".to_string(), Box::new(decoder(&reader.read_x_if_exists(32), "bits_float")));

    user_cmd_info.insert("Buttons".to_string(), Box::new(decoder(&reader.read_x_if_exists(32), "bits_int")));

    user_cmd_info.insert("Impulse".to_string(), Box::new(decoder(&reader.read_x_if_exists(8), "bits_byte")));

    user_cmd_info.insert("WeaponSelect".to_string(), Box::new(decoder(&reader.read_x_if_exists(11), "bits_int")));

    if user_cmd_info["WeaponSelect"].downcast_ref::<String>().unwrap() == &"Null".to_string() {
        user_cmd_info.insert("WeaponSubtype".to_string(), Box::new("Null".to_string()));
    } else {
        user_cmd_info.insert("WeaponSubtype".to_string(), Box::new(decoder(&reader.read_bits(6), "bits_int")));
    }

    user_cmd_info.insert("MouseDx".to_string(), Box::new(decoder(&reader.read_x_if_exists(16), "bits_short")));
    user_cmd_info.insert("MouseDy".to_string(), Box::new(decoder(&reader.read_x_if_exists(16), "bits_short")));

    return user_cmd_info;
}
