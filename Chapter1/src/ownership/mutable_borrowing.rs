//effectively acting as main
pub fn mutable_borrowing() {
    let mut one: i8 = 5;
    print(&mut one);
    println!("In main the value is: {}", one);

    // the following wont work as there can only be 1 mutable reference at a time.
    // print_again(&mut one, &mut one); //  error[E0499]: cannot borrow `one` as mutable more than once at a time
    scopes_incorrect();
    scopes_correct();
}

fn scopes_correct() {
    let one = &"one";
    let two: &str;
    {
        println!("Correct inner {}", one);
        two = &"two";
    }
    println!("Correct {}", one);
    println!("Correct {}", two);
}

fn scopes_incorrect() {
    let one = &"one";
    {
        println!("{}", one);
        let two = &"two";
    }

    println!("{}", one);
    // two cannot be found in this scope as it finished in the line above
    // println!("{}", two); // Err
}

fn print(value: &mut i8) {
    // remember dereferencing needs to occur to mutate
    *value += 1;
    println!("In function the value is {}", value)
}

fn print_again(value: &mut i8, value_two: &mut i8) {
    *value += 1;
    println!("In function the value is: {}", value);
    *value_two += 1;
}
