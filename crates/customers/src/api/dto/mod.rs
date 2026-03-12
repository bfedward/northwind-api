use serde::Serialize;

use crate::domain::Customer;
use platform::impl_dto_from;

#[derive(Debug, Serialize)]
pub struct CustomerResponseDto {
    pub id: i64,
    pub customer_code: String,
    pub company_name: String,
    pub contact_name: Option<String>,
    pub contact_title: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
}

impl_dto_from!(Customer => CustomerResponseDto {
    id,
    customer_code,
    company_name,
    contact_name,
    contact_title,
    city,
    country
});
