use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct RaidProgression {
    pub summary: String,
    pub total_bosses: u8,
    pub normal_bosses_killed: u8,
    pub heroic_bosses_killed: u8,
    pub mythic_bosses_killed: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildProfile {
    pub name: String,
    pub raid_progression: HashMap<String, RaidProgression>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildProfileMembers {
    pub name: String,
    pub members: Vec<GuildMember>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildMember {
    pub rank: u8,
    pub character: Character,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Character {
    pub name: String,
    pub realm: String,
    pub class: String,
    pub active_spec_name: String,
}
