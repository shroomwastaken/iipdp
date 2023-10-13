use std::collections::HashMap;
use crate::bitreader::BitReader;
use crate::structs::utils::{Vec3, EHandle};
use crate::enum_primitive::enum_from_primitive;
use crate::enum_primitive::FromPrimitive;

// used in data_manager.rs
// this is all the possible usermessage types (except l4d and l4d2) which i borrowed from untitledparser
// only like 20% of these are actually implemented but ill have them all here cause why not
#[derive(Debug)]
pub enum UserMessageType {
    // book keeping
    Unknown,
    // 3420 types
    Geiger,Train, HudText, SayText, SayText2, TextMsg,
    HUDMsg, ResetHUD, GameTitle, ItemPickup, ShowMenu,
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
    AddLocator, MPMapCompleted, MPMapIncomplete,
    MPMapCompletedData, MPTauntEarned, MPTauntUnlocked,
    MPTauntLocked, MPAllTauntsLocked, PortalFXSurface,
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
pub enum UserMessageDataType {
    Unknown,
    AchievementEvent(AchievementEvent), Battery(Battery),
    CloseCaption(CloseCaption), Damage(Damage), EntityPortalled(EntityPortalled),
    Fade(Fade), Geiger(Geiger), HudMsg(HudMsg), HudText(HudText),
    KeyHintText(KeyHintText), KillCam(KillCam), LogoTimeMsg(LogoTimeMsg),
    MPMapCompleted(MpMapCompleted), MpTauntEarned(MpTauntEarned), MPTauntLocked(MpTauntLocked),
    PaintEntity(PaintEntity), PaintWorld(PaintWorld), PortalFxSurface(PortalFxSurface),
    ResetHUD(ResetHUD), Rumble(Rumble), SayText(SayText), SayText2(SayText2),
    ScoreboardTempUpdate(ScoreboardTempUpdate), Shake(Shake), TextMsg(TextMsg),
    Train(Train), TransitionFade(TransitionFade), VguiMenu(VguiMenu), PlayerMask(PlayerMask),
    VoiceMask(VoiceMask), HapPunch(HapPunch), HapSetConstForce(HapSetConstForce),
    HapSetDrag(HapSetDrag), SpHapWeaponEvent(SpHapWeaponEvent),
}

pub struct UserMessage {
    msg_type: UserMessageType,
    data: UserMessageDataType
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
        };
    }
}

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
            flags: CloseCaptionFlags::from_i32(reader.read_int(8)).unwrap(),
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
            visible_bits_damage: DamageType::from_i32(reader.read_int(32)).unwrap(),
            vec_from: reader.read_vec3(),
        }
    }
}

pub struct EmptyUserMessage;

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
            flags: FadeFlags::from_i32(reader.read_int(16)).unwrap(),
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

// this is a chonker
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
    spec_mode: SpectatorMode,
    target1: i32,
    target: i32,
    unknown: i32,
}

impl KillCam {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self {
            spec_mode: SpectatorMode::from_i32(reader.read_int(8)).unwrap(),
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

pub struct MpMapCompletedData; // p2 specific so im not gonna bother yet

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
    ent: EHandle,
    paint_type: PaintType, // this is actually a value of type PaintType in untitledparser but idk what that is
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

// not gonna implement this yet because im not sure what the last 2 values are (theyre not vec3 thats a placeholder)
pub struct PortalFxSurface {
    portal_ent: i32,
    owner_ent: i32,
    team: i32,
    portal_num: i32,
    effect: PortalFizzleType,
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
    rumble_type: RumbleLookup,
    scale: f32,
    rumble_flags: RumbleFlags,
}

impl Rumble {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self {
            rumble_type: RumbleLookup::from_i32(reader.read_int(8)).unwrap(),
            scale: reader.read_int(8) as f32 / 100.0,
            rumble_flags: RumbleFlags::from_i32(reader.read_int(8)).unwrap(),
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

pub struct TextMsg {
    destination: TextMsgDestination,
    messages: Vec<String>,
}

impl TextMsg {
    pub fn parse(reader: &mut BitReader) -> Self {
        let destination = TextMsgDestination::from_i32(reader.read_int(8)).unwrap();
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

pub struct HapPunch {
    f1: f32, f2: f32, f3: f32,
}

impl HapPunch {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self { f1: reader.read_float(32), f2: reader.read_float(32), f3: reader.read_float(32) }
    }
}


pub struct HapSetConstForce {
    s1: i32, s2: i32, s3: i32,
}

impl HapSetConstForce {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self { s1: reader.read_int(16), s2: reader.read_int(16), s3: reader.read_int(16) }
    }
}

pub struct HapSetDrag {
    unknown: f32,
}

impl HapSetDrag {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self { unknown: reader.read_float(32) }
    }
}

pub struct SpHapWeaponEvent {
    unk: i32,
}

impl SpHapWeaponEvent {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self { unk: reader.read_int(32) }
    }
}

// enums (various flags and types)

enum_from_primitive! {
    #[derive(Debug)]
    pub enum CloseCaptionFlags {
        None, 
        WarnIfMissing,
        FromPlayer,
        GenderMale,
        GenderFemale,
    }
}

enum_from_primitive! {
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
}

enum_from_primitive! {
    #[derive(Debug)]
    pub enum FadeFlags {
        None = 0,
        FadeIn = 1,
        FadeOut = 1 << 1,
        Modulate = 1 << 2,
        StayOut = 1 << 3,
        Purge = 1 << 4,
    }
}

enum_from_primitive! {
    #[derive(Debug)]
    pub enum HudMsgEffect {
        Fade = 0,
        Flicker = 1,
        WriteOut = 2,
    }
}

enum_from_primitive! {
#[derive(Debug)]
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
#[derive(Debug)]
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
    #[derive(Debug)]
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
    #[derive(Debug)]
    pub enum PaintType {
        JumpPaint,
        SpeedPaintOther,
        SpeedPaint,
        PortalPaint,
        ClearPaint,
    }
}

enum_from_primitive! {
    #[derive(Debug)]
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
    #[derive(Debug)]
    pub enum RumbleFlags {
		None            = 0,
		Stop            = 1,
		Loop            = 1 << 1,
		Restart         = 1 << 2,
		UpdateScale     = 1 << 3, // Apply DATA to this effect if already playing, but don't restart.   <-- DATA is scale * 100
		OnlyOne         = 1 << 4, // Don't play this effect if it is already playing.
		RandomAmplitude = 1 << 5, // Amplitude scale will be randomly chosen. Between 10% and 100%
		InitialScale    = 1 << 6  // Data is the initial scale to start this effect ( * 100 )
	}
}

enum_from_primitive! {
    #[derive(Debug)]
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
    #[derive(Debug)]
    pub enum TextMsgDestination {
		PrintNotify = 1,
		PrintConsole,
		PrintTalk,
		PrintCenter
	}
}