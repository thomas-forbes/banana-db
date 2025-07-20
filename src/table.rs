use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub enum Data {
    Int(Option<i64>),
    Float(Option<f64>),
    String(Option<String>),
    Boolean(Option<bool>),
}

impl Data {
    fn same_type(&self, other: &Self) -> bool {
        match (self, other) {
            (Data::Int(_), Data::Int(_)) => true,
            (Data::Float(_), Data::Float(_)) => true,
            (Data::String(_), Data::String(_)) => true,
            (Data::Boolean(_), Data::Boolean(_)) => true,
            _ => false,
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

pub type Row = Vec<Cell>;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Table {
    name: String,
    columns: Vec<Column>,
    rows: Vec<Row>,
}

impl Table {
    pub fn new(name: String, columns: Vec<Column>) -> Self {
        Self {
            name,
            columns,
            rows: Vec::new(),
        }
    }

    pub fn get_columns(&self) -> &Vec<Column> {
        &self.columns
    }

    pub fn get_column_by_name(&self, column_name: &str) -> Option<&Column> {
        self.columns
            .iter()
            .find(|column| column.name == column_name)
    }

    pub fn insert(&mut self, row: Row) -> Result<(), Box<dyn std::error::Error>> {
        if row.len() != self.columns.len() {
            return Err("Row length does not match number of columns".into());
        }

        for (i, cell) in row.iter().enumerate() {
            if !cell.data.same_type(&self.columns[i].datatype) {
                return Err("Cell datatype does not match column datatype".into());
            }
        }

        self.rows.push(row);
        Ok(())
    }

    pub fn find(&self, column: &Column, value: &Data, limit: Option<usize>) -> Option<Vec<&Row>> {
        let limit = limit.unwrap_or(1);
        let column_index = self
            .columns
            .iter()
            .position(|c| c.name == column.name)
            .expect("Column not found");

        let mut results = Vec::new();
        for row in &self.rows {
            let cell = &row[column_index];
            if &cell.data == value {
                results.push(row);
            }
            if results.len() >= limit {
                break;
            }
        }

        return Some(results);
    }
}
