pub fn lifetimes() {
    how_not_to_lifetime();
    incorrect_lifetime_2();
    correct_lifetimes();
}

fn correct_lifetimes() {
    let one: i8 = 1;
    let outcome: &i8;
    {
        let two: i8 = 2;
        outcome = filter(&one, &two);
        // filter(two, one) wouldnt work as the lifetime ends in the inner scope
    }
    println!("Correct Lifetime Outcome: {}", outcome);
}
// because a and b are different liftimes and b (two) is not returned the 
// compiler is happy
fn filter<'a, 'b>(first_number: &'a i8, second_number: &'b i8) -> &'a i8 {
    if first_number < second_number {
        return &0;
    } else {
        return first_number;
    }
}

fn incorrect_lifetime_2() {
    //--liftimes demo bad ex
    let one: i8 = 1;
    let outcome: &i8;
    {
        let two: i8 = 2;
        let outcome: &i8 = get_highest(&one, &two);
    }
    // println!("{}", outcome); // gives error outcome isnt initialized
    // this is because the compiler doesnt know which liftime to return,
    // as they are both labelled &'a
}

// demonstrating lifetimes also apply to functions
fn get_highest<'a>(first_number: &'a i8, second_number: &'a i8) -> &'a i8 {
    if first_number > second_number {
        return first_number;
    } else {
        return second_number;
    }
}

fn how_not_to_lifetime() {
    // let one: &i8;
    // {
    //     let two: i8 = 2;
    //     one = &two // two does not live long enough
    // } // --- two liftime ends here
    // println!("r: {}", one);
}
