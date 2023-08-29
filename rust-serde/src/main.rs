use std::fs;
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Foo {
    a: String,
    b: u64,
    // #[serde(rename="hubEui")]
    hub_eui: Option<String>,
    // #[serde(rename="hubId")]
    hub_id: Option<String>,
}

impl Foo {
    fn new(a: &str, b: u64) -> Self {
        Self {
            a: a.to_string(),
            b,
            hub_eui: Some("eui".to_string()),
            hub_id: None,
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

    let test1: Option<String> = Some(String::from("test"));
    let _len1 = match test1 { Some(s) => s.as_str().len(), None => 0 };
    let test2: Option<String> = None;
    let _len2 = match &test2 { Some(s) => s.len(), None => 0 };
    let _len3 = match &test2 { Some(s) => s.len(), None => 0 };

    let foo_json = serde_json::to_string(&f).unwrap();
    println!("{:?}", foo_json);

    let _sumo_file = fs::read_to_string("/etc/sumo-sources.json");

    // let mut sumo_nodes = serde_json::Deserializer::from_str(sumo_file.as_str());

    // let sumo_string = serde_json::Serializer::from(sumo_nodes);
    // println!("{:?}", sumo_string);
}
