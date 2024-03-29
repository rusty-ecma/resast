use crate::spanned::decl::VarDecl;
use crate::spanned::expr::Expr;
use crate::spanned::pat::Pat;
use crate::spanned::VarKind;
use crate::spanned::{Ident, ProgramPart};
use crate::IntoAllocated;

use super::decl::VarDecls;
use super::tokens::{
    Break, Catch, CloseBrace, CloseParen, Colon, Continue, Debugger, Do, Else, Finally, For, If,
    In, Of, OpenBrace, OpenParen, Return, Semicolon, Switch, SwitchCaseKeyword, Throw, Token, Try,
    While, With,
};
use super::{ListEntry, Node, SourceLocation};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A slightly more granular part of an es program than ProgramPart
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Stmt<T> {
    /// Any expression
    Expr {
        expr: Expr<T>,
        semi_colon: Option<Semicolon>,
    },
    /// A collection of program parts wrapped in curly braces
    Block(BlockStmt<T>),
    /// A single semi-colon
    Empty(Semicolon),
    /// The contextual keyword `debugger`
    Debugger {
        keyword: Debugger,
        semi_colon: Option<Semicolon>,
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
        keyword: Return,
        value: Option<Expr<T>>,
        semi_colon: Option<Semicolon>,
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
        keyword: Break,
        label: Option<Ident<T>>,
        semi_colon: Option<Semicolon>,
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
        keyword: Continue,
        label: Option<Ident<T>>,
        semi_colon: Option<Semicolon>,
    },
    /// An if statement
    /// ```js
    /// if (1 < 2) {
    ///     console.log('Always true');
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
        keyword: Throw,
        expr: Expr<T>,
        semi_colon: Option<Semicolon>,
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
        semi_colon: Option<Semicolon>,
    },
}

impl<T> IntoAllocated for Stmt<T>
where
    T: ToString,
{
    type Allocated = Stmt<String>;
    fn into_allocated(self) -> Self::Allocated {
        match self {
            Stmt::Expr { expr, semi_colon } => Stmt::Expr {
                expr: expr.into_allocated(),
                semi_colon,
            },
            Stmt::Block(inner) => Stmt::Block(inner.into_allocated()),
            Stmt::Empty(inner) => Stmt::Empty(inner),
            Stmt::Debugger {
                keyword,
                semi_colon,
            } => Stmt::Debugger {
                keyword,
                semi_colon,
            },
            Stmt::With(inner) => Stmt::With(inner.into_allocated()),
            Stmt::Return {
                keyword,
                value,
                semi_colon,
            } => Stmt::Return {
                keyword,
                value: value.into_allocated(),
                semi_colon,
            },
            Stmt::Labeled(inner) => Stmt::Labeled(inner.into_allocated()),
            Stmt::Break {
                keyword,
                label,
                semi_colon,
            } => Stmt::Break {
                keyword,
                label: label.into_allocated(),
                semi_colon,
            },
            Stmt::Continue {
                keyword,
                label,
                semi_colon,
            } => Stmt::Continue {
                keyword,
                label: label.into_allocated(),
                semi_colon,
            },
            Stmt::If(inner) => Stmt::If(inner.into_allocated()),
            Stmt::Switch(inner) => Stmt::Switch(inner.into_allocated()),
            Stmt::Throw {
                keyword,
                expr,
                semi_colon,
            } => Stmt::Throw {
                keyword,
                expr: expr.into_allocated(),
                semi_colon,
            },
            Stmt::Try(inner) => Stmt::Try(inner.into_allocated()),
            Stmt::While(inner) => Stmt::While(inner.into_allocated()),
            Stmt::DoWhile(inner) => Stmt::DoWhile(inner.into_allocated()),
            Stmt::For(inner) => Stmt::For(inner.into_allocated()),
            Stmt::ForIn(inner) => Stmt::ForIn(inner.into_allocated()),
            Stmt::ForOf(inner) => Stmt::ForOf(inner.into_allocated()),
            Stmt::Var { decls, semi_colon } => Stmt::Var {
                decls: decls.into_allocated(),
                semi_colon,
            },
        }
    }
}

