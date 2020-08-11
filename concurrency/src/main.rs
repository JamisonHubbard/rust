use std::thread;
use std::time::Duration;

fn main() {
    concurrency_unjoined();

    concurrency_joined_after();

    concurrency_joined_between();
}

fn concurrency_unjoined() {
    thread::spawn(
        || {
            for i in 1..3 {
                println!("Spawn Thread: {}", i);
                thread::sleep(Duration::from_millis(1));
            }
        }  
    );

    for i in 1..3 {
        println!("Normal Thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    println!();
}

fn concurrency_joined_after() {
    let handle = thread::spawn (
        || {
            for i in 1..3 {
                println!("Spawn Thread {}", i);
                thread::sleep(Duration::from_millis(1));
            }
        }
    );

    for j in 1..3 {
        println!("Normal thread {}", j);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    println!();
}

fn concurrency_joined_between() {
    let handle = thread::spawn(
        || {
            for i in 1..3 {
                println!("Spawn Thread: {}", i);
                thread::sleep(Duration::from_millis(1));
            }
        }
    );

    handle.join().unwrap();

    for j in 1..3 {
        println!("Normal thread: {}", j);
        thread::sleep(Duration::from_millis(1));
    }

    println!();
}