use platform::errors::app_error::AppError;
use platform::pagination::cursor::CursorPagination;
use platform::pagination::page::CursorPage;

use sqlx::FromRow;
use sqlx::PgPool;

use crate::domain::Customer;

#[derive(Debug, FromRow)]
pub struct CustomerDbRow {
    pub id: i64,
    pub customer_code: String,
    pub company_name: String,
    pub contact_name: Option<String>,
    pub contact_title: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
}

impl From<CustomerDbRow> for Customer {
    fn from(row: CustomerDbRow) -> Self {
        Self {
            id: row.id,
            customer_code: row.customer_code,
            company_name: row.company_name,
            contact_name: row.contact_name,
            contact_title: row.contact_title,
            city: row.city,
            country: row.country,
        }
    }
}

pub async fn list_customers(
    pool: &PgPool,
    pagination: CursorPagination,
) -> Result<CursorPage<Customer>, AppError> {
    let page_size = pagination.page_size();
    let limit = page_size + 1;

    let rows = if let Some(cursor) = pagination.cursor {
        sqlx::query_as!(
            CustomerDbRow,
            r#"
            SELECT id, customer_code, company_name, contact_name, contact_title, city, country
            FROM customers
            WHERE id > $1
            ORDER BY id
            LIMIT $2
            "#,
            cursor,
            limit as i64
        )
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::DatabaseQueryError(e))?
    } else {
        sqlx::query_as!(
            CustomerDbRow,
            r#"
            SELECT id, customer_code, company_name, contact_name, contact_title, city, country
            FROM customers
            ORDER BY id
            LIMIT $1
            "#,
            limit as i64
        )
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::DatabaseQueryError(e))?
    };

    let rows: Vec<Customer> = rows.into_iter().map(Customer::from).collect();

    let (items, next_cursor) = CursorPage::build_cursor_page(rows, page_size);

    Ok(CursorPage::new(items, next_cursor))
}
