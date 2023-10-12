use std::collections::HashMap;
use crate::bitreader::BitReader;
use crate::structs::utils::Vec3;

// all of the structs and parsing
pub struct AchievementEvent {
    achievement_id: i32,
}

impl AchievementEvent {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self { achievement_id: reader.read_int(32) }
    }
}

pub struct Battery {
    battery_val: i32,
}

impl Battery {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self { battery_val: reader.read_int(16) }
    }
}

pub struct CloseCaption {
    token_name: String,
    duration: f32,
    flags: CloseCaptionFlags,
}

impl CloseCaption {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self {
            token_name: reader.read_ascii_string_nulled(),
            duration: reader.read_int(16) as f32 * 0.1,
            flags: CloseCaptionFlags::from_i32(reader.read_int(8)),
        }
    }
}

pub struct Damage {
    armor: i32,
    damage_taken: i32,
    visible_bits_damage: DamageType,
    vec_from: Vec3,
}

impl Damage {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self {
            armor: reader.read_int(8),
            damage_taken: reader.read_int(8),
            visible_bits_damage: DamageType::from_i32(reader.read_int(32)),
            vec_from: reader.read_vec3(),
        }
    }
}

pub struct EmptyUserMessage;

pub struct EntityPortalled {
    portal: i32, // these two are actually EHandles (whatever that is)
    portalled: i32, // ill just read them as ints for now
    new_position: Vec3,
    new_angles: Vec3,
}

impl EntityPortalled {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self {
            portal: reader.read_int(32),
            portalled: reader.read_int(32),
            new_position: reader.read_vec3(),
            new_angles: reader.read_vec3(),
        }
    }
}

pub struct Fade {
    duration: f32,
    hold_time: i32,
    flags: i32,
    r: i32,
    g: i32,
    b: i32,
    a: i32,
}

impl Fade {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self {
            duration: reader.read_int(16) as f32 / (1 << 9) as f32,
            hold_time: reader.read_int(16),
            flags: reader.read_int(16),
            r: reader.read_int(8),
            g: reader.read_int(8),
            b: reader.read_int(8),
            a: reader.read_int(8),
        }
    }
}

pub struct Geiger {
    geiger_range: i32,
}

impl Geiger {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self { geiger_range: reader.read_int(8) }
    }
}

pub struct HudMsg; // dunno how to parse yet

pub struct HudText {
    string: String,
}

impl HudText {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self { string: reader.read_ascii_string_nulled() } // this has a defined length but im pretty sure it should be null terminated
    }
}

pub struct KeyHintText {
    count: i32, // should always be 1
    key_string: String,
}

impl KeyHintText {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self { count: reader.read_int(8), key_string: reader.read_ascii_string_nulled() }
    }
}

pub struct KillCam {
    spec_mode: i32,
    target1: i32,
    target: i32,
    unknown: i32,
}

impl KillCam {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self {
            spec_mode: reader.read_int(8),
            target1: reader.read_int(8),
            target: reader.read_int(8),
            unknown: reader.read_int(8),
        }
    }
}

pub struct LogoTimeMsg {
    time: f32,
}

impl LogoTimeMsg {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self { time: reader.read_float(32) }
    }
}

pub struct MpMapCompleted {
    branch: i32,
    level: i32,
}

impl MpMapCompleted {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self { branch: reader.read_int(8), level: reader.read_int(8) }
    }
}

pub struct MpMapCompletedData; // no clue

pub struct MpTauntEarned {
    taunt_name: String,
    award_silently: bool,
}

impl MpTauntEarned {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self { taunt_name: reader.read_ascii_string_nulled(), award_silently: reader.read_bool() }
    }
}

pub struct MpTauntLocked {
    taunt_name: String,
}

impl MpTauntLocked {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self { taunt_name: reader.read_ascii_string_nulled() }
    }
}

pub struct PaintEntity {
    ent: i32, // also an ehandle
    paint_type: i32,
    pos: Vec3,
}

impl PaintEntity {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self {
            ent: reader.read_int(32),
            paint_type: reader.read_int(8),
            pos: reader.read_vec3(),
        }
    }
}

pub struct PaintWorld {
    paint_type: i32,
    ehandle: i32,
    unkhf1: f32, // again, no clue
    unkhf2: f32,
    length: i32,
    center: Vec3,
    positions: Vec<Vec3>,
}

