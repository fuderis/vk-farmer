use crate::prelude::*;

/// The bot task
#[derive(Debug)]
pub struct Task {
    closed: bool,
    limits: usize,
    completed: usize
}

impl Task {
    /// Creates a new bot task
    pub fn new(limits: usize) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(
            Self {
                closed: false,
                limits,
                completed: 0
            }
        ))
    }

    /// Closes bot session
    pub fn close(&mut self) {
        self.closed = true;
    }
    
    /// Checks bot session to closed state
    pub fn is_closed(&self) -> bool {
        self.closed
    }
    
    /// Returns true if limits not achieved
    pub fn check_limits(&self) -> bool {
        self.completed < self.limits
    }

    /// Regs a completed task
    pub fn complete_task(&mut self) {
        self.completed += 1;
    }

    /// Gets a bot limits percentage
    pub fn calc_limits_percentage(&self) -> usize {
        self.completed * 100 / self.limits
    }
}
