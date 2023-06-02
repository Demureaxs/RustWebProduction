pub fn immutable_borrow() {
    let string_to_pass = "one".to_string();

    print_string(&string_to_pass);
    println!("{}", string_to_pass);

    // the following works because we can have unlimited immutable references
    print_again(&string_to_pass, &string_to_pass);
}

fn print_again(value: &String, value_two: &String) {
    println!("Value 1: {}", value);
    println!("Value 2: {}", value_two);
}

pub fn print_string(value: &String) {
    println!("{}", value);
}
