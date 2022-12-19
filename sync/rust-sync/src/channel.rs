
use crate::semaphore::*;
use std::{
    collections::LinkedList, 
    sync::{Arc, Mutex, Condvar}
};

pub fn create_channel<T>(max: i32) -> (Sender<T>, Receiver<T>) {

    let sem = Arc::new(SemaPhore::new(max));
    let buf = Arc::new(Mutex::new(LinkedList::new()));
    let cond = Arc::new(Condvar::new());

    let tx = Sender {
        sem: sem.clone(),
        buf: buf.clone(),
        cond: cond.clone(),
    };
    let rx = Receiver { sem, buf, cond };

    (tx, rx)
}

#[derive(Clone)]
pub struct Sender<T> {
    sem: Arc<SemaPhore>,
    buf: Arc<Mutex<LinkedList<T>>>,
    cond: Arc<Condvar>
}

impl<T: Send> Sender<T> {
    pub fn send(&self, data: T) {
        self.sem.increase_or_wait();
        let mut shared_list = self.buf.lock().unwrap();
        shared_list.push_back(data);
        self.cond.notify_one();
    }
}

pub struct Receiver<T> {
    sem: Arc<SemaPhore>,
    buf: Arc<Mutex<LinkedList<T>>>,
    cond: Arc<Condvar>
}

impl<T: Send> Receiver<T> {
    pub fn recv(&self) -> T {
        let mut shared_list = self.buf.lock().unwrap();
        loop {
            if let Some(data) = shared_list.pop_front() {
                self.sem.decrease_or_notify();
                return data;
            }
            shared_list = self.cond.wait(shared_list).unwrap();
        }
    }
}


