use crate::bitreader::BitReader;

// cmdinfo is present in every Packet packet and contains information about
// where the player is looking and what their position is
// there are MSSC amount of these in every Packet packet
// MSSC being MaxSplitScreenClients, which is 1 for p1, 2 for p2 and 4(?) for l4d games

#[derive(Debug)]
pub struct CmdInfo {
    pub flags: InterpFlags,
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
            flags: InterpFlags::None,
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
    
        cmd_info.flags = InterpFlags::from_bits_truncate(reader.read_int(32));
    
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

bitflags::bitflags! {
    #[derive(Debug)]
    pub struct InterpFlags : i32 {
        const None = 0;
        const UseOrigin2 = 1;
        const UseAngles2 = 1 << 1;
        const NoInterp = 1 << 2; // don't interpolate between this and last view
    }
}