use std::{borrow::Cow, cmp, collections::HashMap, fmt::Display};

use serde::{Deserialize, Serialize};
use tabled::Tabled;

use crate::bql::{ast::Where, token::TokenType};

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, PartialOrd)]
pub enum Data {
    Int(Option<i64>),
    Float(Option<f64>),
    String(Option<String>),
    Boolean(Option<bool>),
}

impl Data {
    pub fn from_token_type(token_type: &TokenType) -> Option<Self> {
        match token_type {
            TokenType::BooleanWord => Some(Data::Boolean(None)),
            TokenType::IntWord => Some(Data::Int(None)),
            TokenType::FloatWord => Some(Data::Float(None)),
            TokenType::StringWord => Some(Data::String(None)),
            _ => None,
        }
    }
    pub fn same_type(&self, other: &Self) -> bool {
        match (self, other) {
            (Data::Int(_), Data::Int(_)) => true,
            (Data::Float(_), Data::Float(_)) => true,
            (Data::String(_), Data::String(_)) => true,
            (Data::Boolean(_), Data::Boolean(_)) => true,
            _ => false,
        }
    }
    pub fn operator_compare(&self, other_data: &Data, comparison_operator: &TokenType) -> bool {
        match comparison_operator {
            TokenType::Equals => self == other_data,
            TokenType::NotEquals => self != other_data,
            TokenType::Less => self < other_data,
            TokenType::LessEquals => self <= other_data,
            TokenType::Greater => self > other_data,
            TokenType::GreaterEquals => self >= other_data,
            _ => panic!(
                "Invalid token `{:?}` passed to `operator_compare`",
                comparison_operator
            ),
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

impl Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
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
        write!(f, "`{}`: {}", self.name, self.datatype)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Row {
    pub values: HashMap<String, Cell>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Table {
    name: String,
    columns: Vec<Column>,
    rows: Vec<Row>,
}

impl Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", tabled::Table::new(vec![self]))
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

    pub fn get_columns(&self) -> &Vec<Column> {
        &self.columns
    }

    pub fn get_column_by_name(&self, column_name: &str) -> Option<&Column> {
        self.columns
            .iter()
            .find(|column| column.name == column_name)
    }

    pub fn insert(&mut self, row: Row) -> Result<(), String> {
        if row.values.len() != self.columns.len() {
            return Err("Row length does not match number of columns".into());
        }

        // for (i, cell) in row.values.iter().enumerate() {
        //     if !cell.data.same_type(&self.columns[i].datatype) {
        //         return Err("Cell datatype does not match column datatype".into());
        //     }
        // }
        for (key, cell) in row.values.iter() {
            let column = self
                .columns
                .iter()
                .find(|&c| c.name == *key)
                .ok_or(format!("Invalid field `{}`", key).to_owned())?;

            if !column.datatype.same_type(&cell.data) {
                return Err(format!(
                    "Cell datatype `{}` does not match column datatype `{}`",
                    cell.data, column.datatype
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
    ) -> Result<Vec<&Row>, String> {
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
                    .ok_or("Invalid field name".to_owned())?;

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

        return Ok(results);
    }
}
