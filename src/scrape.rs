use rocket::serde::{Serialize, js};
use rocket::json

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Data {

}

pub fn scrape() -> Json<Data> {

}