#[macro_use] extern crate rocket;

use rocket::serde::Deserialize;
use rocket::serde::json::Json;
use rocket::serde::Serialize;

#[serde(crate = "rocket::serde")]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub title: String,
    pub shortname: String,
    pub description: String,
    pub genre: Vec<String>,
    pub rating: String,
    pub language: String,
    pub cast: Vec<Cast>,
    pub crew: Crew,
    pub distributor: Vec<String>,
    pub box_office: BoxOffice,
    pub release: Release,
    pub country_of_origin: Vec<String>,
    pub runtime: Runtime,
}

#[serde(crate = "rocket::serde")]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cast {
    pub name: String,
    pub role: String,
}

#[serde(crate = "rocket::serde")]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Crew {
    pub director: Vec<String>,
    pub producer: Vec<String>,
    pub writer: Vec<String>,
}

#[serde(crate = "rocket::serde")]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoxOffice {
    pub budget: String,
    pub gross: Gross,
    pub opening_weekend: OpeningWeekend,
}

#[serde(crate = "rocket::serde")]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gross {
    pub us: String,
    pub worldwide: String,
}
#[serde(crate = "rocket::serde")]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpeningWeekend {
    pub us: String,
    pub worldwide: String,
}

#[serde(crate = "rocket::serde")]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Release {
    pub year: i64,
    pub month: i64,
    pub day: i64,
}

#[serde(crate = "rocket::serde")]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Runtime {
    pub hours: i64,
    pub minutes: i64,
    pub seconds: i64,
}


#[get("/")]
fn index() -> Json<Root> {
    let data = Root {
        title: "Deadpool".to_string(),

        shortname: "".to_string(),
        description: "".to_string(),
        genre: vec![],
        rating: "".to_string(),
        language: "".to_string(),
        cast: vec![],
        crew: Default::default(),
        distributor: vec![],
        box_office: Default::default(),
        release: Default::default(),
        country_of_origin: vec![],
        runtime: Default::default(),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}