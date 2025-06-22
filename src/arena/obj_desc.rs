use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Description {
    name: String,
    display_name: Option<String>
}

impl Into<Description> for &Description {
    fn into(self) -> Description {
        self.clone()
    }
}

impl Into<Description> for &str {
    fn into(self) -> Description {
        Description {
            name: self.to_string(),
            display_name: None
        }
    }
}

impl Display for Description {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}