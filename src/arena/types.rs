use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum SystemType {
    BigQuery,
    SqlServer,
}

impl std::fmt::Display for SystemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SystemType::BigQuery => write!(f, "BigQuery"),
            SystemType::SqlServer => write!(f, "SqlServer"),
        }
    }
}

impl From<&str> for SystemType {
    fn from(value: &str) -> Self {
        match value.trim().to_lowercase().replace(" ", "_").as_str() {
            "sql_server" => {Self::SqlServer}
            "big_query" => {Self::BigQuery}
            val => {
                unimplemented!("{} not yet implemented",val);
            }
        }
    }
}