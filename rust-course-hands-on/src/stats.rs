//! The stats module contains the stats loop
//! 
//! # Some header
//! More discussion!

mod timer;

use std::io::{self, Result, Stderr, Write};
// use std::sync::{Arc, Mutex};
// use std::sync::mpsc::{Receiver, Sender};
use crossbeam::channel::Receiver;
use crossterm::{
    cursor, execute,
    style::{self, Color, PrintStyledContent},
    terminal::{Clear, ClearType},
};
use std::time::Instant;
use timer::Timer;

// pub fn stats_loop(silent: bool, quit: Arc<Mutex<bool>>) -> Result<()> {
// pub fn stats_loop(
//     silent: bool,
//     stats_rx: Receiver<Vec<u8>>,
//     write_tx: Sender<Vec<u8>>,
pub fn stats_loop(silent: bool, stats_rx: Receiver<usize>) -> Result<()> {
    let mut total_bytes = 0;
    let start = Instant::now();

    // let mut last_instant = Instant::now();
    let mut timer = Timer::new();
    let mut stderr = io::stderr();

    loop {
        // receive the vector of bytes from read loop
        // let buffer: Vec<u8> = Vec::new(); // so we can compile
        // let buffer = stats_rx.recv().unwrap();
        // let num_bytes = buffer.len();
        let num_bytes = stats_rx.recv().unwrap();

        // let now = Instant::now();
        timer.update();

        // let rate_per_second = num_bytes as f64 / (now - last_instant).as_secs_f64();
        // last_instant = now;
        let rate_per_second = num_bytes as f64 / timer.delta.as_secs_f64();

        total_bytes += num_bytes;

        if !silent && timer.ready {
            timer.ready = false;

            // eprintln!(
            //     "\r{} {} [{:.0} b/s]",
            //     total_bytes,
            //     start.elapsed().as_secs().as_time(),
            //     rate_per_second
            // );
            output_progress(
                &mut stderr,
                total_bytes,
                start.elapsed().as_secs().as_time(),
                rate_per_second,
            );
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

fn output_progress(stderr: &mut Stderr, bytes: usize, elapsed: String, rate: f64) {
    let bytes = style::style(format!("{} ", bytes)).with(Color::Red);
    let elapsed = style::style(elapsed).with(Color::Green);
    let rate = style::style(format!(" [{:.0} b/s]", rate)).with(Color::Blue);
    let _ = execute!(
        stderr,
        cursor::MoveToColumn(0),
        Clear(ClearType::CurrentLine),
        PrintStyledContent(bytes),
        PrintStyledContent(elapsed),
        PrintStyledContent(rate),
    );
    let _ = stderr.flush();
}

/// The TimeOutput trait adds a '.as_time()' method to 'u64'
/// 
/// # Example
/// Here is an example of how to use it.
/// 
/// ```rust
/// use pipeviewer::stats::TimeOutput;
/// assert_eq!(65_u64.as_time(), String::from("0:01:05"))
/// ```
pub trait TimeOutput {
    fn as_time(&self) -> String;
}

impl TimeOutput for u64 {
    /// Renders the u64 into a time string
    fn as_time(&self) -> String {
        let (hours, left) = (*self / 3600, *self % 3600);
        let (minutes, seconds) = (left / 60, left % 60);
        format!("{}:{:02}:{:02}", hours, minutes, seconds)
    }
}

#[cfg(test)]
mod tests {
    use super::TimeOutput;

    #[test]
    fn as_time_format() {
        let pairs = vec![
            (5_u64, "0:00:05"),
            (60_u64, "0:01:00"),
            (154_u64, "0:02:34"),
            (3603_u64, "1:00:03"),
        ];
        for (input, output) in pairs {
            assert_eq!(input.as_time().as_str(), output);
        }
    }
}
