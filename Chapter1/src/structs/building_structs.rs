#[derive(Debug)]
enum Friend {
    HUMAN(Box<Human>),
    NIL,
}

#[derive(Debug)]
struct Human {
    name: String,
    age: i8,
    current_thought: Option<String>,
    friend: Friend,
}
//implementation of a constructor for Human
impl Human {
    fn new(name: &str, age: i8) -> Human {
        return Human {
            name: name.to_string(),
            age: age,
            current_thought: None,
            friend: Friend::NIL,
        };
    }
    // these can be chained on to the creation
    // see with constructor function
    fn with_thought(mut self, thought: &str) -> Human {
        self.current_thought = Some(thought.to_string());
        return self;
    }
    fn with_friend(mut self, friend: Box<Human>) -> Human {
        self.friend = Friend::HUMAN(friend);
        return self;
    }
}

fn building_structs() {
    let another_developer = Human {
        name: "Caroline Morton".to_string(),
        age: 30,
        current_thought: Some("I need to code".to_string()),
        friend: Friend::NIL,
    };
    let developer = Human {
        name: "Maxwell Flitton".to_string(),
        age: 32,
        current_thought: Some("nothing".to_string()),
        friend: Friend::HUMAN(Box::new(another_developer)),
    };

    match &developer.friend {
        Friend::HUMAN(data) => {
            println!("{}", data.name)
        }
        Friend::NIL => {}
    }

    println!("{:#?}", developer);
    println!("{}", developer.name);
}

fn building_structs_with_impl() {
    let developer_friend = Human::new("Caroline Morton", 30);
    // chained as explained in implementation block
    let developer = Human::new("Maxwell Flitton", 32)
        .with_thought("I love rust")
        .with_friend(Box::new(developer_friend));
    println!("{:#?}", developer)
}

pub fn run_building_structs() {
    building_structs();
    building_structs_with_impl()
}
