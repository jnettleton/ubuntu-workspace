use std::io::Result;
// use std::sync::{Arc, Mutex};
// use std::sync::mpsc::{Receiver, Sender};
use crossbeam::channel::Receiver;

// pub fn stats_loop(silent: bool, quit: Arc<Mutex<bool>>) -> Result<()> {
// pub fn stats_loop(
//     silent: bool,
//     stats_rx: Receiver<Vec<u8>>,
//     write_tx: Sender<Vec<u8>>,
pub fn stats_loop(silent: bool, stats_rx: Receiver<usize>) -> Result<()> {
    let mut total_bytes = 0;
    loop {
        // receive the vector of bytes from read loop
        // let buffer: Vec<u8> = Vec::new(); // so we can compile
        // let buffer = stats_rx.recv().unwrap();
        // let num_bytes = buffer.len();
        let num_bytes = stats_rx.recv().unwrap();
        total_bytes += num_bytes;

        if !silent {
            eprintln!("\r{}", total_bytes);
        }

        // send vector to write loop
        // let quit = quit.lock().unwrap();
        // if *quit {
        //     break;
        // }

        // if write_tx.send(buffer).is_err() {
        //     break;
        // }

        if num_bytes == 0 {
            break;
        }
    }
    if !silent {
        eprintln!();
    }

    Ok(())
}
