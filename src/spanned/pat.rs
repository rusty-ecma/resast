use crate::spanned::expr::{Expr, Prop};
use crate::spanned::Ident;
use crate::IntoAllocated;

use super::tokens::{CloseBrace, CloseBracket, Comma, Ellipsis, OpenBrace, OpenBracket, Token};
use super::{AssignOp, ListEntry, Node, SourceLocation};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// All of the different ways you can declare an identifier
/// and/or value
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Pat<T> {
    Ident(Ident<T>),
    Obj(ObjPat<T>),
    Array(ArrayPat<T>),
    Assign(AssignPat<T>),
}

impl<T> IntoAllocated for Pat<T>
where
    T: ToString,
{
    type Allocated = Pat<String>;
    fn into_allocated(self) -> Self::Allocated {
        match self {
            Pat::Ident(inner) => Pat::Ident(inner.into_allocated()),
            Pat::Obj(inner) => Pat::Obj(inner.into_allocated()),
            Pat::Array(inner) => Pat::Array(inner.into_allocated()),
            Pat::Assign(inner) => Pat::Assign(inner.into_allocated()),
        }
    }
}

impl<T> Node for Pat<T> {
    fn loc(&self) -> super::SourceLocation {
        match self {
            Pat::Ident(inner) => inner.loc(),
            Pat::Obj(inner) => inner.loc(),
            Pat::Array(inner) => inner.loc(),
            Pat::Assign(inner) => inner.loc(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ArrayPat<T> {
    pub open_bracket: OpenBracket,
    pub elements: Vec<ListEntry<Option<ArrayPatPart<T>>>>,
    pub close_bracket: CloseBracket,
}

impl<T> IntoAllocated for ArrayPat<T>
where
    T: ToString,
{
    type Allocated = ArrayPat<String>;
    fn into_allocated(self) -> Self::Allocated {
        ArrayPat {
            open_bracket: self.open_bracket,
            elements: self
                .elements
                .into_iter()
                .map(IntoAllocated::into_allocated)
                .collect(),
            close_bracket: self.close_bracket,
        }
    }
}

impl<T> Node for ArrayPat<T> {
    fn loc(&self) -> super::SourceLocation {
        SourceLocation {
            start: self.open_bracket.start(),
            end: self.close_bracket.end(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ArrayElement<T> {
    pub part: Option<ArrayPatPart<T>>,
    pub comma: Option<Comma>,
}

impl<T> IntoAllocated for ArrayElement<T>
where
    T: ToString,
{
    type Allocated = ArrayElement<String>;
    fn into_allocated(self) -> Self::Allocated {
        ArrayElement {
            part: self.part.into_allocated(),
            comma: self.comma,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum ArrayPatPart<T> {
    Pat(Pat<T>),
    Expr(Expr<T>),
    Rest(RestPat<T>),
}

impl<T> IntoAllocated for ArrayPatPart<T>
where
    T: ToString,
{
    type Allocated = ArrayPatPart<String>;
    fn into_allocated(self) -> Self::Allocated {
        match self {
            ArrayPatPart::Pat(inner) => ArrayPatPart::Pat(inner.into_allocated()),
            ArrayPatPart::Expr(inner) => ArrayPatPart::Expr(inner.into_allocated()),
            ArrayPatPart::Rest(inner) => ArrayPatPart::Rest(inner.into_allocated()),
        }
    }
}

impl<T> Node for ArrayPatPart<T> {
    fn loc(&self) -> SourceLocation {
        match self {
            ArrayPatPart::Pat(inner) => inner.loc(),
            ArrayPatPart::Expr(inner) => inner.loc(),
            ArrayPatPart::Rest(inner) => inner.loc(),
        }
    }
}

type ObjEntry<T> = ListEntry<ObjPatPart<T>>;

/// similar to an `ObjectExpr`
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ObjPat<T> {
    pub open_brace: OpenBrace,
    pub props: Vec<ObjEntry<T>>,
    pub close_brace: CloseBrace,
}

impl<T> IntoAllocated for ObjPat<T>
where
    T: ToString,
{
    type Allocated = ObjPat<String>;
    fn into_allocated(self) -> Self::Allocated {
        ObjPat {
            open_brace: self.open_brace,
            props: self
                .props
                .into_iter()
                .map(IntoAllocated::into_allocated)
                .collect(),
            close_brace: self.close_brace,
        }
    }
}

impl<T> Node for ObjPat<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.open_brace.start(),
            end: self.close_brace.end(),
        }
    }
}

/// A single part of an ObjectPat
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum ObjPatPart<T> {
    Assign(Prop<T>),
    Rest(Box<RestPat<T>>),
}

impl<T> IntoAllocated for ObjPatPart<T>
where
    T: ToString,
{
    type Allocated = ObjPatPart<String>;
    fn into_allocated(self) -> Self::Allocated {
        match self {
            ObjPatPart::Assign(inner) => ObjPatPart::Assign(inner.into_allocated()),
            ObjPatPart::Rest(inner) => ObjPatPart::Rest(inner.into_allocated()),
        }
    }
}

impl<T> Node for ObjPatPart<T> {
    fn loc(&self) -> SourceLocation {
        match self {
            ObjPatPart::Assign(prop) => prop.loc(),
            ObjPatPart::Rest(inner) => inner.loc(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct RestPat<T> {
    pub dots: Ellipsis,
    pub pat: Pat<T>,
}

impl<T> IntoAllocated for RestPat<T>
where
    T: ToString,
{
    type Allocated = RestPat<String>;
    fn into_allocated(self) -> Self::Allocated {
        RestPat {
            dots: self.dots,
            pat: self.pat.into_allocated(),
        }
    }
}

impl<T> Node for RestPat<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.dots.start(),
            end: self.pat.loc().end,
        }
    }
}

/// An assignment as a pattern
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct AssignPat<T> {
    pub left: Box<Pat<T>>,
    pub operator: AssignOp,
    pub right: Box<Expr<T>>,
}

impl<T> IntoAllocated for AssignPat<T>
where
    T: ToString,
{
    type Allocated = AssignPat<String>;
    fn into_allocated(self) -> Self::Allocated {
        AssignPat {
            left: self.left.into_allocated(),
            operator: self.operator,
            right: self.right.into_allocated(),
        }
    }
}

impl<T> Node for AssignPat<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.left.loc().start,
            end: self.right.loc().end,
        }
    }
}
