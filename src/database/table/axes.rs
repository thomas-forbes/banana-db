use std::{collections::HashMap, fmt::Display};

use colored::Colorize;
use serde::{Deserialize, Serialize};

use crate::{database::data::Data, utils};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Column {
    pub(super) name: String,
    pub(super) datatype: Data,
    pub(super) index: Option<Vec<usize>>,
    pub(super) primary: bool,
}

impl Column {
    pub fn new(name: String, datatype: Data, primary: bool, index: bool) -> Self {
        Self {
            name,
            datatype,
            primary,
            index: if index { Some(Vec::new()) } else { None },
        }
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

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct Cell {
    pub(super) data: Data,
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
