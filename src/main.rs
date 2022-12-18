use std::sync::{Arc, Mutex};
use std::thread;

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
        // We use an Arc (atomic reference counted pointer) to share the tasks vector among the threads.
        // The Arc allows multiple threads to access the tasks vector concurrently.
        let tasks = self.tasks.clone();

        // Create a thread for each CPU core.
        for _ in 0..num_cpus::get() {
            // We clone the Arc to give each thread its own copy.
            let thread_tasks = tasks.clone();

            // Spawn the thread and move the thread_tasks variable into the closure.
            thread::spawn(move || {
                // Loop until there are no more tasks left.
                loop {
                    // Retrieve a task from the tasks vector.
                    // We use a mutex to synchronize access to the tasks vector.
                    let task = {
                        let mut tasks = thread_tasks.lock().unwrap();
                        tasks.pop()
                    };

                    // If there is a task, run it. Otherwise, break out of the loop.
                    if let Some(task) = task {
                        task.run();
                    } else {
                        break;
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