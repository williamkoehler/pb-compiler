use super::parser::lexer::Token;

pub enum Report {
    Error {
        message: String,
        position: Option<Position>,
        selection: Option<Selection>,
    },
}

impl Report {
    pub fn with_source_code(&self, _source_code: &String) -> String {
        match self {
            Report::Error {
                message,
                position,
                selection: _,
            } => {
                let line1 = if let Some(position) = position {
                    format!("error : {} (Ln {})", message, position.line)
                } else {
                    format!("error : {}", message)
                };

                // let relative_position: usize;
                // let slice = match selection {
                //     &Selection::Offset(offset) => {
                //         relative_position = std::cmp::min(offset - 10, 10);
                //         source_code
                //             .slice(std::ops::Range {
                //                 start: offset - relative_position,
                //                 end: std::cmp::min(offset + 10, source_code.len() - 1),
                //             })
                //             .unwrap()
                //     }
                //     &Selection::Span(start_offset, end_offset) => {
                //         relative_position = std::cmp::min(start_offset - 10, 10);
                //         source_code
                //             .slice(std::ops::Range {
                //                 start: start_offset - relative_position,
                //                 end: std::cmp::min(end_offset + 5, source_code.len() - 1),
                //             })
                //             .unwrap()
                //     }
                // };

                // // Remove newline before selection
                // {
                //     if let Some(index) = slice.find('\n') {
                //         if index < relative_position {
                //             slice = slice.slice(std::ops::Range { start: index + 1 })
                //         }
                //     }
                // }

                // format!("{}\n{}\n{}", line1, line1, line1)
                format!("{}", line1)
            }
        }
    }
}

pub struct Position {
    pub line: usize,
}

impl From<Token<'_>> for Position {
    fn from(token: Token) -> Self {
        Position { line: token.line }
    }
}

pub enum Selection {
    Offset(usize),
    Span(usize, usize),
}

impl From<(usize, usize)> for Selection {
    fn from(span: (usize, usize)) -> Self {
        Selection::Span(span.0, span.1)
    }
}

impl From<usize> for Selection {
    fn from(offset: usize) -> Self {
        Selection::Offset(offset)
    }
}

pub fn internal_error(message: &str) -> Report {
    Report::Error {
        message: message.to_string(),
        position: None,
        selection: None,
    }
}

// ---- General Errors ----

pub fn err_unexp_token(token: Token) -> Report {
    Report::Error {
        message: format!("Unexpected token '{}'", token.kind),
        position: Some(token.into()),
        selection: Some(token.span.into()),
    }
}

pub fn err_exp_semicolon(token: Token) -> Report {
    Report::Error {
        message: "Expected semicolon ';'".to_string(),
        position: Some(token.into()),
        selection: Some(token.span.into()),
    }
}

pub fn err_miss_semicolon(token: Token) -> Report {
    Report::Error {
        message: "Missing semicolon ';'".to_string(),
        position: Some(token.into()),
        selection: Some(token.span.0.into()),
    }
}

pub fn err_exp_colon(token: Token) -> Report {
    Report::Error {
        message: "Expected colon ':'".to_string(),
        position: Some(token.into()),
        selection: Some(token.span.into()),
    }
}

pub fn err_miss_colon(token: Token) -> Report {
    Report::Error {
        message: "Missing colon ':'".to_string(),
        position: Some(token.into()),
        selection: Some(token.span.0.into()),
    }
}

pub fn err_exp_equal(token: Token) -> Report {
    Report::Error {
        message: "Expected equal '='".to_string(),
        position: Some(token.into()),
        selection: Some(token.span.into()),
    }
}

pub fn err_miss_equal(token: Token) -> Report {
    Report::Error {
        message: "Missing equal '='".to_string(),
        position: Some(token.into()),
        selection: Some(token.span.0.into()),
    }
}

pub fn err_exp_rparen(token: Token) -> Report {
    Report::Error {
        message: "Expected equal ')'".to_string(),
        position: Some(token.into()),
        selection: Some(token.span.into()),
    }
}

