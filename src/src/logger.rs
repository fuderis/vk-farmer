use crate::prelude::*;
use std::fs;
use chrono::Local;

/// The program logger
pub struct Logger {
    pub logs: StdMutex<Vec<String>>,
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Info
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let dt = Local::now().format("%Y/%m/%d/T%H:%M:%S%.6f");
            let mut log = fmt!("[{dt}] [{}] {}", record.level(), record.args());

            // printing to terminal:
            println!("{log}");
            
            self.logs.lock().unwrap().push(log.clone());

            // preparing log message:
            for (k, v) in [
                ("<", "&lt;"),
                (">", "&gt;"),
            ] {
                log = log.replace(k, v);
            }
            // send log to frontend:
            emit_event("update-logs", hash_map!{
                "log": Value::String(log),
            });
        }
    }

    fn flush(&self) {}
}

impl Logger {
    /// Creates a new logger
    pub fn new() -> Self {
        Self {
            logs: StdMutex::new(vec![])
        }
    }
    
    /// Collects logs and clears them
    pub fn take(&self) -> Vec<String> {
        let mut logs_lock = self.logs.lock().unwrap();

        std::mem::take(&mut *logs_lock)
    }

    /// Saves logs to file
    pub fn save(&self) -> Result<()> {
        let now = Local::now();
        let fname = now.format("logs/%Y-%m-%d_%H-%M-%S.txt").to_string();
        let path = root_path(fname)?;

        // create file dir:
        let dir = path.parent().unwrap();
        if !dir.exists() {
            fs::create_dir_all(dir)?;
        }

        // writing logs to file:
        let logs_str = self.logs.lock().unwrap().join("\n");
        fs::write(path, logs_str)?;

        Ok(())
    }
}
