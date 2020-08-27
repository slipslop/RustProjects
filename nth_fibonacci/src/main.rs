use std::io;
fn main() {

    println!("Which Fibonacci number would you like to retrieve?");

    let mut index = String::new();
        
    io::stdin()
        // & means reference of a variable.
        .read_line(&mut index)
        .expect("Failed to read line");

        //remove whitespace
    let n: u32 = index
                .trim()
                .parse()
                .expect("Invalid input");

    let mut current = 1;

    let mut a;
    let (mut b, mut total) = (1,0);

    while current <= n {
        a = b;
        b = total;
        total = a + b;
        
        current += 1;
    }
    
    println!("Fibonacci number at index {} is: {}",n,total);
}