pub fn err_exp_identifier(token: Token) -> Report {
    Report::Error {
        message: "Expected valid identifier".to_string(),
        position: Some(token.into()),
        selection: Some(token.span.0.into()),
    }
}

pub fn err_miss_identifier(token: Token) -> Report {
    Report::Error {
        message: "Missing valid identifier".to_string(),
        position: Some(token.into()),
        selection: Some(token.span.0.into()),
    }
}

// ---- Body Errors ----

pub fn err_miss_body(token: Token) -> Report {
    Report::Error {
        message: "Missing struct body".to_string(),
        position: Some(token.into()),
        selection: Some(token.span.0.into()),
    }
}

pub fn err_exp_body_lcurly(token: Token) -> Report {
    Report::Error {
        message: "Expected '{'".to_string(),
        position: Some(token.into()),
        selection: Some(token.span.into()),
    }
}

pub fn err_exp_body_rcurly(token: Token) -> Report {
    Report::Error {
        message: "Expected '}'".to_string(),
        position: Some(token.into()),
        selection: Some(token.span.into()),
    }
}

// ---- Expression Errors ----

pub fn err_exp_binary_operator(token: Token) -> Report {
    Report::Error {
        message: "Expected binary operator (+, -, *, /, %, etc...)".to_string(),
        position: Some(token.into()),
        selection: Some(token.span.into()),
    }
}

// ---- Field Errors ----

pub fn err_exp_field_type(token: Token) -> Report {
    Report::Error {
        message: "Expected field type".to_string(),
        position: Some(token.into()),
        selection: Some(token.span.into()),
    }
}

pub fn err_miss_field_type(token: Token) -> Report {
    Report::Error {
        message: "Missing field type".to_string(),
        position: Some(token.into()),
        selection: Some(token.span.0.into()),
    }
}

// ---- Alias Errors ----

pub fn err_exp_alias_data_type(token: Token) -> Report {
    Report::Error {
        message: "Expected type type".to_string(),
        position: Some(token.into()),
        selection: Some(token.span.into()),
    }
}

pub fn err_miss_alias_data_type(token: Token) -> Report {
    Report::Error {
        message: "Missing type type".to_string(),
        position: Some(token.into()),
        selection: Some(token.span.0.into()),
    }
}

// ---- Semantic Analysis Errors ----

pub fn err_invalid_data_type_identifer(identifier: &str) -> Report {
    Report::Error {
        message: format!(
            "Data type identifier '{}' should use pascal case",
            identifier
        ),
        position: None,
        selection: None,
    }
}

pub fn err_undeclared_data_type(identifier: &str) -> Report {
    Report::Error {
        message: format!("Use of undeclared data type '{}'", identifier),
        position: None,
        selection: None,
    }
}

pub fn err_redefined_data_type(identifier: &str) -> Report {
    Report::Error {
        message: format!("Redefinition of data type '{}'", identifier),
        position: None,
        selection: None,
    }
}

pub fn err_redefined_field(identifier: &str) -> Report {
    Report::Error {
        message: format!("Redefinition of field '{}'", identifier),
        position: None,
        selection: None,
    }
}

pub fn err_invalid_field_identifer(identifier: &str) -> Report {
    Report::Error {
        message: format!(
            "Field identifier '{}' should use snake case and not start with an underscore",
            identifier
        ),
        position: None,
        selection: None,
    }
}

pub fn err_cyclical_dependency(identifiers: &[String]) -> Report {
    Report::Error {
        message: format!(
            "Cyclical dependency between the types {}",
            identifiers.join(", ")
        ),
        position: None,
        selection: None,
    }
}

pub fn err_invalid_expression_operand(
    operator: super::ast::UnaryOperator,
    operands: &super::ast::Value,
) -> Report {
    Report::Error {
        message: format!("Invalid {} operation for {}", operator, operands),
        position: None,
        selection: None,
    }
}

pub fn err_invalid_expression_operands(
    operator: super::ast::BinaryOperator,
    operand_1: &super::ast::Value,
    operand_2: &super::ast::Value,
) -> Report {
    Report::Error {
        message: format!(
            "Invalid {} operation for {} and {}",
            operator, operand_1, operand_2
        ),
        position: None,
        selection: None,
    }
}
