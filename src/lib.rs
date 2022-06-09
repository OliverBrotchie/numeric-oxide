use rust_decimal::prelude::*;
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone)]
enum OxideErr {
    InvalidString,
    UnmatchedToken(String),
}

impl From<OxideErr> for JsError {
    fn from(val: OxideErr) -> Self {
        let msg = match val {
            OxideErr::InvalidString => "Passed string was invalid".into(),
            OxideErr::UnmatchedToken(s) => {
                format!(
                    "Incorrect number of values were passed for operation: {}",
                    s
                )
            }
        };

        JsError::new(&msg)
    }
}

impl From<rust_decimal::Error> for OxideErr {
    fn from(_: rust_decimal::Error) -> Self {
        OxideErr::InvalidString
    }
}

#[wasm_bindgen]
pub fn oxidate(input: String, precision: Option<u32>) -> Result<String, JsError> {
    set_panic_hook();

    match evaluate(input, precision) {
        Ok(v) => Ok(v),
        Err(e) => Err(e.into()),
    }
}

#[wasm_bindgen]
pub fn oxidate_multiple(input: JsValue, precision: Option<u32>) -> Result<JsValue, JsError> {
    set_panic_hook();

    let mut errors: Vec<OxideErr> = vec![];
    let results: Vec<String> = input
        .into_serde::<Vec<String>>()?
        .into_iter()
        .map(|v| evaluate(v, precision))
        .filter_map(|r| r.map_err(|e| errors.push(e)).ok())
        .collect();

    if !errors.is_empty() {
        return Err(errors[0].clone().into());
    }

    Ok(JsValue::from_serde(&results)?)
}

/// Evaluate a string of functions and decimal arguments
fn evaluate(mut input: String, precision: Option<u32>) -> Result<String, OxideErr> {
    input.retain(|c| !c.is_whitespace());
    let tokens = input
        .split(|c| c == '(' || c == ')' || c == ',')
        .filter(|s| !s.is_empty())
        .rev()
        .collect::<Vec<&str>>();
    let mut stack: Vec<Decimal> = Vec::new();

    for token in tokens {
        // If the token is a number, push it to the stack.
        if let Ok(val) = token.parse::<Decimal>() {
            stack.push(val);
        } else {
            // If the token is a function, pop two values from the stack
            if stack.len() < 2 {
                return Err(OxideErr::UnmatchedToken(token.to_string()));
            }
            match (stack.pop(), stack.pop()) {
                (Some(left), Some(right)) => stack.push(match token {
                    "add" => left + right,
                    "sub" => left - right,
                    "mult" => left * right,
                    "div" => left / right,
                    "mod" => left % right,
                    "pow" => Decimal::powd(&left, right),
                    _ => return Err(OxideErr::InvalidString),
                }),
                _ => return Err(OxideErr::UnmatchedToken(token.to_string())),
            }
        }
    }
    Ok(if let Some(p) = precision {
        stack[0].round_dp(p)
    } else {
        stack[0]
    }
    .to_string())
}

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    console_error_panic_hook::set_once();
}

#[cfg(test)]
mod rust {
    use crate::{evaluate, OxideErr};

    #[test]
    fn simple_add() -> Result<(), OxideErr> {
        assert_eq!(evaluate("add(1,2)".to_string(), None)?, "3");
        Ok(())
    }

    #[test]
    fn add_with_white_space() -> Result<(), OxideErr> {
        assert_eq!(evaluate("  add ( 1, 2 )  ".to_string(), None)?, "3");
        Ok(())
    }

    #[test]
    fn add_right() -> Result<(), OxideErr> {
        assert_eq!(evaluate("add(1,add(1,2.2))".to_string(), None)?, "4.2");
        Ok(())
    }

    #[test]
    fn add_left() -> Result<(), OxideErr> {
        assert_eq!(evaluate("add(add(1,2.2),1)".to_string(), None)?, "4.2");
        Ok(())
    }

    #[test]
    fn sub() -> Result<(), OxideErr> {
        assert_eq!(evaluate("sub(2,2.5)".to_string(), None)?, "-0.5");
        Ok(())
    }

    #[test]
    fn mult() -> Result<(), OxideErr> {
        assert_eq!(evaluate("mult(2,2.5)".to_string(), None)?, "5.0");
        Ok(())
    }

    #[test]
    fn div() -> Result<(), OxideErr> {
        assert_eq!(evaluate("div(10,2)".to_string(), None)?, "5");
        Ok(())
    }

    #[test]
    fn modulo() -> Result<(), OxideErr> {
        assert_eq!(evaluate("mod(10,2)".to_string(), None)?, "0");
        Ok(())
    }

    #[test]
    fn modulo_with_remainder() -> Result<(), OxideErr> {
        assert_eq!(evaluate("mod(5,3)".to_string(), None)?, "2");
        Ok(())
    }

    #[test]
    fn power() -> Result<(), OxideErr> {
        assert_eq!(evaluate("pow(5,2)".to_string(), None)?, "25");
        Ok(())
    }

    #[test]
    fn precision() -> Result<(), OxideErr> {
        assert_eq!(evaluate("add(1,0.5)".to_string(), Some(0))?, "2");
        Ok(())
    }
}
