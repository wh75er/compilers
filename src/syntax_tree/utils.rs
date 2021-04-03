use super::{
    Operations,
};

use std::error;

pub fn extend_concat_op(regex: &String) -> String {
    let mut result = String::from("");

    let regex = String::from(regex) + Operations::TERMINATOR.as_string();

    let regex_chars: Vec<char> = regex.chars().collect();

    let mut escape = false;

    for (i, c) in regex_chars.iter().enumerate() {
        let next_char = regex_chars.get(i+1);

        result += &c.to_string();

        if escape {
            escape = false;
            result += Operations::CONCAT.as_string();
            continue;
        }

        match Operations::from_char(c) {
            Some(v) => match v {
                Operations::REPETITION => {
                    if next_char.map(|v| v.to_string()) != Some(Operations::RBRACKET.as_string().to_string()) {
                        result += Operations::CONCAT.as_string()
                    }
                },
                Operations::ESCAPE => {
                    escape = true;
                    continue;
                },
                _ => ()
            },
            None => match next_char {
                Some(v) => match Operations::from_char(v) {
                    Some(v) => match v {
                        Operations::TERMINATOR => result += Operations::CONCAT.as_string(),
                        Operations::LBRACKET => result += Operations::CONCAT.as_string(),
                        Operations::ESCAPE => result += Operations::CONCAT.as_string(),
                        _ => ()
                    },
                    None => result += Operations::CONCAT.as_string()
                },
                None => ()
            }
        }
    }

    result
}

fn validate_forbidden_chars(regex: &String) -> Result<(), Box<dyn error::Error>> {
    let forbidden_symbols = vec!{
        Operations::CONCAT.as_string(),
        Operations::TERMINATOR.as_string()
    };

    for c in forbidden_symbols.iter() {
        let escaped_pattern = Operations::ESCAPE.as_string().to_string() + c;

        let escaped = regex.matches(c).count() == regex.matches(&escaped_pattern).count();

        if Operations::from_string(c) == Some(Operations::TERMINATOR) &&
            regex.find(c).is_some() || !escaped {

            return Err((String::from("Symbol ") + *c + " is not allowed").into());
        }
    }

    Ok(())
}

fn validate_repeated_op(regex: &String) -> Result<(), Box<dyn error::Error>> {
    let operations = vec!{
        Operations::REPETITION.as_string(),
        Operations::OR.as_string(),
    };

    for op in operations.iter() {
        let repeated_pattern = op.to_string() + op;
        if regex.matches(&repeated_pattern).count() > 0 {
            return Err((String::from("Invalid syntax ") + &repeated_pattern).into());
        }
    }

    Ok(())
}

pub fn validate_regex(regex: &String) -> Result<(), Box<dyn error::Error>> {
    validate_forbidden_chars(regex)?;

    validate_repeated_op(regex)?;

    Ok(())
}
