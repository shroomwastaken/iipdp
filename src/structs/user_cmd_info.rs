use bitflags::bitflags;

use crate::bitreader::BitReader;

// has info about the players view angles, how much they moved their mouse
// what buttons they pressed etc.
// dont know what the difference is between view angles here and in cmdinfo
// all of the values in UserCmdInfo are Optional, meaning that the value exists only if the bit before its supposed value is 1; 

#[derive(Debug)]
pub struct UserCmdInfo {
    pub command_number: Option<i32>,
    pub tick_count: Option<i32>,
    pub view_angles_x: Option<f32>,
    pub view_angles_y: Option<f32>,
    pub view_angles_z: Option<f32>,
    pub forward_move: Option<f32>,
    pub side_move: Option<f32>,
    pub up_move: Option<f32>,
    pub buttons: Buttons,
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
            buttons: Buttons::None,
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
        let buttons: Buttons = Buttons::from_bits_truncate(reader.read_int_if_exists(32).unwrap_or(0));
        let impulse: Option<i32> = reader.read_int_if_exists(8);
        let weapon_select: Option<i32> = reader.read_int_if_exists(11);
        let mut weapon_subtype: Option<i32> = None;

        if weapon_select != None { weapon_subtype = Some(reader.read_int(6)) }

        let mouse_dx: Option<i32> = reader.read_signed_int_if_exists(16);
        let mouse_dy: Option<i32> = reader.read_signed_int_if_exists(16);

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

// thanks untitledparser :)
bitflags! {
    #[derive(Debug)]
    pub struct Buttons : i32 {
        const None            = 0;
		const Attack          = 1;
		const Jump            = 1 << 1;
		const Duck            = 1 << 2;
		const Forward         = 1 << 3;
		const Back            = 1 << 4;
		const Use             = 1 << 5;
		const Cancel          = 1 << 6;
		const Left            = 1 << 7;
		const Right           = 1 << 8;
		const MoveLeft        = 1 << 9;
		const MoveRight       = 1 << 10;
		const Attack2         = 1 << 11;
		const Run             = 1 << 12;
		const Reload          = 1 << 13;
		const Alt1            = 1 << 14;
		const Alt2            = 1 << 15;
		const Score           = 1 << 16;
		const Speed           = 1 << 17;
		const Walk            = 1 << 18;
		const Zoom            = 1 << 19;
		const Weapon1         = 1 << 20;
		const Weapon2         = 1 << 21;
		const BullRush        = 1 << 22;
		const Grenade1        = 1 << 23;
		const Grenade2        = 1 << 24;
		const LookSpin        = 1 << 25;
		const CurrentAbility  = 1 << 26;
		const PreviousAbility = 1 << 27;
		const Ability1        = 1 << 28;
		const Ability2        = 1 << 29;
		const Ability3        = 1 << 30;
		const Ability4        = 1 << 31;
    }
}