use std::collections::HashMap;

use crate::database::{data::Data, table::axes::Cell};

use super::*;

#[test]
#[should_panic]
fn primary_key_violation() {
    let mut table = Table::new(
        "test".to_string(),
        "id".to_string(),
        vec![Column::new("id".to_string(), Data::Int(None))],
    );

    table
        .insert(Row {
            values: HashMap::from([("id".to_string(), Cell::new(Data::Int(Some(1))))]),
        })
        .unwrap();

    table
        .insert(Row {
            values: HashMap::from([("id".to_string(), Cell::new(Data::Int(Some(1))))]),
        })
        .unwrap();
}
