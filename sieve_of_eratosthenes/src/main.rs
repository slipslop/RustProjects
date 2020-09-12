use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Not enough arguments");
        process::exit(1);
    }

    let n = &args[1];

    let primes_count: usize = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => process::exit(1)
    };
    
    let mut primes = create_vector(primes_count);

    primes[0] = false;

    if primes_count >= 1 {
        primes[1] = false;
    }

    for num in 2..primes_count+1 {
        if primes[num] {
            let mut multiple = num*num;
            while multiple <= primes_count {
                primes[multiple] = false;
                multiple += num;
            }
        }
    }

    for num in 0..primes.len(){
        if primes[num] == true {
            println!("{}",num);
        }
    }

}

fn create_vector(primes_count: usize) -> Vec<bool> {

    let vector: Vec<bool> = vec![true; primes_count+1];
    return vector;

}
