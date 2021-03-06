use std::io;
use std::cmp::Ordering;
use rand::Rng;
// above is dependency.

fn main() {
    
    println!("Guess the number");
    
    let secret_number = rand::thread_rng().gen_range(1,101);

    let mut guesses_count = 1;

    println!("Please input your guess!");
    
    loop {

        let mut guess = String::new();
    
        io::stdin()
            // & means reference of a variable.
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You guessed: {}",guess);
    
        if check_guess(guess,secret_number){
            println!("Correct!");
            println!("Guesses: {}",guesses_count);
            break;
        }
        guesses_count += 1;
    }        
}

fn check_guess(guess: u32, secret_number: u32) -> bool{

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => return true,
    }

    return false;

}
