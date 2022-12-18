use std::sync::Arc;
use std::thread;

fn main() {
    let data = Arc::new(5.0);

    let data_clone = data.clone();
    thread::spawn(move || {
        let data = data_clone;
        println!("data in new thread: {}", *data);    });

    println!("data in main thread: {}", *data);
    std::thread::sleep(std::time::Duration::from_secs(1));
}