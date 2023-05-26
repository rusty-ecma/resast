use crate::expr::{Expr, Prop};
use crate::Ident;
/// All of the different ways you can declare an identifier
/// and/or value
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    all(feature = "serde", not(feature = "esprima")),
    derive(Deserialize, Serialize)
)]
#[cfg_attr(all(feature = "serde", feature = "esprima"), derive(Deserialize))]
#[cfg_attr(all(feature = "serde", feature = "esprima"), serde(untagged))]
pub enum Pat<T> {
    Ident(Ident<T>),
    Obj(ObjPat<T>),
    Array(Vec<Option<ArrayPatPart<T>>>),
    RestElement(Box<Pat<T>>),
    Assign(AssignPat<T>),
}

// impl<T> Pat<T> {
//     pub fn ident_from(s: T) -> Self {
//         Pat::Ident(Ident::from(s))
//     }
// }

#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
#[cfg_attr(all(feature = "serde", feature = "esprima"), serde(untagged))]
pub enum ArrayPatPart<T> {
    Pat(Pat<T>),
    Expr(Expr<T>),
}

/// similar to an `ObjectExpr`
pub type ObjPat<T> = Vec<ObjPatPart<T>>;
/// A single part of an ObjectPat
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
#[cfg_attr(all(feature = "serde", feature = "esprima"), serde(untagged))]
pub enum ObjPatPart<T> {
    Assign(Prop<T>),
    Rest(Box<Pat<T>>),
}

/// An assignment as a pattern
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    all(feature = "serde", not(feature = "esprima")),
    derive(Deserialize, Serialize)
)]
#[cfg_attr(all(feature = "serde", feature = "esprima"), derive(Deserialize))]
pub struct AssignPat<T> {
    pub left: Box<Pat<T>>,
    pub right: Box<Expr<T>>,
}