impl<T> Node for Stmt<T> {
    fn loc(&self) -> SourceLocation {
        match self {
            Stmt::Expr { expr, semi_colon } => {
                if let Some(semi) = semi_colon {
                    return SourceLocation {
                        start: expr.loc().start,
                        end: semi.end(),
                    };
                }
                expr.loc()
            }
            Stmt::Block(inner) => inner.loc(),
            Stmt::Empty(inner) => inner.loc(),
            Stmt::Debugger {
                keyword,
                semi_colon,
            } => {
                if let Some(semi) = semi_colon {
                    return SourceLocation {
                        start: keyword.start(),
                        end: semi.end(),
                    };
                }
                keyword.loc()
            }
            Stmt::With(inner) => inner.loc(),
            Stmt::Return {
                keyword,
                value,
                semi_colon,
            } => {
                if let Some(semi) = semi_colon {
                    return SourceLocation {
                        start: keyword.start(),
                        end: semi.end(),
                    };
                }
                if let Some(value) = value {
                    return SourceLocation {
                        start: keyword.start(),
                        end: value.loc().end,
                    };
                }
                keyword.loc()
            }
            Stmt::Labeled(inner) => inner.loc(),
            Stmt::Break {
                keyword,
                label,
                semi_colon,
            } => {
                if let Some(semi_colon) = semi_colon {
                    return SourceLocation {
                        start: keyword.start(),
                        end: semi_colon.end(),
                    };
                }
                if let Some(label) = label {
                    return SourceLocation {
                        start: keyword.start(),
                        end: label.loc().end,
                    };
                }
                keyword.loc()
            }
            Stmt::Continue {
                keyword,
                label,
                semi_colon,
            } => {
                if let Some(semi_colon) = semi_colon {
                    return SourceLocation {
                        start: keyword.end(),
                        end: semi_colon.end(),
                    };
                }
                if let Some(label) = label {
                    return SourceLocation {
                        start: keyword.start(),
                        end: label.loc().end,
                    };
                }
                keyword.loc()
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
                        start: keyword.start(),
                        end: semi.end(),
                    };
                }
                SourceLocation {
                    start: keyword.start(),
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
                        end: semi.end(),
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
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct WithStmt<T> {
    pub keyword: With,
    pub open_paren: OpenParen,
    pub object: Expr<T>,
    pub close_paren: CloseParen,
    pub body: Box<Stmt<T>>,
}

impl<T> IntoAllocated for WithStmt<T>
where
    T: ToString,
{
    type Allocated = WithStmt<String>;
    fn into_allocated(self) -> Self::Allocated {
        WithStmt {
            keyword: self.keyword,
            open_paren: self.open_paren,
            object: self.object.into_allocated(),
            close_paren: self.close_paren,
            body: self.body.into_allocated(),
        }
    }
}
impl<T> Node for WithStmt<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.keyword.start(),
            end: self.body.loc().end,
        }
    }
}
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
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct LabeledStmt<T> {
    pub label: Ident<T>,
    pub colon: Colon,
    pub body: Box<Stmt<T>>,
}

impl<T> IntoAllocated for LabeledStmt<T>
where
    T: ToString,
{
    type Allocated = LabeledStmt<String>;

    fn into_allocated(self) -> Self::Allocated {
        LabeledStmt {
            label: self.label.into_allocated(),
            colon: self.colon,
            body: self.body.into_allocated(),
        }
    }
}

impl<T> Node for LabeledStmt<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.label.loc().start,
            end: self.body.loc().end,
        }
    }
}

/// An if statement
/// ```js
/// if (1 < 2) {
///     console.log('Always true');
/// } else {
///     console.log('Never true');
/// }
/// ```
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct IfStmt<T> {
    pub keyword: If,
    pub open_paren: OpenParen,
    pub test: Expr<T>,
    pub close_paren: CloseParen,
    pub consequent: Box<Stmt<T>>,
    pub alternate: Option<Box<ElseStmt<T>>>,
}

impl<T> IntoAllocated for IfStmt<T>
where
    T: ToString,
{
    type Allocated = IfStmt<String>;

    fn into_allocated(self) -> Self::Allocated {
        IfStmt {
            keyword: self.keyword,
            open_paren: self.open_paren,
            test: self.test.into_allocated(),
            close_paren: self.close_paren,
            consequent: self.consequent.into_allocated(),
            alternate: self.alternate.into_allocated(),
        }
    }
}

