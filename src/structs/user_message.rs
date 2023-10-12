use std::collections::HashMap;

use crate::bitreader::BitReader;

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
    flags: i32,
}

impl CloseCaption {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self {
            token_name: reader.read_ascii_string_nulled(),
            duration: reader.read_int(16) as f32 * 0.1,
            flags: reader.read_int(8),
        }
    }
}

pub struct Damage {
    armor: i32,
    damage_taken: i32,
    visible_bits_damage: i32,
    vec_from: Vec<f32>, // again assuming this is f32
}

impl Damage {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self {
            armor: reader.read_int(8),
            damage_taken: reader.read_int(8),
            visible_bits_damage: reader.read_int(32),
            vec_from: vec![reader.read_float(32), reader.read_float(32), reader.read_float(32)],
        }
    }
}

pub struct EmptyUserMessage;

pub struct EntityPortalled {
    portal: i32, // these two are actually EHandles (whatever that is)
    portalled: i32, // ill just read them as ints for now
    new_position: Vec<f32>,
    new_angles: Vec<f32>,
}

impl EntityPortalled {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self {
            portal: reader.read_int(32),
            portalled: reader.read_int(32),
            new_position: vec![reader.read_float(32), reader.read_float(32), reader.read_float(32)],
            new_angles: vec![reader.read_float(32), reader.read_float(32), reader.read_float(32)],
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

pub struct PaintEntity;

pub struct PaintWorld;

pub struct PortalFxSurface {
    portal_ent: i32,
    owner_ent: i32,
    team: i32,
    portal_num: i32,
    effect: i32,
    origin: Vec<f32>, // dont know what these are, just assuming theyre f32
    angles: Vec<f32>,
}

pub struct ResetHUD {
    unknown: i32,
}

pub struct Rumble {
    rumble_type: i32,
    scale: f32,
    rumble_flags: i32,
}

pub struct SayText {
    client_id: i32,
    text: String,
    wants_to_chat: bool,
}

pub struct SayText2 {
    client: i32,
    wants_to_chat: bool,
    msg_name: String,
    msgs: Vec<String>,
}

pub struct ScoreboardTempUpdate {
    num_portals: i32,
    time_taken: i32, // centi-seconds
}

pub struct Shake {
    command: i32,
    amplitude: f32,
    frequency: f32,
    duration: f32,
}

pub struct TextMsg {
    destination: i32,
    messages: Vec<String>,
}

pub struct Train {
    pos: i32,
}

pub struct TransitionFade {
    seconds: f32,
}

pub struct UnknownUserMessage;

pub struct VguiMenu {
    message: String,
    show: bool,
    count: i32,
    key_values: Vec<HashMap<String, String>>,
}

pub struct VoiceMask;

pub struct WitchBloodSplatter {
    pos: Vec<f32>, // again assuming this is just f32, also wtf is this message lol
}