impl PaintWorld {
    pub fn parse(reader: &mut BitReader) -> Self {
        let paint_type = reader.read_int(8);
        let ehandle = reader.read_int(32);
        let unkhf1 = reader.read_float(32);
        let unkhf2 = reader.read_float(32);
        let length = reader.read_int(8);
        let center = reader.read_vec3();
        let mut positions: Vec<Vec3> = Vec::new();
        for _ in 0..length {
            positions.push(center.add_vec_int(vec![reader.read_int(16), reader.read_int(16), reader.read_int(16)]));
        }

        Self {
            paint_type: paint_type,
            ehandle: ehandle,
            unkhf1: unkhf1,
            unkhf2: unkhf2,
            length: length,
            center: center,
            positions: positions,
        }
    }
}

// not gonna implement this yet because im not sure what the last 2 values are (theyre not vec3 thats a placeholder)
pub struct PortalFxSurface {
    portal_ent: i32,
    owner_ent: i32,
    team: i32,
    portal_num: i32,
    effect: i32,
    origin: Vec3,
    angles: Vec3,
}

pub struct ResetHUD {
    unknown: i32,
}

impl ResetHUD {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self { unknown: reader.read_int(8) }
    }
}

pub struct Rumble {
    rumble_type: i32,
    scale: f32,
    rumble_flags: i32,
}

impl Rumble {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self {
            rumble_type: reader.read_int(8),
            scale: reader.read_int(8) as f32 / 100.0,
            rumble_flags: reader.read_int(8),
        }
    }
}

pub struct SayText {
    client_id: i32,
    text: String,
    wants_to_chat: bool,
}

impl SayText {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self {
            client_id: reader.read_int(8),
            text: reader.read_ascii_string_nulled(),
            wants_to_chat: reader.read_int(8) != 0,
        }
    }
}

pub struct SayText2 {
    client: i32,
    wants_to_chat: bool,
    msg_name: String,
    msgs: Vec<String>,
}

impl SayText2 {
    pub fn parse(reader: &mut BitReader) -> Self {
        let client = reader.read_int(8);
        let wants_to_chat = reader.read_int(8) != 0;
        let msg_name = reader.read_ascii_string_nulled();
        let mut msgs: Vec<String> = Vec::new();

        for _ in 0..4 {
            msgs.push(reader.read_ascii_string_nulled());
        }

        Self { client: client, wants_to_chat: wants_to_chat, msg_name: msg_name, msgs: msgs }
    }
}

pub struct ScoreboardTempUpdate {
    num_portals: i32,
    time_taken: i32, // centi-seconds
}

impl ScoreboardTempUpdate {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self { num_portals: reader.read_int(32), time_taken: reader.read_int(32) }
    }
}

pub struct Shake {
    command: i32,
    amplitude: f32,
    frequency: f32,
    duration: f32,
}

impl Shake {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self {
            command: reader.read_int(8),
            amplitude: reader.read_float(32),
            frequency: reader.read_float(32),
            duration: reader.read_float(32),
        }
    }
}

pub struct TextMsg {
    destination: i32,
    messages: Vec<String>,
}

impl TextMsg {
    pub fn parse(reader: &mut BitReader) -> Self {
        let destination = reader.read_int(8);
        let mut messages: Vec<String> = Vec::new();
    
        for _ in 0..4 {
            messages.push(reader.read_ascii_string_nulled());
        }

        Self { destination: destination, messages: messages }
    }
}

pub struct Train {
    pos: i32,
}

impl Train {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self { pos: reader.read_int(8) }
    }
}

pub struct TransitionFade {
    seconds: f32,
}

impl TransitionFade {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self { seconds: reader.read_float(32) }
    }
}

pub struct UnknownUserMessage;

pub struct VguiMenu {
    message: String,
    show: bool,
    count: i32,
    key_values: Vec<HashMap<String, String>>,
}

impl VguiMenu {
    pub fn parse(reader: &mut BitReader) -> Self {
        let message = reader.read_ascii_string_nulled();
        let show = reader.read_int(8) != 0;
        let count = reader.read_int(8);
        let mut key_values: Vec<HashMap<String, String>> = Vec::new();

        for _ in 0..count {
            let mut cur_pair: HashMap<String, String> = HashMap::new();
            cur_pair.insert(reader.read_ascii_string_nulled(), reader.read_ascii_string_nulled());
            key_values.push(cur_pair);
        }

        Self { message: message, show: show, count: count, key_values: key_values }
    }
}


// for VoiceMask user message
pub struct PlayerMask {
    pub game_rules_mask: i32,
    pub ban_mask: i32,
}

