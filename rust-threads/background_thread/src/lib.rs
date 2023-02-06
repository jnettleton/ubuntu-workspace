use std::thread::{self, JoinHandle};
use std::time::Duration;
use std::{collections::HashMap};

pub struct BackgroundThread {
    // thread handler
    handle: Option<JoinHandle<()>>,

    // cancellation token

    // output variable
    _items: Box<HashMap<i32,i32>>
}

impl BackgroundThread {
    pub fn new() -> Self {
        Self {
            handle: None,
            _items: Box::new(HashMap::new()),
        }
    }

    pub fn spawn(&mut self) -> Option<&JoinHandle<()>> {
        let handle = thread::spawn(move || {
            println!("test thread");
        });
        self.handle = Some(handle);

        // if handler.is_finished() { return; };
        // handler.join().unwrap();

        self.handle.as_ref()
    }

    fn _background_thread(&mut self) {
        let mut count = 5;
        let interval = Duration::new(1, 0);

        loop {
            println!("{count}");

            count -= 1;
            if count == 0 { break };

            thread::sleep(interval);
        }
    }
}

impl Drop for BackgroundThread {
    fn drop(&mut self) {
        // match &self.handle {
        //     Some(h) => {
        //         if !*h.is_finished() {
        //             *h.join();
        //         }
        //         return
        //     },
        //     None => return
        // }
    }
}
