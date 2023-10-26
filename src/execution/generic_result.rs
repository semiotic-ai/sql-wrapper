use super::Value;
use clickhouse_rs::{types::Complex, Block};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Clone, Serialize)]
pub struct GenericResult {
    pub data: Vec<HashMap<String, Value>>,
}

impl TryFrom<Block<Complex>> for GenericResult {
    type Error = clickhouse_rs::errors::Error;

    fn try_from(block: Block<Complex>) -> std::result::Result<Self, Self::Error> {
        let mut data = Vec::with_capacity(block.row_count());
        for row in block.rows() {
            let mut map = HashMap::with_capacity(row.len());
            for i in 0..row.len() {
                let col_name = row.name(i)?;
                let value = super::convert_col_to_value(&row, i)?;
                map.insert(col_name.to_string(), value);
            }
            data.push(map);
        }
        Ok(Self { data })
    }
}

impl std::fmt::Debug for GenericResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GenericResult")
            .field("length", &self.data.len())
            .field("first_row", &self.data.first())
            .finish()
    }
}
