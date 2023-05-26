use crate::spanned::decl::VarDecl;
use crate::spanned::expr::Expr;
use crate::spanned::pat::Pat;
use crate::spanned::VarKind;
use crate::spanned::{Ident, ProgramPart};

use super::decl::VarDecls;
use super::{ListEntry, Node, SourceLocation, Position};

/// A slightly more granular part of an es program than ProgramPart
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt<T> {
    /// Any expression
    Expr {
        expr: Expr<T>,
        semi_colon: Option<Position>,
    },
    /// A collection of program parts wrapped in curly braces
    Block(BlockStmt<T>),
    /// A single semi-colon
    Empty(SourceLocation),
    /// The contextual keyword `debugger`
    Debugger {
        keyword: Position,
        semi_colon: Option<Position>,
    },
    /// A with statement, this puts one object at the top of
    /// the identifier search tree.
    /// > note: this cannot be used in a strict context
    /// ```js
    /// function random() {
    ///     return 0;
    /// }
    /// let rand;
    /// with (Math) {
    ///     rand = floor(random() * 100) + 1;
    /// }
    /// //rand !== 0
    /// ```
    With(WithStmt<T>),
    /// A return statement
    /// ```js
    /// function thing() {
    ///     return 'stuff';
    /// }
    /// function stuff() {
    ///     return;
    /// }
    Return {
        keyword: Position,
        value: Option<Expr<T>>,
        semi_colon: Option<Position>,
    },
    /// A labeled statement
    /// ```js
    /// label: {
    ///     break label;
    /// }
    /// ```
    Labeled(LabeledStmt<T>),
    /// A break statement
    /// ```js
    /// label: {
    ///     break label;
    /// }
    /// while (true) {
    ///     break;
    /// }
    /// ```
    Break {
        keyword: Position,
        label: Option<Ident<T>>,
        semi_colon: Option<Position>,
    },
    /// A short circuit continuation of a loop
    /// ```js
    /// label: while (true) {
    ///     if (Math.floor(Math.random() * 100) > 50) {
    ///         continue;
    ///     } else {
    ///         console.log('too low')
    ///     }
    /// }
    /// ```
    Continue {
        keyword: Position,
        label: Option<Ident<T>>,
        semi_colon: Option<Position>,
    },
    /// An if statement
    /// ```js
    /// if (1 < 2) {
    ///     console.log(Tlways true');
    /// } else {
    ///     console.log('Never true');
    /// }
    /// ```
    If(IfStmt<T>),
    /// A switch statement
    /// ```js
    /// switch (Math.floor(Math.random()) * 10) {
    ///     case 1:
    ///
    ///     break;
    ///     case 2:
    ///     case 3:
    ///     case 4:
    ///         return false;
    ///     default:
    ///         return true;
    /// }
    /// ```
    Switch(SwitchStmt<T>),
    /// A statement that throws an error
    /// ```js
    /// function error() {
    ///     throw 'hahahaha';
    /// }
    ///
    /// function newError() {
    ///     throw new Error('hohoho');
    /// }
    /// ```
    Throw {
        keyword: Position,
        expr: Expr<T>,
        semi_colon: Option<Position>,
    },
    /// A try/catch block
    /// ```js
    /// try {
    ///
    /// } catch (e) {
    ///
    /// } finally {
    ///
    /// }
    /// ```
    Try(TryStmt<T>),
    /// A while loop
    /// ```js
    /// while (false) {
    ///
    /// }
    /// var i = 0;
    /// while (i < 100) {
    ///     if (Math.floor(Math.random() * 100) > 50) {
    ///         i--;
    ///     } else {
    ///         i += 5;
    ///     }
    /// }
    /// ```
    While(WhileStmt<T>),
    /// A while loop that executes its body first
    /// ```js
    /// do {
    ///     console.log(Tt least once')
    /// } while (Math.floor(Math.random() * 100) < 75)
    /// ```
    DoWhile(DoWhileStmt<T>),
    /// A "c-style" for loop
    /// ```js
    /// for (var i = 0; i < 100; i++) console.log(i);
    /// for (;;) {
    ///     console.log('forever!');
    /// }
    /// ```
    For(ForStmt<T>),
    /// A for in statement, this kind of for statement
    /// will extract each key from an indexable thing
    /// ```js
    /// for (var i in [2,3,4,5,6]) {
    ///     console.log(i);
    /// }
    /// //prints 0, 1, 2, 3, 4
    /// for (var k in {a: 'b', c: 'd'}) {
    ///     console.log(k);
    /// }
    /// //prints a, b
    /// ```
    ForIn(ForInStmt<T>),
    /// A for of statement, this kind of for statement
    /// will extract the value from a generator or iterator
    /// ```js
    /// for (var k of [2, 3, 4, 5, 6]) {
    ///     console.log(k);
    /// }
    /// //prints 2, 3, 4, 5, 6
    /// ```
    ForOf(ForOfStmt<T>),
    /// A var statement
    /// ```js
    /// var x;
    /// var x, y = 'huh?';
    /// ```
    Var {
        decls: VarDecls<T>,
        semi_colon: Option<Position>,
    },
}

