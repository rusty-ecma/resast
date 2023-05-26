use crate::decl::VarDecl;
use crate::expr::Expr;
use crate::pat::Pat;
use crate::VarKind;
use crate::{Ident, ProgramPart};
/// A slightly more granular part of an es program than ProgramPart
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    all(feature = "serde", not(feature = "esprima")),
    derive(Deserialize, Serialize)
)]
#[cfg_attr(all(feature = "serde", feature = "esprima"), derive(Deserialize))]
pub enum Stmt<T> {
    /// Any expression
    Expr(Expr<T>),
    /// A collection of program parts wrapped in curly braces
    Block(BlockStmt<T>),
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
    With(WithStmt<T>),
    /// A return statement
    /// ```js
    /// function thing() {
    ///     return 'stuff';
    /// }
    /// function stuff() {
    ///     return;
    /// }
    Return(Option<Expr<T>>),
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
    Break(Option<Ident<T>>),
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
    Continue(Option<Ident<T>>),
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
    Throw(Expr<T>),
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
    Var(Vec<VarDecl<T>>),
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
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub struct WithStmt<T> {
    pub object: Expr<T>,
    pub body: Box<Stmt<T>>,
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
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub struct LabeledStmt<T> {
    pub label: Ident<T>,
    pub body: Box<Stmt<T>>,
}

/// An if statement
/// ```js
/// if (1 < 2) {
///     console.log(Tlways true');
/// } else {
///     console.log('Never true');
/// }
/// ```
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub struct IfStmt<T> {
    pub test: Expr<T>,
    pub consequent: Box<Stmt<T>>,
    pub alternate: Option<Box<Stmt<T>>>,
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
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub struct SwitchStmt<T> {
    pub discriminant: Expr<T>,
    pub cases: Vec<SwitchCase<T>>,
}

/// A single case part of a switch statement
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    all(feature = "serde", not(feature = "esprima")),
    derive(Deserialize, Serialize)
)]
#[cfg_attr(all(feature = "serde", feature = "esprima"), derive(Deserialize))]
pub struct SwitchCase<T> {
    pub test: Option<Expr<T>>,
    pub consequent: Vec<ProgramPart<T>>,
}

/// A collection of program parts wrapped in curly braces
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    all(feature = "serde", not(feature = "esprima")),
    derive(Deserialize, Serialize)
)]
#[cfg_attr(all(feature = "serde", feature = "esprima"), derive(Deserialize))]
pub struct BlockStmt<T>(pub Vec<ProgramPart<T>>);

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
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub struct TryStmt<T> {
    pub block: BlockStmt<T>,
    pub handler: Option<CatchClause<T>>,
    pub finalizer: Option<BlockStmt<T>>,
}

/// The error handling part of a `TryStmt`
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    all(feature = "serde", not(feature = "esprima")),
    derive(Deserialize, Serialize)
)]
#[cfg_attr(all(feature = "serde", feature = "esprima"), derive(Deserialize))]
pub struct CatchClause<T> {
    pub param: Option<Pat<T>>,
    pub body: BlockStmt<T>,
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
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub struct WhileStmt<T> {
    pub test: Expr<T>,
    pub body: Box<Stmt<T>>,
}

/// A while loop that executes its body first
/// ```js
/// do {
///     console.log(Tt least once')
/// } while (Math.floor(Math.random() * 100) < 75)
/// ```
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub struct DoWhileStmt<T> {
    pub test: Expr<T>,
    pub body: Box<Stmt<T>>,
}

/// A "c-style" for loop
/// ```js
/// for (var i = 0; i < 100; i++) console.log(i);
/// for (;;) {
///     console.log('forever!');
/// }
/// ```
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub struct ForStmt<T> {
    pub init: Option<LoopInit<T>>,
    pub test: Option<Expr<T>>,
    pub update: Option<Expr<T>>,
    pub body: Box<Stmt<T>>,
}

/// The left most triple of a for loops parenthetical
/// ```js
///  //  vvvvvvvvv
/// for (var i = 0;i < 100; i++)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    all(feature = "serde", not(feature = "esprima")),
    derive(Deserialize, Serialize)
)]
#[cfg_attr(all(feature = "serde", feature = "esprima"), derive(Deserialize))]
pub enum LoopInit<T> {
    Variable(VarKind, Vec<VarDecl<T>>),
    Expr(Expr<T>),
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
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub struct ForInStmt<T> {
    pub left: LoopLeft<T>,
    pub right: Expr<T>,
    pub body: Box<Stmt<T>>,
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
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub struct ForOfStmt<T> {
    pub left: LoopLeft<T>,
    pub right: Expr<T>,
    pub body: Box<Stmt<T>>,
    pub is_await: bool,
}

/// The values on the left hand side of the keyword
/// in a for in or for of loop
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    all(feature = "serde", not(feature = "esprima")),
    derive(Deserialize, Serialize)
)]
#[cfg_attr(all(feature = "serde", feature = "esprima"), derive(Deserialize))]
pub enum LoopLeft<T> {
    Expr(Expr<T>),
    Variable(VarKind, VarDecl<T>),
    Pat(Pat<T>),
}
