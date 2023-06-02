pub fn handling_results_and_errors() {
    println!("{:?}", error_check(false)); // Output: Ok(1)
    println!("{:?}", error_check(false).is_err()); // Output: false
    println!("{:?}", error_check(true)); // Output Err("This is an error")
    println!("{:?}", error_check(true).is_err()); // Output: true

    // let result: i8 = error_check(true).expect("this has been caught");
    // Output: thread 'main' panicked at 'this has been caught: "This is an error"', src/foundation/handling_results_and_errors.rs:7:39
}

pub fn error_check(check: bool) -> Result<i8, &'static str> {
    if check {
        Err("This is an error")
    } else {
        Ok(1)
    }
}
