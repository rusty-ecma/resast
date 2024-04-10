pub mod decl;
pub mod expr;
pub mod pat;
pub mod spanned;
pub mod stmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use std::{borrow::Cow, fmt::Debug};

use decl::Decl;
use expr::{Expr, Lit, Prop};
use pat::Pat;
use stmt::Stmt;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Ident<T> {
    pub name: T,
}

impl<T> IntoAllocated for Ident<T>
where
    T: ToString,
{
    type Allocated = Ident<String>;

    fn into_allocated(self) -> Self::Allocated {
        Ident {
            name: self.name.to_string(),
        }
    }
}

impl<'a> From<&'a str> for Ident<&'a str> {
    fn from(value: &'a str) -> Self {
        Self { name: value }
    }
}

impl From<String> for Ident<String> {
    fn from(value: String) -> Self {
        Self { name: value }
    }
}

impl<'a> From<Cow<'a, str>> for Ident<Cow<'a, str>> {
    fn from(value: Cow<'a, str>) -> Self {
        Self { name: value }
    }
}

/// A fully parsed javascript program.
///
/// It is essentially a collection of `ProgramPart`s
/// with a flag denoting if the representation is
/// a ES6 Mod or a Script.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Program<T> {
    /// An ES6 Mod
    Mod(Vec<ProgramPart<T>>),
    /// Not an ES6 Mod
    Script(Vec<ProgramPart<T>>),
}

impl<T> IntoAllocated for Program<T>
where
    T: ToString,
{
    type Allocated = Program<String>;

    fn into_allocated(self) -> Self::Allocated {
        match self {
            Program::Mod(inner) => {
                Program::Mod(inner.into_iter().map(|p| p.into_allocated()).collect())
            }
            Program::Script(inner) => {
                Program::Script(inner.into_iter().map(|p| p.into_allocated()).collect())
            }
        }
    }
}

impl<T> Program<T> {
    pub fn module(parts: Vec<ProgramPart<T>>) -> Self {
        Program::Mod(parts)
    }
    pub fn script(parts: Vec<ProgramPart<T>>) -> Self {
        Program::Script(parts)
    }
}

/// A single part of a Javascript program.
/// This will be either a Directive, Decl or a Stmt
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum ProgramPart<T> {
    /// A Directive like `'use strict';`
    Dir(Dir<T>),
    /// A variable, function or module declaration
    Decl(Decl<T>),
    /// Any other kind of statement
    Stmt(Stmt<T>),
}

impl<T> IntoAllocated for ProgramPart<T>
where
    T: ToString,
{
    type Allocated = ProgramPart<String>;

    fn into_allocated(self) -> Self::Allocated {
        match self {
            ProgramPart::Dir(inner) => ProgramPart::Dir(inner.into_allocated()),
            ProgramPart::Decl(inner) => ProgramPart::Decl(inner.into_allocated()),
            ProgramPart::Stmt(inner) => ProgramPart::Stmt(inner.into_allocated()),
        }
    }
}

impl<T> ProgramPart<T> {
    pub fn decl(inner: Decl<T>) -> Self {
        ProgramPart::Decl(inner)
    }
    pub fn stmt(inner: Stmt<T>) -> Self {
        ProgramPart::Stmt(inner)
    }
}

/// pretty much always `'use strict'`, this can appear at the
/// top of a file or function
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Dir<T> {
    pub expr: Lit<T>,
    pub dir: T,
}

impl<T> IntoAllocated for Dir<T>
where
    T: ToString,
{
    type Allocated = Dir<String>;

    fn into_allocated(self) -> Self::Allocated {
        Dir {
            expr: self.expr.into_allocated(),
            dir: self.dir.to_string(),
        }
    }
}

/// A function, this will be part of either a function
/// declaration (ID is required) or a function expression
/// (ID is optional)
/// ```js
/// //function declaration
/// function thing() {}
/// //function expressions
/// var x = function() {}
/// let y = function q() {}
/// ```
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Func<T> {
    pub id: Option<Ident<T>>,
    pub params: Vec<FuncArg<T>>,
    pub body: FuncBody<T>,
    pub generator: bool,
    pub is_async: bool,
}

