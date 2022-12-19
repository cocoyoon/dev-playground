
use std::sync::{Mutex, Condvar};

#[derive(Debug)]
pub struct SemaPhore {
    pub thread_cnt: Mutex<i32>,
    pub cond: Condvar,
    pub thread_max: i32,
}

impl SemaPhore {
    pub fn new(max: i32) -> Self {
        SemaPhore {
            thread_cnt: Mutex::new(0),
            cond: Condvar::new(),
            thread_max: max
        }
    }

    pub fn increase_or_wait(&self) {
        let mut cnt = self.thread_cnt.lock().unwrap();
        // Wait until less than max
        while *cnt >= self.thread_max {
            cnt = self.cond.wait(cnt).unwrap();
        }
        *cnt += 1;
    }

    pub fn decrease_or_notify(&self) {
        let mut cnt = self.thread_cnt.lock().unwrap();
        *cnt -= 1;
        // Wake one waiting thread
        if *cnt <= self.thread_max {
            self.cond.notify_one();
        }
    }
}

