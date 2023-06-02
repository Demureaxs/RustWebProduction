use async_std;
use futures::join;
use futures::{
    executor::block_on,
    future::{self, join, join_all},
};
use std::vec::Vec;
use std::{result, thread, time};

async fn do_something(number: i8) -> i8 {
    println!("async number {} is running", number);
    let two_seconds = time::Duration::new(2, 0);
    thread::sleep(two_seconds);
    return 2;
}

fn run_async_await_a() {
    let now = time::Instant::now();
    let future_one = do_something(1);
    let two_seconnds = time::Duration::new(2, 0);
    thread::sleep(two_seconnds);
    let outcome = block_on(future_one);
    println!("Time elapsed {:?}", now.elapsed());
    println!("Here is the outcome: {}", outcome);
}

fn run_async_await_b() {
    let now = time::Instant::now();
    let future_two = async { return do_something(2).await };
    let future_two = block_on(future_two);
    println!("Time elapsed {:?}", now.elapsed());
    println!("Here is the outcome: {}", future_two)
}

fn run_async_await_c() {
    let now = time::Instant::now();
    let future_three = async {
        let outcome_one = do_something(2).await;
        let outcome_two = do_something(3).await;
        return outcome_one + outcome_two;
    };
    let future_outcome = block_on(future_three);
    println!("Time elapsed {:?}", now.elapsed());
    println!("Here is the outcome: {:?}", future_outcome)
}

fn run_async_await_d() {
    let future_four = async {
        let outcome_one = do_something(2);
        let outcome_two = do_something(3);
        let results = join!(outcome_one, outcome_two);
        return results.0 + results.1;
    };
    let now = time::Instant::now();
    let result = block_on(future_four);

    println!("Time elapsed {:?}", now.elapsed());
    println!("Here is the result: {:?}", result);
}

fn run_async_await_e() {
    let async_outcome = async {
        //1.
        let mut futures_vec = Vec::new();
        let future_four = do_something(4);
        let future_five = do_something(5);
        //2.
        futures_vec.push(future_four);
        futures_vec.push(future_five);

        //3.
        let handles = futures_vec
            .into_iter()
            .map(async_std::task::spawn)
            .collect::<Vec<_>>();

        //4.
        let results = join_all(handles).await;
        return results.into_iter().sum::<i8>();
    };

    let now = time::Instant::now();
    let result = block_on(async_outcome);
    println!("Time elapsed for join vec {:?}", now.elapsed());
    println!("Here is the result: {:?}", result);
}

use std::sync::Arc;

// two methods for beig able to reference data in threads
fn run_async_await_arc() {
    let names = Arc::new(vec!["dave", "chloe", "simon"]);
    let reference_data = Arc::clone(&names);

    let new_thread = thread::spawn(move || println!("{}", reference_data[1]));
}

// and mutex for mutating data outside the thread.
use std::sync::Mutex;

fn run_async_await_mutex() {
    let count = Mutex::new(0);

    let new_thread = thread::spawn(move || {
        *count.lock().unwrap() += 1;
        count
    });
    println!("{:?}", new_thread.join().unwrap().into_inner().unwrap());
}

pub fn run_async_await() {
    // run_async_await_a();
    // run_async_await_b()
    // run_async_await_c();
    // run_async_await_d()
    // run_async_await_e()
    run_async_await_arc();
    run_async_await_mutex()
}
