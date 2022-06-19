// concurrency != parallel

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn test_closure() {
    let some_closure = |mut i:i32| ->i32 { i *= 2; i };
    assert_eq!( 2 * 2, some_closure(2));
}

fn some_function() {
    for i in 0..10 {
        println!("Spawned thread print {}", i);
        thread::sleep(Duration::from_millis(1));
    }
}

// not recommended
pub fn test_thread_case_1() {
    thread::spawn(some_function);

    // main thread
    for i in 0..5 {
        println!("Main thread print {}", i);
        thread::sleep(Duration::from_millis(1));
    }
}

// use closure instead, lambda function (nested function)
pub fn test_thread_case_2() {
    // closure: |args| -> output { ... }
    thread::spawn(|| {
        for i in 0..10 {
            println!("Spawned thread print {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // main thread
    for i in 0..5 {
        println!("Main thread print {}", i);
        thread::sleep(Duration::from_millis(1));
    }
}

pub fn test_thread_case_3() {
    let handle = thread::spawn(|| {
        for i in 0..10 {
            println!("Spawned thread print {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // main thread
    for i in 0..5 {
        println!("Main thread print {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

pub fn test_thread_case_4() {
    let (transmitter, receiver) = mpsc::channel();
    let mut vector = vec![10, 20, 30, 40, 50];

    let handle = thread::spawn(move || {
        for value in &mut vector {
            *value += 5;
        }
        transmitter.send(vector).unwrap();
    });

    handle.join().unwrap();
    let received = receiver.recv().unwrap();
    println!("received {:?}", &received);
}
