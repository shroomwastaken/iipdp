use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

use crate::bitreader::BitReader;
use crate::structs::utils::{Vec3, EHandle};
use crate::enum_primitive::enum_from_primitive;
use crate::enum_primitive::FromPrimitive;
use crate::bitflags::bitflags;

// used in data_manager.rs
// this is all the possible usermessage types (except l4d and l4d2) which i borrowed from untitledparser
// only like 20% of these are actually implemented but ill have them all here cause why not
#[allow(unused)]
#[derive(Debug, Copy, Clone)]
pub enum UserMessageType {
    // book keeping
    Unknown,
    // 3420 types
    Geiger,Train, HudText, SayText, SayText2, TextMsg,
    HudMsg, ResetHUD, GameTitle, ItemPickup, ShowMenu,
    Shake, Fade, VguiMenu, Rumble, Battery, Damage,
    VoiceMask, RequestState, CloseCaption, HintText,
    KeyHintText, SquadMemberDied, AmmoDenied, CreditsMsg,
    CreditsPortalMsg, LogoTimeMsg, AchievementEvent,
    EntityPortalled, KillCam,
    // additional types from unpack
    SPHapWeaponEvent, HapDmg,
    HapPunch, HapSetDrag,
    HapSetConstForce, HapMeleeContact,
    // additional types from portal 2
    Tilt, CloseCaptionDirect, UpdateJalopyRadar,
    CurrentTimescale, DesiredTimescale, InventoryFlash,
    IndicatorFlash, ControlHelperAnimate, TakePhoto,
    Flash, HudPingIndicator, OpenRadialMenu,
    AddLocator, MpMapCompleted, MpMapIncomplete,
    MpMapCompletedData, MpTauntEarned, MpTauntUnlocked,
    MpTauntLocked, MpAllTauntsLocked, PortalFXSurface,
    PaintWorld, PaintEntity, ChangePaintColor, PaintBombExplode,
    RemoveAllPaint, PaintAllSurfaces, RemovePaint,
    StartSurvey, ApplyHitBoxDamageEffect, SetMixLayerTriggerFactor,
    TransitionFade, ScoreboardTempUpdate, ChallengeModCheatSession,
    ChallengeModCloseAllUI, 
    // additional types from portal 1 steampipe
    CallVoteFailed, VoteStart,
    VotePass, VoteFailed,
    VoteSetup,
}

// all of the currently parseable types
#[derive(PartialEq, Clone)]
pub enum UserMessageDataType {
    Unknown,
    AchievementEvent(AchievementEvent), Battery(Battery),
    CloseCaption(CloseCaption), Damage(Damage), EntityPortalled(EntityPortalled),
    Fade(Fade), Geiger(Geiger), HudMsg(HudMsg), HudText(HudText),
    KeyHintText(KeyHintText), KillCam(KillCam), LogoTimeMsg(LogoTimeMsg),
    MpMapCompleted(MpMapCompleted), MpTauntEarned(MpTauntEarned), MpTauntLocked(MpTauntLocked),
    PaintEntity(PaintEntity), PaintWorld(PaintWorld), PortalFXSurface(PortalFXSurface),
    ResetHUD(ResetHUD), Rumble(Rumble), SayText(SayText), SayText2(SayText2),
    ScoreboardTempUpdate(ScoreboardTempUpdate), Shake(Shake), TextMsg(TextMsg),
    Train(Train), TransitionFade(TransitionFade), VguiMenu(VguiMenu),
    VoiceMask(VoiceMask), HapPunch(HapPunch), HapSetConstForce(HapSetConstForce),
    HapSetDrag(HapSetDrag), SpHapWeaponEvent(SpHapWeaponEvent),
}

#[derive(Clone)]
pub struct UserMessage {
    pub msg_type: UserMessageType,
    pub data: UserMessageDataType,
}

impl UserMessage {
    pub fn new() -> Self {
        Self { msg_type: UserMessageType::Unknown, data: UserMessageDataType::Unknown }
    }

