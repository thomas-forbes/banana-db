use crate::bql::ast;
use crate::table::{self, Column, Data, Row, Table};

use crate::storage::{self, Record, RecordType};

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

    fn get_table_by_name(&self, name: String) -> Option<&Table> {
        self.tables.iter().find(|&table| table.name() == &name)
    }

    pub fn handle_query(&mut self, query: ast::Query) -> Result<String, String> {
        match query {
            ast::Query::Gimme(gimme) => match self.gimme(gimme) {
                Ok(rows) => Ok(tabled::Table::new(rows).to_string()),
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
    fn gimme(&self, gimme: ast::Gimme) -> Result<Vec<&Row>, String> {
        let table = match self.get_table_by_name(gimme.table_identifier.value) {
            Some(t) => t,
            None => return Err("Table not found".to_owned()),
        };
        let limit_number = match gimme.limit_statement {
            Some(l) => Some(l.number),
            None => None,
        };

        let condition = Engine::get_condition_fn_from_where(gimme.where_statement);

        return Ok(table.find(condition, limit_number));
    }
    fn get_condition_fn_from_where(
        where_statement: Option<ast::Where>,
    ) -> impl Fn(&Row, &Vec<Column>) -> bool {
        move |row, columns| match &where_statement {
            Some(statement) => {
                let column_index = match columns
                    .iter()
                    .position(|c| *c.name() == statement.field.value)
                {
                    Some(index) => index,
                    None => panic!("Invalid field name"),
                };

                let row_value = row.values[column_index].data();
                let where_value = table::Data::Int(Some(statement.value));

                return row_value
                    .operator_compare(&where_value, statement.comparison_operator.token_type());
            }
            None => true,
        }
    }

    // INSERT

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
            .map(|field| {
                Ok(Column::new(
                    field.key.value.clone(),
                    match Data::from_token_type(field.value.token_type()) {
                        Some(v) => v,
                        None => return Err("Invalid datatype in table creation".to_owned()),
                    },
                ))
            })
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
