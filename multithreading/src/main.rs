use std::{
    sync::mpsc,
    thread::{self, spawn},
};

fn main() {
    //multithreading

    let handle = thread::spawn(|| {
        for i in 0..5 {
            println!("Hi from spawned thread {}", i);
        }
    });

    for i in 0..5 {
        println!("Hi from main thread {}", i);
    }
    handle.join();

    let x = 1;
    {
        let v = vec![1, 2, 3];
        thread::spawn(move || {
            println!("{:?}", v);
        });
    }
    print!("{}", x);

    //message passing
    let (tx, rx) = mpsc::channel();
    spawn(move || {
        tx.send(String::from("Hello world"));
    });

    let value = rx.recv();
    match value {
        Ok(value) => println!("{}", value),
        Err(err) => println!("Error while reading the data"),
    }
    // drop(tx);
    //
    let (tx, rx) = mpsc::channel();

    for i in 0..10 {
        let producer = tx.clone();
        spawn(move || {
            let mut sum = 0;
            for j in i * 1000000..(i + 1 * 1000000) - 1 {
                sum = sum + j;
            }
            producer.send(sum).unwrap();
        });
    }

    drop(tx);
    let mut final_sum = 0;
    for val in rx.recv() {
        final_sum = final_sum + val;
    }
    print!("{}", final_sum);
}
