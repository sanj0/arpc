use std::collections::HashMap;
use std::io::{stdin, stdout, Write};

pub const QUIT: &str = "quit\n";

pub enum Error {
    IoErr(String),
    MathErr(String),
    StackErr(String),
    EnvErr(String),
    ParseErr(String),
}

fn main() -> Result<(), String> {
    println!("arpc -- arpc reverse polish calculator\n-- absolutely no warranty --");
    let mut input = String::new();
    let mut stack = Vec::<f64>::new();
    let mut bindings = HashMap::<String, f64>::new();
    loop {
        print!(">>>");
        stdout().flush().map_err(|e| Error::IoErr(e.to_string()))?;
        stdin()
            .read_line(&mut input)
            .map_err(|e| Error::IoErr(e.to_string()))?;
        if input == QUIT {
            break;
        }
        execute(&input, &mut stack, &mut bindings)?;
        input.clear();
    }
    Ok(())
}

enum State {
    Start,
    Num(String),
    Ident(String),
}

// when after executing, there is only one number on the stack, pop and print it
pub fn execute(
    input: &str,
    stack: &mut Vec<f64>,
    bindings: &mut HashMap<String, f64>,
) -> Result<(), Error> {
    let chars = input.chars();
    let mut state = State::Start;
    for c in chars {
        match &mut state {
            State::Start => {
                if c.is_ascii_digit() {
                    state = State::Num(c.to_string());
                } else if c.is_alphabetic() {
                    state = State::Ident(c.to_string());
                } else {
                    op_or_err(c, stack)?;
                }
            }
            State::Num(ref mut buf) => {
                if c.is_ascii_digit() || c == '.' {
                    buf.push(c);
                } else {
                    stack.push(
                        buf.parse::<f64>()
                            .map_err(|e| Error::MathErr(e.to_string()))?,
                    );
                    state = State::Start;
                }
            }
            State::Ident(ref mut buf) => {
                if c.is_alphabetic() {
                    buf.push(c);
                } else {
                    stack.push(
                        *bindings.get(buf).ok_or_else(|| {
                            Error::EnvErr(format!("binding '{buf}' does not exist"))
                        })?,
                    );
                    state = State::Start;
                }
            }
        }
    }
    if stack.len() == 1 {
        println!("{}", stack.pop().unwrap());
    }
    Ok(())
}

fn op_or_err(c: char, stack: &mut Vec<f64>) -> Result<(), Error> {
    macro_rules! req_n_numbers {
        ($n:literal) => {
            if stack.len() < $n {
                return Err(Error::StackErr(format!(
                    "{} stack items required where only {} exist!",
                    $n,
                    stack.len()
                )));
            }
        };
    }
    macro_rules! binary_op {
        ($op:tt) => {
            req_n_numbers!(2);
            let b = stack.pop().unwrap();
            let a = stack.pop().unwrap();
            stack.push(a $op b);
        }
    }
    match c {
        '+' => {
            binary_op!(+);
        }
        '-' => {
            binary_op!(-);
        }
        '*' => {
            binary_op!(*);
        }
        '/' => {
            binary_op!(/);
        }
        '%' => {
            binary_op!(%);
        }
        '^' => {
            req_n_numbers!(2);
            let exp = stack.pop().unwrap();
            let base = stack.pop().unwrap();
            stack.push(base.powf(exp));
        }
        c if c.is_whitespace() => {}
        _ => return Err(Error::ParseErr(format!("unexpected char: '{c}'"))),
    }
    Ok(())
}

impl From<Error> for String {
    fn from(err: Error) -> String {
        "An error occured: ".to_owned()
            + &match err {
                Error::IoErr(ref msg) => "io error: ".to_owned() + msg,
                Error::MathErr(ref msg) => "math error: ".to_owned() + msg,
                Error::StackErr(ref msg) => "stack error: ".to_owned() + msg,
                Error::EnvErr(ref msg) => "env error: ".to_owned() + msg,
                Error::ParseErr(ref msg) => "parse error: ".to_owned() + msg,
            }
    }
}
