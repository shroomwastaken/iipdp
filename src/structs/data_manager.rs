use crate::structs::utils::{GameEventList, ServerClass};
use crate::structs::demo_header::DemoHeader;
use crate::structs::user_message::UserMessageType;
use crate::structs::send_table::SendPropType;

// will be used more later in development
// for now this stores information vital for parsing the demo
// all of it is inferred from the demo header
// later all the datatable and stringtable stuff will be stored here so that its data can be fetched by other messages

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
pub enum Game {
    PORTAL_1_3420,
    PORTAL_1_5135,
    PORTAL_1_1910503, // steampipe

    UNKNOWN
}


pub struct DataManager {
    pub demo_protocol: i32,
    pub network_protocol: i32,
    pub last_packet_tick: i32,
    pub game_event_list: GameEventList,
    pub user_message_list: Vec<UserMessageType>,
    pub game: Game,
    pub net_svc_type_bits: i32,
    pub adj_start_tick: i32,
    pub adj_end_tick: i32,
    pub server_class_info: Vec<ServerClass>,
    pub send_prop_amount_of_bits_to_get_num_bits: i32, // hehehe
    pub send_prop_type_list: Vec<SendPropType>,
    pub paused: bool,
    pub dumping: bool,
}

impl DataManager {
    pub fn new() -> Self {
        Self {
            game_event_list: GameEventList::new(),
            demo_protocol: 0,
            network_protocol: 0,
            last_packet_tick: 0,
            user_message_list: Vec::new(),
            game: Game::UNKNOWN,
            net_svc_type_bits: 6, // default for everything other than 3420 iirc
            adj_start_tick: 0,
            adj_end_tick: 0,
            server_class_info: Vec::new(),
            send_prop_amount_of_bits_to_get_num_bits: 0,
            send_prop_type_list: Vec::new(),
            paused: false,
            dumping: false,
        }
    }

