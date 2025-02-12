fn main() {
    let x = five();
    another_function(x);

    let y = plus_one(x);
    print_labeled_measurement(y, 'h');
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
} 

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
    
    let y: i32 = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}
 
fn another_function(x: i32) {
    println!("The value of x is: {x}");
} 
