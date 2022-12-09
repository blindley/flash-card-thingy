#[macro_use] extern crate rocket;

use rocket::response::content::RawHtml;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

#[get("/")]
fn index() -> RawHtml<String>
{
    let mut note = Note::new("basic");
    note.set_field("front", "Front of Card");
    note.set_field("back", "Back of Card");

    let mut page = String::new();

    let p = format!("data/templates/{}.html", note.get_field("template").unwrap());
    let template = std::fs::read_to_string(&p)
            .unwrap_or_else(|e| format!("Error loading '{p}' : {e}"));

    page.push_str("<script>\ncard = {};\n");
    for (k, v) in note.fields.iter() {
        let line = format!("card[\"{k}\"] = \"{v}\"\n");
        page.push_str(&line);
    }
    page.push_str("</script>\n");

    page.push_str(&template);

    RawHtml(page)
}

struct Note {
    fields: std::collections::HashMap<String, String>,
}

impl Note {
    pub fn new<S>(template: S) -> Note
        where S: Into<String>
    {
        let mut fields = std::collections::HashMap::new();
        fields.insert("template".into(), template.into());

        Note {
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
}
