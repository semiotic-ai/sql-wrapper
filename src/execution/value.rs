use chrono::NaiveDate;
use clickhouse_rs::{
    errors::FromSqlError,
    types::{Complex, Row, SqlType},
};
use serde::{Deserialize, Serialize};
use std::net::{Ipv4Addr, Ipv6Addr};
use uuid::Uuid;

use super::{
    datetime::CHDateTime,
    integers::{I256, U256},
};

macro_rules! convert_values {
    ( $($sql_type:ident$(($($lt:tt $(,)?)+))? => $($col_type:ident$(<$extra:tt>)?)?,)* ) => {

        #[derive(Debug, Serialize, Deserialize, Clone)]
        #[serde(untagged)]
        pub enum Value {
            $( $($sql_type($col_type$(<$extra>)?),)?)*
        }

        pub fn convert_col_to_value(
            row: &Row<'_, Complex>,
            index: usize,
        ) -> Result<Value, clickhouse_rs::errors::Error> {
            let sql_type = row.sql_type(index)?;
            match sql_type {
                $(SqlType::$sql_type$(($($lt, )*))? => {
                    return convert_values!(@internal row $sql_type index $($col_type$(<$extra>)?)?);
                },)*
            }
        }
    };
    (@internal $row:ident $sql_type:ident $index:ident) => {
        Err(clickhouse_rs::errors::Error::FromSql(FromSqlError::UnsupportedOperation))
    };
    (@internal $row:ident $sql_type:ident $index:ident $col_type:ident$(<$extra:tt>)?) => {
        match $row.get::<$col_type$(<$extra>)?, _>($index) {
            Ok(value) => Ok(Value::$sql_type(value)),
            Err(e) => Err(e),
        }
    };
}

convert_values! {
    String => String,
    Bool => bool,
    FixedString(_) => String,
    DateTime(_) => CHDateTime,
    Date => NaiveDate,
    UInt8 => u8,
    UInt16 => u16,
    UInt32 => u32,
    UInt64 => u64,
    UInt128 => u128,
    UInt256 => U256,
    Int8 => i8,
    Int16 => i16,
    Int32 => i32,
    Int64 => i64,
    Int128 => i128,
    Int256 => I256,
    Float32 => f32,
    Float64 => f64,
    Ipv4 => Ipv4Addr,
    Ipv6 => Ipv6Addr,
    Nullable(_) => ,
    Enum8(_) => ,
    Enum16(_) => ,
    Uuid => Uuid,
    Array(_) => ,
    Decimal(..) => ,
    SimpleAggregateFunction(..) => ,
    Map(..) => ,
}
