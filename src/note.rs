#![allow(dead_code)]

use std::collections::BTreeMap;
use std::cmp::Ord;

use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Note {
    #[serde(default = "Uuid::nil")]
    pub uuid: uuid::Uuid,

    #[serde(default = "one")]
    pub instances: u32,

    #[serde(default = "String::new")]
    pub template: String,

    #[serde(flatten)]
    pub fields: BTreeMap<String, String>,
}

fn one() -> u32 { 1 }

impl Note {

    pub fn get_uuid(&self) -> uuid::Uuid
    {
        self.uuid
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

    pub fn to_javascript_object(&self, variable_name: Option<&str>, instance: u32) -> String
    {
        let mut result = String::new();

        if let Some(varname) = variable_name {
            result.push_str(&format!("var {varname} = "));
        }

        result.push('{');
        result.push_str(&format!("\"instance\":{instance}"));
        for (k, v) in self.fields.iter() {
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


