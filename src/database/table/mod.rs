use std::{borrow::Cow, fmt::Display};

use serde::{Deserialize, Serialize};
use tabled::Tabled;

use crate::{
    bql::ast::{Identifier, Where},
    database::{
        data::Comparison,
        table::axes::{Column, Row, Rows},
    },
    utils,
};

pub mod axes;

#[derive(Debug, Clone)]
pub enum TableError {
    RowColumnCountMismatch,
    FieldDoesNotExist(String),
    TypeMismatch(String, String),
    PrimaryKeyViolation,
}

impl Display for TableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TableError::RowColumnCountMismatch => {
                write!(f, "Row length does not match number of columns")
            }
            TableError::FieldDoesNotExist(field) => write!(f, "Field `{}` does not exist", field),
            TableError::TypeMismatch(cell_type, column_type) => write!(
                f,
                "Cell datatype `{}` does not match column datatype `{}`",
                cell_type, column_type
            ),
            TableError::PrimaryKeyViolation => write!(f, "Primary key violation"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Table {
    name: String,
    columns: Vec<Column>,
    rows: Vec<Row>,
}

impl Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            utils::format_table(&mut tabled::Table::new(vec![self]))
        )
    }
}

impl Tabled for Table {
    const LENGTH: usize = 3;

    fn fields(&self) -> Vec<Cow<'_, str>> {
        vec![
            Cow::Borrowed(&self.name),
            Cow::Owned(
                self.columns
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
            ),
            Cow::Owned(format!("{}", self.rows.len())),
        ]
    }

    fn headers() -> Vec<Cow<'static, str>> {
        vec![
            Cow::Borrowed("name"),
            Cow::Borrowed("columns"),
            Cow::Borrowed("# rows"),
        ]
    }
}

impl Table {
    pub fn new(name: String, columns: Vec<Column>) -> Self {
        Self {
            name,
            columns,
            rows: Vec::new(),
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn insert(&mut self, row: Row) -> Result<(), TableError> {
        if row.values.len() != self.columns.len() {
            return Err(TableError::RowColumnCountMismatch);
        }

        for (key, cell) in row.values.iter() {
            let column = self
                .columns
                .iter()
                .find(|&c| c.name == *key)
                .ok_or(TableError::FieldDoesNotExist(key.clone()))?;

            if !column.datatype.same_type(&cell.data) {
                return Err(TableError::TypeMismatch(
                    cell.data.to_string(),
                    column.datatype.to_string(),
                ));
            }

            if column.primary
                && let Ok(rows) = self.find(
                    &Some(Where {
                        field: Identifier { value: key.clone() },
                        value: cell.data.clone(),
                        comparison: Comparison::Equals,
                    }),
                    None,
                )
                && rows.0.len() > 0
            {
                return Err(TableError::PrimaryKeyViolation);
            }
        }

        self.rows.push(row);
        Ok(())
    }

    pub fn find(
        &self,
        where_statement: &Option<Where>,
        limit: Option<usize>,
    ) -> Result<Rows, TableError> {
        let limit = limit.unwrap_or(1);

        let mut results = Vec::new();
        for row in &self.rows {
            if results.len() >= limit {
                break;
            }
            if let Some(where_statement) = where_statement {
                let field = &where_statement.field.value;
                let row_value = row
                    .values
                    .get(field)
                    .ok_or(TableError::FieldDoesNotExist(field.clone()))?;

                if where_statement
                    .comparison
                    .apply(&row_value.data, &where_statement.value)
                {
                    results.push(row);
                }
            } else {
                results.push(row);
            }
        }

        return Ok(Rows(results));
    }
}
