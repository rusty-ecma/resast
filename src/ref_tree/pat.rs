use crate::ref_tree::expr::{Expr, Property};
use crate::ref_tree::{AsConcrete, Identifier};
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

impl<'a> AsConcrete<crate::pat::Pat> for Pat<'a> {
    fn as_concrete(&self) -> crate::pat::Pat {
        match self {
            Pat::Identifier(ref i) => crate::pat::Pat::Identifier(String::from(*i)),
            Pat::Object(ref o) => {
                crate::pat::Pat::Object(o.iter().map(|p| p.as_concrete()).collect())
            }
            Pat::Array(ref a) => crate::pat::Pat::Array(
                a.iter()
                    .map(|p| {
                        if let Some(ref i) = p {
                            Some(i.as_concrete())
                        } else {
                            None
                        }
                    })
                    .collect(),
            ),
            Pat::RestElement(ref r) => crate::pat::Pat::RestElement(Box::new(r.as_concrete())),
            Pat::Assignment(ref a) => crate::pat::Pat::Assignment(a.as_concrete()),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum ArrayPatPart<'a> {
    Pat(Pat<'a>),
    Expr(Expr<'a>),
}

impl<'a> AsConcrete<crate::pat::ArrayPatPart> for ArrayPatPart<'a> {
    fn as_concrete(&self) -> crate::pat::ArrayPatPart {
        match self {
            ArrayPatPart::Pat(ref p) => crate::pat::ArrayPatPart::Pat(p.as_concrete()),
            ArrayPatPart::Expr(ref e) => crate::pat::ArrayPatPart::Expr(e.as_concrete()),
        }
    }
}

/// similar to an `ObjectExpr`
pub type ObjectPat<'a> = Vec<ObjectPatPart<'a>>;
/// A single part of an ObjectPat
#[derive(PartialEq, Debug, Clone)]
pub enum ObjectPatPart<'a> {
    Assignment(Property<'a>),
    Rest(Box<Pat<'a>>),
}

impl<'a> AsConcrete<crate::pat::ObjectPatPart> for ObjectPatPart<'a> {
    fn as_concrete(&self) -> crate::pat::ObjectPatPart {
        match self {
            ObjectPatPart::Assignment(ref p) => {
                crate::pat::ObjectPatPart::Assignment(p.as_concrete())
            }
            ObjectPatPart::Rest(ref p) => {
                crate::pat::ObjectPatPart::Rest(Box::new(p.as_concrete()))
            }
        }
    }
}

/// An assignment as a pattern
#[derive(PartialEq, Debug, Clone)]
pub struct AssignmentPat<'a> {
    pub left: Box<Pat<'a>>,
    pub right: Box<Expr<'a>>,
}

impl<'a> AsConcrete<crate::pat::AssignmentPat> for AssignmentPat<'a> {
    fn as_concrete(&self) -> crate::pat::AssignmentPat {
        crate::pat::AssignmentPat {
            left: Box::new(self.left.as_concrete()),
            right: Box::new(self.right.as_concrete()),
        }
    }
}
