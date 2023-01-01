fn main() {
    let s = String::from("book");
    let p = pluralize(&s);

    println!("I have one {}, you have two {}", s, p);
}

fn pluralize(s: &String) -> String {
    let mut p = s.clone();
    p.push_str("s");
    p
}
