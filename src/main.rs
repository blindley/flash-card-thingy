#[macro_use] extern crate rocket;
use rocket::response::content::RawHtml;

mod card;
use card::Card;


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

#[get("/")]
fn index() -> RawHtml<String>
{
    let mut card = Card::new("basic");
    card.set_field("front", "Front of Card");
    card.set_field("back", "Back of Card");

    let page_content = card.to_html();

    RawHtml(page_content)
}
