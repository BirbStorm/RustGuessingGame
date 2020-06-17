use std::io;
use std::cmp::Ordering;

use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret = rand::thread_rng().gen_range(1, 101);

    loop{
        println!("Input a guess.");

        //allows guess to be mutable
        let mut guess = String::new();

        //could be written as std::io::stdin().readline without the use at the top
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        
        //shadowing
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed: {}", guess);
        match guess.cmp(&secret){
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {println!("You win!");break;},
            Ordering::Greater => println!("Too large!")
        }
    }
}
