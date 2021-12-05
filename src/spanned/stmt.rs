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

impl<'a> From<Stmt<'a>> for crate::stmt::Stmt<'a> {
    fn from(other: Stmt<'a>) -> Self {
        match other {
            Stmt::Expr(inner) => Self::Expr(inner.into()),
            Stmt::Block(inner) => Self::Block(inner.into()),
            Stmt::Empty(_) => Self::Empty,
            Stmt::Debugger(_) => Self::Debugger,
            Stmt::With(inner) => Self::With(inner.into()),
            Stmt::Return(inner) => Self::Return(inner.map(From::from)),
            Stmt::Labeled(inner) => Self::Labeled(inner.into()),
            Stmt::Break(inner) => Self::Break(inner.map(From::from)),
            Stmt::Continue(inner) => Self::Continue(inner.map(From::from)),
            Stmt::If(inner) => Self::If(inner.into()),
            Stmt::Switch(inner) => Self::Switch(inner.into()),
            Stmt::Throw(inner) => Self::Throw(inner.into()),
            Stmt::Try(inner) => Self::Try(inner.into()),
            Stmt::While(inner) => Self::While(inner.into()),
            Stmt::DoWhile(inner) => Self::DoWhile(inner.into()),
            Stmt::For(inner) => Self::For(inner.into()),
            Stmt::ForIn(inner) => Self::ForIn(inner.into()),
            Stmt::ForOf(inner) => Self::ForOf(inner.into()),
            Stmt::Var(inner) => Self::Var(inner.into_iter().map(From::from).collect()),
        }
    }
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
    pub keyword: Slice<'a>,
    pub open_paren: Slice<'a>,
    pub object: Expr<'a>,
    pub close_paren: Slice<'a>,
    pub body: Box<Stmt<'a>>,
}

impl<'a> Node for WithStmt<'a> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.keyword.loc.start,
            end: self.body.loc().end,
        }
    }
}

impl<'a> From<WithStmt<'a>> for crate::stmt::WithStmt<'a> {
    fn from(other: WithStmt<'a>) -> Self {
        Self {
            object: other.object.into(),
            body: Box::new(From::from(*other.body)),
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

impl<'a> From<LabeledStmt<'a>> for crate::stmt::LabeledStmt<'a> {
    fn from(other: LabeledStmt<'a>) -> Self {
        Self {
            label: other.label.into(),
            body: Box::new(From::from(*other.body)),
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

impl<'a> From<IfStmt<'a>> for crate::stmt::IfStmt<'a> {
    fn from(other: IfStmt<'a>) -> Self {
        Self {
            test: other.test.into(),
            consequent: Box::new(From::from(*other.consequent)),
            alternate: other.alternate.map(|s| Box::new(From::from(*s))),
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

impl<'a> From<SwitchStmt<'a>> for crate::stmt::SwitchStmt<'a> {
    fn from(other: SwitchStmt<'a>) -> Self {
        Self {
            discriminant: other.discriminant.into(),
            cases: other.cases.into_iter().map(From::from).collect(),
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

impl<'a> From<SwitchCase<'a>> for crate::stmt::SwitchCase<'a> {
    fn from(other: SwitchCase<'a>) -> Self {
        Self {
            test: other.test.map(From::from),
            consequent: other.consequent.into_iter().map(From::from).collect(),
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

impl<'a> From<BlockStmt<'a>> for crate::stmt::BlockStmt<'a> {
    fn from(other: BlockStmt<'a>) -> Self {
        Self(other.stmts.into_iter().map(From::from).collect())
    }
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

impl<'a> From<TryStmt<'a>> for crate::stmt::TryStmt<'a> {
    fn from(other: TryStmt<'a>) -> Self {
        Self {
            block: other.block.into(),
            handler: other.handler.map(From::from),
            finalizer: other.finalizer.map(From::from),
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

impl<'a> From<CatchClause<'a>> for crate::stmt::CatchClause<'a> {
    fn from(other: CatchClause<'a>) -> Self {
        Self {
            param: other.param.map(|a| a.param.into()),
            body: other.body.into(),
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

impl<'a> From<FinallyClause<'a>> for crate::stmt::BlockStmt<'a> {
    fn from(other: FinallyClause<'a>) -> Self {
        other.body.into()
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

impl<'a> From<WhileStmt<'a>> for crate::stmt::WhileStmt<'a> {
    fn from(other: WhileStmt<'a>) -> Self {
        Self {
            test: other.test.into(),
            body: Box::new(From::from(*other.body)),
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

impl<'a> From<DoWhileStmt<'a>> for crate::stmt::DoWhileStmt<'a> {
    fn from(other: DoWhileStmt<'a>) -> Self {
        let body = other
            .body
            .into_iter()
            .map(From::from)
            .map(crate::ProgramPart::Stmt)
            .collect();
        let body = crate::stmt::BlockStmt(body);
        let body = crate::stmt::Stmt::Block(body);
        Self {
            test: other.test.into(),
            body: Box::new(body),
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
    pub semi1: Slice<'a>,
    pub test: Option<Expr<'a>>,
    pub semi2: Slice<'a>,
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

impl<'a> From<ForStmt<'a>> for crate::stmt::ForStmt<'a> {
    fn from(other: ForStmt<'a>) -> Self {
        Self {
            init: other.init.map(From::from),
            test: other.test.map(From::from),
            update: other.update.map(From::from),
            body: Box::new(From::from(*other.body)),
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

impl<'a> From<LoopInit<'a>> for crate::stmt::LoopInit<'a> {
    fn from(other: LoopInit<'a>) -> Self {
        match other {
            LoopInit::Expr(inner) => Self::Expr(inner.into()),
            LoopInit::Variable(kind, decls) => {
                Self::Variable(kind.into(), decls.into_iter().map(From::from).collect())
            }
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

impl<'a> From<ForInStmt<'a>> for crate::stmt::ForInStmt<'a> {
    fn from(other: ForInStmt<'a>) -> Self {
        Self {
            left: other.left.into(),
            right: other.right.into(),
            body: Box::new(From::from(*other.body)),
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

impl<'a> From<ForOfStmt<'a>> for crate::stmt::ForOfStmt<'a> {
    fn from(other: ForOfStmt<'a>) -> Self {
        Self {
            left: other.left.into(),
            right: other.right.into(),
            body: Box::new(From::from(*other.body)),
            is_await: other.is_await,
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

impl<'a> From<LoopLeft<'a>> for crate::stmt::LoopLeft<'a> {
    fn from(other: LoopLeft<'a>) -> Self {
        match other {
            LoopLeft::Expr(inner) => Self::Expr(inner.into()),
            LoopLeft::Variable(kind, decl) => Self::Variable(kind.into(), decl.into()),
            LoopLeft::Pat(inner) => Self::Pat(inner.into()),
        }
    }
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
