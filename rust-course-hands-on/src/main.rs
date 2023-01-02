use pipeviewer::{args::Args, read, stats, write};
use std::io::Result;
// use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;

fn main() -> Result<()> {
    let args = Args::parse();
    let Args {
        infile,
        outfile,
        silent,
    } = args;

    // let quit = Arc::new(Mutex::new(false));
    // let (quit1, quit2, quit3) = (quit.clone(), quit.clone(), quit.clone());

    // let read_handle = thread::spawn(move || read::read_loop(&infile, quit1));
    // let stats_handle = thread::spawn(move || stats::stats_loop(silent, quit2));
    // let write_handle = thread::spawn(move || write::write_loop(&outfile, quit3));

    let (stats_tx, stats_rx) = mpsc::channel();
    let (write_tx, write_rx) = mpsc::channel();

    let read_handle = thread::spawn(move || read::read_loop(&infile, stats_tx));
    let stats_handle = thread::spawn(move || stats::stats_loop(silent, stats_rx, write_tx));
    let write_handle = thread::spawn(move || write::write_loop(&outfile, write_rx));

    // crash if any thread have crashed
    // '.join()' returns a 'thread::Result<io::Result<()>>'
    let read_io_result = read_handle.join().unwrap();
    let stats_io_rsult = stats_handle.join().unwrap();
    let write_io_result = write_handle.join().unwrap();

    // return an error if any threads returned an error
    read_io_result?;
    stats_io_rsult?;
    write_io_result?;

    Ok(())
}
