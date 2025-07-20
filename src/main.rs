mod bql; // banana query language
mod repl;
mod storage;
mod table;

use table::{Cell, Column, Data, Table};

use crate::storage::{File, Record, RecordType};

// XXX: debug code
fn load_tables(records: &Vec<Record<Table>>) -> Vec<&Table> {
    let mut tables = Vec::new();
    for record in records {
        if record.record_type == RecordType::Table {
            tables.push(&record.data)
        }
    }
    return tables;
}

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

fn get_record_by_value(table: &Table) -> Option<Vec<&Vec<Cell>>> {
    table.find(
        &Column::new("id".to_string(), Data::Int(None)),
        &Data::Int(Some(1)),
        None,
    )
}

fn main() {
    let mut file: File<Table> = storage::File::open("db.bin".to_string());
    let records = file.get_records();

    // create_new_table(&mut file);

    // let tables = load_tables(records);
    // let table = tables[0];

    // let out = get_record_by_value(table).unwrap();
    // println!("{:?}", out[0]);
}
