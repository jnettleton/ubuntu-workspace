use std::fmt;

pub static FILTER_NAMES: [&'static str; 3] = ["Info", "Warn", "Error"];
impl FilterLevel {
    /// Convert to `str` from `FILTER_NAMES`
    pub fn as_str(&self) -> &'static str {
        FILTER_NAMES[self.as_usize()]
    }

    #[inline]
    pub fn as_usize(&self) -> usize {
        match *self {
            FilterLevel::Info => 0,
            FilterLevel::Warn => 1,
            FilterLevel::Error => 2,
        }
    }
}

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

    // println!("Level: {}", &level as i16);
    println!("Level: {}", level); // Debug
    println!("Level: {}", level.as_str()); // as_str()
    println!("Level: {}", level_string); // fmt::Display
}
