pub mod lexer;

use self::lexer::*;

use super::ast::*;

pub struct Parser<'a> {
    // Input
    lexer: lexer::Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn from(input: &'a str) -> Self {
        Self {
            lexer: Lexer::new(input),
        }
    }

    pub fn parse(mut self, compiler: &mut super::Compiler, file: &mut super::ast::File) {
        loop {
            let token = self.lexer.current();
            match token.kind {
                TokenKind::StructKeyword => self.parse_structure(compiler, file),
                TokenKind::VariantKeyword => self.parse_variant(compiler, file),
                TokenKind::NoToken => break,
                _ => {
                    self.lexer.consume();
                    compiler.diagnose(super::diagnostic::err_unexp_token(token));
                }
            }
        }
    }

    fn parse_structure(&mut self, compiler: &mut super::Compiler, file: &mut super::ast::File) {
        let mut structure = super::ast::Structure::new();

        // Skip struct keyword
        self.lexer.consume();

        // Parse name
        let token = self.lexer.current();
        match token.kind {
            TokenKind::Identifier => {
                self.lexer.consume();
                structure.identifier_mut().set(token.slice.to_string());
            }
            _ => compiler.diagnose(super::diagnostic::err_miss_identifier(token)),
        };

        // Parse body
        let token = self.lexer.current();
        match token.kind {
            TokenKind::LCurly => {
                self.parse_body(compiler, &mut structure);
            }
            TokenKind::LAngle | lexer::TokenKind::LBrack | lexer::TokenKind::LParen => {
                compiler.diagnose(super::diagnostic::err_exp_body_lcurly(token));
                self.parse_body(compiler, &mut structure);
            }
            _ => {
                compiler.diagnose(super::diagnostic::err_miss_body(token));
            }
        }

        file.add_structure(structure);
    }

    fn parse_variant(&mut self, compiler: &mut super::Compiler, file: &mut super::ast::File) {
        let mut variant = super::ast::Variant::new();

        // Skip struct keyword
        self.lexer.next();

        // Parse name
        let token = self.lexer.current();
        match token.kind {
            lexer::TokenKind::Identifier => {
                self.lexer.consume();
                variant.identifier_mut().set(token.slice.to_string());
            }
            lexer::TokenKind::LCurly | lexer::TokenKind::Semicolon => {
                compiler.diagnose(super::diagnostic::err_miss_identifier(token));
            }
            lexer::TokenKind::TrueKeyword
            | TokenKind::FalseKeyword
            | TokenKind::OptKeyword
            | TokenKind::VarKeyword
            | TokenKind::StructKeyword
            | TokenKind::VariantKeyword => {
                self.lexer.consume();
                compiler.diagnose(super::diagnostic::err_exp_identifier(token));
            }
            _ => compiler.diagnose(super::diagnostic::err_miss_identifier(token)),
        };

        // Parse body
        let token = self.lexer.current();
        match token.kind {
            lexer::TokenKind::LCurly => {
                self.parse_body(compiler, &mut variant);
            }
            lexer::TokenKind::LAngle | lexer::TokenKind::LBrack | lexer::TokenKind::LParen => {
                compiler.diagnose(super::diagnostic::err_exp_body_lcurly(token));
                self.parse_body(compiler, &mut variant);
            }
            _ => {
                compiler.diagnose(super::diagnostic::err_miss_body(token));
            }
        }

        file.add_variant(variant);
    }

    fn parse_body<T: super::ast::Fielded + super::ast::Optioned>(
        &mut self,
        compiler: &mut super::Compiler,
        output: &mut T,
    ) {
        // Skip {
        self.lexer.consume();

        loop {
            let token = self.lexer.current();
            match token.kind {
                lexer::TokenKind::VarKeyword => self.parse_field(compiler, output),
                lexer::TokenKind::OptKeyword => self.parse_option(compiler, output),
                lexer::TokenKind::Semicolon => {
                    self.lexer.consume();
                }
                lexer::TokenKind::RAngle
                | lexer::TokenKind::RBrack
                | lexer::TokenKind::RParen
                | lexer::TokenKind::NoToken => {
                    self.lexer.consume();
                    compiler.diagnose(super::diagnostic::err_exp_body_rcurly(token));
                    break;
                }
                lexer::TokenKind::RCurly => {
                    self.lexer.consume();
                    break;
                }
                _ => {
                    self.lexer.consume();
                    compiler.diagnose(super::diagnostic::err_unexp_token(token));
                }
            }
        }
    }

    fn parse_field(
        &mut self,
        compiler: &mut super::Compiler,
        output: &mut dyn super::ast::Fielded,
    ) {
        let mut field = super::ast::Field::new();

        // Skip var keyword
        self.lexer.consume();

        // Parse name
        let token = self.lexer.current();
        match token.kind {
            lexer::TokenKind::Identifier => {
                self.lexer.consume();
                field.identifier_mut().set(token.slice.to_string());
            }
            lexer::TokenKind::Colon | lexer::TokenKind::Eq => {
                compiler.diagnose(super::diagnostic::err_miss_identifier(token));
            }
            lexer::TokenKind::TrueKeyword
            | TokenKind::FalseKeyword
            | TokenKind::OptKeyword
            | TokenKind::VarKeyword
            | TokenKind::StructKeyword
            | TokenKind::VariantKeyword => {
                self.lexer.consume();
                compiler.diagnose(super::diagnostic::err_exp_identifier(token));
            }
            _ => {
                compiler.diagnose(super::diagnostic::err_exp_identifier(token));
            }
        };

        // Check colon
        let token = self.lexer.current();
        match token.kind {
            lexer::TokenKind::Colon => {
                self.lexer.consume();
            }
            lexer::TokenKind::Identifier => {
                compiler.diagnose(super::diagnostic::err_miss_colon(token));
            }
            _ => {
                compiler.diagnose(super::diagnostic::err_exp_colon(token));
            }
        };

        // Parse type
        let token = self.lexer.current();
        match token.kind {
            lexer::TokenKind::Identifier => {
                self.lexer.consume();
                field.reference_mut().set(token.slice.to_string());
            }
            lexer::TokenKind::Semicolon => {
                compiler.diagnose(super::diagnostic::err_miss_field_type(token));
            }
            _ => {
                compiler.diagnose(super::diagnostic::err_exp_field_type(token));
            }
        };

        // Check semicolon
        let token = self.lexer.current();
        match token.kind {
            lexer::TokenKind::Semicolon => {
                self.lexer.consume();
            }
            _ => {
                compiler.diagnose(super::diagnostic::err_exp_semicolon(token));
            }
        };

        output.add_field(field);
    }

    fn parse_option(&mut self, compiler: &mut super::Compiler, output: &mut dyn Optioned) {
        let mut name = None;
        let mut arguments = Vec::new();

        // Skip opt keyword
        self.lexer.consume();

        // Parse name
        let token = self.lexer.current();
        match token.kind {
            lexer::TokenKind::Identifier => {
                self.lexer.consume();
                name = Some(token.slice.to_string());
            }
            lexer::TokenKind::Colon | lexer::TokenKind::Eq => {
                compiler.diagnose(super::diagnostic::err_miss_identifier(token));
            }
            lexer::TokenKind::TrueKeyword
            | TokenKind::FalseKeyword
            | TokenKind::OptKeyword
            | TokenKind::VarKeyword
            | TokenKind::StructKeyword
            | TokenKind::VariantKeyword => {
                self.lexer.consume();
                compiler.diagnose(super::diagnostic::err_exp_identifier(token));
            }
            _ => {
                compiler.diagnose(super::diagnostic::err_exp_identifier(token));
            }
        };

        // Parse arguments
        let token = self.lexer.current();
        match token.kind {
            TokenKind::LParen => {
                self.lexer.consume();

                loop {
                    // Handle left parenthesis after comma
                    if self.lexer.current().kind == TokenKind::RParen {
                        break;
                    }

                    let argument = self.parse_expression(compiler, u8::MAX, |t| {
                        t == TokenKind::Comma || t == TokenKind::RParen
                    });

                    arguments.push(argument);

                    let token = self.lexer.current();
                    match token.kind {
                        TokenKind::Comma => {
                            self.lexer.consume();
                            continue;
                        }
                        TokenKind::RParen => {
                            self.lexer.consume();
                            break;
                        }
                        _ => {
                            compiler.diagnose(super::diagnostic::err_exp_rparen(token));
                            break;
                        }
                    }
                }
            }
            TokenKind::Eq => {
                arguments.push(
                    self.parse_expression(compiler, u8::MAX, |t| t == lexer::TokenKind::Semicolon),
                );
            }
            TokenKind::Semicolon => {
                arguments.push(super::ast::Expression::Value(super::ast::Value::Null))
            }
            lexer::TokenKind::TrueKeyword
            | lexer::TokenKind::FalseKeyword
            | lexer::TokenKind::Integer(_)
            | lexer::TokenKind::Real(_)
            | lexer::TokenKind::Literal
            | lexer::TokenKind::Identifier => {
                compiler.diagnose(super::diagnostic::err_miss_equal(token));
            }
            _ => {
                compiler.diagnose(super::diagnostic::err_exp_equal(token));
            }
        }

        // Check semicolon
        let token = self.lexer.current();
        match token.kind {
            lexer::TokenKind::Semicolon => {
                self.lexer.consume();
            }
            _ => {
                compiler.diagnose(super::diagnostic::err_exp_semicolon(token));
            }
        };

        if let Some(name) = name {
            output.add_option(name, arguments);
        }
    }

    fn parse_expression(
        &mut self,
        compiler: &mut super::Compiler,
        max_precedence: u8,
        is_end: fn(token_kind: TokenKind) -> bool,
    ) -> Expression {
        let token = self.lexer.current();
        let mut expression = match token.kind {
            TokenKind::Identifier => {
                self.lexer.consume();
                let name = token.slice.to_string();

                let token = self.lexer.current();
                if token.kind == TokenKind::LParen {
                    self.lexer.consume();

                    let mut arguments = Vec::new();

                    loop {
                        // Handle left parenthesis after comma
                        if self.lexer.current().kind == TokenKind::LParen {
                            break;
                        }

                        let argument = self.parse_expression(compiler, u8::MAX, |t| {
                            t == TokenKind::Comma || t == TokenKind::RParen
                        });

                        arguments.push(argument);

                        let token = self.lexer.current();
                        match token.kind {
                            TokenKind::Comma => {
                                self.lexer.consume();
                                continue;
                            }
                            TokenKind::RParen => {
                                self.lexer.consume();
                                break;
                            }
                            _ => {
                                compiler.diagnose(super::diagnostic::err_exp_rparen(token));
                                break;
                            }
                        }
                    }

                    Expression::Call(name, arguments)
                } else {
                    Expression::Variable(Identifier::from(name))
                }
            }

            TokenKind::NullKeyword => {
                self.lexer.consume();
                super::ast::Expression::Value(super::ast::Value::Null)
            }
            TokenKind::TrueKeyword => {
                self.lexer.consume();
                super::ast::Expression::Value(super::ast::Value::True)
            }
            TokenKind::FalseKeyword => {
                self.lexer.consume();
                super::ast::Expression::Value(super::ast::Value::False)
            }
            TokenKind::Integer(integer) => {
                self.lexer.consume();
                super::ast::Expression::Value(super::ast::Value::Integer(integer))
            }
            TokenKind::Real(real) => {
                self.lexer.consume();
                super::ast::Expression::Value(super::ast::Value::Real(real))
            }
            TokenKind::Literal => {
                self.lexer.consume();
                super::ast::Expression::Value(super::ast::Value::Literal(token.slice.to_string()))
            }

            TokenKind::Minus => {
                self.lexer.consume();
                let expression = self.parse_expression(compiler, 1, is_end);
                Expression::UnaryOperator(UnaryOperator::Negation, Box::new(expression))
            }
            TokenKind::Plus => {
                self.lexer.consume();
                self.parse_expression(compiler, 1, is_end)
            }
            TokenKind::Bang => {
                self.lexer.consume();
                let expression = self.parse_expression(compiler, 1, is_end);
                Expression::UnaryOperator(UnaryOperator::LogicalNot, Box::new(expression))
            }
            _ => super::ast::Expression::Value(super::ast::Value::Null),
        };

        loop {
            let token = self.lexer.current();

            if is_end(token.kind) {
                break;
            }

            let operator = match token.kind {
                TokenKind::Star => (BinaryOperator::Multiplication, 2),
                TokenKind::Slash => (BinaryOperator::Division, 2),
                TokenKind::Percent => (BinaryOperator::Modulo, 2),
                TokenKind::Plus => (BinaryOperator::Addition, 3),
                TokenKind::Minus => (BinaryOperator::Subtraction, 3),
                TokenKind::RAngle => (BinaryOperator::GreaterThan, 4),
                TokenKind::GtEq => (BinaryOperator::GreaterThanEqual, 4),
                TokenKind::LAngle => (BinaryOperator::LessThan, 4),
                TokenKind::LtEq => (BinaryOperator::LessThanEqual, 4),
                TokenKind::Eq2 => (BinaryOperator::Equal, 5),
                TokenKind::NEq => (BinaryOperator::NotEqual, 5),
                TokenKind::Amp2 => (BinaryOperator::LogicalAnd, 6),
                TokenKind::Pipe2 => (BinaryOperator::LogicalOr, 7),
                _ => {
                    compiler.diagnose(super::diagnostic::err_exp_binary_operator(token));
                    break;
                }
            };

            if operator.1 > max_precedence {
                break;
            }

            // Consume operator
            self.lexer.consume();

            let right_expression = self.parse_expression(compiler, operator.1 - 1, is_end);
            expression = Expression::BinaryOperator(
                Box::new(expression),
                operator.0,
                Box::new(right_expression),
            );
        }

        expression
    }
}