pub struct VoiceMask {
    voice_max_players: i32,
    player_masks: Vec<PlayerMask>,
    player_mod_enable: bool,
}

impl VoiceMask {
    pub fn parse(reader: &mut BitReader) -> Self {
        let voice_max_players: i32 = 2; // const for p1 and p2 at least
        let mut player_masks: Vec<PlayerMask> = Vec::new();
        for _ in 0..voice_max_players {
            player_masks.push(PlayerMask { game_rules_mask: reader.read_int(32), ban_mask: reader.read_int(32) })
        }
        let player_mod_enable = reader.read_int(8) != 0;

        Self { voice_max_players: voice_max_players, player_masks: player_masks, player_mod_enable: player_mod_enable }
    }
}

pub struct WitchBloodSplatter; // no clue how ReadVectorCoords() works in untitledparser so im not gonna parse this one yet

// enums (various flags and types)

#[derive(Debug)]
pub enum CloseCaptionFlags {
    None, 
    WarnIfMissing,
    FromPlayer,
    GenderMale,
    GenderFemale,
}

impl CloseCaptionFlags {
    pub fn from_i32(val: i32) -> Self {
        match val {
            0 => Self::None,
            1 => Self::WarnIfMissing,
            2 => Self::FromPlayer,
            4 => Self::GenderMale,
            8 => Self::GenderFemale,
            _ => Self::None, // not gonna happen (probably) so ill just use None again
        }
    } 
}

#[derive(Debug)]
pub enum DamageType {
    None = -1,
    DmgGeneric             = 0,
    DmgCrush               = 1 << 0,
    DmgBullet              = 1 << 1,
    DmgSlash               = 1 << 2,
    DmgBurn                = 1 << 3,
    DmgVehicle             = 1 << 4,
    DmgFall                = 1 << 5,
    DmgBlast               = 1 << 6,
    DmgClub                = 1 << 7,
    DmgShock               = 1 << 8,
    DmgSonic               = 1 << 9,
    DmgEnergyBeam          = 1 << 10,
    DmgPreventPhysicsForce = 1 << 11,
    DmgNeverGib            = 1 << 12,
    DmgAlwaysGib           = 1 << 13,
    DmgDrown               = 1 << 14,
    DmgParalyze            = 1 << 15,
    DmgNerveGas            = 1 << 16,
    DmgPoison              = 1 << 17,
    DmgRadiation           = 1 << 18,
    DmgDrownRecover        = 1 << 19,
    DmgAcid                = 1 << 20,
    DmgSlowBurn            = 1 << 21,
    DmgRemoveNoRagdoll     = 1 << 22,
    DmgPhysGun             = 1 << 23,
    DmgPlasma              = 1 << 24,
    DmgAirboat             = 1 << 25,
    DmgDissolve            = 1 << 26,
    DmgBlastSurface        = 1 << 27,
    DmgDirect              = 1 << 28,
    DmgBuckshot            = 1 << 29,
}

// dont ask
impl DamageType {
    pub fn from_i32(val: i32) -> Self {
        match val {
            0 => Self::DmgGeneric,
            1 => Self::DmgCrush,
            2  => Self::DmgBullet,
            4  => Self::DmgSlash,
            8  => Self::DmgBurn,
            16 => Self::DmgVehicle,
            32 => Self::DmgFall,
            64 => Self::DmgBlast,
            128 => Self::DmgClub,
            256 => Self::DmgShock,
            512 => Self::DmgSonic,
            1024=> Self::DmgEnergyBeam,
            2048 => Self::DmgPreventPhysicsForce,
            4096 => Self::DmgNeverGib,
            8192 => Self::DmgAlwaysGib,
            16384 => Self::DmgDrown,
            32768 => Self::DmgParalyze,
            65536 => Self::DmgNerveGas,
            131072 => Self::DmgPoison,
            262144 => Self::DmgRadiation,
            524288 => Self::DmgDrownRecover,
            1048576 => Self::DmgAcid,
            2097152 => Self::DmgSlowBurn,
            4194304 => Self::DmgRemoveNoRagdoll,
            8388608 => Self::DmgPhysGun,
            16777216 => Self::DmgPlasma,           
            33554432 => Self::DmgAirboat,     
            67108864 => Self::DmgDissolve,
            134217728 => Self::DmgBlastSurface,
            268435456 => Self::DmgDirect,
            536870912 => Self::DmgBuckshot,
            _ => Self::None,
        }   
    }
}
