use std::vec;

enum SomeValue {
    StringValue(String),
    IntValue(i32),
}

pub fn arrays_and_vectors() {
    println!("--Arrays--");
    let multi_array: [SomeValue; 4] = [
        SomeValue::StringValue("One".to_string()),
        SomeValue::IntValue(2),
        SomeValue::StringValue("Three".to_string()),
        SomeValue::IntValue(4),
    ];

    for item in multi_array {
        match item {
            SomeValue::StringValue(data) => {
                println!("The string is: {}", data);
            }
            SomeValue::IntValue(data) => {
                println!("The int is: {}", data);
            }
        }
    } // Prints -
      // The string is: One
      // The int is: 2
      // The string is: Three
      // The int is: 4

    println!("--Vecs--");
    let mut string_vector: Vec<&str> = vec!["one", "two", "three"];
    println!("{:?}", string_vector); // Prints ["one", "two", "three"]
    string_vector.push("four");
    println!("{:?}", string_vector); // Prints ["one", "two", "three", "four"]

    // declaring an empty vector required a type declaration
    let _empty_vector: Vec<&str> = Vec::new();

    println!("--HashMaps--");

    use std::collections::HashMap;
    #[derive(Debug)]
    enum CharacterValue {
        Name(String),
        Age(i32),
        Items(Vec<String>),
    }

    // creates a hasmap that holds a string as a key and a character value as a value
    let mut profile: HashMap<&str, CharacterValue> = HashMap::new();
    // inserting values into a HashMap
    profile.insert("name", CharacterValue::Name("Maxwell".to_string()));
    profile.insert("age", CharacterValue::Age(32));
    profile.insert(
        "items",
        CharacterValue::Items(vec![
            "Laptop".to_string(),
            "Phone".to_string(),
            "book".to_string(),
        ]),
    );
    println!("{:?}", profile); // Prints {"name": Name("Maxwell"), "age": Age(32), "items": Items(["Laptop", "Phone", "book"])}

    // getting values from a HashMap
    match profile.get("name") {
        // checks if a value is present
        Some(value_data) => {
            // if so checks if value data is a string
            match value_data {
                CharacterValue::Name(name) => {
                    // if it is print the name
                    println!("The name is: {}", name); // Output: The name is: Maxwell
                }
                // otherwise panic with name should be a string
                _ => panic!("Name should be a string!"),
            }
        }
        // if no name is present print that to console.
        None => {
            println!("Name is not present!")
        }
    }

    // we can simplify the above as
    match profile.get("name").unwrap() {
        CharacterValue::Name(name) => {
            println!("The name is: {}", name)
        }
        _ => panic!("The name should be a string"),
    }
}
