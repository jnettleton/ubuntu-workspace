enum Test {
    Red,
    Blue,
    Green,
}

#[derive(Debug, Clone)]
pub enum PlatformError {
    NotImplemented,
    InvalidData(String), // TODO: This is not a good error
    GetLocationFailed,
    GetLocationModeFailed,
    SetLocationModeFailed,
    SendDeviceCommandsFailed,
    ExecuteScenesFailed,
    GetDeviceFailed,
}

impl std::fmt::Display for PlatformError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug)]
pub enum HiveErrorResult {
    NotImplemented,
    NoValue(String),
    Failure(String),
}

impl From<PlatformError> for HiveErrorResult {
    fn from(other: PlatformError) -> Self {
        Self::Failure(format!("{:?}", other))
    }
}

#[derive(Clone, Debug)]
pub struct HiveError {
    result: HiveErrorResult,
}

impl HiveError {
    pub fn new(result: HiveErrorResult) -> Self {
        Self { result }
    }

    pub fn result(&self) -> &HiveErrorResult {
        &self.result
    }
}

impl std::fmt::Display for HiveError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn main() {
    println!("Hello, world!");
    // let test: &str = Test::Blue.into();
    // println!("Test - {}", test);
    let device_ids: Vec<&str> = [ "1", "2", "3" ].to_vec();
    let platform_err = PlatformError::InvalidData("testing".to_string());
    let hive_error = HiveError::new(HiveErrorResult::Failure(format!("{:?}", platform_err)));
    let error = format!("{hive_error:?}");
    let _str = format!("test error: {}", error);
    let _devices = format!("{:?}", device_ids.clone());
    println!("{}", _str);
}
