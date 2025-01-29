use crate::domain;

use super::Response;

pub struct Value {
    key: String,
    id: domain::value_object::Id,
}

impl Value {
    pub fn new(key: &str, id: domain::value_object::Id) -> Self {
        Self {
            key: key.to_string(),
            id,
        }
    }
}

impl Response for Value {
    fn to_string(&self) -> String {
        format!("VALUE {} 0 {}\r\n", self.key, u64::from(self.id.clone()))
    }
}
