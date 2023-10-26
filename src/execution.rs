mod datetime;
pub mod filter_sql;
mod generic_result;
mod helpers;
mod integers;
mod value;

pub use generic_result::GenericResult;
pub use value::Value;
pub use value::convert_col_to_value;
use clickhouse_rs::Pool;


#[derive(Clone)]
pub struct DatabaseExecutor {
    pool: Pool,
}

const DATABASE_HARD_LIMIT: usize = 10000;

impl DatabaseExecutor {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
    pub async fn execute_query(&self, sql: String) -> anyhow::Result<GenericResult> {
        let mut client = self.pool.get_handle().await?;
        let sql = filter_sql::filter_sql(&sql)?;
        let sql = sql.trim_end_matches(';');
        let sql = format!("SELECT * FROM ({sql}) LIMIT {DATABASE_HARD_LIMIT}");
        let block = client.query(sql).fetch_all().await?;
        let result = block.try_into()?;
        Ok(result)
    }
}
