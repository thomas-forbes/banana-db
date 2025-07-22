use crate::table::{Comparison, Data};

#[derive(Debug)]
pub enum Query {
    Gimme(Gimme),
    Tables(Tables),
    NewTable(NewTable),
    DeleteTable(DeleteTable),
    Insert(Insert),
}

// TODO: remove pub from fields

#[derive(Debug, Clone)]
pub struct Identifier {
    pub value: String,
}

#[derive(Debug)]
pub struct Gimme {
    pub table_identifier: Identifier,
    pub limit_statement: Option<Limit>,
    pub where_statement: Option<Where>,
}

#[derive(Debug)]
pub struct Limit {
    pub number: usize,
}

#[derive(Debug, Clone)]
pub struct Where {
    pub field: Identifier,
    pub value: Data,
    pub comparison: Comparison,
}

#[derive(Debug, Clone)]
pub struct Insert {
    pub values: Map,
    pub table_identifier: Identifier,
}

#[derive(Debug, Clone)]
pub struct Tables {}

#[derive(Debug, Clone)]
pub struct NewTable {
    pub identifier: Identifier,
    pub fields: Map,
}

#[derive(Debug, Clone)]
pub struct DeleteTable {
    pub identifier: Identifier,
}

#[derive(Debug, Clone)]
pub struct MapItem {
    pub key: Identifier,
    pub value: Data,
}

pub type Map = Vec<MapItem>;
