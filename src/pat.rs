use crate::expr::{Expr, Prop};
use crate::{Ident};
/// All of the different ways you can declare an identifier
/// and/or value
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Pat<'a> {
    Ident(Ident<'a>),
    Obj(ObjPat<'a>),
    Array(Vec<Option<ArrayPatPart<'a>>>),
    RestElement(Box<Pat<'a>>),
    Assign(AssignPat<'a>),
}

impl<'a> Pat<'a> {
    pub fn ident_from(s: &'a str) -> Self {
        Pat::Ident(
            Ident::from(s)
        )
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ArrayPatPart<'a> {
    Pat(Pat<'a>),
    Expr(Expr<'a>),
}

/// similar to an `ObjectExpr`
pub type ObjPat<'a> = Vec<ObjPatPart<'a>>;
/// A single part of an ObjectPat
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ObjPatPart<'a> {
    Assign(Prop<'a>),
    Rest(Box<Pat<'a>>),
}

/// An assignment as a pattern
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssignPat<'a> {
    pub left: Box<Pat<'a>>,
    pub right: Box<Expr<'a>>,
}
