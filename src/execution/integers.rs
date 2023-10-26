
use super::helpers::invalid_type_error;
use clickhouse_rs::types::{i256, u256, FromSql, FromSqlResult, ValueRef};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct U256(#[serde(with = "ethnum::serde::decimal")] u256);


impl<'a> FromSql<'a> for U256 {
    fn from_sql(value: ValueRef<'a>) -> FromSqlResult<Self> {
        match value {
            ValueRef::UInt256(u) => Ok(Self(u)),

            _ => Err(invalid_type_error(value, "U256")),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct I256(#[serde(with = "ethnum::serde::decimal")] i256);

impl<'a> FromSql<'a> for I256 {
    fn from_sql(value: ValueRef<'a>) -> FromSqlResult<Self> {
        match value {
            ValueRef::Int256(u) => Ok(Self(u)),
            _ => Err(invalid_type_error(value, "I256")),
        }
    }
}
