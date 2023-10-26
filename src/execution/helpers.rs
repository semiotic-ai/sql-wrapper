
use clickhouse_rs::{
    errors::{Error, FromSqlError},
    types::{SqlType, ValueRef},
};

/// Helper function to generate an error for invalid types.
pub fn invalid_type_error(value: ValueRef, expected: &'static str) -> Error {
    let from = SqlType::from(value).to_string();
    Error::FromSql(FromSqlError::InvalidType {
        src: from,
        dst: expected.into(),
    })
}
