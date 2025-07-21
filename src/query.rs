use crate::bql::ast::{Query, Where};
use crate::bql::{ast, parser};
use crate::table::{Cell, Column, Data, Row, Table};

use crate::storage::{self, File, Record, RecordType};

// create_new_table(&mut file);

// let out = get_record_by_value(table).unwrap();
// println!("{:?}", out[0]);

fn create_new_table(file: &mut File<Table>) {
    let mut table = Table::new(
        "users".to_string(),
        vec![
            Column::new("id".to_string(), Data::Int(None)),
            Column::new("name".to_string(), Data::String(None)),
            Column::new("age".to_string(), Data::Int(None)),
        ],
    );
    table
        .insert(vec![
            Cell::new(Data::Int(Some(1))),
            Cell::new(Data::String(Some("John".to_string()))),
            Cell::new(Data::Int(Some(25))),
        ])
        .unwrap();
    file.add_record(storage::Record::new(
        storage::RecordType::Table,
        table.clone(),
    ));
}

pub struct Engine {
    tables: Vec<Table>,
}

impl Engine {
    pub fn new(file: storage::File<Table>) -> Self {
        let tables = Engine::load_tables(file);
        Engine { tables }
    }
    fn load_tables(file: storage::File<Table>) -> Vec<Table> {
        let records = file.get_records();
        let mut tables = Vec::new();
        for record in records {
            if record.record_type == RecordType::Table {
                tables.push(record.data)
            }
        }
        return tables;
    }

    fn get_table_by_name(&self, name: String) -> Option<&Table> {
        self.tables.iter().find(|&table| table.name() == &name)
    }

    pub fn handle_query(&self, query: Query) {
        match query {
            Query::Gimme(gimme) => {
                println!("{:?}", self.gimme(gimme));
            }
        }
    }

    // GIMME
    fn gimme(&self, gimme: ast::Gimme) -> Option<Vec<&Row>> {
        let table = match self.get_table_by_name(gimme.table_identifier.value) {
            Some(t) => t,
            None => panic!("TODO"),
        };
        let limit_number = match gimme.limit_statement {
            Some(l) => Some(l.number),
            None => None,
        };

        let condition = Engine::get_condition_fn_from_where(gimme.where_statement);

        return table.find(condition, limit_number);
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

                match row[column_index].data() {
                    Data::Int(v) => match v {
                        Some(v) => *v == statement.value,
                        None => false,
                    },
                    // TODO: handle more datatypes
                    _ => false,
                }
            }
            None => true,
        }
    }

    // INSERT
}
