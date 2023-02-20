#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/users")]
fn get_all() -> &'static str {
    "TODO GET ALL USERS"
}

#[post("/users")]
fn create_one() -> &'static str {
    "TODO CREATE USER"
}

#[put("/users")]
fn update_one() -> &'static str {
    "TODO UPDATE USER"
}

#[delete("/users")]
fn delete_one() -> &'static str {
    "TODO DELETE USER"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![get_all,create_one,update_one,delete_one])
}