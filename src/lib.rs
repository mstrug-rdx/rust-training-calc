#![allow(dead_code)]

#[derive(Debug)]
enum Expr {
    Number(f64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Sqrt(Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

#[derive(Debug, PartialEq)]
enum EvalError {
    DivisionByZero,
    SqrtOfNegativeNumber,
}

fn eval(expr: &Expr) -> Result<f64, EvalError> {
    Ok( match expr {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let expr = Expr::Add( Expr::Number(1.0).into(), Expr::Number(2.0).into() );
        let res = eval(&expr).unwrap();
        assert_eq!(res, 3.0)
    }

    #[test]
    fn test_sub() {
        let expr = Expr::Sub( Expr::Number(1.0).into(), Expr::Number(2.0).into() );
        let res = eval(&expr).unwrap();
        assert_eq!(res, -1.0)
    }

    #[test]
    fn test_mul() {
        let expr = Expr::Mul( Expr::Number(3.0).into(), Expr::Number(2.0).into() );
        let res = eval(&expr).unwrap();
        assert_eq!(res, 6.0)
    }

    #[test]
    fn test_div() {
        let expr = Expr::Div( Expr::Number(3.0).into(), Expr::Number(2.0).into() );
        let res = eval(&expr).unwrap();
        assert_eq!(res, 1.5)
    }

    #[test]
    fn test_number() {
        let expr = Expr::Number( 123.0 );
        let res = eval(&expr).unwrap();
        assert_eq!(res, 123.0)
    }

    #[test]
    fn test_sqrt() {
        let expr = Expr::Sqrt( Expr::Number(16.0).into() );
        let res = eval(&expr).unwrap();
        assert_eq!(res, 4.0)
    }

    #[test]
    fn test_div_zero() {
        let expr = Expr::Div( Expr::Number(-1.0).into(), Expr::Number(0.0).into() );
        let res = eval(&expr);
        assert_eq!(res, Err(EvalError::DivisionByZero))
    }

    #[test]
    fn test_sqrt_error() {
        let expr = Expr::Sqrt( Expr::Number(-1.0).into() );
        let res = eval(&expr);
        assert_eq!(res, Err(EvalError::SqrtOfNegativeNumber))
    }

    #[test]
    fn test_complicated() {
        let expr = Expr::Add( Expr::Mul( Expr::Number(-1.0).into(), Expr::Number(2.0).into() ).into(), Expr::Sqrt( Expr::Number(25.0).into() ).into() );
        let res = eval(&expr).unwrap();
        assert_eq!(res, 3.0)
    }
}
