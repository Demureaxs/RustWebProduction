#[derive(Debug)]
struct CoordinateA<T> {
    x: T,
    y: T,
}

fn mp_with_macros() {
    let one = CoordinateA { x: 50, y: 50 };
    let two = CoordinateA { x: 500, y: 500 };
    let three = CoordinateA { x: 5.6, y: 5.6 };

    println!("{:#?} {:#?} {:#?}", one, two, three);
    // output:
    // CoordinateA {
    //     x: 50,
    //     y: 50,
    // } CoordinateA {
    //     x: 500,
    //     y: 500,
    // } CoordinateA {
    //     x: 5.6,
    //     y: 5.6,
    // }
}
#[derive(Debug)]
struct CoordinateB<T, X> {
    x: T,
    y: X,
}

fn mp_with_macros_two_types() {
    let one = CoordinateB { x: 50, y: 500 };
    let two = CoordinateB { x: 5.6, y: 500 };
    let three = CoordinateB { x: 5.6, y: 50 };

    println!("{:#?}{:#?}{:#?}", one, two, three);
    // Output:
    // CoordinateB {
    //     x: 50,
    //     y: 500,
    // }CoordinateB {
    //     x: 5.6,
    //     y: 500,
    // }CoordinateB {
    //     x: 5.6,
    //     y: 50,
    // }
}
// macro to capitalize
macro_rules! capitalize {
    ($a: expr) => {
        // create a vector of chars from the input $a
        let mut v: Vec<char> = $a.chars().collect();
        // capitalize the first letter of the word
        v[0] = v[0].to_uppercase().nth(0).unwrap();
        // collect the word into an iterable
        $a = v.into_iter().collect();
    };
}

fn run_macro_rules() {
    let mut x = "test".to_string();
    capitalize!(x);
    println!("{}", x); // prints "Test"

    // note this can only be used with String types due to the methods
    // involked in the method
    // capitalize!(32) would rerturn an error as chars cannot be derived
    // from integers
}

pub fn run_mp_with_macros() {
    // mp_with_macros();
    // mp_with_macros_two_types()
    run_macro_rules()
}