// impl<T> From<Stmt<T>> for crate::stmt::Stmt<T> {
//     fn from(other: Stmt<T>) -> Self {
//         match other {
//             Stmt::Expr { expr, .. } => Self::Expr(expr.into()),
//             Stmt::Block(inner) => Self::Block(inner.into()),
//             Stmt::Empty(_) => Self::Empty,
//             Stmt::Debugger { .. } => Self::Debugger,
//             Stmt::With(inner) => Self::With(inner.into()),
//             Stmt::Return { value, .. } => Self::Return(value.map(From::from)),
//             Stmt::Labeled(inner) => Self::Labeled(inner.into()),
//             Stmt::Break { label, .. } => Self::Break(label.map(From::from)),
//             Stmt::Continue { label, .. } => Self::Continue(label.map(From::from)),
//             Stmt::If(inner) => Self::If(inner.into()),
//             Stmt::Switch(inner) => Self::Switch(inner.into()),
//             Stmt::Throw { expr, .. } => Self::Throw(expr.into()),
//             Stmt::Try(inner) => Self::Try(inner.into()),
//             Stmt::While(inner) => Self::While(inner.into()),
//             Stmt::DoWhile(inner) => Self::DoWhile(inner.into()),
//             Stmt::For(inner) => Self::For(inner.into()),
//             Stmt::ForIn(inner) => Self::ForIn(inner.into()),
//             Stmt::ForOf(inner) => Self::ForOf(inner.into()),
//             Stmt::Var { decls, .. } => {
//                 Self::Var(decls.decls.into_iter().map(|e| e.item.into()).collect())
//             }
//         }
//     }
// }

