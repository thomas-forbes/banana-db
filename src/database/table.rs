use std::{borrow::Cow, collections::HashMap, fmt::Display};

use colored::Colorize;
use serde::{Deserialize, Serialize};
use tabled::Tabled;

use crate::{bql::ast::Where, database::data::Data, utils};

#[derive(Debug, Clone)]
pub enum TableError {
    RowColumnCountMismatch,
    FieldDoesNotExist(String),
    TypeMismatch(String, String),
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
        }
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct Cell {
    data: Data,
}

impl Cell {
    pub fn new(data: Data) -> Self {
        Self { data }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Column {
    name: String,
    datatype: Data,
}

impl Column {
    pub fn new(name: String, datatype: Data) -> Self {
        Self { name, datatype }
    }
}

impl Display for Column {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.datatype, self.name.dimmed(),)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Row {
    pub values: HashMap<String, Cell>,
}

pub struct Rows<'a>(pub Vec<&'a Row>);

impl<'a> Display for Rows<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut builder = tabled::builder::Builder::default();

        let mut columns = std::collections::BTreeSet::new();
        for row in &self.0 {
            for key in row.values.keys() {
                columns.insert(key.clone());
            }
        }
        let columns: Vec<String> = columns.into_iter().collect();
        builder.push_record(columns.iter().cloned());

        for row in &self.0 {
            let mut record = Vec::new();
            for column in &columns {
                if let Some(cell) = row.values.get(column) {
                    record.push(cell.to_string());
                } else {
                    record.push(String::new());
                }
            }
            builder.push_record(record);
        }

        let mut table = builder.build();
        write!(f, "{}", utils::format_table(&mut table))
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
