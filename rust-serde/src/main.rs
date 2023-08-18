use std::fs;
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Foo {
    a: String,
    b: u64,
    // #[serde(rename="hubEui")]
    hub_eui: String,
    // #[serde(rename="hubId")]
    hub_id: String,
}

impl Foo {
    fn new(a: &str, b: u64) -> Self {
        Self {
            a: a.to_string(),
            b,
            hub_eui: "eui".to_string(),
            hub_id: "".to_string(),
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

    let _sumo_file = fs::read_to_string("/etc/sumo-sources.json");

    // let mut sumo_nodes = serde_json::Deserializer::from_str(sumo_file.as_str());

    // let sumo_string = serde_json::Serializer::from(sumo_nodes);
    // println!("{:?}", sumo_string);
}
