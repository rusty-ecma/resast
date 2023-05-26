use crate::spanned::expr::{Expr, Prop};
use crate::spanned::Ident;

use super::{AssignOp, ListEntry, Node, SourceLocation, Position};
/// All of the different ways you can declare an identifier
/// and/or value
#[derive(Debug, Clone, PartialEq)]
pub enum Pat<T> {
    Ident(Ident<T>),
    Obj(ObjPat<T>),
    Array(ArrayPat<T>),
    Assign(AssignPat<T>),
}

// impl<T> From<Pat<T>> for crate::pat::Pat<T> {
//     fn from(other: Pat<T>) -> Self {
//         match other {
//             Pat::Ident(inner) => Self::Ident(inner.into()),
//             Pat::Obj(inner) => Self::Obj(inner.into()),
//             Pat::Array(inner) => Self::Array(inner.into()),
//             Pat::Assign(inner) => Self::Assign(inner.into()),
//         }
//     }
// }

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
pub struct ArrayPat<T> {
    pub open_bracket: Position,
    pub elements: Vec<ListEntry<Option<ArrayPatPart<T>>>>,
    pub close_bracket: Position,
}

impl<T> Node for ArrayPat<T> {
    fn loc(&self) -> super::SourceLocation {
        SourceLocation {
            start: self.open_bracket,
            end: self.close_bracket+1,
        }
    }
}

// impl<T> From<ArrayPat<T>> for Vec<Option<crate::pat::ArrayPatPart<T>>> {
//     fn from(other: ArrayPat<T>) -> Self {
//         other
//             .elements
//             .into_iter()
//             .map(|e| e.item.map(Into::into))
//             .collect()
//     }
// }

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayElement<T> {
    pub part: Option<ArrayPatPart<T>>,
    pub comma: Option<Position>,
}

// impl<T> From<ArrayElement<T>> for Option<crate::pat::ArrayPatPart<T>> {
//     fn from(other: ArrayElement<T>) -> Self {
//         other.part.map(From::from)
//     }
// }

#[derive(PartialEq, Debug, Clone)]
pub enum ArrayPatPart<T> {
    Pat(Pat<T>),
    Expr(Expr<T>),
    Rest(RestPat<T>),
}

// impl<T> From<ArrayPatPart<T>> for crate::pat::ArrayPatPart<T> {
//     fn from(other: ArrayPatPart<T>) -> Self {
//         match other {
//             ArrayPatPart::Pat(inner) => Self::Pat(inner.into()),
//             ArrayPatPart::Expr(inner) => Self::Expr(inner.into()),
//             ArrayPatPart::Rest(inner) => {
//                 Self::Pat(crate::pat::Pat::RestElement(Box::new(inner.pat.into())))
//             }
//         }
//     }
// }

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
pub struct ObjPat<T> {
    pub open_brace: Position,
    pub props: Vec<ObjEntry<T>>,
    pub close_brace: Position,
}

impl<T> Node for ObjPat<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.open_brace,
            end: self.close_brace+1,
        }
    }
}

// impl<T> From<ObjPat<T>> for crate::pat::ObjPat<T> {
//     fn from(other: ObjPat<T>) -> Self {
//         other.props.into_iter().map(|e| e.item.into()).collect()
//     }
// }

/// A single part of an ObjectPat
#[derive(PartialEq, Debug, Clone)]
pub enum ObjPatPart<T> {
    Assign(Prop<T>),
    Rest(Box<RestPat<T>>),
}

impl<T> Node for ObjPatPart<T> {
    fn loc(&self) -> SourceLocation {
        match self {
            ObjPatPart::Assign(prop) => prop.loc(),
            ObjPatPart::Rest(inner) => inner.loc(),
        }
    }
}

// impl<T> From<ObjPatPart<T>> for crate::pat::ObjPatPart<T> {
//     fn from(other: ObjPatPart<T>) -> Self {
//         match other {
//             ObjPatPart::Assign(prop) => Self::Assign(prop.into()),
//             ObjPatPart::Rest(inner) => Self::Rest(Box::new(From::from(inner.pat))),
//         }
//     }
// }

#[derive(Debug, Clone, PartialEq)]
pub struct RestPat<T> {
    pub dots: Position,
    pub pat: Pat<T>,
}

impl<T> Node for RestPat<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.dots,
            end: self.pat.loc().end,
        }
    }
}

/// An assignment as a pattern
#[derive(Debug, Clone, PartialEq)]
pub struct AssignPat<T> {
    pub left: Box<Pat<T>>,
    pub operator: AssignOp,
    pub right: Box<Expr<T>>,
}

impl<T> Node for AssignPat<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.left.loc().start,
            end: self.right.loc().end,
        }
    }
}

// impl<T> From<AssignPat<T>> for crate::pat::AssignPat<T> {
//     fn from(other: AssignPat<T>) -> Self {
//         Self {
//             left: Box::new(From::from(*other.left)),
//             right: Box::new(From::from(*other.right)),
//         }
//     }
// }
