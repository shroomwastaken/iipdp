use crate::bitreader::BitReader;

// the first 1072 bytes of the demo contain the "header" which
// contatins all of the important info about the demo
// for example we infer information about the games version with the network protocol
// this is the stuff that gets printed out to the console when the user parses the demo without the -dump option

pub struct DemoHeader {
    pub demo_file_stamp: String,
    pub demo_protocol: i32,
    pub network_protocol: i32,
    pub server_name: String,
    pub client_name: String,
    pub map_name: String,
    pub game_directory: String,
    pub playback_time: f32,
    pub playback_ticks: i32,
    pub playback_frames: i32,
    pub sign_on_length: i32,
}

impl DemoHeader {
    pub fn new() -> Self {
        Self { 
            demo_file_stamp: "".to_string(),
            demo_protocol: 0,
            network_protocol: 0,
            server_name: "".to_string(),
            client_name: "".to_string(),
            map_name: "".to_string(),
            game_directory: "".to_string(),
            playback_time: 0f32,
            playback_ticks: 0,
            playback_frames: 0,
            sign_on_length: 0,
        }
    }
    
    pub fn parse(reader: &mut BitReader) -> Self {
        Self { 
            demo_file_stamp: reader.read_ascii_string(64),
            demo_protocol: reader.read_int(32),
            network_protocol: reader.read_int(32),
            server_name: reader.read_ascii_string(2080),
            client_name: reader.read_ascii_string(2080),
            map_name: reader.read_ascii_string(2080),
            game_directory: reader.read_ascii_string(2080),
            playback_time: reader.read_float(32),
            playback_ticks: reader.read_int(32),
            playback_frames: reader.read_int(32),
            sign_on_length: reader.read_int(32),
        }
    }
}
