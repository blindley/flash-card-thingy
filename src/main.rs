#![allow(dead_code)]
#![allow(unused_imports)]

#[macro_use] extern crate rocket;
use rocket::{response::content::RawHtml, State};
use rocket::serde::json::{Json};
use rocket::form::Form;


use std::sync::{Arc, Mutex};
use std::collections::{VecDeque, BTreeMap};

mod note;
mod deck;
mod progress;
mod session;

use deck::Deck;
use progress::UserProgress;

struct AppData {
    session_data: session::SessionData,
}

#[launch]
fn rocket() -> _ {
    let figment = rocket::Config::figment()
         .merge(("port", 8080));

    let session_data = session::SessionData::new();

    let app_data = AppData {
        session_data,
    };

    let app_data = Arc::new(Mutex::new(app_data));
    rocket::custom(figment)
        .manage(app_data)
        .mount("/", routes![index, index2])
}

fn standard_html_header() -> String
{
    let title = "pending...";
    format!(
        "<!DOCTYPE html>\n<html>\n<head>\n<title>{title}</title>\n<body>"
    )
}

fn standard_html_footer() -> String
{
    format!(
        "</body>\n</html>\n"
    )
}

#[get("/")]
fn index(app_data: &State<Arc<Mutex<AppData>>>) -> RawHtml<String>
{
    let mut lines: Vec<String> = Vec::new();
    lines.push(standard_html_header());

    {
        let app_data = app_data.lock().unwrap();
        lines.push(app_data.session_data.next_card_html());
    }

    lines.push(standard_html_footer());

    let mut html = String::new();
    for line in lines.iter() {
        html.push_str(&line);
        html.push('\n');
    }
    return RawHtml(html)
}

#[derive(FromForm)]
struct MyForm<'r> {
    pass: &'r str,
}

#[post("/", data = "<myform>")]
fn index2(myform: Form<MyForm<'_>>, app_data: &State<Arc<Mutex<AppData>>>) -> RawHtml<String>
{
    let pass = myform.pass == "pass";
    
    let mut lines: Vec<String> = Vec::new();
    lines.push(standard_html_header());

    {
        let mut app_data = app_data.lock().unwrap();

        if pass {
            app_data.session_data.pass();
        } else {
            app_data.session_data.fail();
        }

        lines.push(app_data.session_data.next_card_html());
    }

    lines.push(standard_html_footer());

    let mut html = String::new();
    for line in lines.iter() {
        html.push_str(&line);
        html.push('\n');
    }
    return RawHtml(html)
}
