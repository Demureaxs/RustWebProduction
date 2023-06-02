fn test_closure() {
    // closures are just annonymous functions
    // however their type will be locked and cannot use generics
    // to be used inside methods
    let test_closure = |string_input| {
        println!("standard closure: {}", string_input);
    };
    test_closure("test")
}

fn closure_scope_fail() {
    {
        let test_closure = |string_input: String| {
            println!("{}", string_input);
        };
    }
    println!("Closure scope fail")
    // test_closure("test") // wont work as closure no longer exists
    // liftime expired in the block
}

fn closure_scope_correct() {
    // this works because closures can see outer scopes
    let another_str = "case";
    let test_closure =
        |string_input| println!("outer scopes closures: {} {}", string_input, another_str);
    test_closure("test")
}

fn taking_ownership_in_closures() {
    let another_str = "case";
    let test_closure = move |string_input| {
        println!("closer ownership {} {}", string_input, another_str);
    };
    // note after test closure another_str can no longer be used
    // due to move
    test_closure("test")
}

fn passing_closures_into_fnctns() {
    // function takes a closure as an argument that returns i-32
    // takes 2 numbers one and two
    // returns the result of the closure(one) + closure(two)
    fn add_doubles(closure: fn(i32) -> i32, one: i32, two: i32) -> i32 {
        let temp = closure(one) + closure(two);
        return temp;
    }
    // defines the closure used in the above function
    let closure = |int_input| {
        return int_input * 2;
    };
    // utilizes the function with the closure called
    let outcome = add_doubles(closure, 2, 3);
    println!("closures in functions {}", outcome);
}

fn moves_and_mutability() {
    fn add_doubles(closure: Box<dyn Fn(i32) -> i32>, one: i32, two: i32) -> i32 {
        return closure(one) + closure(two);
    }
    let one = 2;
    let closure = move |int_input| {
        return int_input * one;
    };
    let outcome = add_doubles(Box::new(closure), 2, 3);
    println!("moves and mutability {}", outcome);
}

pub fn run_understanding_closures() {
    test_closure();
    closure_scope_fail();
    closure_scope_correct();
    taking_ownership_in_closures();
    passing_closures_into_fnctns();
    moves_and_mutability();
}
