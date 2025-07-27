use std::collections::HashMap;
use std::fmt;

use colored::Colorize;

use crate::bql::ast;
use crate::storage::{self, Record, RecordType};
use crate::table::{Cell, Column, Row, Rows, Table, TableError};
use crate::utils;

#[derive(Debug)]
pub enum QueryError {
    TableError(TableError),
    TableDoesNotExist(String),
    TableAlreadyExists(String),
}

impl QueryError {
    pub fn to_message(&self) -> String {
        match self {
            QueryError::TableError(e) => e.to_string(),
            QueryError::TableDoesNotExist(table_name) => {
                format!("Table `{}` does not exist", table_name)
            }
            QueryError::TableAlreadyExists(table_name) => {
                format!("Table `{}` already exists", table_name)
            }
        }
    }
}

impl std::error::Error for QueryError {}

impl fmt::Display for QueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            utils::format_message(
                &"query error".bright_yellow().to_string(),
                &self.to_message()
            )
        )
    }
}

pub struct Engine<'a> {
    file: &'a mut storage::File,
    tables: Vec<Table>,
}

impl<'a> Engine<'a> {
    pub fn new(mut file: &'a mut storage::File) -> Engine<'a> {
        let tables = Engine::load_tables(&mut file);
        Engine { tables, file }
    }
    fn load_tables(file: &mut storage::File) -> Vec<Table> {
        let records = file.load_records().expect("Unable to read records");
        let mut tables = Vec::new();
        for record in records {
            if record.record_type == RecordType::Table {
                tables.push(record.data)
            }
        }
        return tables;
    }
    fn flush(&mut self) {
        let records = Record::from_vec(&self.tables);
        self.file
            .write_records(records)
            .expect("Unable to save tables");
    }

    fn get_table_by_name(&mut self, name: String) -> Option<&mut Table> {
        self.tables.iter_mut().find(|table| table.name() == &name)
    }

    pub fn handle_query(&mut self, query: ast::Query) -> Result<String, QueryError> {
        match query {
            ast::Query::Gimme(gimme) => match self.gimme(gimme) {
                Ok(rows) => Ok(format!("{}", rows)),
                Err(e) => Err(e),
            },
            ast::Query::Insert(insert) => match self.insert(insert) {
                Ok(_) => Ok(utils::format_message(
                    &"success".bright_green().to_string(),
                    &"Inserted row",
                )),
                Err(e) => Err(e),
            },
            ast::Query::Tables(tables) => match self.tables(tables) {
                Ok(tables) => Ok(utils::format_table(&mut tabled::Table::new(tables)).to_string()),
                Err(e) => Err(e),
            },
            ast::Query::NewTable(new_table) => match self.new_table(new_table) {
                Ok(nt) => Ok(format!("{}", nt)),
                Err(e) => Err(e),
            },
            ast::Query::DeleteTable(delete_table) => match self.delete_table(&delete_table) {
                Ok(_) => Ok(utils::format_message(
                    &"success".bright_green().to_string(),
                    &format!("Removed table `{}`", delete_table.identifier.value),
                )),
                Err(e) => Err(e),
            },
        }
    }

    // GIMME
    fn gimme(&mut self, gimme: ast::Gimme) -> Result<Rows, QueryError> {
        let table = self
            .get_table_by_name(gimme.table_identifier.value.clone())
            .ok_or_else(|| QueryError::TableDoesNotExist(gimme.table_identifier.value))?;

        let limit_number = match gimme.limit_statement {
            Some(l) => Some(l.number),
            None => None,
        };

        return table
            .find(&gimme.where_statement, limit_number)
            .map_err(QueryError::TableError);
    }

    // INSERT
    fn insert(&mut self, insert: ast::Insert) -> Result<(), QueryError> {
        let table = self
            .get_table_by_name(insert.table_identifier.value.clone())
            .ok_or_else(|| QueryError::TableDoesNotExist(insert.table_identifier.value))?;

        let mut row_values = HashMap::new();
        for item in insert.values {
            row_values.insert(item.key.value, Cell::new(item.value));
        }

        table
            .insert(Row { values: row_values })
            .map_err(QueryError::TableError)?;
        self.flush();
        Ok(())
    }

    // TABLES
    fn tables(&self, _tables: ast::Tables) -> Result<&Vec<Table>, QueryError> {
        Ok(&self.tables)
    }
    fn new_table(&mut self, new_table: ast::NewTable) -> Result<Table, QueryError> {
        if self
            .get_table_by_name(new_table.identifier.value.clone())
            .is_some()
        {
            return Err(QueryError::TableAlreadyExists(new_table.identifier.value));
        }
        let columns = new_table
            .fields
            .iter()
            .map(|field| Column::new(field.key.value.clone(), field.value.clone()))
            .collect::<Vec<Column>>();

        let table = Table::new(new_table.identifier.value, columns);
        self.tables.push(table.clone());
        self.flush();

        Ok(table)
    }
    fn delete_table(&mut self, delete_table: &ast::DeleteTable) -> Result<(), QueryError> {
        let table_index_to_remove = self
            .tables
            .iter()
            .position(|table| table.name() == &delete_table.identifier.value)
            .ok_or_else(|| QueryError::TableDoesNotExist(delete_table.identifier.value.clone()))?;

        self.tables.remove(table_index_to_remove);
        self.flush();

        return Ok(());
    }
}
