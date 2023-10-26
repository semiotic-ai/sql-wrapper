use std::ops::ControlFlow;

use sqlparser::ast::{Statement, Visit, Visitor};
use sqlparser::dialect::ClickHouseDialect;
use sqlparser::parser::{Parser, ParserError};

/// Apply pre defined filters to SQL
pub fn filter_sql(sql: &str) -> Result<String, ParserError> {
    let dialect = ClickHouseDialect {};
    let ast = Parser::parse_sql(&dialect, sql)?;
    let mut visitor = OnlySelect;
    let a = ast.visit(&mut visitor);
    if a.is_break() {
        return Err(ParserError::ParserError("Only queries allowed".to_string()));
    }
    Ok(ast
        .into_iter()
        .map(|stmt| stmt.to_string())
        .collect::<Vec<_>>()
        .join(" "))
}

struct OnlySelect;

impl Visitor for OnlySelect {
    type Break = ();
    fn pre_visit_statement(&mut self, stmt: &Statement) -> ControlFlow<Self::Break> {
        if let Statement::Query(_) = stmt {
            ControlFlow::Continue(())
        } else {
            ControlFlow::Break(())
        }
    }
}

#[cfg(test)]
mod tests {
    use sqlparser::parser::ParserError;

    use super::filter_sql;

    #[test]
    fn should_allow_selects() {
        let sql = "SELECT * FROM test";
        let err = filter_sql(sql).err();
        assert_eq!(err, None);
    }

    #[test]
    fn should_allow_unions() {
        let sql = "SELECT * FROM test UNION SELECT * FROM test2";
        let err = filter_sql(sql).err();
        assert_eq!(err, None);
    }

    #[test]
    fn should_disallow_inserts() {
        let sql = "INSERT INTO test VALUES (1, 2)";
        let err = filter_sql(sql).err();
        assert_eq!(
            err,
            Some(ParserError::ParserError("Only queries allowed".to_string()))
        );
    }

    #[test]
    fn should_disallow_deletes() {
        let sql = "DELETE FROM test";
        let err = filter_sql(sql).err();
        assert_eq!(
            err,
            Some(ParserError::ParserError("Only queries allowed".to_string()))
        );
    }

    #[test]
    fn should_disallow_drops() {
        let sql = "DROP TABLE test";
        let err = filter_sql(sql).err();
        assert_eq!(
            err,
            Some(ParserError::ParserError("Only queries allowed".to_string()))
        );
    }

    #[test]
    fn should_disallow_alters() {
        let sql = "ALTER TABLE test ADD COLUMN test TEXT;";
        let err = filter_sql(sql).err();
        assert_eq!(
            err,
            Some(ParserError::ParserError("Only queries allowed".to_string()))
        );
    }
}
