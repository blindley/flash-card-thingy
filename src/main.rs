#![allow(dead_code)]
#![allow(unused_imports)]

#[macro_use] extern crate rocket;
use rocket::{response::content::RawHtml, State};
use rocket::serde::json::{Json};
use rocket::form::Form;


use std::sync::{Arc, Mutex};
use std::collections::{VecDeque, BTreeMap};

mod card;
mod deck;
mod progress;

use deck::Deck;
use progress::UserProgress;

struct AppData {
    pub user_progress_filepath: std::path::PathBuf,
    pub user_progress: UserProgress,
    pub deck: Deck,
    pub due_cards: VecDeque<progress::CardId>,
}

#[launch]
fn rocket() -> _ {
    let figment = rocket::Config::figment()
         .merge(("port", 8080));

    let user_progress_filepath = "data/users/sample-user/user-progress.json".into();

    let mut user_progress = {
        UserProgress::load(&user_progress_filepath).unwrap()
    };

    let deck = {
        let path = "data/decks/sample.json";
        Deck::load(path).unwrap()
    };

    let mut due_cards = VecDeque::new();

    for card in deck.cards.iter() {
        let uuid = card.get_uuid();
        let instances = match card.get_field("instances") {
            None => 1,
            Some(instance_str) => {
                // TODO: Introduce some logging here to warn about invalid values
                match instance_str.parse() {
                    Err(_) => 1,
                    Ok(n) => if n < 1 { 1 } else { n },
                }
            },
        };

        for instance in 1..=instances {
            let card_id = progress::CardId { uuid, instance, };
            let card_progress =
                user_progress.cards.entry(card_id)
                .or_insert_with(|| progress::CardProgress::new());
            if card_progress.due >= chrono::Local::now().date_naive() {
                due_cards.push_back(card_id);
            }
        }
    }

    let app_data = AppData {
        user_progress_filepath, user_progress, deck, due_cards,
    };

    let app_data = Arc::new(Mutex::new(app_data));
    rocket::custom(figment)
        .manage(app_data)
        .mount("/", routes![index, index2])
}

#[get("/")]
fn index(app_data: &State<Arc<Mutex<AppData>>>) -> RawHtml<String>
{
    let name_of_this_app = "pending...";

    let mut lines: Vec<String> = Vec::new();
    lines.push("<!DOCTYPE html>".into());
    lines.push("<html>".into());
    lines.push(format!("<head><title>{}</title></head>", name_of_this_app));
    lines.push("<body>".into());

    {
        let app_data = app_data.lock().unwrap();
        if app_data.due_cards.is_empty() {
            lines.push("<h1>No cards due</h1>".into());
        } else {
            let mut card_found = false;
            let card_id = app_data.due_cards[0];
            for card in app_data.deck.cards.iter() {
                if card.get_uuid() == card_id.uuid {
                    lines.push(card.to_html(card_id.instance));
                    card_found = true;
                    break;
                }
            }
            if !card_found {
                lines.push("<h1>Error: card not found</h1>".into());
            }
        }
    }

    lines.push("</body>".into());
    lines.push("</html>".into());

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
    println!("pass is {}", myform.pass);
    let pass = myform.pass == "pass";
    println!("pass is {pass}");

    let name_of_this_app = "pending...";

    let mut lines: Vec<String> = Vec::new();
    lines.push("<!DOCTYPE html>".into());
    lines.push("<html>".into());
    lines.push(format!("<head><title>{}</title></head>", name_of_this_app));
    lines.push("<body>".into());

    {
        let mut app_data = app_data.lock().unwrap();
        {
            let card_id = app_data.due_cards[0];
            let card_progress = app_data.user_progress.cards.get_mut(&card_id).unwrap();

            if pass {
                card_progress.pass();
                app_data.due_cards.pop_front();
            } else {
                card_progress.fail();
                app_data.due_cards.push_back(card_id);
                app_data.due_cards.pop_front();
            }
        }

        if app_data.due_cards.is_empty() {
            lines.push("<h1>No cards due</h1>".into());
        } else {
            let mut card_found = false;
            let card_id = app_data.due_cards[0];
            for card in app_data.deck.cards.iter() {
                if card.get_uuid() == card_id.uuid {
                    lines.push(card.to_html(card_id.instance));
                    card_found = true;
                    break;
                }
            }
            if !card_found {
                lines.push("<h1>Error: card not found</h1>".into());
            }
        }
    }

    lines.push("</body>".into());
    lines.push("</html>".into());

    let mut html = String::new();
    for line in lines.iter() {
        html.push_str(&line);
        html.push('\n');
    }
    return RawHtml(html)
}
