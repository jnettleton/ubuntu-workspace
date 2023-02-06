use std::thread;

pub struct BackgroundThread {
}

impl BackgroundThread {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn spawn(&mut self) {
        let handler = thread::spawn(move || {
            println!("test thread");
        });
        handler.join().unwrap();
    }

}