impl<T> Node for IfStmt<T> {
    fn loc(&self) -> SourceLocation {
        let start = self.keyword.start();
        let end = if let Some(alt) = &self.alternate {
            alt.loc().end
        } else {
            self.consequent.loc().end
        };
        SourceLocation { start, end }
    }
}

#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ElseStmt<T> {
    pub keyword: Else,
    pub body: Stmt<T>,
}

impl<T> IntoAllocated for ElseStmt<T>
where
    T: ToString,
{
    type Allocated = ElseStmt<String>;

    fn into_allocated(self) -> Self::Allocated {
        ElseStmt {
            keyword: self.keyword,
            body: self.body.into_allocated(),
        }
    }
}

impl<T> Node for ElseStmt<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.keyword.start(),
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
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct SwitchStmt<T> {
    pub keyword: Switch,
    pub open_paren: OpenParen,
    pub discriminant: Expr<T>,
    pub close_paren: CloseParen,
    pub open_brace: OpenBrace,
    pub cases: Vec<SwitchCase<T>>,
    pub close_brace: CloseBrace,
}

impl<T> IntoAllocated for SwitchStmt<T>
where
    T: ToString,
{
    type Allocated = SwitchStmt<String>;

    fn into_allocated(self) -> Self::Allocated {
        SwitchStmt {
            keyword: self.keyword,
            open_paren: self.open_paren,
            discriminant: self.discriminant.into_allocated(),
            close_paren: self.close_paren,
            open_brace: self.open_brace,
            cases: self
                .cases
                .into_iter()
                .map(IntoAllocated::into_allocated)
                .collect(),
            close_brace: self.close_brace,
        }
    }
}

impl<T> Node for SwitchStmt<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.keyword.start(),
            end: self.close_paren.end(),
        }
    }
}

/// A single case part of a switch statement
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct SwitchCase<T> {
    pub keyword: SwitchCaseKeyword,
    pub test: Option<Expr<T>>,
    pub colon: Colon,
    pub consequent: Vec<ProgramPart<T>>,
}

impl<T> Node for SwitchCase<T> {
    fn loc(&self) -> SourceLocation {
        let end = if let Some(last) = self.consequent.last() {
            last.loc().end
        } else {
            self.colon.end()
        };
        SourceLocation {
            start: self.keyword.start(),
            end,
        }
    }
}

impl<T> IntoAllocated for SwitchCase<T>
where
    T: ToString,
{
    type Allocated = SwitchCase<String>;

    fn into_allocated(self) -> Self::Allocated {
        SwitchCase {
            keyword: self.keyword,
            colon: self.colon,
            consequent: self
                .consequent
                .into_iter()
                .map(IntoAllocated::into_allocated)
                .collect(),
            test: self.test.into_allocated(),
        }
    }
}

/// A collection of program parts wrapped in curly braces
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct BlockStmt<T> {
    pub open_brace: OpenBrace,
    pub stmts: Vec<ProgramPart<T>>,
    pub close_brace: CloseBrace,
}

impl<T> IntoAllocated for BlockStmt<T>
where
    T: ToString,
{
    type Allocated = BlockStmt<String>;

    fn into_allocated(self) -> Self::Allocated {
        BlockStmt {
            open_brace: self.open_brace,
            stmts: self
                .stmts
                .into_iter()
                .map(IntoAllocated::into_allocated)
                .collect(),
            close_brace: self.close_brace,
        }
    }
}

impl<T> Node for BlockStmt<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.open_brace.start(),
            end: self.close_brace.end(),
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
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct TryStmt<T> {
    pub keyword: Try,
    pub block: BlockStmt<T>,
    pub handler: Option<CatchClause<T>>,
    pub finalizer: Option<FinallyClause<T>>,
}

impl<T> IntoAllocated for TryStmt<T>
where
    T: ToString,
{
    type Allocated = TryStmt<String>;
    fn into_allocated(self) -> Self::Allocated {
        TryStmt {
            keyword: self.keyword,
            block: self.block.into_allocated(),
            handler: self.handler.into_allocated(),
            finalizer: self.finalizer.into_allocated(),
        }
    }
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
            start: self.keyword.start(),
            end,
        }
    }
}

