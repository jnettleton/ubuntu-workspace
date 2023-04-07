use std::fmt;

#[derive(Debug)]
pub enum FilterLevel {
    Info,
    Warn,
    Error,
}

impl fmt::Display for FilterLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn main() {
    let level = FilterLevel::Info;
    let level_string = level.to_string();
    // let level_str = level.to_str();

    println!("Level: {}", level); // Debug
    println!("Level: {}", level_string); // fmt::Display
}
