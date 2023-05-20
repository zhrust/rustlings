// threads2.rs
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a hint.
// Building on the last exercise, we want all of the threads to complete their work but this time
// the spawned threads need to be in charge of updating a shared value: JobStatus.jobs_completed


use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    //let status = Arc::new(JobStatus { jobs_completed: 0 });
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // TODO: You must take an action before you update a shared value
            let mut status = status_shared.lock().unwrap();
            //status_shared.jobs_completed += 1;
            status.jobs_completed += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
        // TODO: Print the value of the JobStatus.jobs_completed. Did you notice anything
        // interesting in the output? Do you have to 'join' on all the handles?
        let status = status.lock().unwrap();
        println!("jobs completed {}", status.jobs_completed);
    }
}

/*

Arc 是一个共享引用计数类型，它允许多个所有者共享相同的数据。但是，由于多个线程可以同时访问数据，如果允许直接修改 Arc 的内部数据，会导致不安全的访问和数据竞争问题。因此，Arc 只允许对其包含的数据进行不可变引用，并不允许修改其内部数据。

为了克服这个问题，您可以通过使用互斥锁（mutex）来保护 JobStatus 中的数据，并在需要修改数据时获得锁。这将确保只有一个线程可以访问 JobStatus 中的数据，并且可以避免数据竞争和其他并发问题。

以下是您可以使用的修改后的代码，使用 Mutex 来保护 JobStatus 中的数据：

JobStatus 被封装在一个 Mutex 中，并通过 Arc 共享。在子线程中，我们使用 lock 方法来获取 Mutex 的互斥锁，并获取 JobStatus 的可变引用。只有一个线程可以同时持有 Mutex 的互斥锁，并且在该线程释放锁之前，其他线程将阻塞在调用 lock 方法时。

一旦子线程修改了 JobStatus 中的数据，它会释放 Mutex 的互斥锁。在主线程中，我们再次使用 lock 方法来获取 Mutex 的互斥锁，并获取 JobStatus 的不可变引用。由于只有一个线程可以同时持有 Mutex 的互斥锁，因此在获取 JobStatus 的不可变引用时，我们可以确保所有子线程都已经完成并更新了 JobStatus 中的数据。最后，我们打印更新后的 JobStatus 数据。
*/
