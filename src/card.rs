use std::collections::BTreeMap;
use std::cmp::Ord;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Card {
    fields: BTreeMap<String, String>,
}

impl Card {
    pub fn new() -> Card
    {
        let mut fields = BTreeMap::new();

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
            Q: Ord + ?Sized
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

    pub fn to_html(&self) -> String
    {
        let mut page_content = String::new();

        page_content.push_str("<script>\n");
        page_content.push_str(&self.to_javascript_object(Some("card")));
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
        assert_eq!(card.to_javascript_object(None), "{}");

        let mut card = Card::new();
        card.set_field("template", "basic-plus");
        card.set_field("front", "This is the front.");
        card.set_field("back", "This is the back.");
        assert_eq!(
            card.to_javascript_object(Some("card")),
            concat!(
                r#"var card = {"back":"This is the back.","front":"This is the front.","template":"basic-plus"};"#,
                "\n"
            )
        );
    }
}
