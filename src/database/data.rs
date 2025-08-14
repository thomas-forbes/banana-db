use std::cmp::Ordering;
use std::fmt::Display;

use colored::Colorize;
use serde::{Deserialize, Serialize};

use crate::bql::token::TokenType;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Data {
    Int(Option<i64>),
    Float(Option<f64>),
    String(Option<String>),
    Boolean(Option<bool>),
}

impl Data {
    pub fn same_type(&self, other: &Self) -> bool {
        match (self, other) {
            (Data::Int(_), Data::Int(_)) => true,
            (Data::Float(_), Data::Float(_)) => true,
            (Data::String(_), Data::String(_)) => true,
            (Data::Boolean(_), Data::Boolean(_)) => true,
            _ => false,
        }
    }
    fn fmt_data_type(&self) -> String {
        match self {
            Data::Int(_) => "Int".blue().to_string(),
            Data::Float(_) => "Float".cyan().to_string(),
            Data::String(_) => "String".green().to_string(),
            Data::Boolean(_) => "Boolean".purple().to_string(),
        }
    }
    fn fmt_data_value(&self) -> Option<String> {
        match self {
            Data::Int(Some(i)) => Some(i.to_string()),
            Data::Float(Some(f)) => Some(f.to_string()),
            Data::String(Some(s)) => Some(s.clone()),
            Data::Boolean(Some(b)) => Some(b.to_string()),
            _ => None,
        }
    }
}

impl Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data_type = self.fmt_data_type();
        if let Some(value) = self.fmt_data_value() {
            write!(f, "{}({})", data_type, value.dimmed())
        } else {
            write!(f, "{}", data_type)
        }
    }
}

impl PartialEq for Data {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Data::Int(a), Data::Int(b)) => a == b,
            (Data::Float(a), Data::Float(b)) => match (a, b) {
                (None, None) => true,
                (Some(_), None) | (None, Some(_)) => false,
                (Some(af), Some(bf)) => af.to_bits() == bf.to_bits(),
            },
            (Data::String(a), Data::String(b)) => a == b,
            (Data::Boolean(a), Data::Boolean(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for Data {}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Data::Int(a), Data::Int(b)) => option_cmp(a, b),
            (Data::Float(a), Data::Float(b)) => option_cmp_by(a, b, |x, y| x.total_cmp(y)),
            (Data::String(a), Data::String(b)) => option_cmp(a, b),
            (Data::Boolean(a), Data::Boolean(b)) => option_cmp(a, b),
            // Define variant ordering: Int < Float < String < Boolean
            (Data::Int(_), _) => Ordering::Less,
            (Data::Float(_), Data::Int(_)) => Ordering::Greater,
            (Data::Float(_), _) => Ordering::Less,
            (Data::String(_), Data::Int(_) | Data::Float(_)) => Ordering::Greater,
            (Data::String(_), Data::Boolean(_)) => Ordering::Less,
            (Data::Boolean(_), _) => Ordering::Greater,
        }
    }
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn option_cmp<T: Ord>(a: &Option<T>, b: &Option<T>) -> Ordering {
    match (a, b) {
        (None, None) => Ordering::Equal,
        (None, Some(_)) => Ordering::Less,
        (Some(_), None) => Ordering::Greater,
        (Some(av), Some(bv)) => av.cmp(bv),
    }
}

fn option_cmp_by<T>(a: &Option<T>, b: &Option<T>, cmp: impl Fn(&T, &T) -> Ordering) -> Ordering {
    match (a, b) {
        (None, None) => Ordering::Equal,
        (None, Some(_)) => Ordering::Less,
        (Some(_), None) => Ordering::Greater,
        (Some(av), Some(bv)) => cmp(av, bv),
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Comparison {
    Equals,
    NotEquals,
    Less,
    LessEquals,
    Greater,
    GreaterEquals,
}

impl Comparison {
    pub fn apply<T: PartialOrd + PartialEq>(&self, a: &T, b: &T) -> bool {
        match self {
            Comparison::Less => a < b,
            Comparison::LessEquals => a <= b,
            Comparison::Equals => a == b,
            Comparison::Greater => a > b,
            Comparison::GreaterEquals => a >= b,
            Comparison::NotEquals => a != b,
        }
    }
    pub fn from_token_type(token_type: &TokenType) -> Option<Self> {
        match token_type {
            TokenType::Equals => Some(Comparison::Equals),
            TokenType::NotEquals => Some(Comparison::NotEquals),
            TokenType::Less => Some(Comparison::Less),
            TokenType::LessEquals => Some(Comparison::LessEquals),
            TokenType::Greater => Some(Comparison::Greater),
            TokenType::GreaterEquals => Some(Comparison::GreaterEquals),
            _ => None,
        }
    }
}