impl<T> IntoAllocated for Func<T>
where
    T: ToString,
{
    type Allocated = Func<String>;

    fn into_allocated(self) -> Self::Allocated {
        Func {
            id: self.id.map(IntoAllocated::into_allocated),
            params: self
                .params
                .into_iter()
                .map(|p| p.into_allocated())
                .collect(),
            body: self.body.into_allocated(),
            generator: self.generator,
            is_async: self.is_async,
        }
    }
}

impl<T> Func<T> {
    pub fn new(
        id: Option<Ident<T>>,
        params: Vec<FuncArg<T>>,
        body: FuncBody<T>,
        generator: bool,
        is_async: bool,
    ) -> Self {
        Func {
            id,
            params,
            body,
            generator,
            is_async,
        }
    }
}

/// A single function argument from a function signature
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum FuncArg<T> {
    Expr(Expr<T>),
    Pat(Pat<T>),
}

impl<T> IntoAllocated for FuncArg<T>
where
    T: ToString,
{
    type Allocated = FuncArg<String>;

    fn into_allocated(self) -> Self::Allocated {
        match self {
            FuncArg::Expr(inner) => FuncArg::Expr(inner.into_allocated()),
            FuncArg::Pat(inner) => FuncArg::Pat(inner.into_allocated()),
        }
    }
}

impl<T> FuncArg<T> {
    pub fn expr(expr: Expr<T>) -> FuncArg<T> {
        FuncArg::Expr(expr)
    }
    pub fn pat(pat: Pat<T>) -> FuncArg<T> {
        FuncArg::Pat(pat)
    }
}

/// The block statement that makes up the function's body
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct FuncBody<T>(pub Vec<ProgramPart<T>>);

impl<T> IntoAllocated for FuncBody<T>
where
    T: ToString,
{
    type Allocated = FuncBody<String>;

    fn into_allocated(self) -> Self::Allocated {
        FuncBody(self.0.into_iter().map(|p| p.into_allocated()).collect())
    }
}

/// A way to declare object templates
/// ```js
/// class Thing {
///     constructor() {
///         this._a = 0;
///     }
///     stuff() {
///         return 'stuff'
///     }
///     set a(value) {
///         if (value > 100) {
///             this._a = 0;
///         } else {
///             this._a = value;
///         }
///     }
///     get a() {
///         return this._a;
///     }
/// }
/// let y = class {
///     constructor() {
///         this.a = 100;
///     }
/// }
/// ```
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Class<T> {
    pub id: Option<Ident<T>>,
    pub super_class: Option<Box<Expr<T>>>,
    pub body: ClassBody<T>,
}

impl<T> IntoAllocated for Class<T>
where
    T: ToString,
{
    type Allocated = Class<String>;

    fn into_allocated(self) -> Self::Allocated {
        Class {
            id: self.id.map(IntoAllocated::into_allocated),
            super_class: self.super_class.map(IntoAllocated::into_allocated),
            body: self.body.into_allocated(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ClassBody<T>(pub Vec<Prop<T>>);

impl<T> IntoAllocated for ClassBody<T>
where
    T: ToString,
{
    type Allocated = ClassBody<String>;

    fn into_allocated(self) -> Self::Allocated {
        ClassBody(
            self.0
                .into_iter()
                .map(IntoAllocated::into_allocated)
                .collect(),
        )
    }
}

impl<T> Class<T> {
    pub fn new(id: Option<Ident<T>>, super_class: Option<Expr<T>>, body: Vec<Prop<T>>) -> Class<T> {
        Class {
            id,
            super_class: super_class.map(Box::new),
            body: ClassBody(body),
        }
    }
}

/// The ways to access the member of a value
/// Either a Period `.`, Computed `[ ]`, Optional `?.` or optional computed `?.[ ]`
#[derive(Debug, Clone, PartialEq, Copy)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum MemberIndexer {
    Period,
    Computed,
    Optional,
    OptionalComputed,
}

/// The kind of variable being defined (`var`/`let`/`const`)
#[derive(Debug, Clone, PartialEq, Copy)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(
    all(feature = "serde", feature = "esprima"),
    serde(rename_all = "camelCase", untagged)
)]
pub enum VarKind {
    Var,
    Let,
    Const,
}

/// The available operators for assignment Exprs
#[derive(Debug, Clone, PartialEq, Copy)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum AssignOp {
    Equal,
    PlusEqual,
    MinusEqual,
    TimesEqual,
    DivEqual,
    ModEqual,
    LeftShiftEqual,
    RightShiftEqual,
    UnsignedRightShiftEqual,
    OrEqual,
    XOrEqual,
    AndEqual,
    PowerOfEqual,
    DoubleAmpersandEqual,
    DoublePipeEqual,
    DoubleQuestionmarkEqual,
}

