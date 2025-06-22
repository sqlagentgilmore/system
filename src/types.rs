#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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
