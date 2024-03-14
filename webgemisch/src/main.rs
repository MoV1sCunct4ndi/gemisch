use std::collections::HashMap;
use yew::prelude::*;
use gemisch::{Ingredient, Recipe};
use gemisch::amount::{Amount, MeasurementUnit};

#[derive(Eq, Hash, PartialEq, Clone)]
struct RecipeIdentifier(String);

impl From<String> for RecipeIdentifier {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[derive(Default)]
enum State {
    #[default]
    ListRecipes,
    ShowRecipe(RecipeIdentifier),
}

#[derive(Default)]
struct App {
    state: State,
    ingredients: HashMap<String, Ingredient>,
    recipes: HashMap<RecipeIdentifier, Recipe>,
}

fn list_recipes(recipes: &HashMap<RecipeIdentifier, Recipe>, ctx: &Context<App>) -> Html {
    html!{
        <div>
            <h1>{"Recipes"}</h1>
            <ul>
                { for recipes.iter().map(|(_, v)|{
                    let id: RecipeIdentifier = v.name.clone().into();
                    let onclick = ctx.link().callback(move |_| Message::ShowRecipe(id.clone()));
                    html!{ <li {onclick}>{&v.name}</li> }
                })}
            </ul>
        </div>
    }
}

fn show_recipe(recipe: &Recipe) -> Html {
    html! {
        <>
            <div> {"Recipe"} </div>
            <div> {&recipe.name} </div>
            <div> {&recipe.description } </div>
            <ul>
                { for recipe.ingredients.iter().map(|(ingredient_name, amount) | {
                    html! {
                        <li> {amount} {" "} {ingredient_name} </li>
                    }
                })}
            </ul>
            <ol>
                { for recipe.instructions.iter().map(| instruction | {
                    html! {
                        <li> {instruction} </li>
                    }
                })}
            </ol>
        </>

    }
}

enum Message {
    ShowRecipe(RecipeIdentifier),
}

impl Component for App {
    type Message = Message;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let mut self_ = Self::default();
        let coke = Ingredient::new("Cola".to_owned());
        let apple_juice = Ingredient::new("Apfelsaft".to_owned());
        let mut apple_coke = Recipe::new("Apfelcola".to_owned(), vec![
            (&coke, Amount { value: 0.2.into(), unit: MeasurementUnit::Volume }),
            (&apple_juice, Amount { value: 0.2.into(), unit: MeasurementUnit::Volume }),
        ]);
        apple_coke.instructions = vec!["Cola eingießen".to_owned(), "Apfelsaft eingießen".to_owned()];
        self_.ingredients.insert(coke.name.clone(), coke);
        self_.ingredients.insert(apple_juice.name.clone(), apple_juice);
        self_.recipes.insert(apple_coke.name.clone().into(), apple_coke);
        self_
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg { Message::ShowRecipe(recipe) => { self.state = State::ShowRecipe(recipe) } };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match &self.state {
            State::ListRecipes => list_recipes(&self.recipes, ctx),
            State::ShowRecipe(recipe) => {
                match self.recipes.get(recipe) {
                    None => html! {<h1>{format!("Recipe '{}' not found", recipe.0)}</h1>},
                    Some(recipe) => show_recipe(recipe),
                }
            },
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}