    // all of the info i need (for now) can be inferred from the demo header
    pub fn get_info_from_header(&mut self, header: &DemoHeader) {
        self.demo_protocol = header.demo_protocol;
        self.network_protocol = header.network_protocol;
        match header.network_protocol {
            14 => {
                self.game = Game::PORTAL_1_3420;
                self.net_svc_type_bits = 5;
                self.user_message_list = vec![
                    UserMessageType::Geiger,
                    UserMessageType::Train,
                    UserMessageType::HudText,
                    UserMessageType::SayText,
                    UserMessageType::SayText2,
                    UserMessageType::TextMsg,
                    UserMessageType::HudMsg,
                    UserMessageType::ResetHUD,
                    UserMessageType::GameTitle,
                    UserMessageType::ItemPickup,
                    UserMessageType::ShowMenu,
                    UserMessageType::Shake,
                    UserMessageType::Fade,
                    UserMessageType::VguiMenu,
                    UserMessageType::Rumble,
                    UserMessageType::Battery,
                    UserMessageType::Damage,
                    UserMessageType::VoiceMask,
                    UserMessageType::RequestState,
                    UserMessageType::CloseCaption,
                    UserMessageType::HintText,
                    UserMessageType::KeyHintText,
                    UserMessageType::SquadMemberDied,
                    UserMessageType::AmmoDenied,
                    UserMessageType::CreditsMsg,
                    UserMessageType::CreditsPortalMsg,
                    UserMessageType::LogoTimeMsg,
                    UserMessageType::AchievementEvent,
                    UserMessageType::EntityPortalled,
                    UserMessageType::KillCam
                ];
                self.send_prop_amount_of_bits_to_get_num_bits = 6;
                self.send_prop_type_list = Vec::new();
                self.send_prop_type_list.push(SendPropType::Int);
                self.send_prop_type_list.push(SendPropType::Float);
                self.send_prop_type_list.push(SendPropType::Vector3);
                self.send_prop_type_list.push(SendPropType::String);
                self.send_prop_type_list.push(SendPropType::Array);
                self.send_prop_type_list.push(SendPropType::DataTable);
            },
            15 => {
                self.game = Game::PORTAL_1_5135;
                self.user_message_list = vec![
                    UserMessageType::Geiger,
                    UserMessageType::Train,
                    UserMessageType::HudText,
                    UserMessageType::SayText,
                    UserMessageType::SayText2,
                    UserMessageType::TextMsg,
                    UserMessageType::HudMsg,
                    UserMessageType::ResetHUD,
                    UserMessageType::GameTitle,
                    UserMessageType::ItemPickup,
                    UserMessageType::ShowMenu,
                    UserMessageType::Shake,
                    UserMessageType::Fade,
                    UserMessageType::VguiMenu,
                    UserMessageType::Rumble,
                    UserMessageType::Battery,
                    UserMessageType::Damage,
                    UserMessageType::VoiceMask,
                    UserMessageType::RequestState,
                    UserMessageType::CloseCaption,
                    UserMessageType::HintText,
                    UserMessageType::KeyHintText,
                    UserMessageType::SquadMemberDied,
                    UserMessageType::AmmoDenied,
                    UserMessageType::CreditsMsg,
                    UserMessageType::CreditsPortalMsg,
                    UserMessageType::LogoTimeMsg,
                    UserMessageType::AchievementEvent,
                    UserMessageType::EntityPortalled,
                    UserMessageType::KillCam,
                    UserMessageType::SPHapWeaponEvent,
                    UserMessageType::HapDmg,
                    UserMessageType::HapPunch,
                    UserMessageType::HapSetDrag,
                    UserMessageType::HapSetConstForce,
                    UserMessageType::HapMeleeContact
                ];
                self.send_prop_amount_of_bits_to_get_num_bits = 7;
                self.send_prop_type_list = Vec::new();
                self.send_prop_type_list.push(SendPropType::Int);
                self.send_prop_type_list.push(SendPropType::Float);
                self.send_prop_type_list.push(SendPropType::Vector3);
                self.send_prop_type_list.push(SendPropType::Vector2);
                self.send_prop_type_list.push(SendPropType::String);
                self.send_prop_type_list.push(SendPropType::Array);
                self.send_prop_type_list.push(SendPropType::DataTable);
            },
            24 => {
                self.game = Game::PORTAL_1_1910503;
                self.user_message_list = vec![
                    UserMessageType::Geiger,
                    UserMessageType::Train,
                    UserMessageType::HudText,
                    UserMessageType::SayText,
                    UserMessageType::SayText2,
                    UserMessageType::TextMsg,
                    UserMessageType::HudMsg,
                    UserMessageType::ResetHUD,
                    UserMessageType::GameTitle,
                    UserMessageType::ItemPickup,
                    UserMessageType::ShowMenu,
                    UserMessageType::Shake,
                    UserMessageType::Fade,
                    UserMessageType::VguiMenu,
                    UserMessageType::Rumble,
                    UserMessageType::Battery,
                    UserMessageType::Damage,
                    UserMessageType::VoiceMask,
                    UserMessageType::RequestState,
                    UserMessageType::CloseCaption,
                    UserMessageType::HintText,
                    UserMessageType::KeyHintText,
                    UserMessageType::SquadMemberDied,
                    UserMessageType::AmmoDenied,
                    UserMessageType::CreditsMsg,
                    UserMessageType::CreditsPortalMsg,
                    UserMessageType::LogoTimeMsg,
                    UserMessageType::AchievementEvent,
                    UserMessageType::EntityPortalled,
                    UserMessageType::KillCam,
                    UserMessageType::CallVoteFailed,
                    UserMessageType::VoteStart,
                    UserMessageType::VotePass,
                    UserMessageType::VoteFailed,
                    UserMessageType::VoteSetup,
                    UserMessageType::SPHapWeaponEvent,
                    UserMessageType::HapDmg,
                    UserMessageType::HapPunch,
                    UserMessageType::HapSetDrag,
                    UserMessageType::HapSetConstForce,
                    UserMessageType::HapMeleeContact
                ];
                self.send_prop_amount_of_bits_to_get_num_bits = 7;
                self.send_prop_type_list = Vec::new();
                self.send_prop_type_list.push(SendPropType::Int);
                self.send_prop_type_list.push(SendPropType::Float);
                self.send_prop_type_list.push(SendPropType::Vector3);
                self.send_prop_type_list.push(SendPropType::Vector2);
                self.send_prop_type_list.push(SendPropType::String);
                self.send_prop_type_list.push(SendPropType::Array);
                self.send_prop_type_list.push(SendPropType::DataTable);
            },
            _ => self.game = Game::UNKNOWN
        };
    }

    // add one because 0th tick (?)
    pub fn get_measured_ticks_and_time(&self) -> (i32, f32) {
        return (self.last_packet_tick + 1, ((self.last_packet_tick as f32) + 1f32) * 0.015)
    }

    // this has a bunch of "+ 1" because yes
    pub fn get_adjusted_ticks_and_time(&self) -> (i32, f32) {
        if self.adj_end_tick == 0 && self.adj_start_tick != 0 {
            return (self.last_packet_tick - self.adj_start_tick + 1, ((self.last_packet_tick - self.adj_start_tick + 1) as f32 * 0.015));
        } else if self.adj_end_tick != 0 && self.adj_start_tick == 0 {
            return (self.adj_end_tick + 1, ((self.adj_end_tick + 1) as f32 * 0.015));
        } else {
            return self.get_measured_ticks_and_time();
        }
    }
}