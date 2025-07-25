use std::{borrow::Cow, collections::HashMap, fmt::Display};

use colored::Colorize;
use serde::{Deserialize, Serialize};
use tabled::Tabled;

use crate::{
    bql::{ast::Where, token::TokenType},
    utils,
};

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

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, PartialOrd)]
pub enum Data {
    Int(Option<i64>),
    Float(Option<f64>),
    String(Option<String>),
    Boolean(Option<bool>),
}

impl Data {
    pub fn same_type(&self, other: &Self) -> bool {
        match (self, other) {
            (Data::Int(_), Data::Int(_)) => true,
            (Data::Float(_), Data::Float(_)) => true,
            (Data::String(_), Data::String(_)) => true,
            (Data::Boolean(_), Data::Boolean(_)) => true,
            _ => false,
        }
    }
    fn fmt_data_type(&self) -> String {
        match self {
            Data::Int(_) => "Int".blue().to_string(),
            Data::Float(_) => "Float".cyan().to_string(),
            Data::String(_) => "String".green().to_string(),
            Data::Boolean(_) => "Boolean".purple().to_string(),
        }
    }
    fn fmt_data_value(&self) -> Option<String> {
        match self {
            Data::Int(Some(i)) => Some(i.to_string()),
            Data::Float(Some(f)) => Some(f.to_string()),
            Data::String(Some(s)) => Some(s.clone()),
            Data::Boolean(Some(b)) => Some(b.to_string()),
            _ => None,
        }
    }
}

impl Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data_type = self.fmt_data_type();
        if let Some(value) = self.fmt_data_value() {
            write!(f, "{}({})", data_type, value.dimmed())
        } else {
            write!(f, "{}", data_type)
        }
    }
}

#[derive(Debug, Clone)]
pub enum Comparison {
    Equals,
    NotEquals,
    Less,
    LessEquals,
    Greater,
    GreaterEquals,
}

impl Comparison {
    pub fn apply<T: PartialOrd + PartialEq>(&self, a: &T, b: &T) -> bool {
        match self {
            Comparison::Less => a < b,
            Comparison::LessEquals => a <= b,
            Comparison::Equals => a == b,
            Comparison::Greater => a > b,
            Comparison::GreaterEquals => a >= b,
            Comparison::NotEquals => a != b,
        }
    }
    pub fn from_token_type(token_type: &TokenType) -> Option<Self> {
        match token_type {
            TokenType::Equals => Some(Comparison::Equals),
            TokenType::NotEquals => Some(Comparison::NotEquals),
            TokenType::Less => Some(Comparison::Less),
            TokenType::LessEquals => Some(Comparison::LessEquals),
            TokenType::Greater => Some(Comparison::Greater),
            TokenType::GreaterEquals => Some(Comparison::GreaterEquals),
            _ => None,
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
    pub fn data(&self) -> &Data {
        &self.data
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
    pub fn name(&self) -> &String {
        &self.name
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
