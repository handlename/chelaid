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

impl Response for Value {}

impl From<Value> for String {
    fn from(value: Value) -> Self {
        format!("VALUE {} 0 {}", value.key, u64::from(value.id))
    }
}
