use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::Mutex;
use std::sync::Arc;

fn main() {
//    handling();
//    moving()
//    channels()
//    mutexes()
    sharing_between_threads()
}


fn handling() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join();
    for i in 1..5 {
        println!("Hi number {} from main thread ", i);
        thread::sleep(Duration::from_millis(1));
    }

//    handle.join();
}

fn moving() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector {:?}", v);
    });
    handle.join();
}

fn channels() {
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got {}", received);
    }
}

fn mutexes() {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}

fn sharing_between_threads() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 1..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap()
    }

    println!("Counter {}", *counter.lock().unwrap());
}