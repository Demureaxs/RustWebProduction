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
    println!("--Threads lesson 1--");
    fn do_something(number: i8) -> i8 {
        println!("Number: {}", number);
        let two_seconds = time::Duration::new(2, 0);
        thread::sleep(two_seconds);
        return 2;
    }
    fn run_threads_lesson_1b() {
        let now = time::Instant::now();
        let thread_one: JoinHandle<i8> = thread::spawn(|| do_something(1));
        let thread_two: JoinHandle<i8> = thread::spawn(|| do_something(2));
        let thread_three: JoinHandle<i8> = thread::spawn(|| do_something(3));
        let result_one = thread_one.join();
        let result_two = thread_two.join();
        let result_three = thread_three.join();

        println!("Time elapsed {:?}", now.elapsed());
        println!(
            "Result: {}",
            result_one.unwrap() + result_two.unwrap() + result_three.unwrap()
        )
    }

    run_threads_lesson_1b()
}
