use std::io;

fn two_sum(num1: i32, num2: i32) -> i32 {
    let sum = num1 + num2;
    return sum; 
}

fn two_minus(num1: i32, num2: i32) -> i32 {
    num1 - num2
}

fn main() {
    // comments work the same!

    // a macro are like built in methods? I guess..
    // write to console with "print" or "printLn"

    // basically the same, but printLn writes to new line

    let mut input: String = String::new(); // variables are immutable by default 
    io::stdin()
        .read_line( &mut input)
        .expect("Failed to read line");

    // we use the "mut" keyword to make it mutable 

    let sum = two_sum(10, 5);

    let minus = two_minus(sum, 7);

    let message = "The sum is: ";
    let second_message = "The minus of the previous sum is";

    println!("{}", input);

    println!("{} {}", message, sum);

    println!("{} {}", second_message, minus)
}
