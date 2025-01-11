use std::collections::HashMap;

use crate::Lispies;

pub type Func = Box<dyn Fn(Lispies, Lispies) -> Lispies>;

pub fn env(hm: &mut HashMap<String, Func>) {
    hm.insert(
        "+".to_string(),
        Box::new(|left: Lispies, right: Lispies| left.clone() + right.clone()),
    );
    hm.insert(
        "-".to_string(),
        Box::new(|left: Lispies, right: Lispies| left.clone() - right.clone()),
    );
}

pub fn eval(lisps: &[Lispies], env: &HashMap<String, Func>) -> Lispies {
    let mut res = Lispies::Int(0);
    let mut pos = 0;
    while let Some(lisp) = lisps.get(pos) {
        match lisp {
            Lispies::Symbol(x) => {
                let arg1 = eval_next(&mut pos, env, lisps);
                let arg2 = eval_next(&mut pos, env, lisps);
                res = res
                    + env
                        .get(&String::from(x))
                        .map(|val| val(arg1, arg2))
                        .expect("invalid key entry");
            }
            Lispies::Int(x) => {
                res = res + Lispies::Int(*x);
            }
            val => {
                panic!("unhandled Lispie {:?}", val);
            }
        }
        pos += 1;
    }
    res
}

fn eval_next(pos: &mut usize, env: &HashMap<String, Func>, lisps: &[Lispies]) -> Lispies {
    *pos += 1;
    match lisps.get(*pos) {
        Some(Lispies::Int(val)) => Lispies::Int(*val),
        Some(Lispies::List(vals)) => eval(&vals.list, env),
        _ => panic!("not yet handled"),
    }
}

#[cfg(test)]
mod evaluator_test {
    use std::collections::HashMap;

    use crate::{lispies::Lispies, parser::Parser};

    use super::{env, eval};

    #[test]
    fn test_simple_add() {
        let program = "(+ 2 1)";
        let tokens = program.tokenize();
        if let Some(tokens) = tokens {
            let mut hm = HashMap::default();
            env(&mut hm);
            let res = eval(&tokens.list, &hm);
            assert_eq!(res, Lispies::Int(3));
        }
    }

    #[test]
    fn test_simple_nested_add() {
        let program = "(+ (+ 1 1) (- 2 1))";
        let tokens = program.tokenize();
        if let Some(tokens) = tokens {
            let mut hm = HashMap::default();
            env(&mut hm);
            let res = eval(&tokens.list, &hm);
            assert_eq!(res, Lispies::Int(3));
        }
    }
}
