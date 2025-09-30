use super::Response;

pub struct End {}

impl End {
    pub fn new() -> Self {
        Self {}
    }
}

impl Response for End {
    fn to_string(&self) -> String {
        "END\r\n".to_string()
    }
}
