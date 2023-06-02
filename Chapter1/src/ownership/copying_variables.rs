pub fn copying_variables() {
    println!("--primitives--");
    let one: i8 = 10;
    let two: i8 = one + 5;

    println!("{}", one); // output 10
    println!("{}", two); // output 15

    println!("--non-primitives--");
    let one = "one".to_string();
    let two = one;

    // println!("{}", one); // not allowed
    // output: move occurs because `one` has type `String`, which does not implement the `Copy` trait
    println!("{}", two);

    let one: String = String::from("one");
    // let two: String = one + "two";
    // We can use to_owned() to enable us to be able to print
    let two: String = one.to_owned() + "two";

    // to owned is a similar method to clone() and both have the same underlyig mechanics

    println!("{}", one);
    // The above would throw this error:
    // --- move occurs because `one` has type `String`, which does not implement the `Copy` trait
    // let two: String = one + " two";
    // ------------ `one` moved due to usage in
    // operator println!("{}", two); println!("{}", one);
    // ^^^ value borrowed here after move
    println!("{}", two);


    println!("--passing-reference-types-to-functions--");

    let string_to_pass = "one".to_string();

    print_string(string_to_pass);
    // the following printline wont work as the functio takes ownership
    // of the string and doesnt return it anywhere
    // println!("{}", string_to_pass) // ref error
}

pub fn print_string(value: String) {
    println!("{}", value);
}