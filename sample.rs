use std::sync::Arc;
use std::thread;

fn main() {
    let counter = Arc::new(0);

    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = *counter;
            num += 1;
            println!("Thread: {}", num);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter);
}

