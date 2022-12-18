use std::sync::Arc;
use std::thread;

//The Arc type is used to share the value between threads, while the Mutex is used to protect access to the value.
//The Mutex ensures that only one thread can access the value at a time, while the Arc enables multiple threads to hold a reference to the value.
//Using Arc and Mutex together allows you to share a value between multiple threads in a safe and efficient way.
//The Arc type enables multiple threads to hold a reference to the value, while the Mutex ensures that access to the value is properly synchronized.
fn main() {
    let data = Arc::new(5.0);

    let data_clone = data.clone();
    thread::spawn(move || {
        let data = data_clone;
        println!("data in new thread: {}", *data);
    });

    println!("data in main thread: {}", *data);
    std::thread::sleep(std::time::Duration::from_secs(1));
}

pub struct Task {
    pub id: u32,
    pub name: String,
}

impl Task {
    pub fn new(id: u32, name: String, status: bool) -> Task {
        Task { id, name }
    }
    pub fn run<F>(&self, f: F)
    where
        F: Fn(),
    {   
        println!("Task {}:{} is running", self.name, self.id);
        f();
        println!("Task {}:{} is finished", self.name, self.id);
    }
}
