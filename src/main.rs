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

    let mut page_content = String::new();

    let template_path = format!("data/templates/{}.html", card.get_field("template").unwrap());
    let template = std::fs::read_to_string(&template_path)
            .unwrap_or_else(|e| format!("Error loading '{template_path}' : {e}"));

    page_content.push_str("<script>\n");

    page_content.push_str(&card.to_javascript_object(Some("card")));

    page_content.push_str("</script>\n");

    page_content.push_str(&template);

    RawHtml(page_content)
}
