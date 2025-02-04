use std::io; 
use rand::Rng; 
use std::time::Instant;
use std::collections::HashMap;

fn main() {

    println!("Enter your name");

    let mut scores = HashMap::new();

    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();

    let name= name.trim().to_string();
    
    println!("Welcome to GuessIT, {:?}", name);

    let secret_number = rand::thread_rng().gen_range(1..=5);
    let mut attempt = 0;
    let total_attempt= 2;
    let start_time= Instant::now;

    loop{

        if attempt >= total_attempt{
            println!("Game over!!!, Guess better next time,  {} is the number",secret_number);

            break;
        }


        println!("Let's play guessIT, Enter your first Guess");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).unwrap();

        let guess :u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) =>{
                println!("Enter number again");

                continue;
            }
        };


         attempt +=1;

        if guess > secret_number{
            println!("Guess too high, {} attempt left",total_attempt- attempt);
        }else if guess < secret_number{
            println!("Guess too low, {} attempt left", total_attempt - attempt);

            scores.insert(name.clone(),attempt);
        }else{

            let elapsed_time= start_time().elapsed();

            println!("Congratulations guess RIGHT!!! {}",secret_number);
            println!("Time take for the guess after attempts, {}", elapsed_time.as_secs_f64());
            scores.insert(name.clone(),attempt);
            break;
        }
    }
println!("Your final scores is {:?}", scores)
  
}
