use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct ProfileResponse {
    pub _links: Links,
    pub character: Character,
    pub primaries: Vec<PrimaryProfession>,
    pub secondaries: Vec<SecondaryProfession>,
}

#[derive(Deserialize, Clone)]
pub struct Links {
    #[serde(rename = "self")]
    pub self_link: SelfLink,
}

#[derive(Deserialize, Clone)]
pub struct SelfLink {
    pub href: String,
}

#[derive(Deserialize, Clone)]
pub struct Character {
    pub key: KeyHref,
    pub name: String,
    pub id: u64,
    pub realm: Realm,
}

#[derive(Deserialize, Clone)]
pub struct KeyHref {
    pub href: String,
}

#[derive(Deserialize, Clone)]
pub struct Realm {
    pub key: KeyHref,
    pub name: String,
    pub id: u32,
    pub slug: String,
}

#[derive(Deserialize, Clone)]
pub struct PrimaryProfession {
    pub profession: Profession,
    pub tiers: Vec<Tier>,
}

#[derive(Deserialize, Clone)]
pub struct Profession {
    pub key: KeyHref,
    pub name: String,
    pub id: u32,
}

#[derive(Deserialize, Clone)]
pub struct Tier {
    pub skill_points: u32,
    pub max_skill_points: u32,
    pub tier: TierInfo,
    pub known_recipes: Vec<Recipe>,
}

#[derive(Deserialize, Clone)]
pub struct TierInfo {
    pub name: String,
    pub id: u32,
}

#[derive(Deserialize, Clone)]
pub struct Recipe {
    pub key: KeyHref,
    pub name: String,
    pub id: u32,
}

#[derive(Deserialize, Clone)]
#[serde(untagged)]
pub enum SecondaryProfession {
    WithTiers {
        profession: Profession,
        tiers: Vec<Tier>,
    },
    WithoutTiers {
        profession: Profession,
        skill_points: u32,
        max_skill_points: u32,
    },
}
