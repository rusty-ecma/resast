use crate::decl::VarDecl;
use crate::expr::Expr;
use crate::pat::Pat;
use crate::{VarKind, IntoAllocated};
use crate::{Ident, ProgramPart};
/// A slightly more granular part of an es program than ProgramPart
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize)
)]
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

impl<T> IntoAllocated for Stmt<T> where T: ToString {
    type Allocated = Stmt<String>;

    fn into_allocated(self) -> Self::Allocated {
        match self {
            Stmt::Expr(inner) => Stmt::Expr(inner.into_allocated()),
            Stmt::Block(inner) => Stmt::Block(inner.into_allocated()),
            Stmt::Empty => Stmt::Empty,
            Stmt::Debugger => Stmt::Debugger,
            Stmt::With(inner) => Stmt::With(inner.into_allocated()),
            Stmt::Return(inner) => Stmt::Return(inner.map(IntoAllocated::into_allocated)),
            Stmt::Labeled(inner) => Stmt::Labeled(inner.into_allocated()),
            Stmt::Break(inner) => Stmt::Break(inner.map(IntoAllocated::into_allocated)),
            Stmt::Continue(inner) => Stmt::Continue(inner.map(IntoAllocated::into_allocated)),
            Stmt::If(inner) => Stmt::If(inner.into_allocated()),
            Stmt::Switch(inner) => Stmt::Switch(inner.into_allocated()),
            Stmt::Throw(inner) => Stmt::Throw(inner.into_allocated()),
            Stmt::Try(inner) => Stmt::Try(inner.into_allocated()),
            Stmt::While(inner) => Stmt::While(inner.into_allocated()),
            Stmt::DoWhile(inner) => Stmt::DoWhile(inner.into_allocated()),
            Stmt::For(inner) => Stmt::For(inner.into_allocated()),
            Stmt::ForIn(inner) => Stmt::ForIn(inner.into_allocated()),
            Stmt::ForOf(inner) => Stmt::ForOf(inner.into_allocated()),
            Stmt::Var(inner) => Stmt::Var(inner.into_iter().map(|v| v.into_allocated()).collect()),
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
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub struct WithStmt<T> {
    pub object: Expr<T>,
    pub body: Box<Stmt<T>>,
}

impl<T> IntoAllocated for WithStmt<T> where T: ToString {
    type Allocated = WithStmt<String>;

    fn into_allocated(self) -> Self::Allocated {
        WithStmt {
            object: self.object.into_allocated(),
            body: self.body.into_allocated(),
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
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub struct LabeledStmt<T> {
    pub label: Ident<T>,
    pub body: Box<Stmt<T>>,
}

impl<T> IntoAllocated for LabeledStmt<T> where T: ToString {
    type Allocated = LabeledStmt<String>;

    fn into_allocated(self) -> Self::Allocated {
        LabeledStmt {
            label: self.label.into_allocated(),
            body: self.body.into_allocated(),
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
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub struct IfStmt<T> {
    pub test: Expr<T>,
    pub consequent: Box<Stmt<T>>,
    pub alternate: Option<Box<Stmt<T>>>,
}

impl<T> IntoAllocated for IfStmt<T> where T: ToString {
    type Allocated = IfStmt<String>;

    fn into_allocated(self) -> Self::Allocated {
        IfStmt {
            test: self.test.into_allocated(),
            consequent: self.consequent.into_allocated(),
            alternate: self.alternate.map(IntoAllocated::into_allocated),
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
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub struct SwitchStmt<T> {
    pub discriminant: Expr<T>,
    pub cases: Vec<SwitchCase<T>>,
}

impl<T> IntoAllocated for SwitchStmt<T> where T: ToString {
    type Allocated = SwitchStmt<String>;

    fn into_allocated(self) -> Self::Allocated {
        SwitchStmt {
            discriminant: self.discriminant.into_allocated(),
            cases: self.cases.into_iter().map(|c| c.into_allocated()).collect(),
        }
    }
}

/// A single case part of a switch statement
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize)
)]
pub struct SwitchCase<T> {
    pub test: Option<Expr<T>>,
    pub consequent: Vec<ProgramPart<T>>,
}

impl<T> IntoAllocated for SwitchCase<T> where T: ToString {
    type Allocated = SwitchCase<String>;

    fn into_allocated(self) -> Self::Allocated {
        SwitchCase {
            test: self.test.map(IntoAllocated::into_allocated),
            consequent: self.consequent.into_iter().map(|c| c.into_allocated()).collect(),
        }
    }
}

/// A collection of program parts wrapped in curly braces
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize)
)]
pub struct BlockStmt<T>(pub Vec<ProgramPart<T>>);

impl<T> IntoAllocated for BlockStmt<T> where T: ToString {
    type Allocated = BlockStmt<String>;

    fn into_allocated(self) -> Self::Allocated {
        BlockStmt(self.0.into_iter().map(|s| s.into_allocated()).collect())
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
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub struct TryStmt<T> {
    pub block: BlockStmt<T>,
    pub handler: Option<CatchClause<T>>,
    pub finalizer: Option<BlockStmt<T>>,
}

impl<T> IntoAllocated for TryStmt<T> where T: ToString {
    type Allocated = TryStmt<String>;

    fn into_allocated(self) -> Self::Allocated {
        TryStmt {
            block: self.block.into_allocated(),
            handler: self.handler.map(|h| h.into_allocated()),
            finalizer: self.finalizer.map(|f| f.into_allocated()),
        }
    }
}

/// The error handling part of a `TryStmt`
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize)
)]
pub struct CatchClause<T> {
    pub param: Option<Pat<T>>,
    pub body: BlockStmt<T>,
}

impl<T> IntoAllocated for CatchClause<T> where T: ToString {
    type Allocated = CatchClause<String>;

    fn into_allocated(self) -> Self::Allocated {
        CatchClause {
            param: self.param.map(IntoAllocated::into_allocated),
            body: self.body.into_allocated(),
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
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub struct WhileStmt<T> {
    pub test: Expr<T>,
    pub body: Box<Stmt<T>>,
}

impl<T> IntoAllocated for WhileStmt<T> where T: ToString {
    type Allocated = WhileStmt<String>;

    fn into_allocated(self) -> Self::Allocated {
        WhileStmt {
            test: self.test.into_allocated(),
            body: self.body.into_allocated(),
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
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub struct DoWhileStmt<T> {
    pub test: Expr<T>,
    pub body: Box<Stmt<T>>,
}

impl<T> IntoAllocated for DoWhileStmt<T> where T: ToString {
    type Allocated = DoWhileStmt<String>;

    fn into_allocated(self) -> Self::Allocated {
        DoWhileStmt {
            test: self.test.into_allocated(),
            body: self.body.into_allocated()
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
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub struct ForStmt<T> {
    pub init: Option<LoopInit<T>>,
    pub test: Option<Expr<T>>,
    pub update: Option<Expr<T>>,
    pub body: Box<Stmt<T>>,
}

impl<T> IntoAllocated for ForStmt<T> where T: ToString {
    type Allocated = ForStmt<String>;

    fn into_allocated(self) -> Self::Allocated {
        ForStmt {
            init: self.init.map(|i| i.into_allocated()),
            test: self.test.map(|t| t.into_allocated()),
            update: self.update.map(|u| u.into_allocated()),
            body: self.body.into_allocated(),
        }
    }
}

/// The left most triple of a for loops parenthetical
/// ```js
///  //  vvvvvvvvv
/// for (var i = 0;i < 100; i++)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize)
)]
pub enum LoopInit<T> {
    Variable(VarKind, Vec<VarDecl<T>>),
    Expr(Expr<T>),
}

impl<T> IntoAllocated for LoopInit<T> where T: ToString {
    type Allocated = LoopInit<String>;

    fn into_allocated(self) -> Self::Allocated {
        match self {
            LoopInit::Variable(k, v) => LoopInit::Variable(k, v.into_iter().map(|v| v.into_allocated()).collect()),
            LoopInit::Expr(inner) => LoopInit::Expr(inner.into_allocated()),
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
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub struct ForInStmt<T> {
    pub left: LoopLeft<T>,
    pub right: Expr<T>,
    pub body: Box<Stmt<T>>,
}

impl<T> IntoAllocated for ForInStmt<T> where T: ToString {
    type Allocated = ForInStmt<String>;

    fn into_allocated(self) -> Self::Allocated {
        ForInStmt {
            left: self.left.into_allocated(),
            right: self.right.into_allocated(),
            body: self.body.into_allocated(),
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
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub struct ForOfStmt<T> {
    pub left: LoopLeft<T>,
    pub right: Expr<T>,
    pub body: Box<Stmt<T>>,
    pub is_await: bool,
}

impl<T> IntoAllocated for ForOfStmt<T> where T: ToString {
    type Allocated = ForOfStmt<String>;

    fn into_allocated(self) -> Self::Allocated {
        ForOfStmt {
            left: self.left.into_allocated(),
            right: self.right.into_allocated(),
            body: self.body.into_allocated(),
            is_await: self.is_await,
        }
    }
}

/// The values on the left hand side of the keyword
/// in a for in or for of loop
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize)
)]
pub enum LoopLeft<T> {
    Expr(Expr<T>),
    Variable(VarKind, VarDecl<T>),
    Pat(Pat<T>),
}

impl<T> IntoAllocated for LoopLeft<T> where T: ToString {
    type Allocated = LoopLeft<String>;

    fn into_allocated(self) -> Self::Allocated {
        match self {
            LoopLeft::Expr(inner) => LoopLeft::Expr(inner.into_allocated()),
            LoopLeft::Variable(k, v) => LoopLeft::Variable(k, v.into_allocated()),
            LoopLeft::Pat(inner) => LoopLeft::Pat(inner.into_allocated()),
        }
    }
}
