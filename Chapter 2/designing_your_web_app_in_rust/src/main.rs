use clap::{App, Arg};
use rand::prelude::*;
use std::env;

// three slashes allow us to autogenerate markdown documentation
/// This function generates a float number using a number
/// /// generator passed into the function.
///
/// # Arguments
/// * generator (&mut ThreadRng): the random number
/// generator to generate the random number
///
/// # Returns
/// /// (f64): random number between 0 -> 10
fn generate_float(generator: &mut ThreadRng) -> f64 {
    let placeholder: f64 = generator.gen();
    return placeholder * 10.0;
}

/// This trait defines the struct to be a user
trait IsUser {
    /// This function proclaims that the struct is a user. ///
    /// # Arguments
    /// None
    ///
    /// # Returns
    /// (bool) true if user, false if not
    fn is_user() -> bool {
        return true;
    }
}

/// This struct defines a user
///
/// # Attributes
/// * name (String): the name of the user
/// * age (i8): the age of the user
struct User {
    name: String,
    age: i8,
}

fn main() {
    let app = App::new("booking")
        .version("1.0")
        .about("Books in a user")
        .author("Martyn Clarke");

    let first_name = Arg::new("first name")
        .long("f")
        .takes_value(true)
        .help("first name of user")
        .required(true);

    let last_name = Arg::new("last name")
        .long("l")
        .takes_value(true)
        .help("last name of user")
        .required(true);

    let age = Arg::new("age")
        .long("a")
        .takes_value(true)
        .help("age of the user")
        .required(true);

    let app = app.arg(first_name).arg(last_name).arg(age);

    let matches = app.get_matches();

    let name = matches
        .value_of("first name")
        .expect("First name is required");

    let surname = matches.value_of("last name").expect("Surname is required");

    let age: i8 = matches
        .value_of("age")
        .expect("Age is required")
        .parse()
        .unwrap();

    println!("{:?}", name);
    println!("{:?}", surname);
    println!("{:?}", age);
}
