use gemisch::{Ingredient, Recipe};
use gemisch::amount::{Amount, MeasurementUnit, Range};

fn main() {
    let coke_string = std::fs::read_to_string("coke.json").expect("could not find or read coke.json");
    let coke= serde_json::from_str(&coke_string).expect("could not deserialize coke json string");

    //let coke = Ingredient::new("Cola".to_owned());
    let apple_juice = Ingredient::new("Apfelsaft".to_owned());
    let apple_coke = Recipe::new("Apfelcola".to_owned(), vec![
        (&coke, Amount { value: 0.2.into(), unit: MeasurementUnit::Volume }),
        (&apple_juice, Amount { value: 0.2.into(), unit: MeasurementUnit::Volume }),
    ]);
    println!("{apple_coke:#?}");
    let range = Range::from(0.2);
    println!("{range}");
    println!("{}", serde_json::to_string(&apple_coke).unwrap());
}
