use crate::spanned::decl::VarDecl;
use crate::spanned::expr::Expr;
use crate::spanned::pat::Pat;
use crate::spanned::VarKind;
use crate::spanned::{Ident, ProgramPart};

use super::{Node, Slice, SourceLocation};
/// A slightly more granular part of an es program than ProgramPart
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt<'a> {
    /// Any expression
    Expr(Expr<'a>),
    /// A collection of program parts wrapped in curly braces
    Block(BlockStmt<'a>),
    /// A single semi-colon
    Empty(Slice<'a>),
    /// The contextual keyword `debugger`
    Debugger(Slice<'a>),
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
    With(WithStmt<'a>),
    /// A return statement
    /// ```js
    /// function thing() {
    ///     return 'stuff';
    /// }
    /// function stuff() {
    ///     return;
    /// }
    Return(Option<Expr<'a>>),
    /// A labeled statement
    /// ```js
    /// label: {
    ///     break label;
    /// }
    /// ```
    Labeled(LabeledStmt<'a>),
    /// A break statement
    /// ```js
    /// label: {
    ///     break label;
    /// }
    /// while (true) {
    ///     break;
    /// }
    /// ```
    Break(Option<Ident<'a>>),
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
    Continue(Option<Ident<'a>>),
    /// An if statement
    /// ```js
    /// if (1 < 2) {
    ///     console.log('Always true');
    /// } else {
    ///     console.log('Never true');
    /// }
    /// ```
    If(IfStmt<'a>),
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
    Switch(SwitchStmt<'a>),
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
    Throw(Expr<'a>),
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
    Try(TryStmt<'a>),
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
    While(WhileStmt<'a>),
    /// A while loop that executes its body first
    /// ```js
    /// do {
    ///     console.log('at least once')
    /// } while (Math.floor(Math.random() * 100) < 75)
    /// ```
    DoWhile(DoWhileStmt<'a>),
    /// A "c-style" for loop
    /// ```js
    /// for (var i = 0; i < 100; i++) console.log(i);
    /// for (;;) {
    ///     console.log('forever!');
    /// }
    /// ```
    For(ForStmt<'a>),
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
    ForIn(ForInStmt<'a>),
    /// A for of statement, this kind of for statement
    /// will extract the value from a generator or iterator
    /// ```js
    /// for (var k of [2, 3, 4, 5, 6]) {
    ///     console.log(k);
    /// }
    /// //prints 2, 3, 4, 5, 6
    /// ```
    ForOf(ForOfStmt<'a>),
    /// A var statement
    /// ```js
    /// var x;
    /// var x, y = 'huh?';
    /// ```
    Var(Vec<VarDecl<'a>>),
}

impl<'a> Node for Stmt<'a> {
    fn loc(&self) -> super::SourceLocation {
        todo!()
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
pub struct WithStmt<'a> {
    pub object: Expr<'a>,
    pub body: Box<Stmt<'a>>,
}

impl<'a> Node for WithStmt<'a> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.object.loc().start,
            end: self.body.loc().end,
        }
    }
}

/// A break statement
/// ```js
/// label: {
///     break label;
/// }
/// while (true) {
///     break;
/// }
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct LabeledStmt<'a> {
    pub label: Ident<'a>,
    pub colon: Slice<'a>,
    pub body: Box<Stmt<'a>>,
}

impl<'a> Node for LabeledStmt<'a> {
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
pub struct IfStmt<'a> {
    pub keyword: Slice<'a>,
    pub open_paren: Slice<'a>,
    pub test: Expr<'a>,
    pub close_paren: Slice<'a>,
    pub consequent: Box<Stmt<'a>>,
    pub alternate: Option<Box<Stmt<'a>>>,
}

impl<'a> Node for IfStmt<'a> {
    fn loc(&self) -> SourceLocation {
        let start = self.keyword.loc.start;
        let end = if let Some(alt) = &self.alternate {
            alt.loc().end
        } else {
            self.consequent.loc().end
        };
        SourceLocation { start, end }
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
pub struct SwitchStmt<'a> {
    pub keyword: Slice<'a>,
    pub open_paren: Slice<'a>,
    pub discriminant: Expr<'a>,
    pub close_paren: Slice<'a>,
    pub open_brace: Slice<'a>,
    pub cases: Vec<SwitchCase<'a>>,
    pub close_brace: Slice<'a>,
}

impl<'a> Node for SwitchStmt<'a> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.keyword.loc.start,
            end: self.close_paren.loc.end,
        }
    }
}

/// A single case part of a switch statement
#[derive(Debug, Clone, PartialEq)]
pub struct SwitchCase<'a> {
    pub keyword: Slice<'a>,
    pub test: Option<Expr<'a>>,
    pub colon: Slice<'a>,
    pub consequent: Vec<ProgramPart<'a>>,
}

impl<'a> Node for SwitchCase<'a> {
    fn loc(&self) -> SourceLocation {
        let end = if let Some(last) = self.consequent.last() {
            last.loc().end
        } else {
            self.colon.loc.end
        };
        SourceLocation {
            start: self.keyword.loc.start,
            end,
        }
    }
}

/// A collection of program parts wrapped in curly braces
#[derive(Debug, Clone, PartialEq)]
pub struct BlockStmt<'a> {
    pub open_brace: Slice<'a>,
    pub stmts: Vec<ProgramPart<'a>>,
    pub close_brace: Slice<'a>,
}

impl<'a> Node for BlockStmt<'a> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.open_brace.loc.start,
            end: self.close_brace.loc.end,
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
pub struct TryStmt<'a> {
    pub keyword: Slice<'a>,
    pub block: BlockStmt<'a>,
    pub handler: Option<CatchClause<'a>>,
    pub finalizer: Option<FinallyClause<'a>>,
}

impl<'a> Node for TryStmt<'a> {
    fn loc(&self) -> SourceLocation {
        let end = if let Some(finalizer) = &self.finalizer {
            finalizer.loc().end
        } else if let Some(catch) = &self.handler {
            catch.loc().end
        } else {
            self.block.loc().end
        };
        SourceLocation {
            start: self.keyword.loc.start,
            end,
        }
    }
}

/// The error handling part of a `TryStmt`
#[derive(Debug, Clone, PartialEq)]
pub struct CatchClause<'a> {
    pub keyword: Slice<'a>,
    pub param: Option<CatchArg<'a>>,
    pub body: BlockStmt<'a>,
}

impl<'a> Node for CatchClause<'a> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.keyword.loc.start,
            end: self.body.loc().end,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CatchArg<'a> {
    pub open_paren: Option<Slice<'a>>,
    pub param: Pat<'a>,
    pub close_paren: Option<Slice<'a>>,
}

impl<'a> Node for CatchArg<'a> {
    fn loc(&self) -> SourceLocation {
        if let (Some(open), Some(close)) = (&self.open_paren, &self.close_paren) {
            SourceLocation {
                start: open.loc.start,
                end: close.loc.start,
            }
        } else {
            self.param.loc()
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FinallyClause<'a> {
    pub keyword: Slice<'a>,
    pub param: Option<Pat<'a>>,
    pub body: BlockStmt<'a>,
}

impl<'a> Node for FinallyClause<'a> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.keyword.loc.start,
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
pub struct WhileStmt<'a> {
    pub keyword: Slice<'a>,
    pub open_paren: Slice<'a>,
    pub test: Expr<'a>,
    pub close_paren: Slice<'a>,
    pub body: Box<Stmt<'a>>,
}

impl<'a> Node for WhileStmt<'a> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.keyword.loc.start,
            end: self.body.loc().end,
        }
    }
}

/// A while loop that executes its body first
/// ```js
/// do {
///     console.log('at least once')
/// } while (Math.floor(Math.random() * 100) < 75)
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct DoWhileStmt<'a> {
    pub keyword_do: Slice<'a>,
    pub open_brace: Slice<'a>,
    pub body: Vec<Stmt<'a>>,
    pub close_brace: Slice<'a>,
    pub keyword_while: Slice<'a>,
    pub open_paren: Slice<'a>,
    pub test: Expr<'a>,
    pub close_paren: Slice<'a>,
}

impl<'a> Node for DoWhileStmt<'a> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.keyword_do.loc.start,
            end: self.close_paren.loc.end,
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
pub struct ForStmt<'a> {
    pub keyword: Slice<'a>,
    pub open_paren: Slice<'a>,
    pub init: Option<LoopInit<'a>>,
    pub test: Option<Expr<'a>>,
    pub update: Option<Expr<'a>>,
    pub close_paren: Slice<'a>,
    pub body: Box<Stmt<'a>>,
}

impl<'a> Node for ForStmt<'a> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.keyword.loc.start,
            end: self.body.loc().end,
        }
    }
}

/// The left most triple of a for loops parenthetical
/// ```js
///  //  vvvvvvvvv
/// for (var i = 0;i < 100; i++)
#[derive(Debug, Clone, PartialEq)]
pub enum LoopInit<'a> {
    Variable(VarKind<'a>, Vec<VarDecl<'a>>),
    Expr(Expr<'a>),
}

impl<'a> Node for LoopInit<'a> {
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
pub struct ForInStmt<'a> {
    pub keyword_for: Slice<'a>,
    pub open_paren: Slice<'a>,
    pub left: LoopLeft<'a>,
    pub keyword_in: Slice<'a>,
    pub right: Expr<'a>,
    pub body: Box<Stmt<'a>>,
}

impl<'a> Node for ForInStmt<'a> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.keyword_for.loc.start,
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
pub struct ForOfStmt<'a> {
    pub keyword_for: Slice<'a>,
    pub open_paren: Slice<'a>,
    pub left: LoopLeft<'a>,
    pub keyword_of: Slice<'a>,
    pub right: Expr<'a>,
    pub close_paren: Slice<'a>,
    pub body: Box<Stmt<'a>>,
    pub is_await: bool,
}

impl<'a> Node for ForOfStmt<'a> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.keyword_for.loc.start,
            end: self.body.loc().end,
        }
    }
}

/// The values on the left hand side of the keyword
/// in a for in or for of loop
#[derive(Debug, Clone, PartialEq)]
pub enum LoopLeft<'a> {
    Expr(Expr<'a>),
    Variable(VarKind<'a>, VarDecl<'a>),
    Pat(Pat<'a>),
}

impl<'a> Node for LoopLeft<'a> {
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
