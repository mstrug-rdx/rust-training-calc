#![allow(dead_code)]

use std::str::FromStr;

pub type BoxedExpr = Box<Expr>;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(i64),
    Add(BoxedExpr, BoxedExpr),
    Sub(BoxedExpr, BoxedExpr),
    Sqr(BoxedExpr),
    Mul(BoxedExpr, BoxedExpr),
    Div(BoxedExpr, BoxedExpr),
}

impl Expr {
    pub fn eval(&self) -> Result<i64, EvalError> {
        Ok(match self {
            Expr::Number(x) => *x,
            Expr::Add(x, y) => x.eval()? + y.eval()?,
            Expr::Sub(x, y) => x.eval()? - y.eval()?,
            Expr::Mul(x, y) => x.eval()? * y.eval()?,
            Expr::Div(x, y) => {
                let y = y.eval()?;
                if y == 0 {
                    return Err(EvalError::DivisionByZero);
                } else {
                    x.eval()? / y
                }
            }
            Expr::Sqr(x) => {
                let x = x.eval()?;
                x * x
            }
        })
    }
}

impl FromStr for Expr {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse(s)
    }
}

#[derive(Debug, PartialEq)]
pub enum EvalError {
    DivisionByZero,
}

impl std::fmt::Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvalError::DivisionByZero => write!(f, "Divistion by zero"),
        }
    }
}

