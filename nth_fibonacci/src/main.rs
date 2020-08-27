use std::io;
fn main() {

    println!("Which Fibonacci number would you like to retrieve?");

    let mut index = String::new();
        
    index = read_input(index);

    let n = convert_string_to_u32(index);

    let res = generate_nth_fibonacci(n);
    
    println!("Fibonacci number at index {} is: {}",n,res);
    
}

fn read_input(mut string:String) -> String{

    io::stdin()
        // & means reference of a variable.
        .read_line(&mut string)
        .expect("Failed to read line");
    return string;

}

fn convert_string_to_u32(string: String) -> u32 {

    let n: u32 = string
        .trim()
        .parse()
        .expect("Invalid input");
    return n;

}

fn generate_nth_fibonacci(index: u32) -> u32 {

    let mut current = 1;
    let mut a;

    let (mut b, mut total) = (1,0);
    while current <= index {
        a = b;
        b = total;
        total = a + b;
        
        current += 1;
    }
    return total;

}