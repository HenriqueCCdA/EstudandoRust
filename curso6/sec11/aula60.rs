use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[allow(unused_variables)]
#[allow(unused_assignments)]

const NUM_THREADS: usize = 20;

fn start_thread(d: usize, tx: mpsc::Sender<usize>) {
    thread::spawn(move || {
        println!("setting timer {}", d);
        thread::sleep(Duration::from_secs(d as u64));
        println!("ending {}", d);
        tx.send(d).unwrap()
    });
}


fn main() {
    // let (tx, rx) = mpsc::channel();
    // thread::spawn(move || { tx.send(42).unwrap() });
    // println!("received {}", rx.recv().unwrap());

    let (tx, rx) = mpsc::channel();

    for i in 0..NUM_THREADS {
        start_thread(i, tx.clone());
    }

    for j in rx.iter().take(NUM_THREADS) {
        println!("received {}", j);
    }
}