impl std::error::Error for EvalError {}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum ParseError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Wrong arguments count")]
    WrongArgumentsCount,
    #[error("Empty input")]
    EmptyInput,
    #[error("Left arguments")]
    LeftArguments,
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum ParseOrEvalError {
    #[error("Parse error: {0}")]
    Parse(#[from] ParseError),
    #[error("Evaluation error: {0}")]
    Eval(#[from] EvalError),
}

pub fn eval_str(s: &str) -> Result<i64, ParseOrEvalError> {
    Ok(s.parse::<Expr>()?.eval()?) // automatic conversion between error types due to implementation of From<>
}

/*fn eval(expr: &Expr) -> Result<i64, EvalError> {
    Ok(match expr {
        Expr::Number(x) => *x,
        Expr::Add(x, y) => eval(x)? + eval(y)?,
        Expr::Sub(x, y) => eval(x)? - eval(y)?,
        Expr::Mul(x, y) => eval(x)? * eval(y)?,
        Expr::Div(x, y) => {
            let y = eval(y)?;
            if y == 0 {
                return Err(EvalError::DivisionByZero);
            } else {
                eval(x)? / y
            }
        }
        Expr::Sqr(x) => {
            let x = eval(x)?;
            x * x
        }
    })
}*/

// compatible input: "3 sqr 4 sqr + 5 sqr -"
fn parse(input: &str) -> Result<Expr, ParseError> {
    let mut stack: Vec<Expr> = Vec::new();

    for word in input.split_ascii_whitespace() {
        match word {
            "-" => {
                let x = stack.pop().ok_or(ParseError::WrongArgumentsCount)?;
                let y = stack.pop().ok_or(ParseError::WrongArgumentsCount)?;
                stack.push(Expr::Sub(y.into(), x.into()))
            }
            "+" => {
                let x = stack.pop().ok_or(ParseError::WrongArgumentsCount)?;
                let y = stack.pop().ok_or(ParseError::WrongArgumentsCount)?;
                stack.push(Expr::Add(y.into(), x.into()))
            }
            "*" => {
                let x = stack.pop().ok_or(ParseError::WrongArgumentsCount)?;
                let y = stack.pop().ok_or(ParseError::WrongArgumentsCount)?;
                stack.push(Expr::Mul(y.into(), x.into()))
            }
            "/" => {
                let x = stack.pop().ok_or(ParseError::WrongArgumentsCount)?;
                let y = stack.pop().ok_or(ParseError::WrongArgumentsCount)?;
                stack.push(Expr::Div(y.into(), x.into()))
            }
            "sqr" => {
                let x = stack.pop().ok_or(ParseError::WrongArgumentsCount)?;
                stack.push(Expr::Sqr(x.into()))
            }
            _ => {
                let x = word
                    .parse::<i64>()
                    .map_err(|_| ParseError::InvalidInput(word.to_string()))?;
                stack.push(Expr::Number(x));
            }
        }
    }

    if stack.len() > 1 {
        Err(ParseError::LeftArguments)
    } else {
        stack.pop().ok_or(ParseError::EmptyInput)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_add() {
        let input = "1 1 +";
        let res_p = parse(input).unwrap();
        let res = res_p.eval().unwrap();
        assert_eq!(res, 2);
    }

    #[test]
    fn parse_sub() {
        let input = "2 1 -";
        let res_p = parse(input).unwrap();
        let res = res_p.eval().unwrap();
        assert_eq!(res, 1);
    }

    #[test]
    fn parse_mul() {
        let input = "2 3 *";
        let res_p = parse(input).unwrap();
        let res = res_p.eval().unwrap();
        assert_eq!(res, 6);
    }

    #[test]
    fn parse_div() {
        let input = "4 2 /";
        let res_p = parse(input).unwrap();
        let res = res_p.eval().unwrap();
        assert_eq!(res, 2);
    }

    #[test]
    fn parse_sqr() {
        let input = "4 sqr";
        let res_p = parse(input).unwrap();
        let res = res_p.eval().unwrap();
        assert_eq!(res, 16);
    }

    #[test]
    fn parse_add_mul() {
        let input = "4 2 + 6 *";
        let res_p = parse(input).unwrap();
        let res = res_p.eval().unwrap();
        assert_eq!(res, 36);
    }

    #[test]
    fn parse_add_mul_sqr() {
        let input = "4 2 + 1 * sqr";
        let res_p = parse(input).unwrap();
        let res = res_p.eval().unwrap();
        assert_eq!(res, 36);
    }

    #[test]
    fn parse_test() {
        let input = "3 sqr 4 sqr + 5 sqr -";
        let res_p = parse(input).unwrap();
        let res = res_p.eval().unwrap();
        assert_eq!(res, 0);
    }

    #[test]
    fn parse_simple() {
        let input = "1";
        let res_p = parse(input).unwrap();
        let res = res_p.eval().unwrap();
        assert_eq!(res, 1);
    }

    #[test]
    fn parse_error_1() {
        let input = "";
        let res = parse(input);
        assert_eq!(res, Err(ParseError::EmptyInput));
    }

    #[test]
    fn parse_error_2() {
        let input = "1 +";
        let res = parse(input);
        assert_eq!(res, Err(ParseError::WrongArgumentsCount));
    }

    #[test]
    fn parse_error_3() {
        let input = "something";
        let res = parse(input);
        assert_eq!(
            res,
            Err(ParseError::InvalidInput(String::from("something")))
        );
    }

    #[test]
    fn parse_error_4() {
        let input = "1 1";
        let res = parse(input);
        assert_eq!(res, Err(ParseError::LeftArguments));
    }

    #[test]
    fn test_add() {
        let expr = Expr::Add(Expr::Number(1).into(), Expr::Number(2).into());
        let res = expr.eval().unwrap();
        assert_eq!(res, 3)
    }

    #[test]
    fn test_sub() {
        let expr = Expr::Sub(Expr::Number(1).into(), Expr::Number(2).into());
        let res = expr.eval().unwrap();
        assert_eq!(res, -1)
    }

    #[test]
    fn test_mul() {
        let expr = Expr::Mul(Expr::Number(3).into(), Expr::Number(2).into());
        let res = expr.eval().unwrap();
        assert_eq!(res, 6)
    }

    #[test]
    fn test_div() {
        let expr = Expr::Div(Expr::Number(3).into(), Expr::Number(2).into());
        let res = expr.eval().unwrap();
        assert_eq!(res, 1)
    }

    #[test]
    fn test_number() {
        let expr = Expr::Number(123);
        let res = expr.eval().unwrap();
        assert_eq!(res, 123)
    }

    #[test]
    fn test_sqrt() {
        let expr = Expr::Sqr(Expr::Number(4).into());
        let res = expr.eval().unwrap();
        assert_eq!(res, 16)
    }

    #[test]
    fn test_div_zero() {
        let expr = Expr::Div(Expr::Number(-1).into(), Expr::Number(0).into());
        let res = expr.eval();
        assert_eq!(res, Err(EvalError::DivisionByZero))
    }

    #[test]
    fn test_complicated() {
        let expr = Expr::Add(
            Expr::Mul(Expr::Number(-1).into(), Expr::Number(2).into()).into(),
            Expr::Sqr(Expr::Number(25).into()).into(),
        );
        let res = expr.eval().unwrap();
        assert_eq!(res, 623)
    }

    #[test]
    fn test_from_str() {
        let expr = Expr::from_str("4 2 + 3 *").unwrap();
        let res = expr.eval().unwrap();
        assert_eq!(res, 18)
    }
}
