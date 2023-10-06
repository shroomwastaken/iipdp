use crate::bitreader::BitReader;

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

    pub fn parse(&mut self, reader: &mut BitReader) {
        self.demo_file_stamp = reader.read_ascii_string(64);
        self.demo_protocol = reader.read_int(32);
        self.network_protocol = reader.read_int(32);
        self.server_name = reader.read_ascii_string(2080);
        self.client_name = reader.read_ascii_string(2080);
        self.map_name = reader.read_ascii_string(2080);
        self.game_directory = reader.read_ascii_string(2080);
        self.playback_time = reader.read_float(32);
        self.playback_ticks = reader.read_int(32);
        self.playback_frames = reader.read_int(32);
        self.sign_on_length = reader.read_int(32);
    }
}
