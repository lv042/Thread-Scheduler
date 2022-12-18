use std::sync::{Arc, Mutex};
use std::thread;

//The Arc type is used to share the value between threads, while the Mutex is used to protect access to the value.
//The Mutex ensures that only one thread can access the value at a time, while the Arc enables multiple threads to hold a reference to the value.
//Using Arc and Mutex together allows you to share a value between multiple threads in a safe and efficient way.
//The Arc type enables multiple threads to hold a reference to the value, while the Mutex ensures that access to the value is properly synchronized.

#[derive(Clone)]
pub struct Task {
    pub id: u32,
    pub name: String,
    pub closure: Arc<dyn Fn() + Send + Sync + 'static>,
}

impl Task {
    fn new(id: u32, name: String, closure: impl Fn() + Send + Sync + 'static) -> Task {
        Task { 
            id, 
            name,
            closure: Arc::new(closure) 
        }
    }

    pub fn run(self) {
        

        //maybe limit the number of threads to the number of cores
        //run the closure in a new thread
        let tr = thread::spawn(move || {
            println!("Task {}:{} is running", self.name, self.id);
            (self.closure)();
            println!("Task {}:{} is finished", self.name, self.id);
        });
        
        
       
    }

}


pub struct TaskManager {
    //makes the list accessible to multiple threads and protects the list from concurrent access
    pub start_time: std::time::Instant,
    pub list: Arc<Mutex<Vec<Task>>>,
}

impl TaskManager {
    pub fn new() -> TaskManager {
        TaskManager {
            start_time: std::time::Instant::now(),
            list: Arc::new(Mutex::new(Vec::new())),
        }
    }
    fn add_task(&mut self, task: Task) {
        let mut list = self.list.lock().unwrap();
        list.push(task);
    }

    pub fn create_task(&mut self, name: String, closure: impl Fn() -> () + Send + Sync + 'static) {
        //task id is the length of the list + 1
        let task = Task::new(self.list.lock().unwrap().len() as u32 + 1, name, closure);
        self.add_task(task);
    }

    pub fn run_tasks(&self) {
        let list = self.list.lock().unwrap();
        for task in list.iter() {
            let task_copy = (*task).clone();
            task_copy.run();
        }
    }
    
}




fn main() {
    let mut task_manager = TaskManager::new();
    task_manager.create_task("task1".to_string(), || {
        std::thread::sleep(std::time::Duration::from_secs(10));
    });
    task_manager.create_task("task2".to_string(), || {
        std::thread::sleep(std::time::Duration::from_secs(5));
    });

    task_manager.run_tasks();

    //wait for all tasks to finish
    std::thread::sleep(std::time::Duration::from_secs(30));
    println!("All tasks finished")
}
