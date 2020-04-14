fn main() {
    let number = 3;

    if number < 5 {
        println!("Condition is true");
    } else {
        println!("Condition is false");
    }

    long_ifs();
    conditional_if();
}

fn long_ifs() {
    let number = 6;

    if number % 4 == 0 {
        println!("Number is divis by 4");
    } else if number % 3 == 0 {
        println!("Number is divis by 3");
    } else if number % 3 == 0 {
        println!("Number is divis by 2");
    } else {
        println!("Number is ya know");
    }
}

fn conditional_if() {
    let condition = true;

    let number = if condition {
        5
    } else {
        6
    };

    println!("What is the num? {}", number);
}
