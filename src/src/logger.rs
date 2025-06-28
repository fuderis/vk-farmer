use crate::prelude::*;

/// The program logger
pub struct Logger {
    /* pub logs: StdMutex<Vec<String>>, */
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Info
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let log = fmt!("[{}] {}", record.level(), record.args());

            // printing to terminal:
            println!("{log}");

            // preparing log message:
            let mut msg = log;
            for (k, v) in [
                ("<", "&lt;"),
                (">", "&gt;"),
            ] {
                msg = msg.replace(k, v);
            }

            // send log to frontend:
            emit_event("update-logs", hash_map!{
                "log": msg,
            });

            /* self.logs.lock().unwrap().push(log); */
        }
    }

    fn flush(&self) {}
}

impl Logger {
    /// Creates a new logger
    pub fn new() -> Self {
        Self {
            /* logs: StdMutex::new(vec![]) */
        }
    }

    /* /// Collects logs and clears them
    pub fn take(&self) -> Vec<String> {
        let mut logs_lock = self.logs.lock().unwrap();

        std::mem::take(&mut *logs_lock)
    } */
}
