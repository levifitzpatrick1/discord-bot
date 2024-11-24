use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RecipeResponse {
    pub _links: Links,
    pub id: u32,
    pub name: Option<String>, 
    pub description: Option<String>,
    pub media: RecipeMedia,
    pub reagents: Option<Vec<Reagent>>,
    pub modified_crafting_slots: Option<Vec<SlotType>>,
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
pub struct RecipeMedia {
    pub key: Href,
    pub id: u32,
}

#[derive(Debug, Deserialize)]
pub struct Reagent {
    pub reagent: ReagentItem,
    pub quantity: u32,
}

#[derive(Debug, Deserialize)]
pub struct ReagentItem {
    pub key: Href,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Deserialize)]
pub struct SlotType {
    pub slot_type: SlotTypeDetails,
    pub display_order: u32,
}

#[derive(Debug, Deserialize)]
pub struct SlotTypeDetails {
    pub key: Href,
    pub name: String,
    pub id: u32,
}
