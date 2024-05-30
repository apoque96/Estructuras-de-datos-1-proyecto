// use crate::lab1::{lab1_function, Apartment};
use crate::lab3::auction::get_winner;
use rocket::form::Form;
use rocket::fs::TempFile;
use rocket::post;

#[derive(FromForm)]
pub struct Upload<'f> {
    database: TempFile<'f>,
    file: TempFile<'f>,
}

// #[post("/upload", data = "<data>")]
// pub async fn lab1_route(data: Form<Upload<'_>>) -> String {
//     let file = data.into_inner().file;
//     serde_json::to_string(&lab1_function(file).await).unwrap()
// }

#[post("/upload", data = "<data>")]
pub async fn lab3_route(data: Form<Upload<'_>>) -> String {
    let data = data.into_inner();
    let database = data.database;
    let file = data.file;
    serde_json::to_string(&get_winner(database, file).await).unwrap()
}
