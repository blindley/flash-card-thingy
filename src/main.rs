#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

#[get("/")]
fn index() -> String {
    let p = "data/templates/basic.html";
    std::fs::read_to_string(p)
        .unwrap_or_else(|e| format!("Error loading '{p}' : {e}"))
}
