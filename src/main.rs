use std::{clone, io::Write};

mod parser;

use parser::ast::{Expr, Function, Ident, Op};

#[derive(Clone)]
struct State {
    vars: std::collections::HashMap<Ident, Expr>,
}

fn evaluate<'a>(expr: &'a Expr, state: &mut State) -> Expr {
    match expr {
        fun @ Expr::Function(Function { name, args, body }) => {
            state.vars.insert(name.clone(), fun.clone());
            Expr::Noop
        }
        Expr::BinOp(a, op, b) => {
            let a = evaluate(a, state);
            let b = evaluate(b, state);
            match (a, op, b) {
                (Expr::Number(a), Op::Add, Expr::Number(b)) => Expr::Number(a + b),
                (Expr::Number(a), Op::Sub, Expr::Number(b)) => Expr::Number(a - b),
                (Expr::Number(a), Op::Mul, Expr::Number(b)) => Expr::Number(a * b),
                (Expr::Number(a), Op::Div, Expr::Number(b)) => Expr::Number(a / b),
                _ => panic!("Invalid operation"),
            }
        }
        Expr::Call(ident, args) => call_function(ident, args, state),
        Expr::Let(ident, value) => {
            let value = evaluate(value, state);
            state.vars.insert(ident.clone(), value);
            Expr::Noop
        }
        Expr::Var(ident) => state.vars.get(ident).unwrap().clone(),
        n @ Expr::Number(_) => n.clone(),
        s @ Expr::String(_) => s.clone(),
        _ => panic!("NOT IMPLEMENTED"),
    }
}

fn call_function(ident: &Ident, args: &Vec<Box<Expr>>, state: &mut State) -> Expr {
    match ident {
        Ident(val) if val == "exit" => {
            std::process::exit(0);
        }
        Ident(val) if val == "println" => {
            for arg in args {
                let arg = evaluate(arg, state);
                match arg {
                    Expr::Number(n) => println!("{}", n),
                    Expr::String(s) => println!("{}", s),
                    _ => panic!("Invalid argument"),
                }
            }
            Expr::Noop
        }
        Ident(val) if val == "print" => {
            for arg in args {
                let arg = evaluate(arg, state);
                match arg {
                    Expr::Number(n) => print!("{}", n),
                    Expr::String(s) => print!("{}", s),
                    _ => panic!("Invalid argument"),
                }
            }
            Expr::Noop
        }
        ident => {
            let fun = state.vars.get(&ident).unwrap().clone();
            match fun {
                Expr::Function(Function {
                    args: fun_args,
                    body,
                    ..
                }) => {
                    if args.len() != fun_args.len() {
                        panic!("Invalid number of arguments");
                    }
                    let mut cloned_state = state.clone();
                    for (arg, (arg_name, _)) in args.iter().zip(fun_args.iter()) {
                        let mut state_instance = state.clone();
                        cloned_state
                            .vars
                            .insert(arg_name.clone(), evaluate(arg, &mut state_instance));
                    }
                    match body.len() {
                        0 => Expr::Noop,
                        1 => evaluate(&body[0], &mut cloned_state),
                        _ => {
                            for expr in &body[..body.len() - 1] {
                                evaluate(&expr, &mut cloned_state);
                            }
                            evaluate(&body[body.len() - 1], &mut cloned_state)
                        }
                    }
                }
                _ => panic!("Not a function"),
            }
        }
    }
}
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    match &args[..] {
        [_, file] => {
            let input = std::fs::read_to_string(file).unwrap();
            let statements = parser::silly::FileParser::new().parse(&input).unwrap();
            let mut state = State {
                vars: std::collections::HashMap::new(),
            };
            for statement in statements {
                evaluate(&statement, &mut state);
            }
            evaluate(&Expr::Call(Ident("main".to_string()), vec![]), &mut state);
            return;
        }
        _ => {}
    }
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    loop {
        print!("> ");
        stdout.flush().unwrap();
        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();
        let expr = parser::silly::ExprParser::new().parse(&buf);
        println!("{:?}", expr);
    }
}
