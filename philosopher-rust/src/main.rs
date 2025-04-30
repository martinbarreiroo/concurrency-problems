mod table;
mod philosopher;

use std::fmt::Debug;
use std::sync::{Arc};
use std::thread::{spawn};
use crate::philosopher::Philosopher;
use crate::table::Table;

fn main() {
    let table = Arc::new(Table::new(5));
    let mut handles = vec![];

    for i in 0..5 {
        let table_clone = Arc::clone(&table);
        handles.push(spawn(move || {
            let philosopher = Philosopher::new(table_clone, i);
            loop {
                println!("Philosopher {} is thinking.", i);
                philosopher.think();

                println!("Philosopher {} is hungry.", i);
                philosopher.eat();

                println!("Philosopher {} is done eating.", i);
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
