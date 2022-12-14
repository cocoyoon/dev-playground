use std::sync::{Arc, Mutex, Condvar};
use std::thread;

fn main() {
    // test for Mutex
    let mutex_shared = Arc::new(Mutex::new(0)); 
    // teset for Condvar
    let condvar_shared = Arc::new((Mutex::new(false), Condvar::new()));
    let mut handle: Vec<thread::JoinHandle<()>> = vec![];
    for i in 0..3 {
        let mutex_shared = mutex_shared.clone();
        let condvar_shared = condvar_shared.clone();
        let th = thread::spawn(move || {
            increase_by_one(mutex_shared);
            child(i as u8, condvar_shared);
        });
        handle.push(th);
    }
    let p = thread::spawn(move || {
        parent(condvar_shared.clone());
    });
    for th in handle {
        th.join().unwrap(); // wait until thread is done its task
    }
    p.join().unwrap();
    assert_eq!(*mutex_shared.lock().unwrap(), 3);
}

fn increase_by_one(shared: Arc<Mutex<i32>>) {
    // acquire lock
    let mut value = shared.lock().unwrap(); 
    // increase by 1
    *value += 1; 
}
// lock will release when out of scope

fn child(id: u8, shared: Arc<(Mutex<bool>, Condvar)>) {
    // use '&' keyword to match pattern and use 'ref' keyword not to 'move'
    let &(ref shared, ref cond) = &*shared; 

    let mut ready = shared.lock().unwrap();
    while !*ready {
        // give back ownership to 'ready'
        println!("Child{} is waiting", id);
        ready = cond.wait(ready).unwrap(); 
    }
    println!("Child{} done!", id);
} 

fn parent(shared: Arc<(Mutex<bool>, Condvar)>) {
    let &(ref shared, ref cond) = &*shared;
    let mut ready = shared.lock().unwrap();
    *ready = true;
    cond.notify_all();
    println!("Parent thread done!")
}

