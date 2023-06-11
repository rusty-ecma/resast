use crate::expr::{Expr, Prop};
use crate::{Ident, IntoAllocated};

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

/// All of the different ways you can declare an identifier
/// and/or value
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize)
)]
pub enum Pat<T> {
    Ident(Ident<T>),
    Obj(ObjPat<T>),
    Array(Vec<Option<ArrayPatPart<T>>>),
    RestElement(Box<Pat<T>>),
    Assign(AssignPat<T>),
}

impl<T> IntoAllocated for Pat<T> where T: ToString {
    type Allocated = Pat<String>;

    fn into_allocated(self) -> Self::Allocated {
        match self {
            Pat::Ident(inner) => Pat::Ident(inner.into_allocated()),
            Pat::Obj(inner) => Pat::Obj(inner.into_iter().map(|a| a.into_allocated()).collect()),
            Pat::Array(inner) => Pat::Array(inner.into_iter().map(|o| o.map(|a| a.into_allocated())).collect()),
            Pat::RestElement(inner) => Pat::RestElement(inner.into_allocated()),
            Pat::Assign(inner) => Pat::Assign(inner.into_allocated()),
        }
    }
}

impl<T> Pat<T> {
    pub fn ident_from(inner: T) -> Self {
        Self::Ident(Ident { name: inner })
    }
}

#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize)
)]pub enum ArrayPatPart<T> {
    Pat(Pat<T>),
    Expr(Expr<T>),
}

impl<T> IntoAllocated for ArrayPatPart<T> where T: ToString {
    type Allocated = ArrayPatPart<String>;

    fn into_allocated(self) -> Self::Allocated {
        match self {
            ArrayPatPart::Pat(inner) => ArrayPatPart::Pat(inner.into_allocated()),
            ArrayPatPart::Expr(inner) => ArrayPatPart::Expr(inner.into_allocated()),
        }
    }
}

/// similar to an `ObjectExpr`
pub type ObjPat<T> = Vec<ObjPatPart<T>>;
/// A single part of an ObjectPat
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize)
)]pub enum ObjPatPart<T> {
    Assign(Prop<T>),
    Rest(Box<Pat<T>>),
}

impl<T> IntoAllocated for ObjPatPart<T> where T: ToString {
    type Allocated = ObjPatPart<String>;

    fn into_allocated(self) -> Self::Allocated {
        match self {
            ObjPatPart::Assign(inner) => ObjPatPart::Assign(inner.into_allocated()),
            ObjPatPart::Rest(inner) => ObjPatPart::Rest(inner.into_allocated()),
        }
    }
}

/// An assignment as a pattern
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize)
)]
pub struct AssignPat<T> {
    pub left: Box<Pat<T>>,
    pub right: Box<Expr<T>>,
}

impl<T> IntoAllocated for AssignPat<T> where T: ToString {
    type Allocated = AssignPat<String>;

    fn into_allocated(self) -> Self::Allocated {
        AssignPat {
            left: self.left.into_allocated(),
            right: self.right.into_allocated(),
        }
    }
}
