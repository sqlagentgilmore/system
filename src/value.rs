use crate::node::Node;
use compact_str::{CompactString, ToCompactString};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Value {
    name: compact_str::CompactString,
}

impl Value {
    pub fn new(name: impl ToCompactString) -> Self {
        Self {
            name: name.to_compact_string(),
        }
    }

    pub fn get_ref(&self) -> &str {
        &self.name
    }

    pub fn get(&self) -> &CompactString {
        &self.name
    }

    pub fn from_big_query_value<'a>(bq_value: impl AsRef<Option<&'a str>>) -> Option<Self> {
        bq_value.as_ref().as_ref().map(|val| Self::new(*val))
    }
}

impl From<Box<dyn ToCompactString>> for Value {
    fn from(value: Box<dyn ToCompactString>) -> Self {
        Value {
            name: value.to_compact_string(),
        }
    }
}

impl From<Node> for Value {
    fn from(value: Node) -> Self {
        value.value()
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Self {name: value.to_compact_string() }
    }
}

impl From<&Node> for Value {
    fn from(value: &Node) -> Self {
        value.value()
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
