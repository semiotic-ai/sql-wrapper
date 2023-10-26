
use chrono::{NaiveDateTime, TimeZone};
use clickhouse_rs::types::{FromSql, FromSqlResult, ValueRef};
use serde::{Deserialize, Serialize};

use super::helpers::invalid_type_error;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CHDateTime(NaiveDateTime);

impl<'a> FromSql<'a> for CHDateTime {
    fn from_sql(value: ValueRef<'a>) -> FromSqlResult<Self> {
        match value {
            ValueRef::DateTime(v, tz) => {
                let time = tz.timestamp_opt(i64::from(v), 0).unwrap();
                Ok(Self(time.naive_utc()))
            }
            ValueRef::DateTime64(value, params) => {
                let (precision, tz) = *params;

                let base10: i64 = 10;
                let nano = if precision < 19 {
                    value * base10.pow(9 - precision)
                } else {
                    0_i64
                };
                let sec = nano / 1_000_000_000;
                let nsec = nano - sec * 1_000_000_000;
                let time = tz.timestamp_opt(sec, nsec as u32).unwrap();
                Ok(Self(time.naive_utc()))
            }
            _ => Err(invalid_type_error(value, "DateTime<Tz>")),
        }
    }
}
