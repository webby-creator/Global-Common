use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::value::Number;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Filter {
    pub name: String,
    pub cond: FilterConditionType,
    pub value: FilterValue,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FilterConditionType {
    Eq,
    Neq,
    Cont,
    Dnc,
    Gte,
    Gt,
    Lte,
    Lt,
    Between,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FilterValue {
    Number(Number),
    Text(String),
    IdList(Vec<String>),
    Range((Number, Number)),
}

impl FilterValue {
    pub fn is_range(&self) -> bool {
        matches!(self, FilterValue::Range(_))
    }
}

impl Display for FilterValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FilterValue::Text(s) => write!(f, "{s}"),
            FilterValue::Number(n) => write!(f, "{n}"),
            FilterValue::IdList(ids) => write!(f, "{}", ids.join(",")),
            FilterValue::Range((start, end)) => write!(f, "{start}-{end}"),
        }
    }
}
