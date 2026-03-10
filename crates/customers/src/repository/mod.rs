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

pub async fn get_all_customers(pool: &PgPool) -> Result<Vec<Customer>, sqlx::Error> {
    let rows = sqlx::query_as!(
        CustomerDbRow,
        r#"
        SELECT
            id,
            customer_code,
            company_name,
            contact_name,
            contact_title,
            city,
            country
        FROM customers
        ORDER BY id
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(Customer::from).collect())
}
