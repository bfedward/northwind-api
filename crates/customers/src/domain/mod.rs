use serde::{Deserialize, Serialize};

use platform::pagination::cursor::CursorItem;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    pub id: i64,
    pub customer_code: String,
    pub company_name: String,
    pub contact_name: Option<String>,
    pub contact_title: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
}

impl CursorItem for Customer {
    fn cursor(&self) -> i64 {
        self.id
    }
}
