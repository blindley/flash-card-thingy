#[macro_use] extern crate rocket;
use rocket::response::content::RawHtml;

mod card;
mod deck;

use deck::Deck;


#[launch]
fn rocket() -> _ {
    let figment = rocket::Config::figment()
         .merge(("port", 8080));
    rocket::custom(figment)
        .mount("/", routes![index, card_by_index])
}

#[get("/")]
fn index() -> RawHtml<String>
{
    card_by_index(0)
}

#[get("/<index>")]
fn card_by_index(index: usize) -> RawHtml<String>
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
