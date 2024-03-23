use actix_web::http::header::LOCATION;
use actix_web::HttpResponse;

use actix_web::web;
use secrecy::Secret;

use serde::{Serialize, Deserialize};

#[derive(serde::Deserialize)]
pub struct FormData {
    username: String,
    password: String,
}


// https://docs.rs/mongodb/latest/mongodb/
pub async fn login(form: web::Form<FormData>) -> HttpResponse {
    // print!("{} {}", form.username, form.password);

    HttpResponse::SeeOther()
        .insert_header((LOCATION, "/home"))
        .finish()
}