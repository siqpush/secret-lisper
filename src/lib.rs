#![allow(clippy::collapsible_if)]

use std::collections::VecDeque;
use regex::Regex;
use std::sync::OnceLock;
use log::warn;

static IS_NUM: OnceLock<Regex> = OnceLock::new();
static IS_FLOAT: OnceLock<Regex> = OnceLock::new();

#[derive(Debug, PartialEq, Clone)]
pub struct Lisper {
    pub list: Vec<Lispies>,
}

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

impl Lisper {
    pub fn new() -> Self {
        Self { list: vec![] }
    }
}

impl Default for Lisper {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Parser
where
    for<'a, 'b> &'a Self: PartialEq<&'b str>,
{
    fn vectorize(&self) -> Vec<String>;

    /*
         if token is a "(" create a new lisper on the stack
         if token is a ")" pop the stack and add the lisper to the current lisper
         if token is a symbol add it to the current lisper
     */
    fn tokenize(&self) -> Option<Lisper>;
}

impl Parser for str {
    fn vectorize(&self) -> Vec<String> {
        let new_program = self.replace("(", "( ").replace(")", " )");
        new_program
            .split_ascii_whitespace()
            .map(|a: &str| a.to_string())
            .collect()
    }
    fn tokenize(&self) -> Option<Lisper> {
        
        let mut lstack = VecDeque::new();
        let mut lisper = None;

        for token in self.vectorize() {
            match token.as_str() {
                "(" => {
                    if lisper.is_none() {
                        lisper = Some(Lisper::new());
                    } else {
                        lstack.push_front(Lisper::new());
                    }
                }
                ")" => {
                    if lstack.is_empty() {
                        
                    } else if lstack.len() == 1  {
                        // only one item on the stack means we can psuh on the front
                        if let Some(ref mut lisper) = lisper {
                            lisper.list.push(
                                lstack.pop_front().expect("invalid parenthesis").into()
                            );
                        }
                    } else {
                        // pop the front and place it on the back since we are not at an empty stack yet
                        let child = lstack.pop_front().expect("invalid parenthesis");
                        if let Some(parent) = lstack.front_mut() {
                            parent.list.push(Lispies::List(child));
                        }
                        
                    }
                }
                val => {
                    // if the stack is empty then add it to the main lisper
                    if lstack.is_empty() {
                        if let Some(ref mut lisper) = lisper {
                            lisper.list.push(val.into());
                        } else {
                            panic!("program did not initialize properly check for first parenthesis")
                        }
                    } else if let Some(curr_lisper) = lstack.front_mut() {
                        curr_lisper.list.push(val.into())
                    } else {
                        unreachable!("should never be here")
                    }
                }
            }
        }
        lisper
    }
}

#[cfg(test)]
mod parser_tests {
    use std::ops::Index;

    use super::*;

    #[test]
    fn vectorize_test_1() {
        let program = "(begin (define r 10) (* pi (* r r)))";
        let answer = [
            "(", "begin", "(", "define", "r", "10", ")", "(", "*", "pi", "(", "*", "r", "r", ")",
            ")", ")",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
        assert_eq!(program.vectorize(), answer);
    }
}

#[cfg(test)]
mod lisper_tests {
    use super::*;

    #[test]
    pub fn test_one_lisper_1() {
        let program = "(define r 10)";

        let res = Lisper {
            list: vec![
                Lispies::Symbol("define".to_string()),
                Lispies::Symbol("r".to_string()),
                Lispies::Int(10),
            ],
        };
        let act = program.tokenize();
        assert_eq!(act, Some(res));
    }

    #[test]
    pub fn test_nest_lisper_1() {
        let program = "(define (r))";

        let res = Lisper {
            list: vec![
                Lispies::Symbol("define".to_string()),
                Lispies::List(Lisper {
                    list: vec![Lispies::Symbol("r".to_string())],
                }),
            ],
        };
        let act = program.tokenize();
        assert_eq!(act, Some(res));
    }

    #[test]
    pub fn test_nest_lisper_2() {
        let program = "(define (r) (pi))";

        let res = Lisper {
            list: vec![
                Lispies::Symbol("define".to_string()),
                Lispies::List(Lisper {
                    list: vec![Lispies::Symbol("r".to_string())],
                }),
                Lispies::List(Lisper {
                    list: vec![Lispies::Symbol("pi".to_string())],
                }),
            ],
        };
        let act = program.tokenize();
        assert_eq!(act, Some(res));
    }

    #[test]
    pub fn test_nest_lisper_3() {
        let program = "(define (r (r)))";

        let res = Lisper {
            list: vec![
                Lispies::Symbol("define".to_string()),
                Lispies::List(Lisper {
                    list: vec![
                        Lispies::Symbol("r".to_string()),
                        Lispies::List(Lisper {
                            list: vec![Lispies::Symbol("r".to_string())],
                        }),
                    ],
                }),
            ],
        };
        let act = program.tokenize();
        assert_eq!(act, Some(res));
    }

    #[test]
    pub fn test_nest_lisper_4() {
        let program = "(begin (define r 10) (* pi (* r r)))";

        let res = Lisper {
            list: vec![
                Lispies::Symbol("begin".to_string()),
                Lispies::List(Lisper {
                    list: vec![
                        Lispies::Symbol("define".to_string()),
                        Lispies::Symbol("r".to_string()),
                        Lispies::Int(10),
                    ],
                }),
                Lispies::List(Lisper {
                    list: vec![
                        Lispies::Symbol("*".to_string()),
                        Lispies::Symbol("pi".to_string()),
                        Lispies::List(Lisper {
                            list: vec![
                                Lispies::Symbol("*".to_string()),
                                Lispies::Symbol("r".to_string()),
                                Lispies::Symbol("r".to_string()),
                            ],
                        }),
                    ],
                }),
            ],
        };
        let act = program.tokenize();
        println!("{:#?}", act);
        println!("{:#?}", res);
        assert_eq!(act, Some(res));
    }
}
