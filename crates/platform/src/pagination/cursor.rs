use serde::Deserialize;
use validator::Validate;

pub trait CursorItem {
    fn cursor(&self) -> i64;
}

#[derive(Debug, Deserialize, Validate)]
pub struct CursorPagination {
    #[validate(range(min = 1, max = 1000))]
    pub page_size: Option<u32>,
    pub cursor: Option<i64>,
}

impl CursorPagination {
    pub fn page_size(&self) -> u32 {
        self.page_size.unwrap_or(50)
    }
}
