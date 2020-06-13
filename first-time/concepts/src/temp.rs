fn main() {
    let temp = 20.5;
    let res = to_f(temp);
    println!("The temp c to f: {:.1}", res);

    let temp = 98.0;
    let res = to_c(temp);
    println!("The temp f to c: {:.1}", res);
}

fn to_f(temp_in_far: f64) -> f64 {
    (temp_in_far / 5.0) * 9.0 + 32.0
}

fn to_c(temp_in_celcius: f64) -> f64 {
    ((temp_in_celcius - 32.0) * 5.0) / 9.0
}
