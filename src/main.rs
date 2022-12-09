#[macro_use] extern crate rocket;

use rocket::response::content::RawHtml;

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

struct Card {
    fields: std::collections::HashMap<String, String>,
}

impl Card {
    pub fn new<S>(template: S) -> Card
        where S: Into<String>
    {
        let mut fields = std::collections::HashMap::new();
        fields.insert("template".into(), template.into());

        Card {
            fields,
        }
    }

    pub fn set_field<K, V>(&mut self, key: K, value: V)
        where K: Into<String>, V: Into<String>
    {
        self.fields.insert(key.into(), value.into());
    }

    pub fn get_field<Q>(&self, key: &Q) -> Option<&String>
        where String: std::borrow::Borrow<Q>,
            Q: std::hash::Hash + Eq + ?Sized
    {
        self.fields.get(key)
    }

    pub fn to_javascript_object(&self, variable_name: Option<&str>) -> String
    {
        let mut result = String::new();

        if let Some(varname) = variable_name {
            result.push_str(&format!("var {varname} = "));
        }

        result.push('{');
        let mut is_first = true;
        for (k, v) in self.fields.iter() {
            if !is_first {
                result.push(',');
            }

            result.push_str(&format!("\"{k}\":\"{v}\""));
            is_first = false;
        }
        result.push('}');

        if let Some(_) = variable_name {
            result.push_str(";\n");
        }

        result
    }
}

