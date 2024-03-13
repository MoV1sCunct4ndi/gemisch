use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::amount::Amount;

pub mod amount;

#[derive(Debug, Serialize, Deserialize)]
pub struct Recipe<'a> {
    name: String,
    description: String,
    ingredients: Vec<(IngredientRef<'a>, Amount)>,
    instructions: Vec<String>,
    tags: Vec<Tag>,
}

#[derive(Debug)]
pub enum IngredientRef<'a> {
    Ref(&'a Ingredient),
    Name(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ingredient {
    name: String,
    alcohol_content: f32,
    tags: Vec<Tag>,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum Tag {
    Defined(String),
    Custom(String),
    Auto(AutoTag),
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum AutoTag {
    Alcoholic,
    NonAlcoholic,
}

impl Ingredient {
    pub fn new(name: String) -> Self {
        Self {
            name,
            alcohol_content: 0f32,
            tags: Vec::new(),
        }
    }

    pub fn with_alcohol(name: String, alcohol_content: f32) -> Self {
        Self {
            name,
            alcohol_content,
            tags: vec![Tag::Auto(AutoTag::Alcoholic)],
        }
    }

    pub fn with_tags(name: String, tags: Vec<Tag>) -> Self {
        Self {
            name,
            alcohol_content: 0f32,
            tags,
        }
    }

    pub fn with_alcohol_and_tags(name: String, alcohol_content: f32, mut tags: Vec<Tag>) -> Self {
        tags.push(Tag::Auto(AutoTag::Alcoholic));
        Self {
            name,
            alcohol_content,
            tags,
        }
    }
}

fn auto_tags_for(ingredients: &[(&Ingredient, Amount)]) -> Vec<Tag> {
    if ingredients.iter().any(|e| e.0.tags.contains(&Tag::Auto(AutoTag::Alcoholic))) {
        vec![Tag::Auto(AutoTag::Alcoholic)]
    } else {
        Vec::new()
    }
}

impl<'a> Recipe<'a> {
    pub fn new(name: String,  ingredients: Vec<(&'a Ingredient, Amount)>) -> Self {
        let tags = auto_tags_for(&ingredients);
        let ingredients = ingredients.into_iter().map(|(i, a)| (IngredientRef::Ref(i), a)).collect();
        Self {
            name,
            description: String::new(),
            ingredients,
            instructions: Vec::new(),
            tags,
        }
    }
}

impl Serialize for IngredientRef<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        match self {
            IngredientRef::Ref(ingredient) => serializer.serialize_str(&ingredient.name),
            IngredientRef::Name(name) => serializer.serialize_str(&name),
        }
    }
}

impl<'de> Deserialize<'de> for IngredientRef<'_> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let name = String::deserialize(deserializer)?;
        Ok(IngredientRef::Name(name))
    }
}
