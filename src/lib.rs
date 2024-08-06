use chrono::{DateTime, Utc};
use proc_macros::{api_struct, api_enum};

#[api_struct]
pub struct ApiResponse {
    pub data: Option<ApiResponseData>,
}

#[api_struct]
pub struct ApiResponseData {
    pub player_by_username: Option<Player>,
    pub player: Option<Player>,
    pub message: Option<String>,
    pub request_id : Option<String>
}

#[api_struct]
pub struct Player {
    pub uuid: String,
    pub username: Option<String>,
    pub ranks: Vec<Rank>,
    pub crown_level: CrownLevel,
    pub status: Option<Status>,
    pub collections: Option<Collections>,
    pub social: Option<Social>,
}

#[api_enum]
pub enum Rank {
    Champ,
    GrandChamp,
    GrandChampRoyale,
    Creator,
    Contestant,
    Moderator,
    Noxcrew
}

#[api_struct]
pub struct CrownLevel {
    pub level: u32,
    pub next_evolution_level: Option<u32>,
    pub next_level_progress: Option<ProgressionData>,
    pub trophies: TrophyData
}

#[api_struct]
pub struct ProgressionData {
    pub obtained: u32,
    pub obtainable: u32
}

#[api_struct]
pub struct TrophyData {
    pub obtained: u32,
    pub obtainable: u32,
    pub bonus: u32
}

#[api_struct]
pub struct Status {
    pub online: bool,
    pub server: Option<Server>,
    pub first_join: Option<DateTime<Utc>>,
    pub last_join: Option<DateTime<Utc>>
}

#[api_struct]
pub struct Server {
    pub category: ServerCategory,
    pub sub_type: String,
    pub associated_game: Option<Game>
}

#[api_enum]
pub enum ServerCategory {
    Lobby,
    Game,
    Limbo,
    Queue
}

#[api_enum]
pub enum Game {
    HoleInTheWall,
    Tgttos,
    BattleBox,
    SkyBattle,
    ParkourWarrior,
    Dynaball,
    RocketSpleef
}

#[api_struct]
pub struct Collections {
    pub currency: Currency
}

#[api_struct]
pub struct Currency {
    pub coins: u32,
    pub gems: u32,
    pub royal_reputation: u32,
    pub silver: u32,
    pub material_dust: u32
}

#[api_struct]
pub struct Social {
    pub friends: Vec<MinimalPlayer>,
    pub party: Party
}

#[api_struct]
pub struct Party {
    pub active: bool,
    pub leader: Option<MinimalPlayer>,
    pub members: Option<Vec<MinimalPlayer>>
}

#[api_struct]
pub struct MinimalPlayer {
    pub uuid: String,
    pub username: Option<String>
}