fn main() {
    let mut count = 0;

    loop {
        if count > 10 {
            break;
        }

        println!("Count: {}", count);
        count += 1;
    }

    return_val_loop();
    while_loop();
    while_over_collection();
    for_over_collection();
    for_with_countdown();
}

fn return_val_loop() {
    let mut count = 0;
    let result = loop {
        if count > 10 {
            break count + 1;
        }
        count += 1;
    };
    println!("My result: {}", result);
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("The number: {}", number);
        number -= 1;
    }

    println!("And... we're done.");
}

fn while_over_collection() {
    let a = [1;5];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", a[index]);
        index += 1
    }
}

fn for_over_collection() {
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("The element is: {}", element);
    }
}

fn for_with_countdown() {
    for number in (1..4).rev() {
        println!("Ready: {}", number);
    }
    println!("Liftoffffff!");
}
