#[macro_use] extern crate rocket;
use rocket::{response::content::RawHtml, State};

use std::sync::{Arc, Mutex};

mod card;
mod deck;
mod progress;

use deck::Deck;
use progress::UserProgress;

struct AppData {
    pub user_progress: UserProgress,
}

#[launch]
fn rocket() -> _ {
    let figment = rocket::Config::figment()
         .merge(("port", 8080));

    let mut app_data =
        AppData {
            user_progress: UserProgress::load("data/users/sample-user/user-progress.json").unwrap()
        };

    let app_data = Arc::new(Mutex::new(app_data));
    rocket::custom(figment)
        .manage(app_data)
        .mount("/", routes![index, card_by_index])
}

#[get("/")]
fn index() -> RawHtml<String>
{
    let name_of_this_app = "pending...";

    let mut lines: Vec<String> = Vec::new();
    lines.push("<!DOCTYPE html>".into());
    lines.push("<html>".into());
    lines.push(format!("<head><title>{}</title></head>", name_of_this_app));
    lines.push("<body>".into());

    let url = "/0";
    let deck_name = "sample-deck";
    lines.push(format!("<a href=\"{url}\">{deck_name}</a>"));

    lines.push("</body>".into());
    lines.push("</html>".into());

    let mut html = String::new();
    for line in lines.iter() {
        html.push_str(&line);
        html.push('\n');
    }
    return RawHtml(html)
}

#[get("/<index>")]
fn card_by_index(index: usize, app_data: &State<Arc<Mutex<AppData>>>) -> RawHtml<String>
{
    let deck_path = "data/decks/sample.json";
    let deck = Deck::load(deck_path).unwrap();

    if index < deck.cards.len() {
        let card = &deck.cards[index];
        let page_content = card.to_html();
        RawHtml(page_content)
    } else {
        let content = "<h1>ERROR: card does not exist</h1>";
        RawHtml(content.into())
    }
}
