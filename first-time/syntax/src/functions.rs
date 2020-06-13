fn print_number(x: i32) {
    println!("x is {}", x);
}

fn print_sum(x: i32, y: i32) {
    println!("sum is: {}", x + y);
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn diverge() -> ! {
    panic!("And here we are");
}

fn function_pointers() {
    // They are the same thing
    let f: fn(i32) -> i32 = add_one;
    let f = add_one;
}

pub fn main() {
    let x = 12;
    let y = 10;
    print_number(x);
    print_sum(x, y);
    let z = add_one(x);
    println!("z is: {}", z);
    // We can assign the return of a diverging function (a function that returns
    // absolutely nothing).
    let _x = diverge();
}
