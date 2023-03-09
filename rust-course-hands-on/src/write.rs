use std::fs::File;
use std::io::{self, BufWriter, ErrorKind, Result, Write};
// use std::sync::{Arc, Mutex};
// use std::sync::mpsc::Receiver;
use crossbeam::channel::Receiver;

// pub fn write_loop(outfile: &str, quit: Arc<Mutex<bool>>) -> Result<()> {
pub fn write_loop(outfile: &str, write_rx: Receiver<Vec<u8>>) -> Result<()> {
    let mut writer: Box<dyn Write> = if !outfile.is_empty() {
        Box::new(BufWriter::new(File::create(outfile)?))
    } else {
        Box::new(BufWriter::new(io::stdout()))
    };

    loop {
        // receive vector from stats thread
        // let buffer: Vec<u8> = Vec::new(); // so we can compile
        let buffer = write_rx.recv().unwrap();
        // {
        //     let quit = quit.lock().unwrap();
        //     if *quit {
        //         break;
        //     }
        // }
        if buffer.is_empty() {
            break;
        }

        if let Err(e) = writer.write_all(&buffer) {
            if e.kind() == ErrorKind::BrokenPipe {
                // stop the program cleanly
                return Ok(());
            }
            return Err(e);
        }
    }

    Ok(())
}
