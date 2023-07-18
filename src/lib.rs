//Execise 1
// Make it compile in unit test
// Run tests
// Hint: Convert Option to Result
fn generate_nametag_text(name: String) -> Result<String, &'static str> {
    if name.is_empty() {
        // Correct the error message to match the test
        Err("`name` was empty; it must be nonempty.")
    } else {
        Ok(format!("Hi! My name is {}", name))
    }
}


// Exercise 2
// Make it compile in unit test
// Run tests
// Hint: &str to integer conversion by using parse method and return Result
use std::num::ParseIntError;

fn parse_number(s: &str) -> Result<i32, CustomParseError> {
    match s.parse::<i32>() {
        Ok(number) => Ok(number),
        Err(err) => {
            let err_msg = format!("invalid digit found in string: '{}'", s);
            Err(CustomParseError::InvalidDigit(err_msg))
        }
    }
}

#[derive(Debug, PartialEq)]
enum CustomParseError {
    InvalidDigit(String),
}


// Exercise 3
// Make it compile in unit test
// Run tests
// Hint: Custom Error
#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        if value > 0 {
            Ok(PositiveNonzeroInteger(value as u64))
        } else if value == 0 {
            Err(CreationError::Zero)
        } else {
            Err(CreationError::Negative)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    /// Test for exercise 1
    #[test]
    fn exercise1_should_work() {
        assert_eq!(
            generate_nametag_text("Beyoncé".into()),
            Ok("Hi! My name is Beyoncé".into())
        );

        assert_eq!(
            generate_nametag_text("".into()),
            // Don't change this line
            Err("`name` was empty; it must be nonempty.".into())
        );
    }

    /// Test for exercise 2
    use error_handling::*;

    #[test]
    fn exercise2_should_work() {
        assert_eq!(parse_number("42"), Ok(42));
    
        // Manually construct the CustomParseError with the desired error message
        let err_msg = "invalid digit found in string: 'invalid'".to_string();
        let expected_err = CustomParseError::InvalidDigit(err_msg);
        assert_eq!(parse_number("invalid"), Err(expected_err));
    }
    


    /// Test for exercise 3
    #[test]
    fn exercise3_should_work() {
        assert!(PositiveNonzeroInteger::new(10).is_ok());
        assert_eq!(
            Err(CreationError::Negative),
            PositiveNonzeroInteger::new(-10)
        );
        assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
    }
}
