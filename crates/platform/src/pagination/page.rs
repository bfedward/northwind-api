use serde::Serialize;

use crate::pagination::cursor::CursorItem;

#[derive(Debug, Serialize)]
pub struct CursorPage<T> {
    pub items: Vec<T>,
    pub next_cursor: Option<i64>,
}

impl<T> CursorPage<T> {
    pub fn new(items: Vec<T>, next_cursor: Option<i64>) -> Self {
        Self { items, next_cursor }
    }

    pub fn map<U>(self, f: impl Fn(T) -> U) -> CursorPage<U> {
        CursorPage {
            items: self.items.into_iter().map(f).collect(),
            next_cursor: self.next_cursor,
        }
    }

    pub fn build_cursor_page(mut rows: Vec<T>, page_size: u32) -> (Vec<T>, Option<i64>)
    where
        T: CursorItem,
    {
        let has_next = rows.len() as u32 > page_size;

        if has_next {
            rows.pop();
        }

        let next_cursor = if has_next {
            rows.last().map(|r| r.cursor())
        } else {
            None
        };

        (rows, next_cursor)
    }
}
