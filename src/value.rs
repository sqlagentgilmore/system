use crate::node::Node;
use compact_str::{CompactString, ToCompactString};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Copy, Clone, Debug, Serialize, Deserialize, Hash, Eq)]
pub enum ValueKind {
    System,
    Server,
    Project,
    Dataset,
    Database,
    Schema,
    Table,
    Column,
    DataType,
}

impl From<&str> for ValueKind {
    fn from(value: &str) -> Self {
        match value.trim().to_lowercase().as_str() {
            "system" => Self::System,
            "server" => Self::Server,
            "project" => Self::Project,
            "dataset" => Self::Dataset,
            "database" => Self::Database,
            "schema" => Self::Schema,
            "table" => Self::Table,
            "column" => Self::Column,
            "datatype" => Self::DataType,
            _ => panic!("Unknown ValueKind: {}", value),
        }
    }
}

impl From<ValueKind> for &str {
    fn from(kind: ValueKind) -> Self {
        match kind {
            ValueKind::System => "system",
            ValueKind::Server => "server",
            ValueKind::Project => "project",
            ValueKind::Dataset => "dataset",
            ValueKind::Database => "database",
            ValueKind::Schema => "schema",
            ValueKind::Table => "table",
            ValueKind::Column => "column",
            ValueKind::DataType => "datatype",
        }
    }
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize, Hash, Eq)]
pub struct Value {
    name: compact_str::CompactString,
    kind: ValueKind,
}

impl Value {
    pub fn new(name: impl ToCompactString, kind: ValueKind) -> Self {
        Self {
            name: name.to_compact_string(),
            kind,
        }
    }

    pub fn get_name_ref(&self) -> &str {
        &self.name
    }

    pub fn get_name(&self) -> String {
        self.name.to_string()
    }
    
    pub fn get_kind(&self) -> &ValueKind {
        &self.kind
    }
    
    pub fn set_kind(&mut self, kind: ValueKind) {
        self.kind = kind;
    }

    pub fn from_big_query_value<'a>(bq_value: impl AsRef<Option<&'a str>>, kind: ValueKind) -> Option<Self> {
        bq_value.as_ref().as_ref().map(|val| Self::new(*val, kind))
    }
}

impl From<(Box<dyn ToCompactString>, &str)> for Value {
    fn from(value: (Box<dyn ToCompactString>, &str)) -> Self {
        Value {
            name: value.0.to_compact_string(),
            kind: value.1.into(), // Default kind, can be changed as needed
        }
    }
}

impl From<Node> for Value {
    fn from(value: Node) -> Self {
        value.value()
    }
}

impl From<(&str, &str)> for Value {
    fn from(value: (&str, &str)) -> Self {
        Self {name: value.0.to_compact_string(), kind: value.1.into() }
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
