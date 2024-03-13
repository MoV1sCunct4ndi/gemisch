use crate::amount::Amount;

pub mod amount;

#[derive(Debug)]
pub struct Recipe {
    name: String,
    description: String,
    ingredients: Vec<(Ingredient, Amount)>,
    instructions: Vec<String>,
    tags: Vec<Tag>
}

#[derive(Debug)]
pub struct Ingredient {
    name:String,
    alcohol_content:f32,
    tags:Vec<Tag>,
}

#[derive(PartialEq, Debug)]
pub enum Tag{
    Defined(String),
    Custom(String),
    Auto(AutoTag),
}

#[derive(PartialEq, Debug)]
pub enum AutoTag {
    Alcoholic,
    NonAlcoholic,
}

impl Ingredient {
    pub fn new(name: String) -> Self {
        Self {
            name,
            alcohol_content: 0f32,
            tags:Vec::new(),
        }
    }

    pub fn with_alcohol(name: String, alcohol_content: f32) -> Self{
        Self{
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

    pub fn with_alcohol_and_tags(name: String, alcohol_content: f32, mut tags: Vec<Tag>)-> Self{
        tags.push(Tag::Auto(AutoTag::Alcoholic));
        Self {
            name,
            alcohol_content,
            tags,
        }
    }
}

fn auto_tags_for(ingredients: &[(Ingredient, Amount)]) -> Vec<Tag> {
    if ingredients.iter().any(|e|e.0.tags.contains(&Tag::Auto(AutoTag::Alcoholic))) {
        vec![Tag::Auto(AutoTag::Alcoholic)]
    } else {
        Vec::new()
    }
}

impl Recipe {
    pub fn new(name: String, ingredients: Vec<(Ingredient, Amount)>) -> Self {
        let tags = auto_tags_for(&ingredients);
        Self {
            name,
            description: String::new(),
            ingredients,
            instructions: Vec::new(),
            tags
        }
    }
}

