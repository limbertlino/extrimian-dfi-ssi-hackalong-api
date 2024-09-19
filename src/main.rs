#[macro_use]
extern crate rocket;
use rocket::{
    serde::json::{json, Value},
    Build, Rocket,
};

mod models;
mod routes;
mod services;
mod utils;

use crate::routes::{check_health, create_new_vc};

#[rocket::main]
async fn main() {
    let _ = rocket().launch().await;
}

pub fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![check_health, create_new_vc])
        .register(
            "/",
            catchers![
                handle_not_found,
                handle_just_500,
                handle_unproccessable_entity
            ],
        )
}

#[catch(404)]
/// Handler for 404 errors
fn handle_not_found() -> Value {
    json!({"error": 404, "message": "Resource not found"})
}

#[catch(500)]
/// Handler for 500 internal Server Errors.
fn handle_just_500() -> Value {
    json!({"error": 500, "message": "Internal Server Error"})
}

#[catch(422)]
/// Handler for 422 Unprocessable Entity error (validation issued)
fn handle_unproccessable_entity() -> Value {
    json!({"error": 422, "message": "Unprocessable entity: Validation failed"})
}

#[cfg(test)]
mod tests;
