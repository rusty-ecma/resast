use crate::VarKind;
use crate::decl::VarDecl;
use crate::expr::Expr;
use crate::pat::Pat;
use crate::{Ident, ProgramPart};
/// A slightly more granular part of an es program than ProgramPart
#[derive(PartialEq, Debug, Clone, Deserialize)]
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
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct WithStmt<'a> {
    pub object: Expr<'a>,
    pub body: Box<Stmt<'a>>,
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
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct LabeledStmt<'a> {
    pub label: Ident<'a>,
    pub body: Box<Stmt<'a>>,
}

/// An if statement
/// ```js
/// if (1 < 2) {
///     console.log('Always true');
/// } else {
///     console.log('Never true');
/// }
/// ```
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct IfStmt<'a> {
    pub test: Expr<'a>,
    pub consequent: Box<Stmt<'a>>,
    pub alternate: Option<Box<Stmt<'a>>>,
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
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct SwitchStmt<'a> {
    pub discriminant: Expr<'a>,
    pub cases: Vec<SwitchCase<'a>>,
}

/// A single case part of a switch statement
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct SwitchCase<'a> {
    pub test: Option<Expr<'a>>,
    pub consequent: Vec<ProgramPart<'a>>,
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
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct TryStmt<'a> {
    pub block: BlockStmt<'a>,
    pub handler: Option<CatchClause<'a>>,
    pub finalizer: Option<BlockStmt<'a>>,
}

/// The error handling part of a `TryStmt`
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct CatchClause<'a> {
    pub param: Option<Pat<'a>>,
    pub body: BlockStmt<'a>,
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
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct WhileStmt<'a> {
    pub test: Expr<'a>,
    pub body: Box<Stmt<'a>>,
}

/// A while loop that executes its body first
/// ```js
/// do {
///     console.log('at least once')
/// } while (Math.floor(Math.random() * 100) < 75)
/// ```
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct DoWhileStmt<'a> {
    pub test: Expr<'a>,
    pub body: Box<Stmt<'a>>,
}

/// A "c-style" for loop
/// ```js
/// for (var i = 0; i < 100; i++) console.log(i);
/// for (;;) {
///     console.log('forever!');
/// }
/// ```
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct ForStmt<'a> {
    pub init: Option<LoopInit<'a>>,
    pub test: Option<Expr<'a>>,
    pub update: Option<Expr<'a>>,
    pub body: Box<Stmt<'a>>,
}

/// The left most triple of a for loops parenthetical
/// ```js
///  //  vvvvvvvvv
/// for (var i = 0;i < 100; i++)
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum LoopInit<'a> {
    Variable(VarKind, Vec<VarDecl<'a>>),
    Expr(Expr<'a>),
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
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct ForInStmt<'a> {
    pub left: LoopLeft<'a>,
    pub right: Expr<'a>,
    pub body: Box<Stmt<'a>>,
}

/// A for of statement, this kind of for statement
/// will extract the value from a generator or iterator
/// ```js
/// for (var k of [2, 3, 4, 5, 6]) {
///     console.log(k);
/// }
/// //prints 2, 3, 4, 5, 6
/// ```
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct ForOfStmt<'a> {
    pub left: LoopLeft<'a>,
    pub right: Expr<'a>,
    pub body: Box<Stmt<'a>>,
    pub is_await: bool,
}

/// The values on the left hand side of the keyword
/// in a for in or for of loop
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum LoopLeft<'a> {
    Expr(Expr<'a>),
    Variable(VarKind, VarDecl<'a>),
    Pat(Pat<'a>),
}
