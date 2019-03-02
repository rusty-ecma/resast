use crate::decl::{VariableDecl, VariableKind};
use crate::expr::Expr;
use crate::pat::Pat;
use crate::{Identifier, ProgramPart};
/// A slightly more granular part of an es program than ProgramPart
#[derive(PartialEq, Debug, Clone)]
pub enum Stmt {
    /// Any expression
    Expr(Expr),
    /// A collection of program parts wrapped in curly braces
    Block(BlockStmt),
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
    With(WithStmt),
    /// A return statement
    /// ```js
    /// function thing() {
    ///     return 'stuff';
    /// }
    /// function stuff() {
    ///     return;
    /// }
    Return(Option<Expr>),
    /// A labeled statement
    /// ```js
    /// label: {
    ///     break label;
    /// }
    /// ```
    Labeled(LabeledStmt),
    /// A break statement
    /// ```js
    /// label: {
    ///     break label;
    /// }
    /// while (true) {
    ///     break;
    /// }
    /// ```
    Break(Option<Identifier>),
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
    Continue(Option<Identifier>),
    /// An if statement
    /// ```js
    /// if (1 < 2) {
    ///     console.log('Always true');
    /// } else {
    ///     console.log('Never true');
    /// }
    /// ```
    If(IfStmt),
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
    Switch(SwitchStmt),
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
    Throw(Expr),
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
    Try(TryStmt),
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
    While(WhileStmt),
    /// A while loop that executes its body first
    /// ```js
    /// do {
    ///     console.log('at least once')
    /// } while (Math.floor(Math.random() * 100) < 75)
    /// ```
    DoWhile(DoWhileStmt),
    /// A "c-style" for loop
    /// ```js
    /// for (var i = 0; i < 100; i++) console.log(i);
    /// for (;;) {
    ///     console.log('forever!');
    /// }
    /// ```
    For(ForStmt),
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
    ForIn(ForInStmt),
    /// A for of statement, this kind of for statement
    /// will extract the value from a generator or iterator
    /// ```js
    /// for (var k of [2, 3, 4, 5, 6]) {
    ///     console.log(k);
    /// }
    /// //prints 2, 3, 4, 5, 6
    /// ```
    ForOf(ForOfStmt),
    /// A var statement
    /// ```js
    /// var x;
    /// var x, y = 'huh?';
    /// ```
    Var(Vec<VariableDecl>),
}

impl Stmt {
    pub fn with(object: Expr, body: Stmt) -> Self {
        Stmt::With(WithStmt::new(object, body))
    }
    pub fn while_stmt(test: Expr, body: Stmt) -> Self {
        Stmt::While(WhileStmt::new(test, body))
    }
    pub fn if_stmt(test: Expr, consequent: Stmt, alt: Option<Stmt>) -> Self {
        Stmt::If(IfStmt::new(test, consequent, alt))
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
pub struct WithStmt {
    pub object: Expr,
    pub body: Box<Stmt>,
}

impl WithStmt {
    pub fn new(object: Expr, body: Stmt) -> Self {
        WithStmt {
            object,
            body: Box::new(body),
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
pub struct LabeledStmt {
    pub label: Identifier,
    pub body: Box<Stmt>,
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
pub struct IfStmt {
    pub test: Expr,
    pub consequent: Box<Stmt>,
    pub alternate: Option<Box<Stmt>>,
}

impl IfStmt {
    pub fn new(test: Expr, consequent: Stmt, alternate: Option<Stmt>) -> Self {
        IfStmt {
            test,
            consequent: Box::new(consequent),
            alternate: alternate.map(|a| Box::new(a)),
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
pub struct SwitchStmt {
    pub discriminant: Expr,
    pub cases: Vec<SwitchCase>,
}

/// A single case part of a switch statement
#[derive(PartialEq, Debug, Clone)]
pub struct SwitchCase {
    pub test: Option<Expr>,
    pub consequent: Vec<ProgramPart>,
}

/// A collection of program parts wrapped in curly braces
pub type BlockStmt = Vec<ProgramPart>;

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
pub struct TryStmt {
    pub block: BlockStmt,
    pub handler: Option<CatchClause>,
    pub finalizer: Option<BlockStmt>,
}

/// The error handling part of a `TryStmt`
#[derive(PartialEq, Debug, Clone)]
pub struct CatchClause {
    pub param: Option<Pat>,
    pub body: BlockStmt,
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
pub struct WhileStmt {
    pub test: Expr,
    pub body: Box<Stmt>,
}
impl WhileStmt {
    pub fn new(test: Expr, body: Stmt) -> Self {
        WhileStmt {
            test,
            body: Box::new(body),
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
pub struct DoWhileStmt {
    pub test: Expr,
    pub body: Box<Stmt>,
}

/// A "c-style" for loop
/// ```js
/// for (var i = 0; i < 100; i++) console.log(i);
/// for (;;) {
///     console.log('forever!');
/// }
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct ForStmt {
    pub init: Option<LoopInit>,
    pub test: Option<Expr>,
    pub update: Option<Expr>,
    pub body: Box<Stmt>,
}

/// The left most triple of a for loops parenthetical
/// ```js
///  //  vvvvvvvvv
/// for (var i = 0;i < 100; i++)
#[derive(PartialEq, Debug, Clone)]
pub enum LoopInit {
    Variable(VariableKind, Vec<VariableDecl>),
    Expr(Expr),
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
pub struct ForInStmt {
    pub left: LoopLeft,
    pub right: Expr,
    pub body: Box<Stmt>,
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
pub struct ForOfStmt {
    pub left: LoopLeft,
    pub right: Expr,
    pub body: Box<Stmt>,
    pub is_await: bool,
}

/// The values on the left hand side of the keyword
/// in a for in or for of loop
#[derive(PartialEq, Debug, Clone)]
pub enum LoopLeft {
    Expr(Expr),
    Variable(VariableKind, VariableDecl),
    Pat(Pat),
}
