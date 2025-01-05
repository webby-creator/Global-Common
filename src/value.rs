use std::fmt::{Display, Formatter, Result as FmtResult};

use eyre::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};
use time::{Date, OffsetDateTime, Time};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Number {
    Byte(u8),
    Integer(i64),
    Float(f64),
}

impl Number {
    pub fn into_u8(self) -> eyre::Result<u8> {
        if let Self::Byte(v) = self {
            Ok(v)
        } else {
            eyre::bail!("Not u8")
        }
    }

    // TODO: Impl Into
    pub fn convert_f64(self) -> f64 {
        match self {
            Number::Byte(v) => v as f64,
            Number::Integer(v) => v as f64,
            Number::Float(v) => v,
        }
    }

    pub fn convert_i64(self) -> i64 {
        match self {
            Number::Byte(v) => v as i64,
            Number::Integer(v) => v,
            Number::Float(v) => v as i64,
        }
    }
}

impl From<u8> for Number {
    fn from(value: u8) -> Self {
        Self::Byte(value)
    }
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Self::Integer(value as i64)
    }
}

impl From<i64> for Number {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}

impl From<f64> for Number {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl From<Number> for i32 {
    fn from(val: Number) -> Self {
        match val {
            Number::Byte(v) => v as i32,
            Number::Integer(v) => v as i32,
            Number::Float(v) => v as i32,
        }
    }
}

impl From<Number> for i64 {
    fn from(val: Number) -> Self {
        match val {
            Number::Byte(v) => v as i64,
            Number::Integer(v) => v as i64,
            Number::Float(v) => v as i64,
        }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Number::Byte(v) => v.fmt(f),
            Number::Integer(v) => v.fmt(f),
            Number::Float(v) => v.fmt(f),
        }
    }
}

impl Default for Number {
    fn default() -> Self {
        Self::Integer(0)
    }
}

/// A Simple Value is always untagged and the value will go into their respective variant w/o any fuss.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SimpleValue {
    Text(String),
    Number(Number),
    Boolean(bool),

    DateTime(OffsetDateTime),
    Date(Date),
    Time(Time),

    ListString(Vec<String>),
    ListNumber(Vec<Number>),

    ArrayUnknown(Vec<serde_json::Value>),
    ObjectUnknown(serde_json::Value),
}

impl SimpleValue {
    pub fn any_as_text(&self) -> Result<String> {
        Ok(match self {
            Self::Text(s) => s.to_string(),
            Self::Number(n) => n.to_string(),
            Self::Boolean(b) => b.to_string(),
            Self::DateTime(dt) => dt.to_string(),
            Self::Date(d) => d.to_string(),
            Self::Time(t) => t.to_string(),
            Self::ListString(_)
            | Self::ListNumber(_)
            | Self::ArrayUnknown(_)
            | Self::ObjectUnknown(_) => return Err(anyhow!("Unable to convert to String"))?,
        })
    }

    pub fn try_as_text(self) -> Result<String> {
        if let Self::Text(v) = self {
            Ok(v)
        } else {
            return Err(anyhow!("Unable to convert to Text"))?;
        }
    }

    pub fn try_as_number(&self) -> Result<Number> {
        if let Self::Number(v) = self {
            Ok(*v)
        } else {
            return Err(anyhow!("Unable to convert to Number"))?;
        }
    }

    pub fn try_as_boolean(&self) -> Result<bool> {
        if let Self::Boolean(v) = self {
            Ok(*v)
        } else {
            return Err(anyhow!("Unable to convert to Boolean"))?;
        }
    }

    pub fn try_as_date_time(&self) -> Result<OffsetDateTime> {
        if let Self::DateTime(v) = self {
            Ok(*v)
        } else {
            return Err(anyhow!("Unable to convert to DateTime"))?;
        }
    }

    pub fn try_as_date(&self) -> Result<Date> {
        if let Self::Date(v) = self {
            Ok(*v)
        } else {
            return Err(anyhow!("Unable to convert to Date"))?;
        }
    }

    pub fn try_as_time(&self) -> Result<Time> {
        if let Self::Time(v) = self {
            Ok(*v)
        } else {
            return Err(anyhow!("Unable to convert to Time"))?;
        }
    }

    pub fn try_as_list_string(self) -> Result<Vec<String>> {
        if let Self::ListString(v) = self {
            Ok(v)
        } else {
            return Err(anyhow!("Unable to convert to String List"))?;
        }
    }

    pub fn try_as_list_number(self) -> Result<Vec<Number>> {
        if let Self::ListNumber(v) = self {
            Ok(v)
        } else {
            return Err(anyhow!("Unable to convert to Number List"))?;
        }
    }

    pub fn try_as_bytes(self) -> Result<Vec<u8>> {
        if let Self::ListNumber(v) = self {
            Ok(v.into_iter().map(|v| v.into_u8()).collect::<Result<_>>()?)
        } else {
            return Err(anyhow!("Unable to convert to Number List"))?;
        }
    }

    pub fn ensure_text(self) -> Result<Self> {
        if matches!(self, Self::Text(_)) {
            Ok(self)
        } else {
            bail!("Not Text")
        }
    }

    pub fn ensure_number(self) -> Result<Self> {
        if matches!(self, Self::Number(_)) {
            Ok(self)
        } else {
            bail!("Not Number")
        }
    }

    pub fn ensure_boolean(self) -> Result<Self> {
        if matches!(self, Self::Boolean(_)) {
            Ok(self)
        } else {
            bail!("Not Boolean")
        }
    }

    pub fn ensure_date_time(self) -> Result<Self> {
        if matches!(self, Self::DateTime(_)) {
            Ok(self)
        } else {
            bail!("Not Date Time")
        }
    }

    pub fn ensure_date(self) -> Result<Self> {
        if matches!(self, Self::Date(_)) {
            Ok(self)
        } else {
            bail!("Not Date")
        }
    }

    pub fn ensure_time(self) -> Result<Self> {
        if matches!(self, Self::Time(_)) {
            Ok(self)
        } else {
            bail!("Not Time")
        }
    }

    pub fn ensure_list_string(self) -> Result<Self> {
        if matches!(self, Self::ListString(_)) {
            Ok(self)
        } else {
            bail!("Not String List")
        }
    }

    pub fn ensure_list_number(self) -> Result<Self> {
        if matches!(self, Self::ListNumber(_)) {
            Ok(self)
        } else {
            bail!("Not Number List")
        }
    }
}

impl From<String> for SimpleValue {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}

impl From<Number> for SimpleValue {
    fn from(value: Number) -> Self {
        Self::Number(value)
    }
}

impl From<bool> for SimpleValue {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl From<OffsetDateTime> for SimpleValue {
    fn from(value: OffsetDateTime) -> Self {
        Self::DateTime(value)
    }
}

impl From<Date> for SimpleValue {
    fn from(value: Date) -> Self {
        Self::Date(value)
    }
}

impl From<Time> for SimpleValue {
    fn from(value: Time) -> Self {
        Self::Time(value)
    }
}

impl From<Vec<String>> for SimpleValue {
    fn from(value: Vec<String>) -> Self {
        Self::ListString(value)
    }
}

impl From<Vec<Number>> for SimpleValue {
    fn from(value: Vec<Number>) -> Self {
        Self::ListNumber(value)
    }
}