/// The error handling part of a `TryStmt`
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct CatchClause<T> {
    pub keyword: Catch,
    pub param: Option<CatchArg<T>>,
    pub body: BlockStmt<T>,
}

impl<T> IntoAllocated for CatchClause<T>
where
    T: ToString,
{
    type Allocated = CatchClause<String>;

    fn into_allocated(self) -> Self::Allocated {
        CatchClause {
            keyword: self.keyword,
            param: self.param.into_allocated(),
            body: self.body.into_allocated(),
        }
    }
}

impl<T> Node for CatchClause<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.keyword.start(),
            end: self.body.loc().end,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct CatchArg<T> {
    pub open_paren: OpenParen,
    pub param: Pat<T>,
    pub close_paren: CloseParen,
}

impl<T> IntoAllocated for CatchArg<T>
where
    T: ToString,
{
    type Allocated = CatchArg<String>;

    fn into_allocated(self) -> Self::Allocated {
        CatchArg {
            open_paren: self.open_paren,
            param: self.param.into_allocated(),
            close_paren: self.close_paren,
        }
    }
}

impl<T> Node for CatchArg<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.open_paren.start(),
            end: self.close_paren.end(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct FinallyClause<T> {
    pub keyword: Finally,
    pub body: BlockStmt<T>,
}

impl<T> IntoAllocated for FinallyClause<T>
where
    T: ToString,
{
    type Allocated = FinallyClause<String>;
    fn into_allocated(self) -> Self::Allocated {
        FinallyClause {
            keyword: self.keyword,
            body: self.body.into_allocated(),
        }
    }
}

impl<T> Node for FinallyClause<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.keyword.start(),
            end: self.body.loc().end,
        }
    }
}

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
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct WhileStmt<T> {
    pub keyword: While,
    pub open_paren: OpenParen,
    pub test: Expr<T>,
    pub close_paren: CloseParen,
    pub body: Box<Stmt<T>>,
}

impl<T> IntoAllocated for WhileStmt<T>
where
    T: ToString,
{
    type Allocated = WhileStmt<String>;

    fn into_allocated(self) -> Self::Allocated {
        WhileStmt {
            keyword: self.keyword,
            open_paren: self.open_paren,
            test: self.test.into_allocated(),
            close_paren: self.close_paren,
            body: self.body.into_allocated(),
        }
    }
}

impl<T> Node for WhileStmt<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.keyword.start(),
            end: self.body.loc().end,
        }
    }
}

/// A while loop that executes its body first
/// ```js
/// do {
///     console.log(Tt least once')
/// } while (Math.floor(Math.random() * 100) < 75)
/// ```
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct DoWhileStmt<T> {
    pub keyword_do: Do,
    pub body: Box<Stmt<T>>,
    pub keyword_while: While,
    pub open_paren: OpenParen,
    pub test: Expr<T>,
    pub close_paren: CloseParen,
    pub semi_colon: Option<Semicolon>,
}

impl<T> IntoAllocated for DoWhileStmt<T>
where
    T: ToString,
{
    type Allocated = DoWhileStmt<String>;

    fn into_allocated(self) -> Self::Allocated {
        DoWhileStmt {
            keyword_do: self.keyword_do,
            body: self.body.into_allocated(),
            keyword_while: self.keyword_while,
            open_paren: self.open_paren,
            test: self.test.into_allocated(),
            close_paren: self.close_paren,
            semi_colon: self.semi_colon,
        }
    }
}

impl<T> Node for DoWhileStmt<T> {
    fn loc(&self) -> SourceLocation {
        let end = self
            .semi_colon
            .map(|s| s.end())
            .unwrap_or_else(|| self.close_paren.end())
            + 1;
        SourceLocation {
            start: self.keyword_do.start(),
            end,
        }
    }
}

/// A "c-style" for loop
/// ```js
/// for (var i = 0; i < 100; i++) console.log(i);
/// for (;;) {
///     console.log('forever!');
/// }
/// ```
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ForStmt<T> {
    pub keyword: For,
    pub open_paren: OpenParen,
    pub init: Option<LoopInit<T>>,
    pub semi1: Semicolon,
    pub test: Option<Expr<T>>,
    pub semi2: Semicolon,
    pub update: Option<Expr<T>>,
    pub close_paren: CloseParen,
    pub body: Box<Stmt<T>>,
}

