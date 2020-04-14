// Scalar types
// 1. int
// 2. floating point
// 3. boolean
// 4. pointer
//
fn main() {
    let guess: u32 = match "42".parse() {
        Ok(num) => num,
        Err(_) => 0
    };
    println!("What was the guess: {}", guess);

    let x = 2.0;
    let y: f32 = 3.0;
    println!("Here are my numbers: {} {}", x, y);

    let sum = 5 + 10;
    let diff = 5 - 10;
    let product = 5 * 10;
    let div = 5 / 10;
    let remain = 10 % 2;
    println!("{}, {}, {}, {}, {}", sum, diff, product, div , remain);

    let t = true;
    let f: bool = false;
    println!("{}, {}", t, f);

    let c = 'z'; // character literal
    let stuff = "my string";
    println!("{}, {}", c, stuff);

    let tup = (500, "dan", 'c', 1.0);
    let (a, b, d, e) = tup;
    println!("{}, {}, {}, {}", a, b, d, e);

    let another_tup: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = another_tup.0;
    let six_point_four = another_tup.1;
    let one = another_tup.2;
    println!("{}, {}, {}", five_hundred, six_point_four, one);

    let collection = [1, 2, 3, 4];
    let collection_a = ['a'; 5];
    let collection_b = [0; 5];
    println!("{:?}, {:?}, {:?}", collection, collection_a, collection_b);

}
