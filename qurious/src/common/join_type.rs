use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum JoinType {
    Left,
    Right,
    Inner,
    Full,
}

impl Display for JoinType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JoinType::Left => write!(f, "Left Join"),
            JoinType::Right => write!(f, "Right Join"),
            JoinType::Inner => write!(f, "Inner Join"),
            JoinType::Full => write!(f, "Full Join"),
        }
    }
}

impl From<sqlparser::ast::JoinType> for JoinType {
    fn from(value: sqlparser::ast::JoinType) -> Self {
        match value {
            sqlparser::ast::JoinType::Inner => JoinType::Inner,
            sqlparser::ast::JoinType::Left => JoinType::Left,
            sqlparser::ast::JoinType::Right => JoinType::Right,
            sqlparser::ast::JoinType::Full => JoinType::Full,
            _ => unimplemented!(),
        }
    }
}
