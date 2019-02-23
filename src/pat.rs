use crate::expr::{Expr, Property};
use crate::Identifier;
/// All of the different ways you can declare an identifier
/// and/or value
#[derive(PartialEq, Debug, Clone)]
pub enum Pat {
    Identifier(Identifier),
    Object(ObjectPat),
    Array(Vec<Option<ArrayPatPart>>),
    RestElement(Box<Pat>),
    Assignment(AssignmentPat),
}

#[derive(PartialEq, Debug, Clone)]
pub enum ArrayPatPart {
    Patt(Pat),
    Expr(Expr),
}

/// similar to an `ObjectExpr`
pub type ObjectPat = Vec<ObjectPatPart>;
/// A single part of an ObjectPat
#[derive(PartialEq, Debug, Clone)]
pub enum ObjectPatPart {
    Assignment(Property),
    Rest(Box<Pat>),
}

/// An assignment as a pattern
#[derive(PartialEq, Debug, Clone)]
pub struct AssignmentPat {
    pub left: Box<Pat>,
    pub right: Box<Expr>,
}