    // i swear this actually might make sense at some point maybe
    pub fn parse(reader: &mut BitReader, msg_type: UserMessageType, length: i32) -> Self {
        let data = match msg_type {
            UserMessageType::AchievementEvent => UserMessageDataType::AchievementEvent(AchievementEvent::parse(reader)),
            UserMessageType::Battery => UserMessageDataType::Battery(Battery::parse(reader)),
            UserMessageType::CloseCaption => UserMessageDataType::CloseCaption(CloseCaption::parse(reader)),
            UserMessageType::Damage => UserMessageDataType::Damage(Damage::parse(reader)),
            UserMessageType::EntityPortalled => UserMessageDataType::EntityPortalled(EntityPortalled::parse(reader)),
            UserMessageType::Fade => UserMessageDataType::Fade(Fade::parse(reader)),
            UserMessageType::Geiger => UserMessageDataType::Geiger(Geiger::parse(reader)),
            UserMessageType::HudMsg => UserMessageDataType::HudMsg(HudMsg::parse(reader)),
            UserMessageType::HudText => UserMessageDataType::HudText(HudText::parse(reader)),
            UserMessageType::KeyHintText => UserMessageDataType::KeyHintText(KeyHintText::parse(reader)),
            UserMessageType::KillCam => UserMessageDataType::KillCam(KillCam::parse(reader)),
            UserMessageType::LogoTimeMsg => UserMessageDataType::LogoTimeMsg(LogoTimeMsg::parse(reader)),
            UserMessageType::MpMapCompleted => UserMessageDataType::MpMapCompleted(MpMapCompleted::parse(reader)),
            UserMessageType::MpTauntEarned => UserMessageDataType::MpTauntEarned(MpTauntEarned::parse(reader)),
            UserMessageType::MpTauntLocked => UserMessageDataType::MpTauntLocked(MpTauntLocked::parse(reader)),
            UserMessageType::PaintEntity => UserMessageDataType::PaintEntity(PaintEntity::parse(reader)),
            UserMessageType::PaintWorld => UserMessageDataType::PaintWorld(PaintWorld::parse(reader)),
            UserMessageType::PortalFXSurface => UserMessageDataType::PortalFXSurface(PortalFXSurface::parse(reader)),
            UserMessageType::ResetHUD => UserMessageDataType::ResetHUD(ResetHUD::parse(reader)),
            UserMessageType::Rumble => UserMessageDataType::Rumble(Rumble::parse(reader)),
            UserMessageType::SayText => UserMessageDataType::SayText(SayText::parse(reader)),
            UserMessageType::SayText2 => UserMessageDataType::SayText2(SayText2::parse(reader)),
            UserMessageType::ScoreboardTempUpdate => UserMessageDataType::ScoreboardTempUpdate(ScoreboardTempUpdate::parse(reader)),
            UserMessageType::Shake => UserMessageDataType::Shake(Shake::parse(reader)),
            UserMessageType::TextMsg => UserMessageDataType::TextMsg(TextMsg::parse(reader)),
            UserMessageType::Train => UserMessageDataType::Train(Train::parse(reader)),
            UserMessageType::TransitionFade => UserMessageDataType::TransitionFade(TransitionFade::parse(reader)),
            UserMessageType::VguiMenu => UserMessageDataType::VguiMenu(VguiMenu::parse(reader)),
            UserMessageType::VoiceMask => UserMessageDataType::VoiceMask(VoiceMask::parse(reader)),
            UserMessageType::HapPunch => UserMessageDataType::HapPunch(HapPunch::parse(reader)),
            UserMessageType::HapSetConstForce => UserMessageDataType::HapSetConstForce(HapSetConstForce::parse(reader)),
            UserMessageType::HapSetDrag => UserMessageDataType::HapSetDrag(HapSetDrag::parse(reader)),
            UserMessageType::SPHapWeaponEvent => UserMessageDataType::SpHapWeaponEvent(SpHapWeaponEvent::parse(reader)),
            _ => UserMessageDataType::Unknown,
        };

        if data == UserMessageDataType::Unknown {
            reader.skip(length);
            Self { msg_type: UserMessageType::Unknown, data: data }
        } else {
            Self { msg_type: msg_type, data: data }
        }
    }
}

// all of the structs and parsing
#[derive(PartialEq, Clone)]
pub struct AchievementEvent {
    achievement_id: i32,
}

impl AchievementEvent {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self { achievement_id: reader.read_int(32) }
    }
}

#[derive(PartialEq, Clone)]
pub struct Battery {
    battery_val: i32,
}

impl Battery {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self { battery_val: reader.read_int(16) }
    }
}

#[derive(PartialEq, Clone)]
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
            flags: CloseCaptionFlags::from_bits_truncate(reader.read_int(8)),
        }
    }
}

#[derive(PartialEq, Clone)]
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
            visible_bits_damage: DamageType::from_bits_truncate(reader.read_int(32)),
            vec_from: reader.read_vec3(),
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct EntityPortalled {
    portal: EHandle,
    portalled: EHandle,
    new_position: Vec3,
    new_angles: Vec3,
}

