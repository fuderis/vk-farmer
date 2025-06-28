use crate::prelude::*;

/// The bot task
#[derive(Debug)]
pub struct Task {
    bot_id: String,
    limits: usize,
    completed: usize,
    to_close: bool,
    is_closed: bool,
}

impl Task {
    /// Creates a new bot task
    pub fn new<S: Into<String>>(bot_id: S, limits: usize) -> Arc<Mutex<Self>> {
        let bot_id = bot_id.into();
        
        // reset bot progress:
        emit_event("update-bot-progress", hash_map!{
            "bot_id": Value::String(bot_id.clone()),
            "progress": Value::Number(0.into())
        });
        
        Arc::new(Mutex::new(
            Self {
                bot_id,
                limits,
                completed: 0,
                to_close: false,
                is_closed: false,
            }
        ))
    }
    
    /// Returns true if limits not achieved
    pub fn check_limits(&self) -> bool {
        self.completed < self.limits
    }

    /// Regs a completed task
    pub fn complete_task(&mut self) {
        self.completed += 1;

        // calc progress percentage:
        let progress = self.completed * 100 / self.limits;

        // send progress to frontend:
        emit_event("update-bot-progress", hash_map!{
            "bot_id": Value::String(self.bot_id.clone()),
            "progress": Value::Number(progress.into())
        });
    }

    /// Closes bot session
    pub fn close(&mut self) {
        self.to_close = true;
    }

    /// Set bot session status as closed
    pub fn set_as_closed(&mut self) {
        self.is_closed = true;
    }
    
    /// Checks bot session is closed
    pub fn is_closed(&self) -> bool {
        self.is_closed
    }

    /// Checks bot session for need to close
    pub fn to_close(&self) -> bool {
        self.to_close || self.is_closed
    }
}
