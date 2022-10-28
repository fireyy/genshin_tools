fn default_string() -> String {
    "".to_string()
}

fn default_usize() -> u8 {
    1
}

fn default_vec() -> Vec<String> {
    vec![]
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
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
    #[serde(skip, default="default_vec")]
    pub icon: Vec<String>,
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
    #[serde(skip, default="default_string")]
    pub icon: String,
}

#[derive(Clone, serde::Deserialize, Debug)]
pub struct Talent {
    pub description: String,
    #[serde(default = "default_usize")]
    pub level: u8,
    pub name: String,
    pub unlock: String,
}

#[derive(Clone, serde::Deserialize, Debug)]
pub struct Food {
    pub description: String,
    pub effect: String,
    #[serde(rename = "hasRecipe")]
    pub has_recipe: bool,
    pub name: String,
    #[serde(default = "default_usize")]
    pub proficiency: u8,
    pub rarity: u8,
    #[serde(default = "default_recipe")]
    pub recipe: Vec<Recipe>,
    #[serde(rename = "type")]
    pub food_type: String,
    #[serde(skip, default="default_string")]
    pub icon: String,
}

#[derive(Clone, serde::Deserialize, Debug)]
pub struct Recipe {
    pub item: String,
    pub quantity: u8,
}

fn default_recipe() -> Vec<Recipe> {
    vec![]
}

#[derive(Clone, serde::Deserialize, Debug)]
pub struct Potion {
    pub effect: String,
    pub name: String,
    pub rarity: u8,
    pub crafting: Vec<Recipe>,
    #[serde(skip, default="default_string")]
    pub icon: String,
}