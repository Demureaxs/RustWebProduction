use std::{thread, time};


pub fn threads_lesson_1() {
    println!("--Threads lesson 1--");
    fn do_something(number: i8) -> i8 {
        println!("Number: {}", number);
        let two_seconds = time::Duration::new(2, 0);
        thread::sleep(two_seconds);
        return 2;
    }
    fn run_threads_lesson_1() {
        let now = time::Instant::now();
        let one: i8 = do_something(1);
        let two: i8 = do_something(2);
        let three: i8 = do_something(3);

        println!("Time elapsed {:?}", now.elapsed());
        println!("Result: {}", one + two + three)
    }

    run_threads_lesson_1()
}

use std::thread::JoinHandle;

pub fn threads_lesson_1b() {
    println!("--Threads lesson 1b--");
    fn do_something(number: i8) -> i8 {
        println!("Number: {}", number);
        let two_seconds = time::Duration::new(2, 0);
        thread::sleep(two_seconds);
        return 2;
    }
    fn run_threads_lesson_1b() {
        let now = time::Instant::now();
        // feeding a closure into thread handle, note we cannot pass raw data, it expects a closure
        let thread_one: JoinHandle<i8> = thread::spawn(|| do_something(1));
        let thread_two: JoinHandle<i8> = thread::spawn(|| do_something(2));
        let thread_three: JoinHandle<i8> = thread::spawn(|| do_something(3));
        // we have to join each thread before we can use it
        let result_one = thread_one.join();
        // example of handling thread Results safely
        match result_one {
            Ok(result) => {
                println!("the result for {} is {}", result, "result_one");
            }
            Err(ref result) => {
                // downcast allows us to return the error as a string
                if let Some(string) = result.downcast_ref::<String>() {
                    println!("the error for {} is: {}", string, "result_one");
                } else {
                    println!("there error for {} does not have a message", "result_one");
                }
            }
        }
        let result_two = thread_two.join();
        let result_three = thread_three.join();

        println!("Time elapsed {:?}", now.elapsed());
        // each thread needs to be unwrapped as its a result type
        println!(
            "Result: {}",
            result_one.unwrap() + result_two.unwrap() + result_three.unwrap()
        )
    }

    run_threads_lesson_1b()
}