impl<T> Node for Stmt<T> {
    fn loc(&self) -> SourceLocation {
        match self {
            Stmt::Expr { expr, semi_colon } => {
                if let Some(semi) = semi_colon {
                    return SourceLocation {
                        start: expr.loc().start,
                        end: *semi+1,
                    };
                }
                expr.loc()
            }
            Stmt::Block(inner) => inner.loc(),
            Stmt::Empty(inner) => *inner,
            Stmt::Debugger {
                keyword,
                semi_colon,
            } => {
                if let Some(semi) = semi_colon {
                    return SourceLocation {
                        start: *keyword,
                        end: *semi+1,
                    };
                }
                SourceLocation {
                    start: *keyword,
                    end: *keyword + 1,
                }
            }
            Stmt::With(inner) => inner.loc(),
            Stmt::Return {
                keyword,
                value,
                semi_colon,
            } => {
                if let Some(semi) = semi_colon {
                    return SourceLocation {
                        start: *keyword,
                        end: *semi+1,
                    };
                }
                if let Some(value) = value {
                    return SourceLocation {
                        start: *keyword,
                        end: value.loc().end,
                    };
                }
                SourceLocation {
                    start: *keyword,
                    end: *keyword + 1,
                }
            }
            Stmt::Labeled(inner) => inner.loc(),
            Stmt::Break {
                keyword,
                label,
                semi_colon,
            } => {
                if let Some(semi_colon) = semi_colon {
                    return SourceLocation {
                        start: *keyword,
                        end: *semi_colon+1,
                    };
                }
                if let Some(label) = label {
                    return SourceLocation {
                        start: *keyword,
                        end: label.loc().end,
                    };
                }
                SourceLocation {
                    start: *keyword,
                    end: *keyword + 1,
                }
            }
            Stmt::Continue {
                keyword,
                label,
                semi_colon,
            } => {
                if let Some(semi_colon) = semi_colon {
                    return SourceLocation {
                        start: *keyword,
                        end: *semi_colon+1,
                    };
                }
                if let Some(label) = label {
                    return SourceLocation {
                        start: *keyword,
                        end: label.loc().end,
                    };
                }
                SourceLocation {
                    start: *keyword,
                    end: *keyword + 1,
                }
            }
            Stmt::If(inner) => inner.loc(),
            Stmt::Switch(inner) => inner.loc(),
            Stmt::Throw {
                keyword,
                expr,
                semi_colon,
            } => {
                if let Some(semi) = semi_colon {
                    return SourceLocation {
                        start: *keyword,
                        end: *semi+1,
                    };
                }
                SourceLocation {
                    start: *keyword,
                    end: expr.loc().end,
                }
            }
            Stmt::Try(inner) => inner.loc(),
            Stmt::While(inner) => inner.loc(),
            Stmt::DoWhile(inner) => inner.loc(),
            Stmt::For(inner) => inner.loc(),
            Stmt::ForIn(inner) => inner.loc(),
            Stmt::ForOf(inner) => inner.loc(),
            Stmt::Var { decls, semi_colon } => {
                if let Some(semi) = semi_colon {
                    return SourceLocation {
                        start: decls.loc().start,
                        end: *semi+1,
                    };
                }
                decls.loc()
            }
        }
    }
}

/// A with statement, this puts one object at the top of
/// the identifier search tree.
/// > note: this cannot be used in a strict context
/// ```js
/// function random() {
///     return 0;
/// }
/// let rand;
/// with (Math) {
///     rand = floor(random() * 100) + 1;
/// }
/// //rand !== 0
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct WithStmt<T> {
    pub keyword: Position,
    pub open_paren: Position,
    pub object: Expr<T>,
    pub close_paren: Position,
    pub body: Box<Stmt<T>>,
}

impl<T> Node for WithStmt<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.keyword,
            end: self.body.loc().end,
        }
    }
}

// impl<T> From<WithStmt<T>> for crate::stmt::WithStmt<T> {
//     fn from(other: WithStmt<T>) -> Self {
//         Self {
//             object: other.object.into(),
//             body: Box::new(From::from(*other.body)),
//         }
//     }
// }

/// A labeled statement
/// ```js
/// label: {
///     break label;
/// }
/// loop: while (true) {
///     break loop;
/// }
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct LabeledStmt<T> {
    pub label: Ident<T>,
    pub colon: Position,
    pub body: Box<Stmt<T>>,
}

impl<T> Node for LabeledStmt<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.label.loc().start,
            end: self.body.loc().end,
        }
    }
}

// impl<T> From<LabeledStmt<T>> for crate::stmt::LabeledStmt<T> {
//     fn from(other: LabeledStmt<T>) -> Self {
//         Self {
//             label: other.label.into(),
//             body: Box::new(From::from(*other.body)),
//         }
//     }
// }

/// An if statement
/// ```js
/// if (1 < 2) {
///     console.log(Tlways true');
/// } else {
///     console.log('Never true');
/// }
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct IfStmt<T> {
    pub keyword: Position,
    pub open_paren: Position,
    pub test: Expr<T>,
    pub close_paren: Position,
    pub consequent: Box<Stmt<T>>,
    pub alternate: Option<Box<ElseStmt<T>>>,
}

impl<T> Node for IfStmt<T> {
    fn loc(&self) -> SourceLocation {
        let start = self.keyword;
        let end = if let Some(alt) = &self.alternate {
            alt.loc().end
        } else {
            self.consequent.loc().end
        };
        SourceLocation { start, end }
    }
}

// impl<T> From<IfStmt<T>> for crate::stmt::IfStmt<T> {
//     fn from(other: IfStmt<T>) -> Self {
//         Self {
//             test: other.test.into(),
//             consequent: Box::new(From::from(*other.consequent)),
//             alternate: other.alternate.map(|s| Box::new(From::from(s.body))),
//         }
//     }
// }

