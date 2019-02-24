use crate::decl::VariableKind;
use crate::ref_tree::decl::VariableDecl;
use crate::ref_tree::expr::Expr;
use crate::ref_tree::pat::Pat;
use crate::ref_tree::{AsConcrete, Identifier, ProgramPart, ref_map};
/// A slightly more granular part of an es program than ProgramPart
#[derive(PartialEq, Debug, Clone)]
pub enum Stmt<'a> {
    /// Any expression
    Expr(Expr<'a>),
    /// A collection of program parts wrapped in curly braces
    Block(BlockStmt<'a>),
    /// A single semi-colon
    Empty,
    /// The contextual keyword `debugger`
    Debugger,
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
    Break(Option<Identifier<'a>>),
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
    Continue(Option<Identifier<'a>>),
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
    Var(Vec<VariableDecl<'a>>),
}

impl<'a> AsConcrete<crate::stmt::Stmt> for Stmt<'a> {
    fn as_concrete(&self) -> crate::stmt::Stmt {
        match self {
            Stmt::Expr(ref e) => crate::stmt::Stmt::Expr(e.as_concrete()),
            Stmt::Block(ref b) => crate::stmt::Stmt::Block(b.iter().map(|p| p.as_concrete()).collect()),
            Stmt::Empty => crate::stmt::Stmt::Empty,
            Stmt::Debugger => crate::stmt::Stmt::Debugger,
            Stmt::With(ref w) => crate::stmt::Stmt::With(w.as_concrete()),
            Stmt::Return(ref r) => crate::stmt::Stmt::Return(ref_map(r, |r| r.as_concrete())),
            Stmt::Labeled(ref l) => crate::stmt::Stmt::Labeled(l.as_concrete()),
            Stmt::Break(ref b) => crate::stmt::Stmt::Break(ref_map(b, |b| b.as_concrete())),
            Stmt::Continue(ref c) => crate::stmt::Stmt::Continue(ref_map(c, |c| c.as_concrete())),
            Stmt::If(ref i) => crate::stmt::Stmt::If(i.as_concrete()),
            Stmt::Switch(ref s) => crate::stmt::Stmt::Switch(s.as_concrete()),
            Stmt::Throw(ref t) => crate::stmt::Stmt::Throw(t.as_concrete()),
            Stmt::Try(ref t) => crate::stmt::Stmt::Try(t.as_concrete()),
            Stmt::While(ref w) => crate::stmt::Stmt::While(w.as_concrete()),
            Stmt::DoWhile(ref d) => crate::stmt::Stmt::DoWhile(d.as_concrete()),
            Stmt::For(ref f) => crate::stmt::Stmt::For(f.as_concrete()),
            Stmt::ForIn(ref f) => crate::stmt::Stmt::ForIn(f.as_concrete()),
            Stmt::ForOf(ref f) => crate::stmt::Stmt::ForOf(f.as_concrete()),
            Stmt::Var(ref v) => crate::stmt::Stmt::Var(v.iter().map(|v| v.as_concrete()).collect()),
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
pub struct WithStmt<'a> {
    pub object: Expr<'a>,
    pub body: Box<Stmt<'a>>,
}

impl<'a> AsConcrete<crate::stmt::WithStmt> for WithStmt<'a> {
    fn as_concrete(&self) -> crate::stmt::WithStmt {
        crate::stmt::WithStmt {
            object: self.object.as_concrete(),
            body: Box::new(self.body.as_concrete()),
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
    pub label: Identifier<'a>,
    pub body: Box<Stmt<'a>>,
}

impl<'a> AsConcrete<crate::stmt::LabeledStmt> for LabeledStmt<'a> {
    fn as_concrete(&self) -> crate::stmt::LabeledStmt {
        crate::stmt::LabeledStmt {
            label: self.label.as_concrete(),
            body: Box::new(self.body.as_concrete()),
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
    pub test: Expr<'a>,
    pub consequent: Box<Stmt<'a>>,
    pub alternate: Option<Box<Stmt<'a>>>,
}

impl<'a> AsConcrete<crate::stmt::IfStmt> for IfStmt<'a> {
    fn as_concrete(&self) -> crate::stmt::IfStmt {
        crate::stmt::IfStmt {
            test: self.test.as_concrete(),
            consequent: Box::new(self.consequent.as_concrete()),
            alternate: ref_map(&self.alternate, |a| Box::new(a.as_concrete())),
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
    pub discriminant: Expr<'a>,
    pub cases: Vec<SwitchCase<'a>>,
}

impl<'a> AsConcrete<crate::stmt::SwitchStmt> for SwitchStmt<'a> {
    fn as_concrete(&self) -> crate::stmt::SwitchStmt {
        crate::stmt::SwitchStmt {
            discriminant: self.discriminant.as_concrete(),
            cases: self.cases.iter().map(|c| c.as_concrete()).collect(),
        }
    }
}

/// A single case part of a switch statement
#[derive(PartialEq, Debug, Clone)]
pub struct SwitchCase<'a> {
    pub test: Option<Expr<'a>>,
    pub consequent: Vec<ProgramPart<'a>>,
}

impl<'a> AsConcrete<crate::stmt::SwitchCase> for SwitchCase<'a> {
    fn as_concrete(&self) -> crate::stmt::SwitchCase {
        crate::stmt::SwitchCase {
            test: ref_map(&self.test, |t| t.as_concrete()),
            consequent: self.consequent.iter().map(|p| p.as_concrete()).collect(),
        }
    }
}

/// A collection of program parts wrapped in curly braces
pub type BlockStmt<'a> = Vec<ProgramPart<'a>>;

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
    pub block: BlockStmt<'a>,
    pub handler: Option<CatchClause<'a>>,
    pub finalizer: Option<BlockStmt<'a>>,
}

impl<'a> AsConcrete<crate::stmt::TryStmt> for TryStmt<'a> {
    fn as_concrete(&self) -> crate::stmt::TryStmt {
        crate::stmt::TryStmt {
            block: self.block.iter().map(|p| p.as_concrete()).collect(),
            handler: ref_map(&self.handler, |h| h.as_concrete()),
            finalizer: ref_map(&self.finalizer, |f| f.as_concrete())
        }
    }
}

/// The error handling part of a `TryStmt`
#[derive(PartialEq, Debug, Clone)]
pub struct CatchClause<'a> {
    pub param: Option<Pat<'a>>,
    pub body: BlockStmt<'a>,
}

impl<'a> AsConcrete<crate::stmt::CatchClause> for CatchClause<'a> {
    fn as_concrete(&self) -> crate::stmt::CatchClause {
        crate::stmt::CatchClause {
            param: ref_map(&self.param, |p| p.as_concrete()),
            body: self.body.as_concrete(),
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
    pub test: Expr<'a>,
    pub body: Box<Stmt<'a>>,
}

impl<'a> AsConcrete<crate::stmt::WhileStmt> for WhileStmt<'a> {
    fn as_concrete(&self) -> crate::stmt::WhileStmt {
        crate::stmt::WhileStmt {
            test: self.test.as_concrete(),
            body: Box::new(self.body.as_concrete()),
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
    pub test: Expr<'a>,
    pub body: Box<Stmt<'a>>,
}

impl<'a> AsConcrete<crate::stmt::DoWhileStmt> for DoWhileStmt<'a> {
    fn as_concrete(&self) -> crate::stmt::DoWhileStmt {
        crate::stmt::DoWhileStmt {
            test: self.test.as_concrete(),
            body: Box::new(self.body.as_concrete()),
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
    pub init: Option<LoopInit<'a>>,
    pub test: Option<Expr<'a>>,
    pub update: Option<Expr<'a>>,
    pub body: Box<Stmt<'a>>,
}

impl<'a> AsConcrete<crate::stmt::ForStmt> for ForStmt<'a> {
    fn as_concrete(&self) -> crate::stmt::ForStmt {
        crate::stmt::ForStmt {
            init: ref_map(&self.init, |i| i.as_concrete()),
            test: ref_map(&self.test, |t| t.as_concrete()),
            update: ref_map(&self.update, |u| u.as_concrete()),
            body: Box::new(self.body.as_concrete()),
        }
    }
}

/// The left most triple of a for loops parenthetical
/// ```js
///  //  vvvvvvvvv
/// for (var i = 0;i < 100; i++)
#[derive(PartialEq, Debug, Clone)]
pub enum LoopInit<'a> {
    Variable(VariableKind, Vec<VariableDecl<'a>>),
    Expr(Expr<'a>),
}

impl<'a> AsConcrete<crate::stmt::LoopInit> for LoopInit<'a> {
    fn as_concrete(&self) -> crate::stmt::LoopInit {
        match self {
            LoopInit::Variable(ref k, ref v) => crate::stmt::LoopInit::Variable(*k, v.as_concrete()),
            LoopInit::Expr(ref e) => crate::stmt::LoopInit::Expr(e.as_concrete()),
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
    pub left: LoopLeft<'a>,
    pub right: Expr<'a>,
    pub body: Box<Stmt<'a>>,
}

impl<'a> AsConcrete<crate::stmt::ForInStmt> for ForInStmt<'a> {
    fn as_concrete(&self) -> crate::stmt::ForInStmt {
        crate::stmt::ForInStmt {
            left: self.left.as_concrete(),
            right: self.right.as_concrete(),
            body: Box::new(self.body.as_concrete()),
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
    pub left: LoopLeft<'a>,
    pub right: Expr<'a>,
    pub body: Box<Stmt<'a>>,
    pub is_await: bool,
}

impl<'a> AsConcrete<crate::stmt::ForOfStmt> for ForOfStmt<'a> {
    fn as_concrete(&self) -> crate::stmt::ForOfStmt {
        crate::stmt::ForOfStmt {
            left: self.left.as_concrete(),
            right: self.right.as_concrete(),
            body: Box::new(self.body.as_concrete()),
            is_await: self.is_await,
        }
    }
}

/// The values on the left hand side of the keyword
/// in a for in or for of loop
#[derive(PartialEq, Debug, Clone)]
pub enum LoopLeft<'a> {
    Expr(Expr<'a>),
    Variable(VariableKind, VariableDecl<'a>),
    Pat(Pat<'a>),
}

impl<'a> AsConcrete<crate::stmt::LoopLeft> for LoopLeft<'a> {
    fn as_concrete(&self) -> crate::stmt::LoopLeft {
        match self {
            LoopLeft::Expr(ref e) => crate::stmt::LoopLeft::Expr(e.as_concrete()),
            LoopLeft::Variable(ref k, ref v) => crate::stmt::LoopLeft::Variable(*k, v.as_concrete()),
            LoopLeft::Pat(ref p) => crate::stmt::LoopLeft::Pat(p.as_concrete()),
        }
    }
}
