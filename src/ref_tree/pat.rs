use crate::ref_tree::expr::{Expr, Property};
use crate::ref_tree::Identifier;
/// All of the different ways you can declare an identifier
/// and/or value
#[derive(PartialEq, Debug, Clone)]
pub enum Pat<'a> {
    Identifier(Identifier<'a>),
    Object(ObjectPat<'a>),
    Array(Vec<Option<ArrayPatPart<'a>>>),
    RestElement(Box<Pat<'a>>),
    Assignment(AssignmentPat<'a>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum ArrayPatPart<'a> {
    Pat(Pat<'a>),
    Expr(Expr<'a>),
}

/// similar to an `ObjectExpr`
pub type ObjectPat<'a> = Vec<ObjectPatPart<'a>>;
/// A single part of an ObjectPat
#[derive(PartialEq, Debug, Clone)]
pub enum ObjectPatPart<'a> {
    Assignment(Property<'a>),
    Rest(Box<Pat<'a>>),
}

/// An assignment as a pattern
#[derive(PartialEq, Debug, Clone)]
pub struct AssignmentPat<'a> {
    pub left: Box<Pat<'a>>,
    pub right: Box<Expr<'a>>,
}