impl<T> IntoAllocated for ForStmt<T>
where
    T: ToString,
{
    type Allocated = ForStmt<String>;

    fn into_allocated(self) -> Self::Allocated {
        ForStmt {
            keyword: self.keyword,
            open_paren: self.open_paren,
            init: self.init.into_allocated(),
            semi1: self.semi1,
            test: self.test.into_allocated(),
            semi2: self.semi2,
            update: self.update.into_allocated(),
            close_paren: self.close_paren,
            body: self.body.into_allocated(),
        }
    }
}

impl<T> Node for ForStmt<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.keyword.start(),
            end: self.body.loc().end,
        }
    }
}

/// The left most triple of a for loops parenthetical
/// ```js
///  //  vvvvvvvvv
/// for (var i = 0;i < 100; i++)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum LoopInit<T> {
    Variable(VarKind, Vec<ListEntry<VarDecl<T>>>),
    Expr(Expr<T>),
}

impl<T> IntoAllocated for LoopInit<T>
where
    T: ToString,
{
    type Allocated = LoopInit<String>;
    fn into_allocated(self) -> Self::Allocated {
        match self {
            LoopInit::Variable(k, v) => LoopInit::Variable(
                k,
                v.into_iter().map(IntoAllocated::into_allocated).collect(),
            ),
            LoopInit::Expr(inner) => LoopInit::Expr(inner.into_allocated()),
        }
    }
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
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ForInStmt<T> {
    pub keyword_for: For,
    pub open_paren: OpenParen,
    pub left: LoopLeft<T>,
    pub keyword_in: In,
    pub right: Expr<T>,
    pub close_paren: CloseParen,
    pub body: Box<Stmt<T>>,
}

impl<T> IntoAllocated for ForInStmt<T>
where
    T: ToString,
{
    type Allocated = ForInStmt<String>;

    fn into_allocated(self) -> Self::Allocated {
        ForInStmt {
            keyword_for: self.keyword_for,
            open_paren: self.open_paren,
            left: self.left.into_allocated(),
            keyword_in: self.keyword_in,
            right: self.right.into_allocated(),
            close_paren: self.close_paren,
            body: self.body.into_allocated(),
        }
    }
}

impl<T> Node for ForInStmt<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.keyword_for.start(),
            end: self.body.loc().end,
        }
    }
}

/// A for of statement, this kind of for statement
/// will extract the value from a generator or iterator
/// ```js
/// for (var k of [2, 3, 4, 5, 6]) {
///     console.log(k);
/// }
/// //prints 2, 3, 4, 5, 6
/// ```
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ForOfStmt<T> {
    pub keyword_for: For,
    pub open_paren: OpenParen,
    pub left: LoopLeft<T>,
    pub keyword_of: Of,
    pub right: Expr<T>,
    pub close_paren: CloseParen,
    pub body: Box<Stmt<T>>,
    pub is_await: bool,
}

impl<T> IntoAllocated for ForOfStmt<T>
where
    T: ToString,
{
    type Allocated = ForOfStmt<String>;

    fn into_allocated(self) -> Self::Allocated {
        ForOfStmt {
            keyword_for: self.keyword_for,
            open_paren: self.open_paren,
            left: self.left.into_allocated(),
            keyword_of: self.keyword_of,
            right: self.right.into_allocated(),
            close_paren: self.close_paren,
            body: self.body.into_allocated(),
            is_await: self.is_await,
        }
    }
}

impl<T> Node for ForOfStmt<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.keyword_for.start(),
            end: self.body.loc().end,
        }
    }
}

/// The values on the left hand side of the keyword
/// in a for in or for of loop
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum LoopLeft<T> {
    Expr(Expr<T>),
    Variable(VarKind, VarDecl<T>),
    Pat(Pat<T>),
}

impl<T> IntoAllocated for LoopLeft<T>
where
    T: ToString,
{
    type Allocated = LoopLeft<String>;
    fn into_allocated(self) -> Self::Allocated {
        match self {
            LoopLeft::Expr(inner) => LoopLeft::Expr(inner.into_allocated()),
            LoopLeft::Variable(k, v) => LoopLeft::Variable(k, v.into_allocated()),
            LoopLeft::Pat(inner) => LoopLeft::Pat(inner.into_allocated()),
        }
    }
}

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
