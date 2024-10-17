// threads1.rs
//
// This program spawns multiple threads that each run for at least 250ms, and
// each thread returns how much time they took to complete. The program should
// wait until all the spawned threads have finished and should collect their
// return values into a vector.
//
// Execute `rustlings hint threads1` or use the `hint` watch subcommand for a
// hint.

// thread 模块用于创建和管理线程。
// Duration 用于表示时间段。
// Instant 用于测量时间间隔。
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let mut handles = vec![];
    for i in 0..10 {
        // move 关键字用于将所有权转移到闭包中，以便在线程中使用。
        handles.push(thread::spawn(move || {
            let start = Instant::now();     // 记录当前时间
            thread::sleep(Duration::from_millis(250));  // 休眠 250 毫秒
            println!("thread {} is complete", i);
            start.elapsed().as_millis()             // 返回主线程从 start 到现在的时间间隔，以毫秒为单位.
        }));
    }

    let mut results: Vec<u128> = vec![];
    // 遍历 handles 向量中的每个元素，每个元素都是一个 JoinHandle(线程句柄)。
    for handle in handles {
        // 使用 join 方法等待线程完成并获取返回值，方法返回一个 Result<T, E>，其中 T 是目标线程的返回值类型，E 是可能的错误类型。
        // expect("Thread panicked") 用于处理 Result，如果目标线程发生panic，程序会打印 "Thread panicked" 并终止。
        // 如果返回OK()，则会拆包出真正的数据。
        let result = handle.join().expect("Thread panicked");
        results.push(result);
    }

    if results.len() != 10 {
        panic!("Oh no! All the spawned threads did not finish!");
    }

    println!();
    // into_iter() 方法将 results 向量转换为一个迭代器。
    // 与 iter() 不同，into_iter() 会消耗（consume）原始向量，这意味着在调用 into_iter() 后，results 向量将不再可用。
    // enumerate() 方法对迭代器进行扩展，使其在生成每个元素的同时生成一个索引。
    for (i, result) in results.into_iter().enumerate() {
        println!("thread {} took {}ms", i, result);
    }
}
