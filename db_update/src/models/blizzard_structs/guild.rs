use serde::Deserialize;

#[derive(Deserialize)]
pub struct GuildRosterResponse {
    pub _links: Links,
    pub guild: Guild,
    pub members: Vec<Member>,
}

#[derive(Deserialize)]
pub struct Links {
    #[serde(rename = "self")]
    pub self_link: SelfLink,
}

#[derive(Deserialize)]
pub struct SelfLink {
    pub href: String,
}

#[derive(Deserialize)]
pub struct Guild {
    pub key: KeyHref,
    pub name: String,
    pub id: u64,
    pub realm: Realm,
    pub faction: Faction,
}

#[derive(Deserialize)]
pub struct KeyHref {
    pub href: String,
}

#[derive(Deserialize)]
pub struct Realm {
    pub key: KeyHref,
    pub name: String,
    pub id: u32,
    pub slug: String,
}

#[derive(Deserialize)]
pub struct Faction {
    #[serde(rename = "type")]
    pub faction_type: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct Member {
    pub character: Character,
    pub rank: u32,
}

#[derive(Deserialize)]
pub struct Character {
    pub key: KeyHref,
    pub name: String,
    pub id: u64,
    pub realm: CharacterRealm,
    pub level: u32,
    pub playable_class: PlayableClass,
    pub playable_race: PlayableRace,
}

#[derive(Deserialize)]
pub struct CharacterRealm {
    pub key: KeyHref,
    pub id: u32,
    pub slug: String,
}

#[derive(Deserialize)]
pub struct PlayableClass {
    pub key: KeyHref,
    pub id: u32,
}

#[derive(Deserialize)]
pub struct PlayableRace {
    pub key: KeyHref,
    pub id: u32,
}