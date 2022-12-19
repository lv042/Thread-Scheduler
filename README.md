# Thread Scheduler
The Thread Scheduler is a utility for scheduling and running Threads concurrently. It allows you to limit the number of threads that run at the same time to the number of logical cores on the machine, create and add Threads to the scheduler using a closure and Thread name, and run all Threads concurrently in their own threads.



### Usage
Here is an example of how you might use the ThreadScheduler to perform a set of Threads concurrently:

```rs
use task_scheduler::TaskScheduler;

fn main() {
    // Create a new TaskScheduler instance.
    let mut scheduler = TaskScheduler::new();

    // Add tasks to the scheduler.
    scheduler.create_task("task1".to_string(), || {
        std::thread::sleep(std::time::Duration::from_secs(5));
        // Perform some task-specific work here...
    });

    scheduler.create_task("task2".to_string(), || {
        std::thread::sleep(std::time::Duration::from_secs(10));
        // Perform some task-specific work here...
    });

    scheduler.create_task("task3".to_string(), || {
        println!("Running task3");
        // Perform some task-specific work here...
    });

    // Run all tasks concurrently.
    scheduler.run_tasks();
}
```


### Limitations
The ThreadScheduler has the following limitations:

It is not possible to retrieve the results of the Threads that are run.
The Threads are run concurrently, but the order in which they are run is not guaranteed.

### License
This project is licensed under the MIT License.
