use std::sync::{Arc, Mutex, Condvar, RwLock, Barrier};
use std::thread;

fn main() {
    // Test for Mutex
    let mutex_shared = Arc::new(Mutex::new(0)); 
    // Test for Condvar
    let condvar_shared = Arc::new((Mutex::new(false), Condvar::new()));
    // Test for RW Lock
    let rwlock_shared = RwLock::new(0);
    // Test for Barrier Sync
    // Given size represents number of threads
    let barrier_shared = Arc::new(Barrier::new(3)); 
    // No limit on read
    // Read-lock will release when out-of-scope
    {
        let v1 = rwlock_shared.read().unwrap();
        let v2 = rwlock_shared.read().unwrap();
        println!("v1:{}, v2:{}", v1, v2);
    }
    // Write
    {
        let mut v = rwlock_shared.write().unwrap();
        *v = 1;
        println!("Write to {}", v);
    }
    let mut handle: Vec<thread::JoinHandle<()>> = vec![];
    for i in 0..3 {
        let mutex_shared = mutex_shared.clone();
        let condvar_shared = condvar_shared.clone();
        let barrier_shared = barrier_shared.clone();
        let th = thread::spawn(move || {
            increase_by_one(mutex_shared);
            child(i as u8, condvar_shared);
            println!("Before wait!");
            barrier_shared.wait();
            println!("After wait!");
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

