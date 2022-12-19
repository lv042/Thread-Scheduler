use std::sync::{Arc, Mutex};
use std::thread;

//@TODO: Limit the number of threads that can run at the same time to the number of logical cores on the machine.

#[derive(Clone)]
pub struct Task {
    // This field stores the id of the task.
    pub id: u32,
    // This field stores the name of the task.
    pub name: String,
    // This field stores the closure that represents the task's work.
    // The closure is stored in an Arc value, which allows it to be shared between threads.
    pub closure: Arc<dyn Fn() + Send + Sync + 'static>,
}

impl Task {
    // This method creates a new Task instance with the given id, name, and closure.
    // The closure is stored in an Arc value, which allows it to be shared between threads.
    fn new(id: u32, name: String, closure: impl Fn() + Send + Sync + 'static) -> Task {
        Task { 
            id, 
            name,
            closure: Arc::new(closure) 
        }
    }

    // This method runs the task's closure in a new thread.
    // The task's name and id are printed before and after running the closure.
    pub fn run(self) {
        // Create a new thread to run the closure in.
        let tr = thread::spawn(move || {
            println!("Task {}:{} is running", self.name, self.id);
            (self.closure)();
            println!("Task {}:{} is finished", self.name, self.id);
        });
    }
}



pub struct TaskManager {
    // This field stores the time when the TaskManager instance was created.
    pub start_time: std::time::Instant,
    // This field stores a list of tasks managed by the TaskManager.
    // The list is stored in an Arc value, which allows it to be shared between threads.
    // The Mutex value protects the list from concurrent access.
    pub list: Arc<Mutex<Vec<Task>>>,

    pub max_threads: u32,

    pub current_threads: u32,
}

impl TaskManager {
    // This method creates a new TaskManager instance.
    pub fn new() -> TaskManager {
        TaskManager {
            // Store the current time as the start time of the TaskManager instance.
            start_time: std::time::Instant::now(),
            // Initialize the list of tasks with an empty Vec value, wrapped in an Arc and Mutex.
            list: Arc::new(Mutex::new(Vec::new())),

            max_threads: num_cpus::get() as u32,

            current_threads: 0,
        }
    }

    // This method adds a task to the TaskManager's list of tasks.
    fn add_task(&mut self, task: Task) {
        // Lock the list of tasks to ensure that no other threads can access it while we are modifying it.
        let mut list = self.list.lock().unwrap();
        // Add the task to the list.
        list.push(task);
    }

    // This method creates a new task with the given name and closure, and adds it to the TaskManager's list of tasks.
    pub fn create_task(&mut self, name: String, closure: impl Fn() -> () + Send + Sync + 'static) {
        // Calculate the id of the new task by taking the length of the list of tasks + 1.
        let task_id = self.list.lock().unwrap().len() as u32 + 1;
        // Create a new Task instance with the calculated id, name, and closure.
        let task = Task::new(task_id, name, closure);
        // Add the new task to the TaskManager's list of tasks.
        self.add_task(task);
    }

    // This method iterates over the TaskManager's list of tasks and runs each task.

    pub fn run_tasks(&mut self) {
        // Lock the list of tasks to ensure that no other threads can access it while we are iterating over it.
        let list = self.list.lock().unwrap();
        // Iterate over the list of tasks.
        for task in list.iter() {
            
            if (self.current_threads >= self.max_threads) {
                //wait for a thread to finish
                std::thread::sleep(std::time::Duration::from_secs(1));
                continue;
            }


            // Make a copy of the task to ensure that it does not move out of the list.
            let task_copy = (*task).clone();
            self.current_threads += 1;
            // Run the task.
            task_copy.run();
        }
        
    }
    
}




fn main() {
    let mut task_manager = TaskManager::new();
    task_manager.create_task("LongTimer".to_string(), || {
        std::thread::sleep(std::time::Duration::from_secs(10));
    });
    task_manager.create_task("ShortTimer".to_string(), || {
        std::thread::sleep(std::time::Duration::from_secs(5));
    });

    task_manager.run_tasks();

    //wait for all tasks to finish
    std::thread::sleep(std::time::Duration::from_secs(30));
    println!("All tasks finished")
}
