use std::io; 
use rand::Rng; 

fn main() {
    
    println!("Welcome to GuessIT");

    let secret_number = rand::thread_rng().gen_range(1..=5);

    loop{

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).unwrap();

        let guess :u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) =>{
                println!("Enter number again");

                continue;
            }
        };



        if guess > secret_number{
            println!("Guess too high");
        }else if guess < secret_number{
            println!("Guess too low");
        }else{
            println!("Congratulations guess RIGHT!!! {}",secret_number);
            break;
        }
    }

  
}
