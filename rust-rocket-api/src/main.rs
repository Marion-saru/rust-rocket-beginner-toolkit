#[macro_use] extern crate rocket;

use rocket::serde::json::Json;
use rocket::serde::Serialize;

#[derive(Serialize)]
struct Greeting {
    message: &'static str,
}

#[derive(Serialize)]
struct Congrats {
    message: &'static str,
}

#[derive(Serialize)]
struct Health {
    status: &'static str,
}

// Route handlers

#[get("/")]             // Greetings endpoint
fn index() -> Json<Greeting> {
    Json(Greeting { message: "Hello from Rust!" })
}


#[get("/congrats")]             // Congratulations endpoint
fn congrats() -> Json<Congrats> {
    Json(Congrats { message: "Wow! Well done Marion you just made your first Rust-Rocket API.👏" })
}

#[get("/health")]             // Health check endpoint  
fn health() -> Json<Health> {
    Json(Health { status: "ok" })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, congrats, health])
}