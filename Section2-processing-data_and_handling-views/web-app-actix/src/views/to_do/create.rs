use actix_web::HttpRequest;
use serde_json::value::Value;
use serde_json::Map;
use actix_web::HttpResponse;

use crate::processes::process_input;
use crate::state::read_file;
use crate::to_do::{enums::TaskStatus, to_do_factory};
//imports the todo items from json serialization
use crate::json_serialization::to_do_items::ToDoItems;

pub async fn create(req: HttpRequest) -> HttpResponse {
    let state: Map<String, Value> = read_file("./state.json"); // step 1
    let title: String = req.match_info().get("title").unwrap().to_string(); // step 2
    let item = to_do_factory(&title.as_str(), TaskStatus::PENDING); // step 3

    process_input(item, "create".to_string(), &state); // step 4

    return HttpResponse::Ok().json(ToDoItems::get_state());
}