impl EntityPortalled {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self {
            portal: EHandle { val: reader.read_int(32) },
            portalled: EHandle { val: reader.read_int(32) },
            new_position: reader.read_vec3(),
            new_angles: reader.read_vec3(),
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct Fade {
    duration: f32,
    hold_time: i32,
    flags: FadeFlags,
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
            flags: FadeFlags::from_bits_truncate(reader.read_int(16)),
            r: reader.read_int(8),
            g: reader.read_int(8),
            b: reader.read_int(8),
            a: reader.read_int(8),
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct Geiger {
    geiger_range: i32,
}

impl Geiger {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self { geiger_range: reader.read_int(8) }
    }
}

// this is a chonker
#[derive(PartialEq, Clone)]
pub struct HudMsgInfo {
    x: f32, y: f32,
    r1: i32, g1: i32, b1: i32, a1: i32,  
    r2: i32, g2: i32, b2: i32, a2: i32,
    effect: HudMsgEffect,
    fade_in: f32, fade_out: f32, hold_time: f32, fx_time: f32,
    message: String,
}

impl HudMsgInfo {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self {
            x: reader.read_float(32), y: reader.read_float(32),
            r1: reader.read_int(8), g1: reader.read_int(8), b1: reader.read_int(8), a1: reader.read_int(8),
            r2: reader.read_int(8), g2: reader.read_int(8), b2: reader.read_int(8), a2: reader.read_int(8),
            effect: HudMsgEffect::from_i32(reader.read_int(8)).unwrap(),
            fade_in: reader.read_float(32), fade_out: reader.read_float(32),
            hold_time: reader.read_float(32), fx_time: reader.read_float(32),
            message: reader.read_ascii_string_nulled(),
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct HudMsg {
    channel: HudChannel,
    msg_info: Option<HudMsgInfo>,
}

impl HudMsg {
    pub fn parse(reader: &mut BitReader) -> Self {
        let channel = HudChannel::from_i32(reader.read_int(8) % 6).unwrap();
        let mut msg_info: Option<HudMsgInfo> = None;
        if reader.contents.len() * 8 - reader.cur_bit_index as usize >= 148 {
            msg_info = Some(HudMsgInfo::parse(reader));
        }

        Self { channel: channel, msg_info: msg_info }
    }
}

#[derive(PartialEq, Clone)]
pub struct HudText {
    string: String,
}

impl HudText {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self { string: reader.read_ascii_string_nulled() } // this has a defined length but im pretty sure it should be null terminated
    }
}

#[derive(PartialEq, Clone)]
pub struct KeyHintText {
    count: i32, // should always be 1
    key_string: String,
}

impl KeyHintText {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self { count: reader.read_int(8), key_string: reader.read_ascii_string_nulled() }
    }
}

#[derive(PartialEq, Clone)]
pub struct KillCam {
    spec_mode: SpectatorMode,
    target1: i32,
    target2: i32,
    unknown: i32,
}

impl KillCam {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self {
            spec_mode: SpectatorMode::from_i32(reader.read_int(8)).unwrap(),
            target1: reader.read_int(8),
            target2: reader.read_int(8),
            unknown: reader.read_int(8),
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct LogoTimeMsg {
    time: f32,
}

impl LogoTimeMsg {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self { time: reader.read_float(32) }
    }
}

#[derive(PartialEq, Clone)]
pub struct MpMapCompleted {
    branch: i32,
    level: i32,
}

impl MpMapCompleted {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self { branch: reader.read_int(8), level: reader.read_int(8) }
    }
}

pub struct MpMapCompletedData; // p2 specific so im not gonna bother yet

#[derive(PartialEq, Clone)]
pub struct MpTauntEarned {
    taunt_name: String,
    award_silently: bool,
}

impl MpTauntEarned {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self { taunt_name: reader.read_ascii_string_nulled(), award_silently: reader.read_bool() }
    }
}

#[derive(PartialEq, Clone)]
pub struct MpTauntLocked {
    taunt_name: String,
}

impl MpTauntLocked {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self { taunt_name: reader.read_ascii_string_nulled() }
    }
}

#[derive(PartialEq, Clone)]
pub struct PaintEntity {
    ent: EHandle,
    paint_type: PaintType,
    pos: Vec3,
}

impl PaintEntity {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self {
            ent: reader.read_ehandle(),
            paint_type: PaintType::from_i32(reader.read_int(8)).unwrap(),
            pos: reader.read_vec3(),
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct PaintWorld {
    paint_type: PaintType,
    ehandle: EHandle,
    unkhf1: f32, // no idea what these are
    unkhf2: f32,
    length: i32,
    center: Vec3,
    positions: Vec<Vec3>,
}

impl PaintWorld {
    pub fn parse(reader: &mut BitReader) -> Self {
        let paint_type = PaintType::from_i32(reader.read_int(8)).unwrap();
        let ehandle = EHandle { val: reader.read_int(32) };
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

#[derive(PartialEq, Clone)]
pub struct PortalFXSurface {
    portal_ent: i32,
    owner_ent: i32,
    team: i32,
    portal_num: i32,
    effect: PortalFizzleType,
    origin: Vec<Option<f32>>,
    angles: Vec<Option<f32>>,
}

impl PortalFXSurface {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self { 
            portal_ent: reader.read_int(16),
            owner_ent: reader.read_int(16),
            team: reader.read_int(8),
            portal_num: reader.read_int(8),
            effect: PortalFizzleType::from_i32(reader.read_int(8)).unwrap(),
            origin: reader.read_vector_coords(),
            angles: reader.read_vector_coords(),
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct ResetHUD {
    unknown: i32,
}

impl ResetHUD {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self { unknown: reader.read_int(8) }
    }
}

#[derive(PartialEq, Clone)]
pub struct Rumble {
    rumble_type: RumbleLookup,
    scale: f32,
    rumble_flags: RumbleFlags,
}

impl Rumble {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self {
            rumble_type: RumbleLookup::from_i32(reader.read_int(8)).unwrap(),
            scale: reader.read_int(8) as f32 / 100.0,
            rumble_flags: RumbleFlags::from_bits_truncate(reader.read_int(8)),
        }
    }
}

#[derive(PartialEq, Clone)]
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

#[derive(PartialEq, Clone)]
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

#[derive(PartialEq, Clone)]
pub struct ScoreboardTempUpdate {
    num_portals: i32,
    time_taken: i32, // centi-seconds
}

impl ScoreboardTempUpdate {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self { num_portals: reader.read_int(32), time_taken: reader.read_int(32) }
    }
}

#[derive(PartialEq, Clone)]
pub struct Shake {
    command: ShakeCommand,
    amplitude: f32,
    frequency: f32,
    duration: f32,
}

impl Shake {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self {
            command: ShakeCommand::from_i32(reader.read_int(8)).unwrap(),
            amplitude: reader.read_float(32),
            frequency: reader.read_float(32),
            duration: reader.read_float(32),
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct TextMsg {
    destination: TextMsgDestination,
    messages: Vec<String>,
}

impl TextMsg {
    pub fn parse(reader: &mut BitReader) -> Self {
        let destination = TextMsgDestination::from_i32(reader.read_int(8)).unwrap();
        let mut messages: Vec<String> = Vec::new();
    
        for _ in 0..5 {
            messages.push(reader.read_ascii_string_nulled());
        }

        Self { destination: destination, messages: messages }
    }
}

#[derive(PartialEq, Clone)]
pub struct Train {
    pos: i32,
}

impl Train {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self { pos: reader.read_int(8) }
    }
}

#[derive(PartialEq, Clone)]
pub struct TransitionFade {
    seconds: f32,
}

impl TransitionFade {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self { seconds: reader.read_float(32) }
    }
}

#[derive(PartialEq, Clone)]
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
#[derive(PartialEq, Clone)]
pub struct PlayerMask {
    pub game_rules_mask: i32,
    pub ban_mask: i32,
}

#[derive(PartialEq, Clone)]
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

#[derive(PartialEq, Clone)]
pub struct HapPunch {
    f1: f32, f2: f32, f3: f32,
}

impl HapPunch {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self { f1: reader.read_float(32), f2: reader.read_float(32), f3: reader.read_float(32) }
    }
}

#[derive(PartialEq, Clone)]
pub struct HapSetConstForce {
    s1: i32, s2: i32, s3: i32,
}

impl HapSetConstForce {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self { s1: reader.read_int(16), s2: reader.read_int(16), s3: reader.read_int(16) }
    }
}

#[derive(PartialEq, Clone)]
pub struct HapSetDrag {
    unknown: f32,
}

impl HapSetDrag {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self { unknown: reader.read_float(32) }
    }
}

