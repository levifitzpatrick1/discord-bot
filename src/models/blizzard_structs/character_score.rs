use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MythicKeystoneProfileResponse {
    pub _links: Links,
    pub season: Season,
    pub best_runs: Vec<BestRun>,
    pub character: CharacterInfo,
    pub mythic_rating: MythicRating,
}

#[derive(Debug, Deserialize)]
pub struct Links {
    #[serde(rename = "self")]
    pub self_link: Href,
}

#[derive(Debug, Deserialize)]
pub struct Href {
    pub href: String,
}

#[derive(Debug, Deserialize)]
pub struct Season {
    pub key: Href,
    pub id: u32,
}

#[derive(Debug, Deserialize)]
pub struct BestRun {
    pub completed_timestamp: u64,
    pub duration: u32,
    pub keystone_level: u32,
    pub keystone_affixes: Vec<KeystoneAffix>,
    pub members: Vec<Member>,
    pub dungeon: Dungeon,
    pub is_completed_within_time: bool,
    pub mythic_rating: Rating,
    pub map_rating: Rating,
}

#[derive(Debug, Deserialize)]
pub struct KeystoneAffix {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Deserialize)]
pub struct Member {
    pub character: MemberCharacter,
    pub specialization: Specialization,
    pub race: Race,
    pub equipped_item_level: u32,
}

#[derive(Debug, Deserialize)]
pub struct MemberCharacter {
    pub name: String,
    pub id: u64,
    pub realm: Realm,
}

#[derive(Debug, Deserialize)]
pub struct Realm {
    pub key: Href,
    pub id: u32,
    pub slug: String,
}

#[derive(Debug, Deserialize)]
pub struct Specialization {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Deserialize)]
pub struct Race {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Deserialize)]
pub struct Dungeon {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Deserialize)]
pub struct Rating {
    pub color: Color,
    pub rating: f64,
}

#[derive(Debug, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f32,
}

#[derive(Debug, Deserialize)]
pub struct CharacterInfo {
    pub key: Href,
    pub name: String,
    pub id: u64,
    pub realm: Realm,
}

#[derive(Debug, Deserialize)]
pub struct MythicRating {
    pub color: Color,
    pub rating: f64,
}