#[derive(PartialEq, Debug, Clone)]
pub struct ElseStmt<T> {
    pub keyword: Position,
    pub body: Stmt<T>,
}

impl<T> Node for ElseStmt<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.keyword,
            end: self.body.loc().end,
        }
    }
}

/// A switch statement
/// ```js
/// switch (Math.floor(Math.random()) * 10) {
///     case 1:
///
///     break;
///     case 2:
///     case 3:
///     case 4:
///         return false;
///     default:
///         return true;
/// }
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct SwitchStmt<T> {
    pub keyword: Position,
    pub open_paren: Position,
    pub discriminant: Expr<T>,
    pub close_paren: Position,
    pub open_brace: Position,
    pub cases: Vec<SwitchCase<T>>,
    pub close_brace: Position,
}

impl<T> Node for SwitchStmt<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.keyword,
            end: self.close_paren+1,
        }
    }
}

// impl<T> From<SwitchStmt<T>> for crate::stmt::SwitchStmt<T> {
//     fn from(other: SwitchStmt<T>) -> Self {
//         Self {
//             discriminant: other.discriminant.into(),
//             cases: other.cases.into_iter().map(From::from).collect(),
//         }
//     }
// }

/// A single case part of a switch statement
#[derive(Debug, Clone, PartialEq)]
pub struct SwitchCase<T> {
    pub keyword: Position,
    pub test: Option<Expr<T>>,
    pub colon: Position,
    pub consequent: Vec<ProgramPart<T>>,
}

impl<T> Node for SwitchCase<T> {
    fn loc(&self) -> SourceLocation {
        let end = if let Some(last) = self.consequent.last() {
            last.loc().end
        } else {
            self.colon+1
        };
        SourceLocation {
            start: self.keyword,
            end,
        }
    }
}

// impl<T> From<SwitchCase<T>> for crate::stmt::SwitchCase<T> {
//     fn from(other: SwitchCase<T>) -> Self {
//         Self {
//             test: other.test.map(From::from),
//             consequent: other.consequent.into_iter().map(From::from).collect(),
//         }
//     }
// }

/// A collection of program parts wrapped in curly braces
#[derive(Debug, Clone, PartialEq)]
pub struct BlockStmt<T> {
    pub open_brace: Position,
    pub stmts: Vec<ProgramPart<T>>,
    pub close_brace: Position,
}

// impl<T> From<BlockStmt<T>> for crate::stmt::BlockStmt<T> {
//     fn from(other: BlockStmt<T>) -> Self {
//         Self(other.stmts.into_iter().map(From::from).collect())
//     }
// }

impl<T> Node for BlockStmt<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.open_brace,
            end: self.close_brace+1,
        }
    }
}

/// A try/catch block
/// ```js
/// try {
///
/// } catch (e) {
///
/// } finally {
///
/// }
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct TryStmt<T> {
    pub keyword: Position,
    pub block: BlockStmt<T>,
    pub handler: Option<CatchClause<T>>,
    pub finalizer: Option<FinallyClause<T>>,
}

impl<T> Node for TryStmt<T> {
    fn loc(&self) -> SourceLocation {
        let end = if let Some(finalizer) = &self.finalizer {
            finalizer.loc().end
        } else if let Some(catch) = &self.handler {
            catch.loc().end
        } else {
            self.block.loc().end
        };
        SourceLocation {
            start: self.keyword,
            end,
        }
    }
}

// impl<T> From<TryStmt<T>> for crate::stmt::TryStmt<T> {
//     fn from(other: TryStmt<T>) -> Self {
//         Self {
//             block: other.block.into(),
//             handler: other.handler.map(From::from),
//             finalizer: other.finalizer.map(From::from),
//         }
//     }
// }

/// The error handling part of a `TryStmt`
#[derive(Debug, Clone, PartialEq)]
pub struct CatchClause<T> {
    pub keyword: Position,
    pub param: Option<CatchArg<T>>,
    pub body: BlockStmt<T>,
}