#[derive(PartialEq, Clone)]
pub struct SpHapWeaponEvent {
    unk: i32,
}

impl SpHapWeaponEvent {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self { unk: reader.read_int(32) }
    }
}

// enums (various flags and types)

bitflags! {
    #[derive(Debug, PartialEq, Clone)]
    pub struct CloseCaptionFlags: i32 {
        const None = 0; 
        const WarnIfMissing = 1;
        const FromPlayer = 2;
        const GenderMale = 3;
        const GenderFemale = 4;
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct DamageType : i32 {
        const None = -1;
        const DmgGeneric             = 0;
        const DmgCrush               = 1 << 0;
        const DmgBullet              = 1 << 1;
        const DmgSlash               = 1 << 2;
        const DmgBurn                = 1 << 3;
        const DmgVehicle             = 1 << 4;
        const DmgFall                = 1 << 5;
        const DmgBlast               = 1 << 6;
        const DmgClub                = 1 << 7;
        const DmgShock               = 1 << 8;
        const DmgSonic               = 1 << 9;
        const DmgEnergyBeam          = 1 << 10;
        const DmgPreventPhysicsForce = 1 << 11;
        const DmgNeverGib            = 1 << 12;
        const DmgAlwaysGib           = 1 << 13;
        const DmgDrown               = 1 << 14;
        const DmgParalyze            = 1 << 15;
        const DmgNerveGas            = 1 << 16;
        const DmgPoison              = 1 << 17;
        const DmgRadiation           = 1 << 18;
        const DmgDrownRecover        = 1 << 19;
        const DmgAcid                = 1 << 20;
        const DmgSlowBurn            = 1 << 21;
        const DmgRemoveNoRagdoll     = 1 << 22;
        const DmgPhysGun             = 1 << 23;
        const DmgPlasma              = 1 << 24;
        const DmgAirboat             = 1 << 25;
        const DmgDissolve            = 1 << 26;
        const DmgBlastSurface        = 1 << 27;
        const DmgDirect              = 1 << 28;
        const DmgBuckshot            = 1 << 29;
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct FadeFlags: i32 {
        const None = 0;
        const FadeIn = 1;
        const FadeOut = 1 << 1;
        const Modulate = 1 << 2;
        const StayOut = 1 << 3;
        const Purge = 1 << 4;
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct RumbleFlags : i32 {
		const None            = 0;
		const Stop            = 1;
		const Loop            = 1 << 1;
		const Restart         = 1 << 2;
		const UpdateScale     = 1 << 3; // Apply DATA to this effect if already playing, but don't restart.   <-- DATA is scale * 100
		const OnlyOne         = 1 << 4; // Don't play this effect if it is already playing.
		const RandomAmplitude = 1 << 5; // Amplitude scale will be randomly chosen. Between 10% and 100%
		const InitialScale    = 1 << 6; // Data is the initial scale to start this effect ( * 100 )
	}
}

enum_from_primitive! {
    #[derive(Debug, PartialEq, Clone)]
    pub enum HudMsgEffect {
        Fade = 0,
        Flicker = 1,
        WriteOut = 2,
    }
}

enum_from_primitive! {
#[derive(Debug, PartialEq, Clone)]
    pub enum HudChannel {
        NetMessage1 = 0,
        NetMessage2,
        NetMessage3,
        NetMessage4,
        NetMessage5,
        NetMessage6
    }
}

enum_from_primitive! {
    #[derive(Debug, PartialEq, Clone)]
    pub enum SpectatorMode {
        None,      // not in spectator mode
        DeathCam,  // special mode for death cam animation
        FreezeCam, // zooms to a target, and freeze-frames on them
        Fixed,     // view from a fixed camera position
        InEye,     // follow a player in first person view
        Chase,     // follow a player in third person view
        Roaming    // free roaming
    }
}

enum_from_primitive! {
    #[derive(Debug, PartialEq, Clone)]
    pub enum PortalFizzleType {
        PortalFizzleSuccess = 0, // Placed fine (no fizzle)
        PortalFizzleCantFit,
        PortalFizzleOverlappedLinked,
        PortalFizzleBadVolume,
        PortalFizzleBadSurface,
        PortalFizzleKilled,
        PortalFizzleCleanser,
        PortalFizzleClose,
        PortalFizzleNearBlue,
        PortalFizzleNearRed,
        PortalFizzleNone,
    }
}

enum_from_primitive! {
    #[derive(Debug, PartialEq, Clone)]
    pub enum PaintType {
        JumpPaint,
        SpeedPaintOther,
        SpeedPaint,
        PortalPaint,
        ClearPaint,
    }
}

enum_from_primitive! {
    #[derive(Debug, PartialEq, Clone)]
    #[allow(non_camel_case_types)]
    pub enum RumbleLookup {
        RumbleInvalid = -1,
        RumbleStopAll = 0, // cease all current rumbling effects.

        // Weapons
        Pistol, Weap_357, Smg1, Ar2, ShotgunSingle, ShotgunDouble, Ar2AltFire,
        RpgMissile, CrowbarSwing,

        // Vehicles
        AirboatGun, JeepEngineLoop,

        FlatLeft, FlatRight, FlatBoth,

        // Damage
        DmgLow, DmgMed, DmgHigh,

        // Fall damage
        FallLong, FallShort,

        PhyscannonOpen, PhyscannonPunt, PhyscannonLow, PhyscannonMedium, PhyscannonHigh,

        PortalgunLeft, PortalgunRight, PortalPlacementFailure, NumRumbleEffects
    }
}

enum_from_primitive! {
    #[derive(Debug, PartialEq, Clone)]
    pub enum ShakeCommand {
        Start = 0,  // Starts the screen shake for all players within the radius.
        Stop,       // Stops the screen shake for all players within the radius.
        Amplitude,  // Modifies the amplitude of an active screen shake for all players within the radius.
        Frequency,  // Modifies the frequency of an active screen shake for all players within the radius.
        RumbleOnly, // Starts a shake effect that only rumbles the controller, no screen effect.
        NoRumble    // Starts a shake that does NOT rumble the controller.
    }
}

enum_from_primitive! {
    #[derive(Debug, PartialEq, Clone)]
    pub enum TextMsgDestination {
		PrintNotify = 1,
		PrintConsole,
		PrintTalk,
		PrintCenter
	}
}

// implementing Into<type> for every type in the UserMessageDataType enum (needed for printing)

impl Into<AchievementEvent> for UserMessageDataType {
    fn into(self) -> AchievementEvent {
        match self {
            UserMessageDataType::AchievementEvent(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<Battery> for UserMessageDataType {
    fn into(self) -> Battery {
        match self {
            UserMessageDataType::Battery(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<CloseCaption> for UserMessageDataType {
    fn into(self) -> CloseCaption {
        match self {
            UserMessageDataType::CloseCaption(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<Damage> for UserMessageDataType {
    fn into(self) -> Damage {
        match self {
            UserMessageDataType::Damage(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<EntityPortalled> for UserMessageDataType {
    fn into(self) -> EntityPortalled {
        match self {
            UserMessageDataType::EntityPortalled(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<Fade> for UserMessageDataType {
    fn into(self) -> Fade {
        match self {
            UserMessageDataType::Fade(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<Geiger> for UserMessageDataType {
    fn into(self) -> Geiger {
        match self {
            UserMessageDataType::Geiger(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<HudMsg> for UserMessageDataType {
    fn into(self) -> HudMsg {
        match self {
            UserMessageDataType::HudMsg(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<HudText> for UserMessageDataType {
    fn into(self) -> HudText {
        match self {
            UserMessageDataType::HudText(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<KeyHintText> for UserMessageDataType {
    fn into(self) -> KeyHintText {
        match self {
            UserMessageDataType::KeyHintText(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<KillCam> for UserMessageDataType {
    fn into(self) -> KillCam {
        match self {
            UserMessageDataType::KillCam(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<LogoTimeMsg> for UserMessageDataType {
    fn into(self) -> LogoTimeMsg {
        match self {
            UserMessageDataType::LogoTimeMsg(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<MpMapCompleted> for UserMessageDataType {
    fn into(self) -> MpMapCompleted {
        match self {
            UserMessageDataType::MpMapCompleted(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<MpTauntEarned> for UserMessageDataType {
    fn into(self) -> MpTauntEarned {
        match self {
            UserMessageDataType::MpTauntEarned(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<MpTauntLocked> for UserMessageDataType {
    fn into(self) -> MpTauntLocked {
        match self {
            UserMessageDataType::MpTauntLocked(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<PaintEntity> for UserMessageDataType {
    fn into(self) -> PaintEntity {
        match self {
            UserMessageDataType::PaintEntity(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<PaintWorld> for UserMessageDataType {
    fn into(self) -> PaintWorld {
        match self {
            UserMessageDataType::PaintWorld(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<PortalFXSurface> for UserMessageDataType {
    fn into(self) -> PortalFXSurface {
        match self {
            UserMessageDataType::PortalFXSurface(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<ResetHUD> for UserMessageDataType {
    fn into(self) -> ResetHUD {
        match self {
            UserMessageDataType::ResetHUD(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<Rumble> for UserMessageDataType {
    fn into(self) -> Rumble {
        match self {
            UserMessageDataType::Rumble(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<SayText> for UserMessageDataType {
    fn into(self) -> SayText {
        match self {
            UserMessageDataType::SayText(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<SayText2> for UserMessageDataType {
    fn into(self) -> SayText2 {
        match self {
            UserMessageDataType::SayText2(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<ScoreboardTempUpdate> for UserMessageDataType {
    fn into(self) -> ScoreboardTempUpdate {
        match self {
            UserMessageDataType::ScoreboardTempUpdate(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<Shake> for UserMessageDataType {
    fn into(self) -> Shake {
        match self {
            UserMessageDataType::Shake(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<TextMsg> for UserMessageDataType {
    fn into(self) -> TextMsg {
        match self {
            UserMessageDataType::TextMsg(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<Train> for UserMessageDataType {
    fn into(self) -> Train {
        match self {
            UserMessageDataType::Train(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<TransitionFade> for UserMessageDataType {
    fn into(self) -> TransitionFade {
        match self {
            UserMessageDataType::TransitionFade(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<VguiMenu> for UserMessageDataType {
    fn into(self) -> VguiMenu {
        match self {
            UserMessageDataType::VguiMenu(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<VoiceMask> for UserMessageDataType {
    fn into(self) -> VoiceMask {
        match self {
            UserMessageDataType::VoiceMask(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<HapPunch> for UserMessageDataType {
    fn into(self) -> HapPunch {
        match self {
            UserMessageDataType::HapPunch(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<HapSetConstForce> for UserMessageDataType {
    fn into(self) -> HapSetConstForce {
        match self {
            UserMessageDataType::HapSetConstForce(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<HapSetDrag> for UserMessageDataType {
    fn into(self) -> HapSetDrag {
        match self {
            UserMessageDataType::HapSetDrag(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<SpHapWeaponEvent> for UserMessageDataType {
    fn into(self) -> SpHapWeaponEvent {
        match self {
            UserMessageDataType::SpHapWeaponEvent(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

// writing the usermessage data to the dump
#[allow(unused)]
pub fn write_usermsg_data_to_file(msg: UserMessage, file: &mut File) {
    match msg.msg_type {
        UserMessageType::AchievementEvent => {
            let data: AchievementEvent = msg.data.into();
            file.write_fmt(format_args!("\n\t\t\tAchievement ID: {}", data.achievement_id));
        },
        UserMessageType::Battery => {
            let data: Battery = msg.data.into();
            file.write_fmt(format_args!("\n\t\t\tBattery Value: {}", data.battery_val));
        },
        UserMessageType::CloseCaption => {
            let data: CloseCaption = msg.data.into();
            file.write_fmt(format_args!("\n\t\t\tToken Name: {}", data.token_name));
            file.write_fmt(format_args!("\n\t\t\tDuration: {}", data.duration));
            file.write_fmt(format_args!("\n\t\t\tFlags: {:?}", data.flags));
        },
        UserMessageType::Damage => {
            let data: Damage = msg.data.into();
            file.write_fmt(format_args!("\n\t\t\tArmor: {}", data.armor));
            file.write_fmt(format_args!("\n\t\t\tDamage Taken: {}", data.damage_taken));
            file.write_fmt(format_args!("\n\t\t\tVisible Bits Damage: {:?}", data.visible_bits_damage));
            file.write_fmt(format_args!("\n\t\t\tVec From: {}", data.vec_from));
        },
        UserMessageType::EntityPortalled => {
            let data: EntityPortalled = msg.data.into();
            file.write_fmt(format_args!("\n\t\t\tPortal: {}", data.portal));
            file.write_fmt(format_args!("\n\t\t\tPortalled: {}", data.portalled));
            file.write_fmt(format_args!("\n\t\t\tNew Position: {}", data.new_position));
            file.write_fmt(format_args!("\n\t\t\tNew Angles: {}", data.new_angles));
        },
        UserMessageType::Fade => {
            let data: Fade = msg.data.into();
            file.write_fmt(format_args!("\n\t\t\tDuration: {}", data.duration));
            file.write_fmt(format_args!("\n\t\t\tHold Time: {}", data.hold_time));
            file.write_fmt(format_args!("\n\t\t\tFlags: {:?}", data.flags));
            file.write_fmt(format_args!("\n\t\t\tRGBA: {}, {}, {}, {}", data.r, data.g, data.b, data.a));
        },
        UserMessageType::Geiger => {
            let data: Geiger = msg.data.into();
            file.write_fmt(format_args!("\n\t\t\tGeiger Range: {}", data.geiger_range));
        },
        UserMessageType::HudMsg => {
            let data: HudMsg = msg.data.into();
            file.write_fmt(format_args!("\n\t\t\tChannel: {:?}", data.channel));
            let msg_info = data.msg_info.unwrap();
            file.write_fmt(format_args!("\n\t\t\tMessage Info:", ));
            file.write_fmt(format_args!("\n\t\t\t\tX, Y: {}, {}", msg_info.x, msg_info.y));
            file.write_fmt(format_args!("\n\t\t\t\tRGBA 1: {}, {}, {}, {}", msg_info.r1, msg_info.g1, msg_info.b1, msg_info.a1));
            file.write_fmt(format_args!("\n\t\t\t\tRGBA 1: {}, {}, {}, {}", msg_info.r2, msg_info.g2, msg_info.b2, msg_info.a2));
            file.write_fmt(format_args!("\n\t\t\t\tEffect: {:?}", msg_info.effect));
            file.write_fmt(format_args!("\n\t\t\t\tFade In, Fade Out: {}, {}", msg_info.fade_in, msg_info.fade_out));
            file.write_fmt(format_args!("\n\t\t\t\tHold Time, Fx Time: {}, {}", msg_info.hold_time, msg_info.fx_time));
            file.write_fmt(format_args!("\n\t\t\t\tMessage: {}", msg_info.message));
        },
        UserMessageType::HudText => {
            let data: HudText = msg.data.into();
            file.write_fmt(format_args!("\n\t\t\tString: {}", data.string));
        },
        UserMessageType::KeyHintText => {
            let data: KeyHintText = msg.data.into();
            file.write_fmt(format_args!("\n\t\t\tCount: {}", data.count));
            file.write_fmt(format_args!("\n\t\t\tKey String: {}", data.key_string));
        },
        UserMessageType::KillCam => {
            let data: KillCam = msg.data.into();
            file.write_fmt(format_args!("\n\t\t\tSpectator Mode: {:?}", data.spec_mode));
            file.write_fmt(format_args!("\n\t\t\tTarget 1: {}", data.target1));
            file.write_fmt(format_args!("\n\t\t\tTarget 2: {}", data.target2));
            file.write_fmt(format_args!("\n\t\t\tUnknown Byte: {}", data.unknown));
        },
        UserMessageType::LogoTimeMsg => {
            let data: LogoTimeMsg = msg.data.into();
            file.write_fmt(format_args!("\n\t\t\tTime: {}", data.time));
        },
        UserMessageType::MpMapCompleted => {
            let data: MpMapCompleted = msg.data.into();
            file.write_fmt(format_args!("\n\t\t\tBranch: {}", data.branch));
            file.write_fmt(format_args!("\n\t\t\tLevel: {}", data.level));
        },
        UserMessageType::MpTauntEarned => {
            let data: MpTauntEarned = msg.data.into();
            file.write_fmt(format_args!("\n\t\t\tTaunt Name: {}", data.taunt_name));
            file.write_fmt(format_args!("\n\t\t\tAward Silently: {}", data.award_silently));
        },
        UserMessageType::MpTauntLocked => {
            let data: MpTauntLocked = msg.data.into();
            file.write_fmt(format_args!("\n\t\t\tTaunt Name: {}", data.taunt_name));
        },
        UserMessageType::PaintEntity => {
            let data: PaintEntity = msg.data.into();
            file.write_fmt(format_args!("\n\t\t\tEntity: {}", data.ent));
            file.write_fmt(format_args!("\n\t\t\tPaint Type: {:?}", data.paint_type));
            file.write_fmt(format_args!("\n\t\t\tPos: {}", data.pos));
        },
        UserMessageType::PaintWorld => {
            let data: PaintWorld = msg.data.into();
            file.write_fmt(format_args!("\n\t\t\tPaint Type: {:?}", data.paint_type));
            file.write_fmt(format_args!("\n\t\t\tEntity: {}", data.ehandle));
            file.write_fmt(format_args!("\n\t\t\tUnknown HF1: {}", data.unkhf1));
            file.write_fmt(format_args!("\n\t\t\tUnknown HF2: {}", data.unkhf2));
            file.write_fmt(format_args!("\n\t\t\tLength: {}", data.length));
            file.write_fmt(format_args!("\n\t\t\tCenter: {}", data.center));
            file.write_all("\n\t\t\tPositions: [".as_bytes());
            let mut pos_str: String = String::new();
            for i in 0..data.length {
                pos_str.push_str(&data.positions[i as usize].to_string());
                pos_str.push_str(", ");
            }
            file.write_fmt(format_args!("{}]", pos_str[..pos_str.len()-2].to_string()));
        },
        UserMessageType::PortalFXSurface => {
            let data: PortalFXSurface = msg.data.into();
            file.write_fmt(format_args!("\n\t\t\tPortal Ent: {}", data.portal_ent));
            file.write_fmt(format_args!("\n\t\t\tOwner Ent: {}", data.owner_ent));
            file.write_fmt(format_args!("\n\t\t\tTEam: {}", data.team));
            file.write_fmt(format_args!("\n\t\t\tPortal Num: {}", data.portal_num));
            file.write_fmt(format_args!("\n\t\t\tEffect: {:?}", data.effect));
            file.write_fmt(format_args!("\n\t\t\tOrigin: {}, {}, {}",
                data.origin[0].map(|i| {i.to_string()}).unwrap_or_else(|| {"Null".to_string()}),
                data.origin[1].map(|i| {i.to_string()}).unwrap_or_else(|| {"Null".to_string()}),
                data.origin[2].map(|i| {i.to_string()}).unwrap_or_else(|| {"Null".to_string()}),
            ));

            file.write_fmt(format_args!("\n\t\t\tAngles: {}, {}, {}",
                data.angles[0].map(|i| {i.to_string()}).unwrap_or_else(|| {"Null".to_string()}),
                data.angles[1].map(|i| {i.to_string()}).unwrap_or_else(|| {"Null".to_string()}),
                data.angles[2].map(|i| {i.to_string()}).unwrap_or_else(|| {"Null".to_string()}),
            ));
        }
        UserMessageType::ResetHUD => {
            let data: ResetHUD = msg.data.into();
            file.write_fmt(format_args!("\n\t\t\tUnknown Byte: {}", data.unknown));
        },
        UserMessageType::Rumble => {
            let data: Rumble = msg.data.into();
            file.write_fmt(format_args!("\n\t\t\tRumble Type: {:?}", data.rumble_type));
            file.write_fmt(format_args!("\n\t\t\tScale: {}", data.scale));
            file.write_fmt(format_args!("\n\t\t\tRumble Flags: {:?}", data.rumble_flags));
        },
        UserMessageType::SayText => {
            let data: SayText = msg.data.into();
            file.write_fmt(format_args!("\n\t\t\tClient ID: {}", data.client_id));
            file.write_fmt(format_args!("\n\t\t\tText: {}", data.text));
            file.write_fmt(format_args!("\n\t\t\tWants To Chat: {}", data.wants_to_chat));
        },
        UserMessageType::SayText2 => {
            let data: SayText2 = msg.data.into();
            file.write_fmt(format_args!("\n\t\t\tClient: {}", data.client));
            file.write_fmt(format_args!("\n\t\t\tWants To Chat: {}", data.wants_to_chat));
            file.write_fmt(format_args!("\n\t\t\tMessage Name: {}", data.msg_name));
            file.write_all("\n\t\t\tMessages:".as_bytes());
            for i in 0..4 {
                file.write_fmt(format_args!("\n\t\t\t\t{}", data.msgs[i]));
            }
        },
        UserMessageType::ScoreboardTempUpdate => {
            let data: ScoreboardTempUpdate = msg.data.into();
            file.write_fmt(format_args!("\n\t\t\tNum Portals: {}", data.num_portals));
            file.write_fmt(format_args!("\n\t\t\tTime Taken: {}", data.time_taken as f32 / 100.0));
        },
        UserMessageType::Shake => {
            let data: Shake = msg.data.into();
            file.write_fmt(format_args!("\n\t\t\tCommand: {:?}", data.command));
            file.write_fmt(format_args!("\n\t\t\tAmplitude: {}", data.amplitude));
            file.write_fmt(format_args!("\n\t\t\tFrequency: {}", data.frequency));
            file.write_fmt(format_args!("\n\t\t\tDuration: {}", data.duration));
        },
        UserMessageType::TextMsg => {
            let data: TextMsg = msg.data.into();
            file.write_fmt(format_args!("\n\t\t\tDestination: {:?}", data.destination));
            file.write_all("\n\t\t\tMessages:".as_bytes());
            for i in 0..5 {
                file.write_fmt(format_args!("\n\t\t\t\t{}: {}", i + 1, data.messages[i].trim_end_matches("\n")));
            }
        },
        UserMessageType::Train => {
            let data: Train = msg.data.into();
            file.write_fmt(format_args!("\n\t\t\tPos: {}", data.pos));
        },
        UserMessageType::TransitionFade => {
            let data: TransitionFade = msg.data.into();
            file.write_fmt(format_args!("\n\t\t\tSeconds: {}", data.seconds));
        },
        UserMessageType::VguiMenu => {
            let data: VguiMenu = msg.data.into();
            file.write_fmt(format_args!("\n\t\t\tMessage: {}", data.message));
            file.write_fmt(format_args!("\n\t\t\tShow: {}", data.show));
            file.write_fmt(format_args!("\n\t\t\tCount: {}", data.count));
            file.write_all("\n\t\t\tKey Values:".as_bytes());
            for i in 0..data.count {
                file.write_fmt(format_args!("\n\t\t\t\t{:?}", data.key_values[i as usize]));
            }
        },
        UserMessageType::VoiceMask => {
            let data: VoiceMask = msg.data.into();
            file.write_fmt(format_args!("\n\t\t\tVoice Max Players: {}", data.voice_max_players));
            file.write_all("\n\t\t\tPlayer Masks:".as_bytes());
            for i in 0..data.voice_max_players {
                file.write_fmt(format_args!("\n\t\t\t\t({}) Game Rules Mask: {}, Ban Mask {}", i, data.player_masks[i as usize].game_rules_mask, data.player_masks[i as usize].ban_mask));
            }
            file.write_fmt(format_args!("\n\t\t\tPlayer Mod Enable: {}", data.player_mod_enable));
        },
        UserMessageType::HapPunch => {
            let data: HapPunch = msg.data.into();
            file.write_fmt(format_args!("\n\t\t\tF1, F2, F3: {}, {}, {}", data.f1, data.f2, data.f3));
        },
        UserMessageType::HapSetDrag => {
            let data: HapSetDrag = msg.data.into();
            file.write_fmt(format_args!("\n\t\t\tUnknown Float: {}", data.unknown));
        },
        UserMessageType::SPHapWeaponEvent => {
            let data: SpHapWeaponEvent = msg.data.into();
            file.write_fmt(format_args!("\n\t\t\tUnknown Int: {}", data.unk));
        },
        UserMessageType::HapSetConstForce => {
            let data: HapSetConstForce = msg.data.into();
            file.write_fmt(format_args!("\n\t\t\tS1, S2, S3: {}, {}, {}", data.s1, data.s2, data.s3));
        },
        _ => {
            file.write_all("\n\t\t\tDATA UNKNOWN OR NOT IMPLEMENTED".as_bytes());
        },
    }
}