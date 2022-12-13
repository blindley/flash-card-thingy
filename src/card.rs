#![allow(dead_code)]

use std::collections::BTreeMap;
use std::cmp::Ord;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Card(BTreeMap<String, String>);

impl Card {
    pub fn new() -> Card
    {
        let fields = BTreeMap::new();

        Card(fields)
    }

    pub fn get_uuid(&self) -> uuid::Uuid
    {
        use std::str::FromStr;
        uuid::Uuid::from_str(self.0.get("uuid").unwrap().as_str()).unwrap()
    }

    pub fn set_field<K, V>(&mut self, key: K, value: V)
        where K: Into<String>, V: Into<String>
    {
        self.0.insert(key.into(), value.into());
    }

    pub fn get_field<Q>(&self, key: &Q) -> Option<&String>
        where String: std::borrow::Borrow<Q>,
            Q: Ord + ?Sized
    {
        self.0.get(key)
    }

    pub fn to_javascript_object(&self, variable_name: Option<&str>, instance: u32) -> String
    {
        let mut result = String::new();

        if let Some(varname) = variable_name {
            result.push_str(&format!("var {varname} = "));
        }

        result.push('{');
        result.push_str(&format!("\"instance\":{instance}"));
        for (k, v) in self.0.iter() {
            result.push(',');
            result.push_str(&format!("\"{k}\":\"{v}\""));
        }
        result.push('}');

        if let Some(_) = variable_name {
            result.push_str(";\n");
        }

        result
    }

    pub fn to_html(&self, instance: u32) -> String
    {
        let mut page_content = String::new();

        page_content.push_str("<script>\n");
        page_content.push_str(&self.to_javascript_object(Some("card"), instance));
        page_content.push_str("</script>\n");

        let template = self.get_field("template")
            .and_then(|v| Some(v.as_str()))
            .unwrap_or("basic");
        let template_path = format!("data/templates/{}.html", template);
        let template = std::fs::read_to_string(&template_path)
                .unwrap_or_else(|e| format!("Error loading '{template_path}' : {e}"));

        page_content.push_str(&template);

        page_content
    }
}

#[cfg(test)]
mod test {
    use super::Card;

    #[test]
    fn test_to_javascript_object()
    {
        let card = Card::new();
        assert_eq!(card.to_javascript_object(None, 1), r#"{"instance":1}"#);

        let mut card = Card::new();
        card.set_field("template", "basic-plus");
        card.set_field("front", "This is the front.");
        card.set_field("back", "This is the back.");
        assert_eq!(
            card.to_javascript_object(Some("card"), 1),
            concat!(
                r#"var card = {"instance":1,"back":"This is the back.","front":"This is the front.","template":"basic-plus"};"#,
                "\n"
            )
        );
    }
}
