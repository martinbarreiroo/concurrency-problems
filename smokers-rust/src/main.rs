// idea: una mesa con 3 fumadores (uno con un ingrediente de cigarrillo distinto, papel, tabaco, fosforo)
// en la mesa se ponen dos ingredientes random, solo fuma el que tiene la cominacion de los 3 distintos
use std::cmp::PartialEq;
use std::sync::{Arc, Condvar, Mutex};
use std::thread::sleep;
use std::time::Duration;
use rand::Rng;


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Ingredient {
    PAPER,
    TOBACCO,
    MATCH
}

impl Ingredient {
    fn as_usize(&self) -> usize {
        match self {
            Ingredient::PAPER => 0,
            Ingredient::TOBACCO => 1,
            Ingredient::MATCH => 2,
        }
    }
}

struct SmokerTable {
    ingredients: Mutex<Vec<Ingredient>>,
    all_ingredients: Condvar
}


impl SmokerTable {
    fn new(ingredients: Mutex<Vec<Ingredient>>) -> Self {
        SmokerTable {
            ingredients,  all_ingredients: Condvar::new()
        }
    }

    fn acquire_ingredients(&self, smoker_ingredient: Ingredient) {
        let mut ingredients = self.ingredients.lock().unwrap();

        while ingredients.contains(&smoker_ingredient) {
            ingredients = self.all_ingredients.wait(ingredients).unwrap();
        }

        println!("Smoker {} is smoking\n", smoker_ingredient.as_usize());
    }

    fn replenish_ingredients(&self) {
        let mut all_ingredients = vec![
            Ingredient::PAPER,
            Ingredient::TOBACCO,
            Ingredient::MATCH,
        ];
        let mut rng = rand::thread_rng();

        // Remove one random ingredient, leaving two
        let idx = rng.random_range(0..all_ingredients.len());
        all_ingredients.remove(idx);

        let mut ingredients = self.ingredients.lock().unwrap();
        ingredients.clear();
        ingredients.extend(all_ingredients);
        self.all_ingredients.notify_all();
    }
}

struct Smoker {
    id: usize,
    table: Arc<SmokerTable>,
    ingredient: Ingredient,
}

impl Smoker {
    fn new(id: usize, table: Arc<SmokerTable>, ingredient: Ingredient) -> Self {
        Smoker {id, table, ingredient}
    }

    fn smoke(&self) {
        self.table.acquire_ingredients(self.ingredient);
        sleep(Duration::from_millis(5000));
        self.table.replenish_ingredients();
    }
}

fn main() {
    use std::thread;
    use std::sync::Arc;

    let table = Arc::new(SmokerTable::new(Mutex::new(vec![Ingredient::PAPER, Ingredient::TOBACCO])));

    let smokers = [
        Ingredient::PAPER,
        Ingredient::TOBACCO,
        Ingredient::MATCH,
    ];

    let mut handles = Vec::new();

    for &ingredient in &smokers {
        let table_clone = Arc::clone(&table);
        handles.push(thread::spawn(move || {
            let smoker = Smoker::new(ingredient.as_usize() , table_clone, ingredient);
            loop {
                print!("Smoker {} wants to smoke.\n", smoker.id);
                smoker.smoke();
                println!("Smoker {} finished smoking and replenished the table with two random ingredients.\n", smoker.id);
            }
        }));
    }

    for handle in handles {
         handle.join().unwrap();
    }
}
