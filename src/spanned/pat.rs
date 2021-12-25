use crate::spanned::expr::{Expr, Prop};
use crate::spanned::Ident;

use super::{Node, Slice, SourceLocation};
/// All of the different ways you can declare an identifier
/// and/or value
#[derive(Debug, Clone, PartialEq)]
pub enum Pat<'a> {
    Ident(Ident<'a>),
    Obj(ObjPat<'a>),
    Array(ArrayPat<'a>),
    Assign(AssignPat<'a>),
}

impl<'a> From<Pat<'a>> for crate::pat::Pat<'a> {
    fn from(other: Pat<'a>) -> Self {
        match other {
            Pat::Ident(inner) => Self::Ident(inner.into()),
            Pat::Obj(inner) => Self::Obj(inner.into()),
            Pat::Array(inner) => Self::Array(inner.into()),
            Pat::Assign(inner) => Self::Assign(inner.into()),
        }
    }
}

impl<'a> Node for Pat<'a> {
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
pub struct ArrayPat<'a> {
    pub open_bracket: Slice<'a>,
    pub elements: Vec<ArrayElement<'a>>,
    pub close_bracket: Slice<'a>,
}

impl<'a> Node for ArrayPat<'a> {
    fn loc(&self) -> super::SourceLocation {
        SourceLocation {
            start: self.open_bracket.loc.start,
            end: self.close_bracket.loc.end,
        }
    }
}

impl<'a> From<ArrayPat<'a>> for Vec<Option<crate::pat::ArrayPatPart<'a>>> {
    fn from(other: ArrayPat<'a>) -> Self {
        other.elements.into_iter().map(From::from).collect()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayElement<'a> {
    pub part: Option<ArrayPatPart<'a>>,
    pub comma: Option<Slice<'a>>,
}

impl<'a> From<ArrayElement<'a>> for Option<crate::pat::ArrayPatPart<'a>> {
    fn from(other: ArrayElement<'a>) -> Self {
        other.part.map(From::from)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum ArrayPatPart<'a> {
    Pat(Pat<'a>),
    Expr(Expr<'a>),
    Rest(RestPat<'a>),
}

impl<'a> From<ArrayPatPart<'a>> for crate::pat::ArrayPatPart<'a> {
    fn from(other: ArrayPatPart<'a>) -> Self {
        match other {
            ArrayPatPart::Pat(inner) => Self::Pat(inner.into()),
            ArrayPatPart::Expr(inner) => Self::Expr(inner.into()),
            ArrayPatPart::Rest(inner) => {
                Self::Pat(crate::pat::Pat::RestElement(Box::new(inner.pat.into())))
            }
        }
    }
}

impl<'a> Node for ArrayPatPart<'a> {
    fn loc(&self) -> SourceLocation {
        match self {
            ArrayPatPart::Pat(inner) => inner.loc(),
            ArrayPatPart::Expr(inner) => inner.loc(),
            ArrayPatPart::Rest(inner) => inner.loc(),
        }
    }
}

/// similar to an `ObjectExpr`
#[derive(PartialEq, Debug, Clone)]
pub struct ObjPat<'a> {
    pub open_brace: Slice<'a>,
    pub props: Vec<ObjPatPart<'a>>,
    pub close_brace: Slice<'a>,
}

impl<'a> Node for ObjPat<'a> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.open_brace.loc.start,
            end: self.close_brace.loc.end,
        }
    }
}

impl<'a> From<ObjPat<'a>> for crate::pat::ObjPat<'a> {
    fn from(other: ObjPat<'a>) -> Self {
        other.props.into_iter().map(From::from).collect()
    }
}

/// A single part of an ObjectPat
#[derive(PartialEq, Debug, Clone)]
pub enum ObjPatPart<'a> {
    Assign {
        prop: Prop<'a>,
        comma: Option<Slice<'a>>,
    },
    Rest(Box<RestPat<'a>>),
}

impl<'a> Node for ObjPatPart<'a> {
    fn loc(&self) -> SourceLocation {
        match self {
            ObjPatPart::Assign { prop, comma } => {
                if let Some(slice) = comma {
                    SourceLocation {
                        start: prop.loc().start,
                        end: slice.loc.end,
                    }
                } else {
                    prop.loc()
                }
            }
            ObjPatPart::Rest(inner) => inner.loc(),
        }
    }
}

impl<'a> From<ObjPatPart<'a>> for crate::pat::ObjPatPart<'a> {
    fn from(other: ObjPatPart<'a>) -> Self {
        match other {
            ObjPatPart::Assign { prop, comma: _ } => Self::Assign(prop.into()),
            ObjPatPart::Rest(inner) => Self::Rest(Box::new(From::from(inner.pat))),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RestPat<'a> {
    pub dots: Slice<'a>,
    pub pat: Pat<'a>,
}

impl<'a> Node for RestPat<'a> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.dots.loc.start,
            end: self.pat.loc().end,
        }
    }
}

/// An assignment as a pattern
#[derive(Debug, Clone, PartialEq)]
pub struct AssignPat<'a> {
    pub left: Box<Pat<'a>>,
    pub eq: Slice<'a>,
    pub right: Box<Expr<'a>>,
}

impl<'a> Node for AssignPat<'a> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.left.loc().start,
            end: self.right.loc().end,
        }
    }
}

impl<'a> From<AssignPat<'a>> for crate::pat::AssignPat<'a> {
    fn from(other: AssignPat<'a>) -> Self {
        Self {
            left: Box::new(From::from(*other.left)),
            right: Box::new(From::from(*other.right)),
        }
    }
}
