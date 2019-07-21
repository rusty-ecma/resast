#[macro_use]
extern crate serde_derive;

use std::borrow::Cow;

pub mod decl;
pub mod expr;
pub mod pat;
pub mod stmt;
pub mod serde;

use decl::Decl;
use expr::{Expr, Lit, Prop};
use pat::Pat;
use stmt::Stmt;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Ident<'a> {
    pub name: Cow<'a, str>,
}

impl<'a> Ident<'a> {
    pub fn new(s: String) -> Self {
        Ident {
            name: Cow::Owned(s)
        }
    }
    pub fn from(s: &'a str) -> Self {
        Ident {
            name: Cow::Borrowed(s)
        }
    }
}

/// A fully parsed javascript program.
///
/// It is essentially a collection of `ProgramPart`s
/// with a flag denoting if the representation is
/// a ES6 Mod or a Script.
#[derive(PartialEq, Debug, Deserialize)]
pub enum Program<'a> {
    /// An ES6 Mod
    Mod(Vec<ProgramPart<'a>>),
    /// Not an ES6 Mod
    Script(Vec<ProgramPart<'a>>),
}

impl<'a> Program<'a> {
    pub fn module(parts: Vec<ProgramPart<'a>>) -> Self {
        Program::Mod(parts)
    }
    pub fn script(parts: Vec<ProgramPart<'a>>) -> Self {
        Program::Script(parts)
    }
}

/// A single part of a Javascript program.
/// This will be either a Directive, Decl or a Stmt
#[derive(PartialEq, Debug, Clone, Deserialize)]
#[serde(untagged)] 
pub enum ProgramPart<'a> {
    /// A Directive like `'use strict';`
    Dir(Dir<'a>),
    /// A variable, function or module declaration
    Decl(Decl<'a>),
    /// Any other kind of statement
    Stmt(Stmt<'a>),
}

impl<'a> ProgramPart<'a> {
    pub fn decl(inner: Decl<'a>) -> Self {
        ProgramPart::Decl(inner)
    }
    pub fn stmt(inner: Stmt<'a>) -> Self {
        ProgramPart::Stmt(inner)
    }
}

/// pretty much always `'use strict'`, this can appear at the
/// top of a file or function
#[derive(PartialEq, Debug, Clone, Deserialize)]
pub struct Dir<'a> {
    pub expr: Lit<'a>,
    pub dir: Cow<'a, str>,
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
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Func<'a> {
    pub id: Option<Ident<'a>>,
    pub params: Vec<FuncArg<'a>>,
    pub body: FuncBody<'a>,
    pub generator: bool,
    pub is_async: bool,
}

impl<'a> Func<'a> {
    pub fn new(
        id: Option<Ident<'a>>, 
        params: Vec<FuncArg<'a>>, 
        body: FuncBody<'a>, 
        generator: bool, 
        is_async: bool
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
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FuncArg<'a> {
    Expr(Expr<'a>),
    Pat(Pat<'a>),
}

impl<'a> FuncArg<'a> {
    pub fn expr(expr: Expr) -> FuncArg {
        FuncArg::Expr(expr)
    }
    pub fn pat(pat: Pat) -> FuncArg {
        FuncArg::Pat(pat)
    }
}

/// The block statement that makes up the function's body
#[derive(PartialEq, Debug, Clone, Deserialize)]
pub struct FuncBody<'a>(pub Vec<ProgramPart<'a>>);
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
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Class<'a> {
    pub id: Option<Ident<'a>>,
    pub super_class: Option<Box<Expr<'a>>>,
    pub body: ClassBody<'a>,
}
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct ClassBody<'a>(pub Vec<Prop<'a>>);

impl<'a> Class<'a> {
    pub fn new(id: Option<Ident<'a>>, super_class: Option<Expr<'a>>, body: Vec<Prop<'a>>) -> Class<'a> {
        Class {
            id,
            super_class: super_class.map(Box::new),
            body: ClassBody(body),
        }
    }
}

/// The kind of variable being defined (`var`/`let`/`const`)
#[derive(PartialEq, Clone, Debug, Copy, Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum VarKind {
    Var,
    Let,
    Const,
}


/// The available operators for assignment Exprs
#[derive(PartialEq, Debug, Clone, Copy, Deserialize)]
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
#[derive(PartialEq, Debug, Clone, Copy, Deserialize)]
pub enum LogicalOp {
    Or,
    And,
}


/// The available operations for `Binary` Exprs
#[derive(PartialEq, Debug, Clone, Copy, Deserialize)]
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
#[derive(PartialEq, Debug, Clone, Copy, Deserialize)]
pub enum UpdateOp {
    Increment,
    Decrement,
}

/// The allowed operators for an Expr
/// to be `Unary`
#[derive(PartialEq, Debug, Clone, Copy, Deserialize)]
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
#[derive(PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
    pub use crate::{
        AssignOp,
        BinaryOp,
        Class,
        Dir,
        Func,
        FuncArg,
        FuncBody,
        Ident,
        LogicalOp,
        Program,
        ProgramPart,
        PropKind,
        UnaryOp,
        UpdateOp,
        VarKind,
    };
    pub use crate::expr::{
        ArrayExpr,
        ArrowFuncBody,
        ArrowFuncExpr,
        AssignExpr,
        AssignLeft,
        BinaryExpr,
        CallExpr,
        ConditionalExpr,
        Expr,
        Lit,
        LogicalExpr,
        MemberExpr,
        MetaProp,
        NewExpr,
        ObjExpr,
        ObjProp,
        Prop,
        PropKey,
        PropValue,
        RegEx,
        StringLit,
        TaggedTemplateExpr,
        TemplateElement,
        TemplateLit,
        UnaryExpr,
        UpdateExpr,
        YieldExpr,
    };
    pub use crate::decl::{
        Decl,
        VarDecl,
        ModDecl,
        ModImport,
        ImportSpecifier,
        ModExport,
        NamedExportDecl,
        DefaultExportDecl,
        ExportSpecifier,
        NormalImportSpec
    };
    pub use crate::stmt::{
        Stmt,
        WithStmt,
        LabeledStmt,
        IfStmt,
        SwitchStmt,
        SwitchCase,
        BlockStmt,
        TryStmt,
        CatchClause,
        WhileStmt,
        DoWhileStmt,
        ForStmt,
        LoopInit,
        ForInStmt,
        ForOfStmt,
        LoopLeft,
    };
    pub use crate::pat::{
        ArrayPatPart,
        AssignPat,
        ObjPat,
        ObjPatPart,
        Pat,
    };
}