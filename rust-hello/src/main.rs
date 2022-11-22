enum Test {
    Red,
    Blue,
    Green,
}

fn main() {
    println!("Hello, world!");
    let test: &str = Test::Blue.into();
    println!("Test - {}", test);
}
