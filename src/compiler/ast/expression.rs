use super::Identifier;

pub enum Expression {
    Value(Value),
    Variable(Identifier),
    Call(String, Vec<Expression>),
    UnaryOperator(UnaryOperator, Box<Expression>),
    BinaryOperator(Box<Expression>, BinaryOperator, Box<Expression>),
}

impl Expression {
    pub fn as_value(&self) -> &Value {
        match self {
            Self::Value(value) => value,
            _ => &Value::Null,
        }
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Value(value) => value.fmt(f),
            Self::Variable(identifier) => identifier.fmt(f),
            Self::Call(name, arguments) => {
                write!(f, "{}(", name)?;

                for argument in arguments {
                    argument.fmt(f)?;
                }

                write!(f, ")")?;

                Ok(())
            }
            Self::UnaryOperator(op, expression) => write!(f, "{}{}", op, expression),
            Self::BinaryOperator(expression_1, op, expression_2) => {
                write!(f, "{}{}{}", expression_1, op, expression_2)
            }
        }
    }
}

#[derive(Clone)]
pub enum Value {
    Null,
    True,
    False,
    Integer(i64),
    Real(f64),
    Literal(String),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Null => write!(f, "null"),
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
            Self::Integer(integer) => write!(f, "{}", integer),
            Self::Real(real) => write!(f, "{}", real),
            Self::Literal(literal) => write!(f, "{}", literal),
        }
    }
}

impl Value {
    pub fn is_true(&self) -> bool {
        match self {
            Self::True | Self::Null => true,
            _ => false,
        }
    }

    pub fn is_false(&self) -> bool {
        match self {
            Self::False => true,
            _ => false,
        }
    }
}

#[derive(Clone, Copy)]
pub enum UnaryOperator {
    Negation,

    LogicalNot,
}

impl std::fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Negation => write!(f, "-"),
            Self::LogicalNot => write!(f, "!"),
        }
    }
}

#[derive(Clone, Copy)]
pub enum BinaryOperator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulo,

    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,

    LogicalAnd,
    LogicalOr,
}

impl std::fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Addition => write!(f, "+"),
            Self::Subtraction => write!(f, "-"),
            Self::Multiplication => write!(f, "*"),
            Self::Division => write!(f, "/"),
            Self::Modulo => write!(f, "%"),
            Self::Equal => write!(f, "="),
            Self::NotEqual => write!(f, "!="),
            Self::GreaterThan => write!(f, ">"),
            Self::GreaterThanEqual => write!(f, ">="),
            Self::LessThan => write!(f, "<"),
            Self::LessThanEqual => write!(f, "<="),
            Self::LogicalAnd => write!(f, "&&"),
            Self::LogicalOr => write!(f, "||"),
        }
    }
}
