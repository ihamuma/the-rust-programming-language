fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let_statement(false);
}

fn let_statement(condition: bool) {
    let number = if condition { 5 } else { 6 };
//  let number = if condition { 5 } else { "six" }; This won't compile bc Rust needs to know the type of number ahead of time

    println!("The value of the number is {number}")
}