impl<T> Node for CatchClause<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.keyword,
            end: self.body.loc().end,
        }
    }
}

// impl<T> From<CatchClause<T>> for crate::stmt::CatchClause<T> {
//     fn from(other: CatchClause<T>) -> Self {
//         Self {
//             param: other.param.map(|a| a.param.into()),
//             body: other.body.into(),
//         }
//     }
// }

#[derive(Debug, Clone, PartialEq)]
pub struct CatchArg<T> {
    pub open_paren: Position,
    pub param: Pat<T>,
    pub close_paren: Position,
}

impl<T> Node for CatchArg<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.open_paren,
            end: self.close_paren+1,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FinallyClause<T> {
    pub keyword: Position,
    pub body: BlockStmt<T>,
}

impl<T> Node for FinallyClause<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.keyword,
            end: self.body.loc().end,
        }
    }
}

// impl<T> From<FinallyClause<T>> for crate::stmt::BlockStmt<T> {
//     fn from(other: FinallyClause<T>) -> Self {
//         other.body.into()
//     }
// }

/// A while loop
/// ```js
/// while (false) {
///
/// }
/// var i = 0;
/// while (i < 100) {
///     if (Math.floor(Math.random() * 100) > 50) {
///         i--;
///     } else {
///         i += 5;
///     }
/// }
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct WhileStmt<T> {
    pub keyword: Position,
    pub open_paren: Position,
    pub test: Expr<T>,
    pub close_paren: Position,
    pub body: Box<Stmt<T>>,
}

impl<T> Node for WhileStmt<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.keyword,
            end: self.body.loc().end,
        }
    }
}

// impl<T> From<WhileStmt<T>> for crate::stmt::WhileStmt<T> {
//     fn from(other: WhileStmt<T>) -> Self {
//         Self {
//             test: other.test.into(),
//             body: Box::new(From::from(*other.body)),
//         }
//     }
// }

/// A while loop that executes its body first
/// ```js
/// do {
///     console.log(Tt least once')
/// } while (Math.floor(Math.random() * 100) < 75)
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct DoWhileStmt<T> {
    pub keyword_do: Position,
    pub body: Box<Stmt<T>>,
    pub keyword_while: Position,
    pub open_paren: Position,
    pub test: Expr<T>,
    pub close_paren: Position,
    pub semi_colon: Option<Position>,
}

impl<T> Node for DoWhileStmt<T> {
    fn loc(&self) -> SourceLocation {
        let end = self.semi_colon.unwrap_or(self.close_paren) + 1;
        SourceLocation {
            start: self.keyword_do,
            end,
        }
    }
}

// impl<T> From<DoWhileStmt<T>> for crate::stmt::DoWhileStmt<T> {
//     fn from(other: DoWhileStmt<T>) -> Self {
//         Self {
//             test: other.test.into(),
//             body: Box::new(From::from(*other.body)),
//         }
//     }
// }

/// A "c-style" for loop
/// ```js
/// for (var i = 0; i < 100; i++) console.log(i);
/// for (;;) {
///     console.log('forever!');
/// }
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct ForStmt<T> {
    //32
    pub keyword: Position,
    //32
    pub open_paren: Position,
    //312
    pub init: Option<LoopInit<T>>,
    //32
    pub semi1: Position,
    //312
    pub test: Option<Expr<T>>,
    //32
    pub semi2: Position,
    //312
    pub update: Option<Expr<T>>,
    //32
    pub close_paren: Position,
    //8
    pub body: Box<Stmt<T>>,
}

impl<T> Node for ForStmt<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.keyword,
            end: self.body.loc().end,
        }
    }
}

// impl<T> From<ForStmt<T>> for crate::stmt::ForStmt<T> {
//     fn from(other: ForStmt<T>) -> Self {
//         Self {
//             init: other.init.map(From::from),
//             test: other.test.map(From::from),
//             update: other.update.map(From::from),
//             body: Box::new(From::from(*other.body)),
//         }
//     }
// }

/// The left most triple of a for loops parenthetical
/// ```js
///  //  vvvvvvvvv
/// for (var i = 0;i < 100; i++)
#[derive(Debug, Clone, PartialEq)]
pub enum LoopInit<T> {
    Variable(VarKind, Vec<ListEntry<VarDecl<T>>>),
    Expr(Expr<T>),
}

