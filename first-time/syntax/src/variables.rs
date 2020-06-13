fn scoping() {
    let x = 17;
    {
        let y = 10;
        println!("We can see both vars here {} {}", x, y);
    }
    println!("But not here {}", x)
}

fn shadow() {
    let x = 17;
    {
        println!("{}", x);
        let x = 18;
        println!("{}", x);
    }
    println!("{}", x);
    let x = 19;
    println!("{}", x);
}

fn advanced_shadow() {
    let mut x = 7;
    println!("{}", x);
    x = 8;
    println!("{}", x);
    let y = 17;
    println!("{}", y);
    let y = "danny";
    println!("{}", y);
}

pub fn main() {
    let x = 5;
    let (y, z) = (1, 2);
    let dan: i32 = 10;
    println!("{} {} {} {}", x, y, z, dan);
    scoping();
    shadow();
    advanced_shadow();
}
