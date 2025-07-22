use std::collections::HashMap;

use crate::bql::ast;
use crate::storage::{self, Record, RecordType};
use crate::table::{self, Cell, Column, Row, Table};

pub struct Engine {
    file: storage::File,
    tables: Vec<Table>,
}

impl Engine {
    pub fn new(mut file: storage::File) -> Engine {
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

    pub fn handle_query(&mut self, query: ast::Query) -> Result<String, String> {
        match query {
            ast::Query::Gimme(gimme) => match self.gimme(gimme) {
                // Ok(rows) => Ok(tabled::Table::new(rows).to_string()),
                Ok(rows) => Ok(format!("{:?}", rows)),
                Err(e) => Err(e),
            },
            ast::Query::Insert(insert) => match self.insert(insert) {
                // Ok(rows) => Ok(tabled::Table::new(rows).to_string()),
                Ok(_) => Ok(format!("Success")),
                Err(e) => Err(e),
            },
            ast::Query::Tables(tables) => match self.tables(tables) {
                Ok(tables) => Ok(tabled::Table::new(tables).to_string()),
                Err(e) => Err(e),
            },
            ast::Query::NewTable(new_table) => match self.new_table(new_table) {
                Ok(nt) => Ok(format!("{}", nt)),
                Err(e) => Err(e),
            },
            ast::Query::DeleteTable(delete_table) => match self.delete_table(&delete_table) {
                Ok(_) => Ok(format!("Removed table `{}`", delete_table.identifier.value)),
                Err(e) => Err(e),
            },
        }
    }

    // GIMME
    fn gimme(&mut self, gimme: ast::Gimme) -> Result<Vec<&Row>, String> {
        let table = match self.get_table_by_name(gimme.table_identifier.value) {
            Some(t) => t,
            None => return Err("Table not found".to_owned()),
        };
        let limit_number = match gimme.limit_statement {
            Some(l) => Some(l.number),
            None => None,
        };

        return table.find(&gimme.where_statement, limit_number);
    }

    // INSERT
    fn insert(&mut self, insert: ast::Insert) -> Result<(), String> {
        let table = match self.get_table_by_name(insert.table_identifier.value) {
            Some(t) => t,
            None => return Err("Table not found".to_owned()),
        };

        let mut row_values = HashMap::new();
        for item in insert.values {
            row_values.insert(item.key.value, Cell::new(item.value));
        }

        table.insert(Row { values: row_values })?;
        self.flush();
        Ok(())
    }

    // TABLES
    fn tables(&self, _tables: ast::Tables) -> Result<&Vec<Table>, String> {
        Ok(&self.tables)
    }
    fn new_table(&mut self, new_table: ast::NewTable) -> Result<Table, String> {
        if self
            .get_table_by_name(new_table.identifier.value.clone())
            .is_some()
        {
            return Err(format!(
                "Table `{}` already exists",
                new_table.identifier.value
            ));
        }
        let columns = new_table
            .fields
            .iter()
            .map(|field| Ok(Column::new(field.key.value.clone(), field.value.clone())))
            .collect::<Result<Vec<Column>, String>>()?;

        let table = Table::new(new_table.identifier.value, columns);
        self.tables.push(table.clone());
        self.flush();

        Ok(table)
    }
    fn delete_table(&mut self, delete_table: &ast::DeleteTable) -> Result<(), String> {
        let table_index_to_remove = self
            .tables
            .iter()
            .position(|table| table.name() == &delete_table.identifier.value)
            .ok_or("Table not found".to_owned())?;

        self.tables.remove(table_index_to_remove);
        self.flush();

        return Ok(());
    }
}
