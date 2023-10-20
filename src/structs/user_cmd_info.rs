use crate::bitreader::BitReader;

// has info about the players view angles, how much they moved their mouse
// what buttons they pressed etc.
// dont know what the difference is between view angles here and in cmdinfo
// all of the values in UserCmdInfo are Optional, meaning that the value exists only if the bit before its supposed value is 1; 

pub struct UserCmdInfo {
    pub command_number: Option<i32>,
    pub tick_count: Option<i32>,
    pub view_angles_x: Option<f32>,
    pub view_angles_y: Option<f32>,
    pub view_angles_z: Option<f32>,
    pub forward_move: Option<f32>,
    pub side_move: Option<f32>,
    pub up_move: Option<f32>,
    pub buttons: Option<i32>,
    pub impulse: Option<i32>,
    pub weapon_select: Option<i32>,
    pub weapon_subtype: Option<i32>,
    pub mouse_dx: Option<i32>,
    pub mouse_dy: Option<i32>,    
}

impl UserCmdInfo {
    pub fn new() -> Self {
        Self { 
            command_number: None,
            tick_count: None,
            view_angles_x: None,
            view_angles_y: None,
            view_angles_z: None,
            forward_move: None,
            side_move: None,
            up_move: None,
            buttons: None,
            impulse: None,
            weapon_select: None,
            weapon_subtype: None,
            mouse_dx: None,
            mouse_dy: None,
        }
    }

    // this used to be some really bad code but its fine now
    pub fn parse(reader: &mut BitReader) -> Self {
        let command_number: Option<i32> = reader.read_int_if_exists(32);
        let tick_count: Option<i32> = reader.read_int_if_exists(32);
        let view_angles_x: Option<f32> = reader.read_float_if_exists(32);
        let view_angles_y: Option<f32> = reader.read_float_if_exists(32);
        let view_angles_z: Option<f32> = reader.read_float_if_exists(32);
        let forward_move: Option<f32> = reader.read_float_if_exists(32);
        let side_move: Option<f32> = reader.read_float_if_exists(32);
        let up_move: Option<f32> = reader.read_float_if_exists(32);
        let buttons: Option<i32> = reader.read_int_if_exists(32);
        let impulse: Option<i32> = reader.read_int_if_exists(8);
        let weapon_select: Option<i32> = reader.read_int_if_exists(11);
        let mut weapon_subtype: Option<i32> = None;

        if weapon_select != None { weapon_subtype = Some(reader.read_int(6)) }

        let mouse_dx: Option<i32> = reader.read_int_if_exists(16);
        let mouse_dy: Option<i32> = reader.read_int_if_exists(16);

        Self {
            command_number: command_number,
            tick_count: tick_count,
            view_angles_x: view_angles_x,
            view_angles_y: view_angles_y,
            view_angles_z: view_angles_z,
            forward_move: forward_move,
            side_move: side_move,
            up_move: up_move,
            buttons: buttons,
            impulse: impulse,
            weapon_select: weapon_select,
            weapon_subtype: weapon_subtype,
            mouse_dx: mouse_dx,
            mouse_dy: mouse_dy,
        }
    }
}