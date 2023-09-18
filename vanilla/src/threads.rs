use std::thread;
use std::time::Duration;
use std::sync::mpsc;

// 1:1 threads
pub fn start_threads() {

    let v = vec![1, 2, 3, 4];

    // use move to transfer data into new thread
    let handle = thread::spawn(move || {
        println!("{:?}", v);
        for i in v {
            println!("{} from separate thread", i);
        }
    });

    handle.join().unwrap();
    thread::sleep(Duration::from_millis(5));
}

pub fn thread_channel() {

    let (tx, rx) = mpsc::channel();

    // use move to transfer data into new thread
    thread::spawn(move || {
        tx.send("Hello message!".to_string()).unwrap();
    });

    let rec = rx.recv().unwrap();
    println!("{}", rec);

    // MULTIPLE

    let (tx, rx) = mpsc::channel();

    // use move to transfer data into new thread
    thread::spawn(move || {
        let vals = vec![
            String::from("h"),
            String::from("e"),
            String::from("l"),
            String::from("l"),
            String::from("o"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    for rec in rx {
        println!("{}", rec);
    }
}