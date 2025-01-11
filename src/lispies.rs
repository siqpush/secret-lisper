use crate::Lisper;
use regex::Regex;
use std::{
    ops::{Add, Sub},
    sync::OnceLock,
};

static IS_NUM: OnceLock<Regex> = OnceLock::new();
static IS_FLOAT: OnceLock<Regex> = OnceLock::new();

pub type LispieFn = Box<dyn Fn(&[Lispies]) -> Lispies>;

#[derive(Clone, Debug, PartialEq)]
pub enum Lispies {
    Symbol(String),
    Int(i32),
    Float(f32),
    List(Lisper),
    None,
}

impl Lispies {
    pub fn is_int(s: impl AsRef<str>) -> bool {
        let is_num_re =
            IS_NUM.get_or_init(|| Regex::new("[0-9]+").expect("regex failed to initialize"));
        is_num_re.is_match(s.as_ref())
    }

    pub fn is_float(s: impl AsRef<str>) -> bool {
        let is_float_re = IS_FLOAT.get_or_init(|| {
            Regex::new("[0-9]*[.]{1}[0-9]*").expect("regex for float failed to initialize")
        });
        is_float_re.is_match(s.as_ref())
    }

    pub fn to_int(s: impl AsRef<str>) -> Lispies {
        Lispies::Int(s.as_ref().parse::<i32>().expect("failed to parse into i32"))
    }

    pub fn to_float(s: impl AsRef<str>) -> Lispies {
        Lispies::Float(s.as_ref().parse::<f32>().expect("failed to parse to"))
    }
}
impl Add for Lispies {
    type Output = Lispies;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Lispies::Int(left), Lispies::Int(right)) => Lispies::Int(left + right),
            (Lispies::Float(left), Lispies::Float(right)) => Lispies::Float(left + right),
            (_, _) => unimplemented!("only int + int and float + float is allowed"),
        }
    }
}

impl Sub for Lispies {
    type Output = Lispies;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Lispies::Int(left), Lispies::Int(right)) => Lispies::Int(left - right),
            (Lispies::Float(left), Lispies::Float(right)) => Lispies::Float(left - right),
            (_, _) => unimplemented!("only int - int and float - float is allowed"),
        }
    }
}

impl From<Lispies> for Option<Lisper> {
    fn from(val: Lispies) -> Self {
        match val {
            Lispies::List(val) => Some(val),
            _ => None,
        }
    }
}

impl From<Lisper> for Lispies {
    fn from(val: Lisper) -> Self {
        Lispies::List(val)
    }
}

impl From<&str> for Lispies {
    fn from(value: &str) -> Self {
        if Self::is_float(value) {
            Self::to_float(value)
        } else if Self::is_int(value) {
            Self::to_int(value)
        } else {
            Self::Symbol(value.to_string())
        }
    }
}
