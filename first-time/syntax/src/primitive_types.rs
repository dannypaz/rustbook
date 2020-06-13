fn slices() {
    let a = [0,1,2,3,4];
    let complete = &a[..];
    let middle = &a[1..4];
    println!("{:?} {:?} {:?}", a, complete, middle);
}

pub fn main() {
    let a = [1, 2, 3];
    println!("{:?}", a);
    println!("{}", a.len());
    let names = ["dan", "jim", "jeff"];
    println!("the second name is {}", names[1]);
    slices();
}
