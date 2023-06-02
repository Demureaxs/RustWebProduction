mod state;
mod to_do;
mod processes;

use std::{env};

use serde_json::value::Value;
use serde_json::Map;

use state::read_file;
use to_do::to_do_factory;
use to_do::enums::TaskStatus;
use processes::process_input;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command: &String = &args[1];
    let title: &String = &args[2];

    let state: Map<String, Value> = read_file("./state.json");
    let status: String;

    match &state.get(*&title) {
        Some(result) => {
            status = result.to_string().replace('\"', "");
        }
        None => {
            status = "pending".to_owned()
        }
    }

    let item = to_do_factory(title, TaskStatus::from_string(status.to_uppercase()));
    process_input(item, command.to_string(), &state)

}


// Imports before processes
// use serde_json::value::Value;
// use serde_json::{json, Map};
// use std::env;
// use todo_app::state::{read_file, write_to_file};

// main before processes
// fn main() {
//     // obtains the arguments from cl execution
//     let args: Vec<String> = env::args().collect();
//     // status is the first argument
//     let status: &String = &args[1];
//     // title is the second argument
//     let title: &String = &args[2];
//     // uses the Map type to read the file to state
//     let mut state: Map<String, Value> = read_file("./state.json");
//     println!("Before operation: {:?}", state);
//     // inserts the title and status to the map
//     state.insert(title.to_string(), json!(status));
//     println!("After operation: {:?}", state);
//     // writes the Map to file
//     write_to_file("./state.json", &mut state);
// }
