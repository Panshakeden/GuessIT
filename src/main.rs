use std::io; 
use rand::Rng; 

fn main() {
    
    println!("Welcome to GuessIT");

    let secret_number = rand::thread_rng().gen_range(1..=50);
}
