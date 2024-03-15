use gemisch::amount::{Amount, MeasurementUnit};
use gemisch::{Ingredient, Recipe};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Eq, Hash, PartialEq, Clone)]
struct RecipeIdentifier(String);

impl From<String> for RecipeIdentifier {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Display for RecipeIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for RecipeIdentifier {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.to_string().into())
    }
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/recipe/:id")]
    Recipe { id: RecipeIdentifier },
}

fn list_recipes(recipes: &HashMap<RecipeIdentifier, Recipe>) -> Html {
    html! {
        <div>
            <h1>{"Recipes"}</h1>
            <ul>
                { for recipes.iter().map(|(_, v)|{
                    let id: RecipeIdentifier = v.name.clone().into();
                    html!{ <li><Link<Route> to={Route::Recipe{id}}>{&v.name}</Link<Route>></li> }
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

#[derive(Default)]
struct DataStore {
    ingredients: HashMap<String, Ingredient>,
    recipes: HashMap<RecipeIdentifier, Recipe>,
}

impl DataStore {
    fn new() -> Self {
        let mut self_ = Self::default();
        let coke = Ingredient::new("Cola".to_owned());
        let apple_juice = Ingredient::new("Apfelsaft".to_owned());
        let mut apple_coke = Recipe::new(
            "Apfelcola".to_owned(),
            vec![
                (
                    &coke,
                    Amount {
                        value: 0.2.into(),
                        unit: MeasurementUnit::Volume,
                    },
                ),
                (
                    &apple_juice,
                    Amount {
                        value: 0.2.into(),
                        unit: MeasurementUnit::Volume,
                    },
                ),
            ],
        );
        apple_coke.instructions = vec![
            "Cola eingießen".to_owned(),
            "Apfelsaft eingießen".to_owned(),
        ];
        self_.ingredients.insert(coke.name.clone(), coke);
        self_
            .ingredients
            .insert(apple_juice.name.clone(), apple_juice);
        self_
            .recipes
            .insert(apple_coke.name.clone().into(), apple_coke);
        self_
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let data = use_mut_ref(|| DataStore::new());

    let switch = move |route| match route {
        Route::Home => list_recipes(&data.borrow().recipes),
        Route::Recipe { id } => match data.borrow().recipes.get(&id) {
            None => html! {<h1>{format!("Recipe '{}' not found", id.0)}</h1>},
            Some(recipe) => show_recipe(&recipe),
        },
    };

    html! {
        <HashRouter>
            <Switch<Route> render={switch} />
        </HashRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
