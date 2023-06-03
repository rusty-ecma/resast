#[cfg(feature = "serde")]
#[macro_use]
extern crate serde_derive;

pub mod decl;
pub mod expr;
pub mod pat;
#[cfg(feature = "esprima")]
pub mod serde;
pub mod spanned;
pub mod stmt;

use std::{ops::Deref, borrow::Cow};

use decl::Decl;
use expr::{Expr, Lit, Prop};
use pat::Pat;
use stmt::Stmt;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SourceText<T>(T);

impl<T> From<T> for SourceText<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl Deref for SourceText<&str> {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl Deref for SourceText<String> {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}

impl<T> AsRef<str> for SourceText<T> 
where T: AsRef<str> {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    all(feature = "serde", not(feature = "esprima")),
    derive(Deserialize, Serialize)
)]
#[cfg_attr(all(feature = "serde", feature = "esprima"), derive(Deserialize))]
pub struct Ident<T> {
    pub name: SourceText<T>,
}

impl<'a> From<&'a str> for Ident<&'a str> {
    fn from(value: &'a str) -> Self {
        Self {
            name: SourceText(value),
        }
    }
}

impl From<String> for Ident<String> {
    fn from(value: String) -> Self {
        Self {
            name: SourceText(value),
        }
    }
}

impl<'a> From<Cow<'a, str>> for Ident<Cow<'a, str>> {
    fn from(value: Cow<'a, str>) -> Self {
        Self {
            name: SourceText(value),
        }
    }
}

/// A fully parsed javascript program.
///
/// It is essentially a collection of `ProgramPart`s
/// with a flag denoting if the representation is
/// a ES6 Mod or a Script.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    all(feature = "serde", not(feature = "esprima")),
    derive(Deserialize, Serialize)
)]
#[cfg_attr(all(feature = "serde", feature = "esprima"), derive(Deserialize))]
pub enum Program<T> {
    /// An ES6 Mod
    Mod(Vec<ProgramPart<T>>),
    /// Not an ES6 Mod
    Script(Vec<ProgramPart<T>>),
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
#[cfg_attr(
    all(feature = "serde", not(feature = "esprima")),
    derive(Deserialize, Serialize)
)]
#[cfg_attr(all(feature = "serde", feature = "esprima"), derive(Deserialize))]
#[cfg_attr(all(feature = "serde", feature = "esprima"), serde(untagged))]
pub enum ProgramPart<T> {
    /// A Directive like `'use strict';`
    Dir(Dir<T>),
    /// A variable, function or module declaration
    Decl(Decl<T>),
    /// Any other kind of statement
    Stmt(Stmt<T>),
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
#[cfg_attr(
    all(feature = "serde", not(feature = "esprima")),
    derive(Deserialize, Serialize)
)]
#[cfg_attr(all(feature = "serde", feature = "esprima"), derive(Deserialize))]
pub struct Dir<T> {
    pub expr: Lit<T>,
    pub dir: SourceText<T>,
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
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub struct Func<T> {
    pub id: Option<Ident<T>>,
    pub params: Vec<FuncArg<T>>,
    pub body: FuncBody<T>,
    pub generator: bool,
    pub is_async: bool,
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
#[cfg_attr(
    all(feature = "serde", not(feature = "esprima")),
    derive(Deserialize, Serialize)
)]
#[cfg_attr(all(feature = "serde", feature = "esprima"), derive(Deserialize))]
#[cfg_attr(all(feature = "serde", feature = "esprima"), serde(untagged))]
pub enum FuncArg<T> {
    Expr(Expr<T>),
    Pat(Pat<T>),
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
#[cfg_attr(
    all(feature = "serde", not(feature = "esprima")),
    derive(Deserialize, Serialize)
)]
#[cfg_attr(all(feature = "serde", feature = "esprima"), derive(Deserialize))]
pub struct FuncBody<T>(pub Vec<ProgramPart<T>>);
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
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub struct Class<T> {
    pub id: Option<Ident<T>>,
    pub super_class: Option<Box<Expr<T>>>,
    pub body: ClassBody<T>,
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    all(feature = "serde", not(feature = "esprima")),
    derive(Deserialize, Serialize)
)]
#[cfg_attr(all(feature = "serde", feature = "esprima"), derive(Deserialize))]
pub struct ClassBody<T>(pub Vec<Prop<T>>);

impl<T> Class<T> {
    pub fn new(id: Option<Ident<T>>, super_class: Option<Expr<T>>, body: Vec<Prop<T>>) -> Class<T> {
        Class {
            id,
            super_class: super_class.map(Box::new),
            body: ClassBody(body),
        }
    }
}

/// The kind of variable being defined (`var`/`let`/`const`)
#[derive(Debug, Clone, PartialEq, Copy)]
#[cfg_attr(
    all(feature = "serde", not(feature = "esprima")),
    derive(Deserialize, Serialize)
)]
#[cfg_attr(all(feature = "serde", feature = "esprima"), derive(Deserialize))]
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
#[cfg_attr(
    all(feature = "serde", not(feature = "esprima")),
    derive(Deserialize, Serialize)
)]
#[cfg_attr(all(feature = "serde", feature = "esprima"), derive(Deserialize))]
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
}

/// The available logical operators
#[derive(Debug, Clone, PartialEq, Copy)]
#[cfg_attr(
    all(feature = "serde", not(feature = "esprima")),
    derive(Deserialize, Serialize)
)]
#[cfg_attr(all(feature = "serde", feature = "esprima"), derive(Deserialize))]
pub enum LogicalOp {
    Or,
    And,
}

/// The available operations for `Binary` Exprs
#[derive(Debug, Clone, PartialEq, Copy)]
#[cfg_attr(
    all(feature = "serde", not(feature = "esprima")),
    derive(Deserialize, Serialize)
)]
#[cfg_attr(all(feature = "serde", feature = "esprima"), derive(Deserialize))]
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
#[cfg_attr(
    all(feature = "serde", not(feature = "esprima")),
    derive(Deserialize, Serialize)
)]
#[cfg_attr(all(feature = "serde", feature = "esprima"), derive(Deserialize))]
pub enum UpdateOp {
    Increment,
    Decrement,
}

/// The allowed operators for an Expr
/// to be `Unary`
#[derive(Debug, Clone, PartialEq, Copy)]
#[cfg_attr(
    all(feature = "serde", not(feature = "esprima")),
    derive(Deserialize, Serialize)
)]
#[cfg_attr(all(feature = "serde", feature = "esprima"), derive(Deserialize))]
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
#[cfg_attr(
    all(feature = "serde", not(feature = "esprima")),
    derive(Deserialize, Serialize)
)]
#[cfg_attr(all(feature = "serde", feature = "esprima"), derive(Deserialize))]
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
