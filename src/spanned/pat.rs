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
    fn from(_: Pat<'a>) -> Self {
        todo!()
    }
}

impl<'a> Node for Pat<'a> {
    fn loc(&self) -> super::SourceLocation {
        match self {
            Pat::Ident(_) => todo!(),
            Pat::Obj(_) => todo!(),
            Pat::Array(_) => todo!(),
            Pat::Assign(_) => todo!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayPat<'a> {
    pub open_bracket: Slice<'a>,
    pub elements: Vec<Option<ArrayPatPart<'a>>>,
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

#[derive(PartialEq, Debug, Clone)]
pub enum ArrayPatPart<'a> {
    Pat(Pat<'a>),
    Expr(Expr<'a>),
}

impl<'a> Node for ArrayPatPart<'a> {
    fn loc(&self) -> SourceLocation {
        match self {
            ArrayPatPart::Pat(inner) => inner.loc(),
            ArrayPatPart::Expr(inner) => inner.loc(),
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

/// A single part of an ObjectPat
#[derive(PartialEq, Debug, Clone)]
pub enum ObjPatPart<'a> {
    Assign(Prop<'a>),
    Rest(Box<RestPat<'a>>),
}

impl<'a> Node for ObjPatPart<'a> {
    fn loc(&self) -> SourceLocation {
        match self {
            ObjPatPart::Assign(inner) => inner.loc(),
            ObjPatPart::Rest(inner) => inner.loc(),
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
