use crate::structs::utils::GameEventList;
use crate::structs::demo_header::DemoHeader;
use crate::structs::user_message::UserMessageType;

#[allow(non_camel_case_types)]
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
        }
    }

    pub fn get_info_from_header(&mut self, header: &DemoHeader) {
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
                    UserMessageType::HUDMsg,
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
                ]
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
                    UserMessageType::HUDMsg,
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
                ]
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
                    UserMessageType::HUDMsg,
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
                ]
            },
            _ => self.game = Game::UNKNOWN
        };
    }
}