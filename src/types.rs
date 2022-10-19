fn default_string() -> String {
    "-".to_string()
}

#[derive(Clone, Debug)]
pub struct Category {
    pub name: String,
    pub value: String,
}

#[derive(Clone, serde::Deserialize, Debug)]
pub struct Artifact {
    pub name: String,
    pub max_rarity: u8,
    #[serde(rename = "2-piece_bonus", default = "default_string")]
    pub two_piece_bonus: String,
    #[serde(rename = "4-piece_bonus", default = "default_string")]
    pub four_piece_bonus: String,
}

#[derive(Clone, serde::Deserialize, Debug)]
pub struct Character {
    pub affiliation: String,
    pub birthday: String,
    pub constellation: String,
    pub constellations: Vec<Talent>,
    pub description: String,
    pub name: String,
    pub nation: String,
    #[serde(rename = "passiveTalents")]
    pub passive_talents: Vec<Talent>,
    pub rarity: u8,
    #[serde(rename = "skillTalents")]
    pub skill_talents: Vec<Talent>,
    pub title: String,
    pub vision: String,
    pub vision_key: String,
    pub weapon: String,
    pub weapon_type: String,
}

#[derive(Clone, serde::Deserialize, Debug)]
pub struct Talent {
    pub description: String,
    pub level: u8,
    pub name: String,
    pub unlock: String,
}