impl<T> Node for LoopInit<T> {
    fn loc(&self) -> SourceLocation {
        match self {
            LoopInit::Variable(kind, decls) => {
                if let Some(last) = decls.last() {
                    SourceLocation {
                        start: kind.loc().start,
                        end: last.loc().end,
                    }
                } else {
                    kind.loc()
                }
            }
            LoopInit::Expr(inner) => inner.loc(),
        }
    }
}

// impl<T> From<LoopInit<T>> for crate::stmt::LoopInit<T> {
//     fn from(other: LoopInit<T>) -> Self {
//         match other {
//             LoopInit::Expr(inner) => Self::Expr(inner.into()),
//             LoopInit::Variable(kind, decls) => Self::Variable(
//                 kind.into(),
//                 decls.into_iter().map(|e| e.item.into()).collect(),
//             ),
//         }
//     }
// }

/// A for in statement, this kind of for statement
/// will extract each key from an indexable thing
/// ```js
/// for (var i in [2,3,4,5,6]) {
///     console.log(i);
/// }
/// //prints 0, 1, 2, 3, 4
/// for (var k in {a: 'b', c: 'd'}) {
///     console.log(k);
/// }
/// //prints a, b
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct ForInStmt<T> {
    pub keyword_for: Position,
    pub open_paren: Position,
    pub left: LoopLeft<T>,
    pub keyword_in: Position,
    pub right: Expr<T>,
    pub close_paren: Position,
    pub body: Box<Stmt<T>>,
}

impl<T> Node for ForInStmt<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.keyword_for,
            end: self.body.loc().end,
        }
    }
}

// impl<T> From<ForInStmt<T>> for crate::stmt::ForInStmt<T> {
//     fn from(other: ForInStmt<T>) -> Self {
//         Self {
//             left: other.left.into(),
//             right: other.right.into(),
//             body: Box::new(From::from(*other.body)),
//         }
//     }
// }

/// A for of statement, this kind of for statement
/// will extract the value from a generator or iterator
/// ```js
/// for (var k of [2, 3, 4, 5, 6]) {
///     console.log(k);
/// }
/// //prints 2, 3, 4, 5, 6
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct ForOfStmt<T> {
    pub keyword_for: Position,
    pub open_paren: Position,
    pub left: LoopLeft<T>,
    pub keyword_of: Position,
    pub right: Expr<T>,
    pub close_paren: Position,
    pub body: Box<Stmt<T>>,
    pub is_await: bool,
}

impl<T> Node for ForOfStmt<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.keyword_for,
            end: self.body.loc().end,
        }
    }
}

// impl<T> From<ForOfStmt<T>> for crate::stmt::ForOfStmt<T> {
//     fn from(other: ForOfStmt<T>) -> Self {
//         Self {
//             left: other.left.into(),
//             right: other.right.into(),
//             body: Box::new(From::from(*other.body)),
//             is_await: other.is_await,
//         }
//     }
// }

/// The values on the left hand side of the keyword
/// in a for in or for of loop
#[derive(Debug, Clone, PartialEq)]
pub enum LoopLeft<T> {
    Expr(Expr<T>),
    Variable(VarKind, VarDecl<T>),
    Pat(Pat<T>),
}

// impl<T> From<LoopLeft<T>> for crate::stmt::LoopLeft<T> {
//     fn from(other: LoopLeft<T>) -> Self {
//         match other {
//             LoopLeft::Expr(inner) => Self::Expr(inner.into()),
//             LoopLeft::Variable(kind, decl) => Self::Variable(kind.into(), decl.into()),
//             LoopLeft::Pat(inner) => Self::Pat(inner.into()),
//         }
//     }
// }

impl<T> Node for LoopLeft<T> {
    fn loc(&self) -> SourceLocation {
        match self {
            LoopLeft::Expr(inner) => inner.loc(),
            LoopLeft::Variable(inner, decl) => SourceLocation {
                start: inner.loc().start,
                end: decl.loc().end,
            },
            LoopLeft::Pat(inner) => inner.loc(),
        }
    }
}
