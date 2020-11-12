use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let second_val = String::from("Hello from second transmitter");
        tx1.send(second_val).unwrap();
        thread::sleep(Duration::from_millis(1));
    });

    thread::spawn(move || {
       let val = String::from("Hello from transmitter");
        tx.send(val).unwrap();
    });

    for received in rx {
        println!("Got {}", received);
    }
}
