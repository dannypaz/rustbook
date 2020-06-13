fn main() {
    a_function();
    a_a_function(1);
    a_a_a_function(1,2);
}

fn a_function() {
    println!("This is a function");
}

fn a_a_function(x: i32) {
    println!("This is a function with param: {}", x);
}

fn a_a_a_function(x: i32, y: i32) {
    let res = x + y;
    println!("This is a function with result: {}", result);
}
