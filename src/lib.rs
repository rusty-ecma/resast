pub mod decl;
pub mod expr;
pub mod pat;
pub mod ref_tree;
pub mod stmt;
use decl::Decl;
use expr::{Expr, Literal, Property};
use pat::Pat;
use stmt::Stmt;

pub type Identifier = String;
/// A fully parsed javascript program.
///
/// It is essentially a collection of `ProgramPart`s
/// with a flag denoting if the representation is
/// a ES6 Mod or a Script.
#[derive(PartialEq, Debug)]
pub enum Program {
    /// An ES6 Mod
    Mod(Vec<ProgramPart>),
    /// Not an ES6 Mod
    Script(Vec<ProgramPart>),
}

impl Program {
    pub fn module(parts: Vec<ProgramPart>) -> Self {
        Program::Mod(parts)
    }
    pub fn script(parts: Vec<ProgramPart>) -> Self {
        Program::Script(parts)
    }
}

/// A single part of a Javascript program.
/// This will be either a Directive, Decl or a Stmt
#[derive(PartialEq, Debug, Clone)]
pub enum ProgramPart {
    /// A Directive like `'use strict';`
    Dir(Dir),
    /// A variable, function or module declaration
    Decl(Decl),
    /// Any other kind of statement
    Stmt(Stmt),
}

impl ProgramPart {
    pub fn use_strict(double_quotes: bool) -> Self {
        let dir = if double_quotes {
            r#""use strict""#
        } else {
            "'use strict'"
        };
        ProgramPart::Dir(Dir::new(String::from(dir)))
    }
    pub fn decl(inner: Decl) -> Self {
        ProgramPart::Decl(inner)
    }
    pub fn stmt(inner: Stmt) -> Self {
        ProgramPart::Stmt(inner)
    }
}

/// pretty much always `'use strict'`, this can appear at the
/// top of a file or function
#[derive(PartialEq, Debug, Clone)]
pub struct Dir {
    pub expr: Literal,
    pub dir: String,
}

impl Dir {
    pub fn new(orig: String) -> Self {
        Self {
            dir: orig.trim_matches(|c| c == '\'' || c == '"').to_string(),
            expr: Literal::String(orig),
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
pub struct Function {
    pub id: Option<Identifier>,
    pub params: Vec<FunctionArg>,
    pub body: FunctionBody,
    pub generator: bool,
    pub is_async: bool,
}

impl Function {
    pub fn new(id: Option<Identifier>, params: Vec<FunctionArg>, body: FunctionBody, generator: bool, is_async: bool) -> Self {
        Self {
            id,
            params,
            body,
            generator,
            is_async,
        }
    }
}

/// A single function argument from a function signature
#[derive(PartialEq, Debug, Clone)]
pub enum FunctionArg {
    Expr(Expr),
    Pat(Pat),
}

impl FunctionArg {
    pub fn expr(expr: Expr) -> FunctionArg {
        FunctionArg::Expr(expr)
    }
    pub fn pat(pat: Pat) -> FunctionArg {
        FunctionArg::Pat(pat)
    }

    pub fn ident(name: &str) -> Self {
        FunctionArg::Pat(Pat::Identifier(String::from(name)))
    }
}

/// The block statement that makes up the function's body
pub type FunctionBody = Vec<ProgramPart>;
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
pub struct Class {
    pub id: Option<Identifier>,
    pub super_class: Option<Box<Expr>>,
    pub body: Vec<Property>,
}

impl Class {
    pub fn new(id: Option<Identifier>, super_class: Option<Expr>, body: Vec<Property>) -> Class {
        Class {
            id,
            super_class: super_class.map(|e| Box::new(e)),
            body,
        }
    }
}

pub mod prelude {
    pub use crate::{
        decl::{
            Decl,
            DefaultExportDecl,
            ExportSpecifier,
            ImportSpecifier,
            ModDecl,
            ModExport,
            ModImport,
            NamedExportDecl,
            VariableDecl,
            VariableKind,
        },
        expr::{
            Expr,
            ArrayExpr,
            ObjectExpr,
            ObjectProperty,
            Property,
            PropertyKey,
            PropertyValue,
            PropertyKind,
            UnaryExpr,
            UnaryOperator,
            UpdateExpr,
            UpdateOperator,
            BinaryExpr,
            BinaryOperator,
            AssignmentExpr,
            AssignmentLeft,
            AssignmentOperator,
            LogicalExpr,
            LogicalOperator,
            MemberExpr,
            ConditionalExpr,
            CallExpr,
            NewExpr,
            SequenceExpr,
            ArrowFunctionExpr,
            ArrowFunctionBody,
            YieldExpr,
            TaggedTemplateExpr,
            TemplateLiteral,
            TemplateElement,
            MetaProperty,
            Literal,
            RegEx,
        },
        pat::{
            Pat,
            ArrayPatPart,
            ObjectPat,
            ObjectPatPart,
            AssignmentPat,
        },
        stmt::{
            Stmt,
            WithStmt,
            LabeledStmt,
            IfStmt,
            SwitchStmt,
            SwitchCase,
            BlockStmt,
            TryStmt,
            WhileStmt,
            DoWhileStmt,
            ForStmt,
            LoopInit,
            ForInStmt,
            ForOfStmt,
            LoopLeft,
            CatchClause,
        },
        Identifier,
        Program,
        ProgramPart,
        Dir,
        Function,
        FunctionArg,
        FunctionBody,
        Class,
    };
}