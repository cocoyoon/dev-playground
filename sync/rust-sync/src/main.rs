use std::{
    thread,
    sync::atomic::{AtomicUsize, Ordering::SeqCst}
};
use std::sync::{Arc, Mutex, Condvar, RwLock, Barrier};

mod semaphore;
use semaphore::*;

mod channel;
use channel::create_channel;

const SEM_COUNT: usize = 4;
const THREAD_COUNT: usize = 8;
const LOOP_COUNT: usize = 10000;
static mut ATOMIC_CNT: AtomicUsize = AtomicUsize::new(0);

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
    // Test for semaphore
    assert_eq!(*mutex_shared.lock().unwrap(), 3);

    let semaphore_shared = Arc::new(SemaPhore::new(SEM_COUNT as i32));
    let mut thread_handle = Vec::new();
    {
        for i in 0..THREAD_COUNT {
            let s = semaphore_shared.clone();
            let th = thread::spawn(move || {
                for _ in 0..LOOP_COUNT {
                    // Increase reference count or wait until notify
                    s.increase_or_wait();

                    unsafe { ATOMIC_CNT.fetch_add(1, SeqCst) };
                    let n = unsafe { ATOMIC_CNT.load(SeqCst) };
                    println!("Thread #{}, Atomic count #{}", i, n);
                    assert!(n <= SEM_COUNT);
                    unsafe { ATOMIC_CNT.fetch_sub(1, SeqCst) };

                    // Decrase reference count and notify to thread that waits
                    s.decrease_or_notify();
                }
            });
            thread_handle.push(th);
        }

        for th in thread_handle {
            th.join().unwrap();
        }
        
        let (tx, rx) = create_channel(4);
        let mut v = Vec::new();
        // For receiver thread
        let th = thread::spawn(move || {
            let mut cnt = 0;
            while cnt < THREAD_COUNT * LOOP_COUNT {
                let data = rx.recv();
                print!("Data received! {:?}", data);
                cnt += 1;
            }
        });
        v.push(th);
        for i in 0..THREAD_COUNT {
            let tx = tx.clone();
            let th = thread::spawn(move || {
                for j in 0..LOOP_COUNT {
                    tx.send((i,j))
                }
            });
            v.push(th);
        }

        for th in v {
            th.join().unwrap();
        }
    }
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

