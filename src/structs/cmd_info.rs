use crate::bitreader::BitReader;
use crate::structs::utils::Vec3;

// cmdinfo is present in every Packet packet and contains information about
// where the player is looking and what their position is
// there are MSSC amount of these in every Packet packet
// MSSC being MaxSplitScreenClients, which is 1 for p1, 2 for p2 and 4(?) for l4d games

#[derive(Debug)]
pub struct CmdInfo {
    pub flags: InterpFlags,
    pub view_origin: Vec3,
    pub view_angles: Vec3,
    pub local_view_angles: Vec3,
    pub view_origin2: Vec3,
    pub view_angles2: Vec3,
    pub local_view_angles2: Vec3,
}

impl CmdInfo {
    pub fn new() -> Self {
        Self {
            flags: InterpFlags::None,
            view_origin: Vec3::new(),
            view_angles: Vec3::new(),
            local_view_angles: Vec3::new(),
            view_origin2: Vec3::new(),
            view_angles2: Vec3::new(),
            local_view_angles2: Vec3::new()
        }
    }

    pub fn parse(reader: &mut BitReader) -> Self {
        let mut cmd_info: CmdInfo = CmdInfo::new();
    
        cmd_info.flags = InterpFlags::from_bits_truncate(reader.read_int(32));
    
        cmd_info.view_origin = reader.read_vec3();
        cmd_info.view_angles = reader.read_vec3();
        cmd_info.local_view_angles = reader.read_vec3();
        cmd_info.view_origin2 = reader.read_vec3();
        cmd_info.view_angles2 = reader.read_vec3();
        cmd_info.local_view_angles2 = reader.read_vec3();
    
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