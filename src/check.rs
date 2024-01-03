use itertools::Itertools;
use regex;
use std::{error::Error, fmt::Display};

use crate::value::{Value, ValueKind};

pub trait Checker {
    fn check(&self, value: &Value) -> Result<Notice, CheckError>;
    fn expecting(&self) -> Vec<ValueKind>;
}

#[derive(Debug, PartialEq, Eq)]
pub enum Notice {
    Clear,
    Attention(String),
}

#[derive(Debug, PartialEq, Eq)]
pub enum CheckError {
    InvalidKind,
}

impl Display for CheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            CheckError::InvalidKind => "Invalid kind",
        };
        f.write_str(msg)
    }
}

impl Error for CheckError {}

pub struct Flatten<T>(Vec<T>);

impl<T> Flatten<T>
where
    T: Checker,
{
    fn new<Iter>(x: Iter) -> Result<Self, FlattenError>
    where
        Iter: Iterator<Item = T>,
    {
        let v = x.collect_vec();
        if !v.iter().map(|x| x.expecting()).all_equal() {
            return Err(FlattenError::InvalidKind);
        }
        Ok(Flatten(v))
    }
}

impl<T> Checker for Flatten<T>
where
    T: Checker,
{
    fn check(&self, value: &Value) -> Result<Notice, CheckError> {
        let res = self
            .0
            .iter()
            .map(|x| x.check(value))
            .collect::<Result<Vec<Notice>, CheckError>>()?;
        for n in res {
            match n {
                Notice::Clear => {}
                Notice::Attention(msg) => return Ok(Notice::Attention(msg)),
            }
        }
        Ok(Notice::Clear)
    }

    fn expecting(&self) -> Vec<ValueKind> {
        self.0.first().unwrap().expecting()
    }
}

pub trait IntoFlat<T>: Sized {
    fn into_flat(self) -> Result<Flatten<T>, FlattenError>;
}

impl<T, Q> IntoFlat<T> for Q
where
    T: Checker,
    Q: Iterator<Item = T>,
{
    fn into_flat(self) -> Result<Flatten<T>, FlattenError> {
        Flatten::new(self)
    }
}

#[derive(Debug)]
pub enum FlattenError {
    InvalidKind,
}

impl Display for FlattenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            FlattenError::InvalidKind => "Invalid kind",
        };
        f.write_str(msg)
    }
}

impl Error for FlattenError {}

pub enum Checkers {
    Any,
    Exact(String, String),
    Regex(regex::Regex, String),
    Between(f64, f64, String),
    Custom(Box<dyn Checker>),
}

impl Checker for Checkers {
    fn check(&self, value: &Value) -> Result<Notice, CheckError> {
        match self {
            Checkers::Any => Ok(Notice::Clear),
            Checkers::Exact(v, msg) => Ok(match v == &value.to_string() {
                true => Notice::Clear,
                false => Notice::Attention(msg.to_string()),
            }),
            Checkers::Regex(pattern, msg) => Ok(match pattern.is_match(&value.to_string()) {
                true => Notice::Clear,
                false => Notice::Attention(msg.to_string()),
            }),
            Checkers::Between(from, to, msg) => match value.is_kind_of(ValueKind::Number) {
                true => {
                    let v: f64 = value.try_into().unwrap();
                    Ok({
                        if *from <= v && v <= *to {
                            Notice::Clear
                        } else {
                            Notice::Attention(msg.to_string())
                        }
                    })
                }
                false => Err(CheckError::InvalidKind),
            },
            Checkers::Custom(n) => n.check(value),
        }
    }

    fn expecting(&self) -> Vec<ValueKind> {
        match self {
            Checkers::Any => vec![ValueKind::Number, ValueKind::Literal],
            Checkers::Exact(_, _) => vec![ValueKind::Number, ValueKind::Literal],
            Checkers::Regex(_, _) => vec![ValueKind::Number, ValueKind::Literal],
            Checkers::Between(_, _, _) => vec![ValueKind::Number],
            Checkers::Custom(inner) => inner.expecting(),
        }
    }
}
