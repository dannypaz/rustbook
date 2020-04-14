fn main() {
    let y = {
        let x = 1;
        x + 3
    };
    println!("What is y: {}", y);

    let f = five();
    println!("What is five?? {}", f);

    let p = plus_one(1);
    println!("Plus one? {}", p);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
