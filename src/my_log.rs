extern crate log;

// a sort of stupidly simple version of flexi_logger,
// http://emabee.atwebpages.com/rust/src/flexi_logger/lib.rs.html
// in particular, logging to a file.
// i still don't understand the whole Arc<Mutex<RefCell...>>> stack...

use log::{LogRecord,
          LogLevel,
          LogMetadata,
          LogLevelFilter,
          SetLoggerError};

use std::io::prelude::*;
use std::fs::File;
use std::io::LineWriter;
use std::cell::RefCell;
use std::sync::{Mutex, Arc};
use std::ops::DerefMut;
use std::fs::OpenOptions;

struct SimpleLogger {
    lw: Arc<Mutex<RefCell<LineWriter<File>>>>
}

impl SimpleLogger {
    fn new() -> SimpleLogger {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .create(true)
            .open("log.txt").unwrap();
        let linewriter = LineWriter::new(file);
        SimpleLogger {
            lw: Arc::new(Mutex::new(RefCell::new(linewriter)))
        }
    }
}

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Info
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            let mut msg = format!("{} {}", record.level(), record.args());
            msg.push('\n');
            let msgb = msg.as_bytes();

            let m = self.lw.clone();         // Arc<Mutex<RefCell<LW>>>
            let mut mg = m.lock().unwrap();  // MutexGuard<RefCell<LW>>
            let mrc = mg.deref_mut();        // &mut RefCell<LW>
            let mut rm = mrc.borrow_mut();   // RefMut<LW>
            let mlw : &mut LineWriter<File> = rm.deref_mut();

            mlw.write(msgb).unwrap();
        }
    }
}

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(|max_log_level| {
        max_log_level.set(LogLevelFilter::Info);
        Box::new(SimpleLogger::new())
    })
}
