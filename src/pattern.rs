use crate::expression::{Expression, Property};
use crate::Identifier;
/// All of the different ways you can declare an identifier
/// and/or value
#[derive(PartialEq, Debug, Clone)]
pub enum Pattern {
    Identifier(Identifier),
    Object(ObjectPattern),
    Array(Vec<Option<ArrayPatternPart>>),
    RestElement(Box<Pattern>),
    Assignment(AssignmentPattern),
}

#[derive(PartialEq, Debug, Clone)]
pub enum ArrayPatternPart {
    Patt(Pattern),
    Expr(Expression),
}

/// similar to an `ObjectExpression`
pub type ObjectPattern = Vec<ObjectPatternPart>;
/// A single part of an ObjectPattern
#[derive(PartialEq, Debug, Clone)]
pub enum ObjectPatternPart {
    Assignment(Property),
    Rest(Box<Pattern>),
}

/// An assignment as a pattern
#[derive(PartialEq, Debug, Clone)]
pub struct AssignmentPattern {
    pub left: Box<Pattern>,
    pub right: Box<Expression>,
}