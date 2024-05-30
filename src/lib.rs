#![allow(dead_code)]

type BoxedExpr = Box<Expr>;

#[derive(Debug, PartialEq)]
enum Expr {
    Number(f64),
    Add(BoxedExpr, BoxedExpr),
    Sub(BoxedExpr, BoxedExpr),
    Sqrt(BoxedExpr),
    Mul(BoxedExpr, BoxedExpr),
    Div(BoxedExpr, BoxedExpr),
}

#[derive(Debug, PartialEq)]
enum EvalError {
    DivisionByZero,
    SqrtOfNegativeNumber,
}

#[derive(Debug, PartialEq)]
enum ParseError {
    InvalidInput(String),
    WrongArgumentsCount,
    EmptyInput,
}

fn eval(expr: &Expr) -> Result<f64, EvalError> {
    Ok(match expr {
        Expr::Number(x) => *x,
        Expr::Add(x, y) => eval(x)? + eval(y)?,
        Expr::Sub(x, y) => eval(x)? - eval(y)?,
        Expr::Mul(x, y) => eval(x)? * eval(y)?,
        Expr::Div(x, y) => {
            let y = eval(y)?;
            if y == 0f64 {
                return Err(EvalError::DivisionByZero);
            } else {
                eval(x)? / y
            }
        }
        Expr::Sqrt(x) => {
            let x = eval(x)?;
            if x < 0f64 {
                return Err(EvalError::SqrtOfNegativeNumber);
            } else {
                x.sqrt()
            }
        }
    })
}

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
                stack.push(Expr::Sqrt(x.into()))
            }
            _ => {
                let x = word
                    .parse::<f64>()
                    .map_err(|_| ParseError::InvalidInput(word.to_string()))?;
                stack.push(Expr::Number(x));
            }
        }
    }

    stack.pop().ok_or(ParseError::EmptyInput)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_add() {
        let input = "1 1 +";
        let res_p = parse(input).unwrap();
        let res = eval(&res_p).unwrap();
        assert_eq!(res, 2.0);
    }

    #[test]
    fn parse_sub() {
        let input = "2 1 -";
        let res_p = parse(input).unwrap();
        let res = eval(&res_p).unwrap();
        assert_eq!(res, 1.0);
    }

    #[test]
    fn parse_mul() {
        let input = "2 3 *";
        let res_p = parse(input).unwrap();
        let res = eval(&res_p).unwrap();
        assert_eq!(res, 6.0);
    }

    #[test]
    fn parse_div() {
        let input = "4 2 /";
        let res_p = parse(input).unwrap();
        let res = eval(&res_p).unwrap();
        assert_eq!(res, 2.0);
    }

    #[test]
    fn parse_sqrt() {
        let input = "4 sqr";
        let res_p = parse(input).unwrap();
        let res = eval(&res_p).unwrap();
        assert_eq!(res, 2.0);
    }

    #[test]
    fn parse_add_mul() {
        let input = "4 2 + 6 *";
        let res_p = parse(input).unwrap();
        let res = eval(&res_p).unwrap();
        assert_eq!(res, 36.0);
    }

    #[test]
    fn parse_add_mul_sqrt() {
        let input = "4 2 + 6 * sqr";
        let res_p = parse(input).unwrap();
        let res = eval(&res_p).unwrap();
        assert_eq!(res, 6.0);
    }

    #[test]
    fn parse_test() {
        let input = "9 sqr 4 sqr + 25 sqr -";
        let res_p = parse(input).unwrap();
        let res = eval(&res_p).unwrap();
        assert_eq!(res, 0.0);
    }

    #[test]
    fn parse_simple() {
        let input = "1.0";
        let res_p = parse(input).unwrap();
        let res = eval(&res_p).unwrap();
        assert_eq!(res, 1.0);
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
    fn test_add() {
        let expr = Expr::Add(Expr::Number(1.0).into(), Expr::Number(2.0).into());
        let res = eval(&expr).unwrap();
        assert_eq!(res, 3.0)
    }

    #[test]
    fn test_sub() {
        let expr = Expr::Sub(Expr::Number(1.0).into(), Expr::Number(2.0).into());
        let res = eval(&expr).unwrap();
        assert_eq!(res, -1.0)
    }

    #[test]
    fn test_mul() {
        let expr = Expr::Mul(Expr::Number(3.0).into(), Expr::Number(2.0).into());
        let res = eval(&expr).unwrap();
        assert_eq!(res, 6.0)
    }

    #[test]
    fn test_div() {
        let expr = Expr::Div(Expr::Number(3.0).into(), Expr::Number(2.0).into());
        let res = eval(&expr).unwrap();
        assert_eq!(res, 1.5)
    }

    #[test]
    fn test_number() {
        let expr = Expr::Number(123.0);
        let res = eval(&expr).unwrap();
        assert_eq!(res, 123.0)
    }

    #[test]
    fn test_sqrt() {
        let expr = Expr::Sqrt(Expr::Number(16.0).into());
        let res = eval(&expr).unwrap();
        assert_eq!(res, 4.0)
    }

    #[test]
    fn test_div_zero() {
        let expr = Expr::Div(Expr::Number(-1.0).into(), Expr::Number(0.0).into());
        let res = eval(&expr);
        assert_eq!(res, Err(EvalError::DivisionByZero))
    }

    #[test]
    fn test_sqrt_error() {
        let expr = Expr::Sqrt(Expr::Number(-1.0).into());
        let res = eval(&expr);
        assert_eq!(res, Err(EvalError::SqrtOfNegativeNumber))
    }

    #[test]
    fn test_complicated() {
        let expr = Expr::Add(
            Expr::Mul(Expr::Number(-1.0).into(), Expr::Number(2.0).into()).into(),
            Expr::Sqrt(Expr::Number(25.0).into()).into(),
        );
        let res = eval(&expr).unwrap();
        assert_eq!(res, 3.0)
    }
}
