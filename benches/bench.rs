use std::cell::Cell;

use banana_db::database::Database;
use criterion::{Criterion, criterion_group, criterion_main};

fn database(c: &mut Criterion) {
    Database::new("bench.db").delete().unwrap();

    let mut db = Database::new("bench.db");
    db.handle_query("new table users {id: Int, name: String};")
        .unwrap();
    let i_cell = Cell::new(0);
    c.bench_function("insert", |b| {
        b.iter(|| {
            let i = i_cell.get();
            db.handle_query(&format!("insert {{id: {}, name: Thomas}} into users;", i))
                .unwrap();
            i_cell.set(i + 1);
        })
    });
    let i_cell = Cell::new(0);
    c.bench_function("select", |b| {
        b.iter(|| {
            let i = i_cell.get();
            db.handle_query(&format!("gimme users where id == {};", i))
                .unwrap();
            i_cell.set(i + 1);
        })
    });
}

criterion_group!(benches, database);
criterion_main!(benches);
