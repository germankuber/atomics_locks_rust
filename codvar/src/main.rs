use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time::Duration;

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair_clone = Arc::clone(&pair);
    

    // Thread 1: Waits for the condition to be met
    let waiter = thread::spawn(move || {
        let (lock, cvar) = &*pair_clone;
        let mut started = lock.lock().unwrap();
        println!("Waiter: Waiting for the condition...");
        while !*started {
            // Wait for the condition to be met
            started = cvar.wait(started).unwrap();
            println!("Waiter 1 while");
        }
        println!("Waiter: Condition met, proceeding");
    });
    let pair_clone = Arc::clone(&pair);
    let waiter_2 = thread::spawn(move || {
        let (lock, cvar) = &*pair_clone;
        let mut started = lock.lock().unwrap();
        println!("Waiter_2: Waiting for the condition...");
        while !*started {
            // Wait for the condition to be met
            started = cvar.wait(started).unwrap();
            println!("Waiter 1_2 while");
        }
        println!("Waiter_2: Condition met, proceeding");
    });
    thread::sleep(Duration::from_secs(2));
    // Thread 2: Waits for 5 seconds before modifying the condition and notifying
    let notifier = thread::spawn(move || {
        println!("Notifier: Sleeping for 5 seconds...");
        thread::sleep(Duration::from_secs(4));
        let (lock, cvar) = &*pair;
        let mut started = lock.lock().unwrap();
        println!("Notifier: Waking up and modifying the condition");
        // Change the condition
        *started = true;
        // Notify the thread waiting on the `Condvar`
        cvar.notify_all(); // You could use `notify_all()` to wake up all waiting threads
        println!("Notifier: Condition modified and waiter notified");
    });

    // Wait for both threads to finish
    waiter.join().unwrap();
    waiter_2.join().unwrap();
    notifier.join().unwrap();
}

