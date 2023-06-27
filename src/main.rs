use std::io;
use rand::Rng;

fn two_sum(num1: i32, num2: i32) -> i32 {
    let sum = num1 + num2;
    return sum; 
}

fn two_minus(num1: i32, num2: i32) -> i32 {
    num1 - num2 
}

fn promote_love(name: &str) -> &str{
   println!("{0} love's everyone", name);
   return "declaration";
}

fn useTuple() {
    //define a tuple
    let person_data = ("Alex", 48, "35kg", "6ft");
    // define a tuple with type annotated
    let person_data : (&str, i32, &str, &str) = ("Alex", 48, "35kg", "6ft");
    
    let planet_data : (&str, i32, char, &str) = ("Gorgon Prime", 900_000, 'c', "Pegasus Galaxy");
    // wont work -> println!("{}", planet_data);
    println!("");
    println!("{:?}", planet_data); // planet_data;
    // you have to use the debug trait for tuples; 

    // get individual values out of tuple
    let (w ,x, y, z) = planet_data;

    //print values
    println!("Name : {}", w);
    println!("Age : {}", x);
    println!("Class : {}", y);
    println!("Location : {}", z);
}

fn getNames(){
   let mut line = String::new();
   println!("Enter your name :");
   let b1 = std::io::stdin().read_line(&mut line).unwrap();
   println!("Hello , {}", line);
   println!("no of bytes read , {}", b1);
}


fn main() {
    // comments work the same!

promote_love("Ramone");
    // a macro are like built in methods? I guess..
    // write to console with "print" or "printLn"

    // basically the same, but printLn writes to new line

    /*
     let secret_number = rand::thread_rng().gen_range(1, 15);

    let mut input: String = String::new(); // variables are immutable by default 
    io::stdin()
        .read_line( &mut input)
        .expect("Failed to read line");

    // we use the "mut" keyword to make it mutable 

    let sum = two_sum(10, 5);

    let minus = two_minus(sum, 7);

    let message = "The sum is: ";
    let second_message = "The minus of the previous sum is";
    let this_is_my_message = "test something something";

    println!("The secret number is: {}", secret_number);

    if input == "add " {
        println!("you chose add");
    } else {
        println!("you did not choose add");
    };

    println!("{}", this_is_my_message);
    println!("{}", input.trim()); 

    println!("{} {}", message, sum);

    println!("{} {}", second_message, minus)
     */

    //println!("Hello Cruel World!");
    println!("This is {0}. This is {1}", "Batman", "Robin");

    // in string formatting, position arguments can be used inside `{}`
    println!("{0} And now this is {1} and this is {0}", "WonderWoman", "SuperGirl");

    // specific string patterns
    println!("{subject}{verb}{object}", object="dans le parc ", verb="courir ", subject="le rapid lupin ");

     println!("{number:0>width$}", number=1, width=50);

    // throws an error println!("My name is {0}, {1} {0}", "Bond"); 
    println!("My name is {0}, {1} {0}", "Bond", "James");

    let people = "Jamaicans";
     // will throw an error because you cannot use variables without a placeholder `{}` 
     // println!(people);
    println!("{}", people);
    println!("{} people like to {}", people, "dance.");

    // print errors
    eprint!("Coding errors"); // print one line
    eprintln!("being handled like this is cool"); // print with new line 

    let mut xman = "Wolverine";
    println!("{}", xman); // print new line 
    xman = "Night Crawler";
    println!("{}", xman); // print new line 

    let (country, food) = ("Japan", "Sushi");
    println!("{} comes from {}", food, country);

        //implicitly define an integer
    let a = 21; 
    let b = 1;
    let c = 54;
    let d = 343434;
    //print the variable
    println!("a: {}", a);
    println!("b: {}", b);
    println!("c: {}", c);
    println!("d: {}", d);

    let value: &str = "Rust Programming";
println!("{}", value);


// arrays 

let fruits: [&str; 4] = ["apple", "banana", "pineapple", "pear"];
// type, semi-colon, size
// use mut to make array mutable 
let mut ages: [i32; 3] = [21, 22, 23];

println!("I was {}", ages[2]);
ages[2] = 25;
println!("But then I was {}", ages[2]);
// consts must be caps, typed and cannot be mutated (will error )
const POWER_LEVEL: u32 = 9000;

println!("HIS POWER LEVEL IS OVER {}", POWER_LEVEL);

useTuple();

 let arr: [i32; 6] = [ 0,2,4,6,8,10];
 println!("{:?}", arr);


 getNames();


    

}
