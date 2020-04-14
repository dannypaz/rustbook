fn main() {
    let num = 14;
    let res = fib(num);
    println!("Fib of {} is: {}", num, res);
}

fn fib(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    }

    return fib(n-1) + fib(n-2);
}
