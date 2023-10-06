use crate::bitreader::BitReader;

pub struct CmdInfo {
    pub flags: i32,
    pub view_origin: Vec<f32>,
    pub view_angles: Vec<f32>,
    pub local_view_angles: Vec<f32>,
    pub view_origin2: Vec<f32>,
    pub view_angles2: Vec<f32>,
    pub local_view_angles2: Vec<f32>,
}

impl CmdInfo {
    pub fn new() -> Self {
        Self {
            flags: 0,
            view_origin: vec![],
            view_angles: vec![],
            local_view_angles: vec![],
            view_origin2: vec![],
            view_angles2: vec![],
            local_view_angles2: vec![]
        }
    }

    pub fn parse(reader: &mut BitReader) -> Self {
        let mut cmd_info: CmdInfo = CmdInfo::new();
    
        cmd_info.flags = reader.read_int(32);
    
        for _j in 0..3 {
            cmd_info.view_origin.push(reader.read_float(32));
        }
    
        for _j in 0..3 {
            cmd_info.view_angles.push(reader.read_float(32));
        }
    
        for _j in 0..3 {
            cmd_info.local_view_angles.push(reader.read_float(32));
        }
    
        for _j in 0..3 {
            cmd_info.view_origin2.push(reader.read_float(32));
        }
    
        for _j in 0..3 {
            cmd_info.view_angles2.push(reader.read_float(32));
        }
    
        for _j in 0..3 {
            cmd_info.local_view_angles2.push(reader.read_float(32));
        }
    
        return cmd_info;
    }
}