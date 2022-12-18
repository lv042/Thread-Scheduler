use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// The TaskScheduler struct holds a vector of tasks stored in a mutex.
// The mutex allows multiple threads to access and modify the tasks concurrently.
struct TaskScheduler {
    tasks: Arc<Mutex<Vec<Task>>>,
}

impl TaskScheduler {
    // Creates a new task scheduler with an empty tasks vector.
    fn new() -> TaskScheduler {
        TaskScheduler {
            tasks: Arc::new(Mutex::new(Vec::new())),
        }
    }

    // Adds a task to the tasks vector.
    fn add_task(&mut self, task: Task) {
        self.tasks.lock().unwrap().push(task);
    }

    // Starts the task scheduler by creating a number of threads equal to the number of CPU cores available.
    // Each thread retrieves a task from the tasks vector and runs it. When there are no more tasks left,
    // the threads will terminate.
    fn start(&self) {
        let tasks = self.tasks.clone();

        for _ in 0..num_cpus::get() {
            let thread_tasks = tasks.clone();
            thread::spawn(move || {
                loop {
                    let task = {
                        let mut tasks = thread_tasks.lock().unwrap();
                        tasks.pop()
                    };

                    if let Some(task) = task {
                        task.run();
                    } else {
                        // If there are no more tasks, park the thread for 1 second.
                        // This allows other threads to run and makes the task scheduler more fair.
                        thread::park_timeout(Duration::from_secs(1));
                    }
                }
            });
        }
    }
}

// The Task struct holds the data and logic for a task.
struct Task {
    // task data and logic goes here
}

impl Task {
    // Runs the task.
    fn run(&self) {
        // perform task here
    }
}

fn main() {
    // Create a new task scheduler.
    let mut scheduler = TaskScheduler::new();

    // Add some tasks to the scheduler.
    scheduler.add_task(Task {});
    scheduler.add_task(Task {});
    scheduler.add_task(Task {});

    // Start the scheduler.
    scheduler.start();
}