pub fn string_examples() {
    let string_one = "hello world".to_owned();
    let string_two = "hello world".to_string();
    // this is expensive as a string literal being cloned takes a lot of memory.
    let string_three = string_two.clone();
}

pub fn string_joins() {
    let data = ["one", "two", "three", "four"];
    
    // slow method
    let mut string = "".to_string();
    for word in data {
        string.push_str(word);
    }
}
