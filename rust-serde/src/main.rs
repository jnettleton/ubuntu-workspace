use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Foo {
    a: String,
    b: u64,
}

impl Foo {
    fn new(a: &str, b: u64) -> Self {
        Self {
            a: a.to_string(),
            b
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct FooItems {
    pub items: Vec<Foo>,
}

impl FooItems {
    fn new() -> Self {
        Self {
            items: Vec::new(),
        }
    }
}

fn main() {
    let mut f = FooItems::new();
    let foo1 = Foo::new("It's that simple.", 42);
    let foo2 = Foo::new("It's that simple.", 101);
    f.items.push(foo1);
    f.items.push(foo2);

    let foo_json = serde_json::to_string(&f).unwrap();
    println!("{:?}", foo_json);
}
