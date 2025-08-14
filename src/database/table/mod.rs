use std::{borrow::Cow, collections::BTreeMap, fmt::Display};

use serde::{Deserialize, Serialize};
use tabled::Tabled;

use crate::{
    bql::ast::Where,
    database::{
        data::{Comparison, Data},
        table::axes::{Column, Row, Rows},
    },
    utils,
};

#[cfg(test)]
mod tests;

pub mod axes;

#[derive(Debug, Clone)]
pub enum TableError {
    RowColumnCountMismatch,
    FieldDoesNotExist(String),
    TypeMismatch(String, String),
    PrimaryKeyViolation(String),
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
            TableError::PrimaryKeyViolation(message) => {
                write!(f, "Primary key violation: `{}`", message)
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Table {
    name: String,
    primary_key: String,
    columns: Vec<Column>,
    rows: BTreeMap<Data, Row>,
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
    pub fn new(name: String, primary_key: String, columns: Vec<Column>) -> Self {
        Self {
            name,
            primary_key,
            columns,
            rows: BTreeMap::new(),
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn insert(&mut self, row: Row) -> Result<(), TableError> {
        if row.values.len() != self.columns.len() {
            return Err(TableError::RowColumnCountMismatch);
        }

        let mut primary_key_value = None;
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

            if column.name == self.primary_key {
                primary_key_value = Some(cell.data.clone());
            }
        }

        let primary_key_value = primary_key_value.ok_or(TableError::PrimaryKeyViolation(
            "Primary key not found".to_string(),
        ))?;
        if self.rows.contains_key(&primary_key_value) {
            return Err(TableError::PrimaryKeyViolation(
                "Primary key already exists".to_string(),
            ));
        }
        self.rows.insert(primary_key_value, row);
        Ok(())
    }

    pub fn find(
        &self,
        where_statement: &Option<Where>,
        limit: Option<usize>,
    ) -> Result<Rows, TableError> {
        let limit = limit.unwrap_or(1);

        let mut results = Vec::new();
        match where_statement {
            None => {
                for (_, row) in self.rows.iter().take(limit) {
                    results.push(row);
                }
            }
            Some(ws) => {
                let field = &ws.field.value;
                if field == &self.primary_key {
                    use std::ops::Bound::{Excluded, Included, Unbounded};
                    match ws.comparison {
                        Comparison::Equals => {
                            if let Some(row) = self.rows.get(&ws.value) {
                                results.push(row);
                            }
                        }
                        Comparison::Less => {
                            for (_, row) in self.rows.range(..&ws.value).take(limit) {
                                results.push(row);
                            }
                        }
                        Comparison::LessEquals => {
                            for (_, row) in self.rows.range(..=&ws.value).take(limit) {
                                results.push(row);
                            }
                        }
                        Comparison::Greater => {
                            for (_, row) in self
                                .rows
                                .range((Excluded(&ws.value), Unbounded))
                                .take(limit)
                            {
                                results.push(row);
                            }
                        }
                        Comparison::GreaterEquals => {
                            for (_, row) in self
                                .rows
                                .range((Included(&ws.value), Unbounded))
                                .take(limit)
                            {
                                results.push(row);
                            }
                        }
                        Comparison::NotEquals => {
                            for (_, row) in self.rows.range(..&ws.value).take(limit) {
                                results.push(row);
                            }
                            if results.len() < limit {
                                for (_, row) in self.rows.range((Excluded(&ws.value), Unbounded)) {
                                    results.push(row);
                                    if results.len() >= limit {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    for (_, row) in self.rows.iter() {
                        if results.len() >= limit {
                            break;
                        }
                        let row_value = row
                            .values
                            .get(field)
                            .ok_or(TableError::FieldDoesNotExist(field.clone()))?;
                        if ws.comparison.apply(&row_value.data, &ws.value) {
                            results.push(row);
                        }
                    }
                }
            }
        }

        Ok(Rows(results))
    }
}
