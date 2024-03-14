use serde::{Deserialize, Serialize};
use crate::amount::Amount;

pub mod amount;

#[cfg_attr(doc, aquamarine::aquamarine)]
/// text
/// ```mermaid
/// classDiagram
/// class Recipe {
///     -name: String
///     -description: String
///     -instructions: Vec~String~
/// }
/// Recipe --o TODO
/// TODO --o Ingredient
/// TODO --o Amount
/// class Amount {
///     -value: f32
/// }
/// Amount --o MeasurementUnit
/// class MeasurementUnit {
///     <~enumeration~>
///     None
///     Mass
///     Volume
///     Other(String)
/// }
/// Recipe --o Tag
/// class Tag {
///     <~enumeration~>
///     Defined(String)
///     Custom(String)
///     Auto
/// }
/// class Ingredient {
///     -name: String
///     -alcohol_content: f32
/// }
/// Ingredient --o Tag
///
/// ```
/// more text
#[derive(Debug, Serialize, Deserialize)]
pub struct Recipe {
    pub name: String,
    pub description: String,
    pub ingredients: Vec<(String, Amount)>,
    pub instructions: Vec<String>,
    pub tags: Vec<Tag>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ingredient {
    pub name: String,
    pub alcohol_content: f32,
    pub tags: Vec<Tag>,
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

impl Recipe {
    pub fn new(name: String,  ingredients: Vec<(&Ingredient, Amount)>) -> Self {
        let tags = auto_tags_for(&ingredients);
        let ingredients = ingredients.into_iter().map(|(i, a)|(i.name.clone(), a)).collect();
        Self {
            name,
            description: String::new(),
            ingredients,
            instructions: Vec::new(),
            tags,
        }
    }
}