/// The available logical operators
#[derive(Debug, Clone, PartialEq, Copy)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum LogicalOp {
    Or,
    And,
    NullishCoalescing,
}

/// The available operations for `Binary` Exprs
#[derive(Debug, Clone, PartialEq, Copy)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum BinaryOp {
    Equal,
    NotEqual,
    StrictEqual,
    StrictNotEqual,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,
    LeftShift,
    RightShift,
    UnsignedRightShift,
    Plus,
    Minus,
    Times,
    Over,
    Mod,
    Or,
    XOr,
    And,
    In,
    InstanceOf,
    PowerOf,
}

/// `++` or `--`
#[derive(Debug, Clone, PartialEq, Copy)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum UpdateOp {
    Increment,
    Decrement,
}

/// The allowed operators for an Expr
/// to be `Unary`
#[derive(Debug, Clone, PartialEq, Copy)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum UnaryOp {
    Minus,
    Plus,
    Not,
    Tilde,
    TypeOf,
    Void,
    Delete,
}

/// A flag for determining what kind of property
#[derive(Debug, Clone, PartialEq, Copy)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum PropKind {
    /// A property with a value
    Init,
    /// A method with the get keyword
    Get,
    /// A method with the set keyword
    Set,
    /// A constructor
    Ctor,
    /// A standard method
    Method,
}

pub trait IntoAllocated {
    type Allocated;

    fn into_allocated(self) -> Self::Allocated;
}

impl<T> IntoAllocated for Box<T>
where
    T: IntoAllocated,
{
    type Allocated = Box<T::Allocated>;

    fn into_allocated(self) -> Self::Allocated {
        Box::new((*self).into_allocated())
    }
}

impl<T> IntoAllocated for Option<T>
where
    T: IntoAllocated,
{
    type Allocated = Option<T::Allocated>;
    fn into_allocated(self) -> Self::Allocated {
        self.map(IntoAllocated::into_allocated)
    }
}

pub mod prelude {
    pub use crate::decl::{
        Decl, DefaultExportDecl, ExportSpecifier, ImportSpecifier, ModExport, ModImport,
        NamedExportDecl, NormalImportSpec, VarDecl,
    };
    pub use crate::expr::{
        ArrayExpr, ArrowFuncBody, ArrowFuncExpr, AssignExpr, AssignLeft, BinaryExpr, CallExpr,
        ConditionalExpr, Expr, Lit, LogicalExpr, MemberExpr, MetaProp, NewExpr, ObjExpr, ObjProp,
        Prop, PropKey, PropValue, RegEx, StringLit, TaggedTemplateExpr, TemplateElement,
        TemplateLit, UnaryExpr, UpdateExpr, YieldExpr,
    };
    pub use crate::pat::{ArrayPatPart, AssignPat, ObjPat, ObjPatPart, Pat};
    pub use crate::stmt::{
        BlockStmt, CatchClause, DoWhileStmt, ForInStmt, ForOfStmt, ForStmt, IfStmt, LabeledStmt,
        LoopInit, LoopLeft, Stmt, SwitchCase, SwitchStmt, TryStmt, WhileStmt, WithStmt,
    };
    pub use crate::{
        AssignOp, BinaryOp, Class, ClassBody, Dir, Func, FuncArg, FuncBody, Ident, LogicalOp,
        Program, ProgramPart, PropKind, UnaryOp, UpdateOp, VarKind,
    };
}
