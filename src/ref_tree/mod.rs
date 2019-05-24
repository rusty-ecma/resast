pub mod decl;
pub mod expr;
pub mod pat;
pub mod stmt;
use self::decl::Decl;
use self::expr::{Expr, Literal, Property};
use self::pat::Pat;
use self::stmt::Stmt;

pub type Identifier<'a> = &'a str;
/// A fully parsed javascript program.
///
/// It is essentially a collection of `ProgramPart`s
/// with a flag denoting if the representation is
/// a ES6 Mod or a Script.
#[derive(PartialEq, Debug)]
pub enum Program<'a> {
    /// An ES6 Mod
    Mod(Vec<ProgramPart<'a>>),
    /// Not an ES6 Mod
    Script(Vec<ProgramPart<'a>>),
}

impl<'a> AsConcrete<crate::Program> for Program<'a> {
    fn as_concrete(&self) -> crate::Program {
        match self {
            Program::Mod(ref parts) => {
                crate::Program::Mod(parts.as_concrete())
            }
            Program::Script(ref parts) => {
                crate::Program::Script(parts.as_concrete())
            }
        }
    }
}

/// A single part of a Javascript program.
/// This will be either a Directive, Decl or a Stmt
#[derive(PartialEq, Debug, Clone)]
pub enum ProgramPart<'a> {
    /// A Directive like `'use strict';`
    Dir(Dir<'a>),
    /// A variable, function or module declaration
    Decl(Decl<'a>),
    /// Any other kind of statement
    Stmt(Stmt<'a>),
}

impl<'a> AsConcrete<crate::ProgramPart> for ProgramPart<'a> {
    fn as_concrete(&self) -> crate::ProgramPart {
        match self {
            ProgramPart::Dir(ref dir) => crate::ProgramPart::Dir(dir.as_concrete()),
            ProgramPart::Decl(ref decl) => crate::ProgramPart::Decl(decl.as_concrete()),
            ProgramPart::Stmt(ref stmt) => crate::ProgramPart::Stmt(stmt.as_concrete()),
        }
    }
}

/// pretty much always `'use strict'`, this can appear at the
/// top of a file or function
#[derive(PartialEq, Debug, Clone)]
pub struct Dir<'a> {
    pub expr: Literal<'a>,
    pub dir: &'a str,
}

impl<'a> AsConcrete<crate::Dir> for Dir<'a> {
    fn as_concrete(&self) -> crate::Dir {
        crate::Dir {
            expr: self.expr.as_concrete(),
            dir: self.dir.as_concrete(),
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
pub struct Function<'a> {
    pub id: Option<Identifier<'a>>,
    pub params: Vec<FunctionArg<'a>>,
    pub body: FunctionBody<'a>,
    pub generator: bool,
    pub is_async: bool,
}

impl<'a> AsConcrete<crate::Function> for Function<'a> {
    fn as_concrete(&self) -> crate::Function {
        crate::Function {
            id: self.id.map(|i| String::from(i)),
            params: self.params.as_concrete(),
            body: self.body.as_concrete(),
            generator: self.generator,
            is_async: self.is_async,
        }
    }
}

/// A single function argument from a function signature
#[derive(PartialEq, Debug, Clone)]
pub enum FunctionArg<'a> {
    Expr(Expr<'a>),
    Pat(Pat<'a>),
}

impl<'a> AsConcrete<crate::FunctionArg> for FunctionArg<'a> {
    fn as_concrete(&self) -> crate::FunctionArg {
        match self {
            FunctionArg::Expr(ref e) => crate::FunctionArg::Expr(e.as_concrete()),
            FunctionArg::Pat(ref p) => crate::FunctionArg::Pat(p.as_concrete()),
        }
    }
}

/// The block statement that makes up the function's body
pub type FunctionBody<'a> = Vec<ProgramPart<'a>>;
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
pub struct Class<'a> {
    pub id: Option<Identifier<'a>>,
    pub super_class: Option<Box<Expr<'a>>>,
    pub body: Vec<Property<'a>>,
}

impl<'a> AsConcrete<crate::Class> for Class<'a> {
    fn as_concrete(&self) -> crate::Class {
        let super_class = if let Some(ref c) = self.super_class {
            Some(Box::new(c.as_concrete()))
        } else {
            None
        };
        crate::Class {
            id: self.id.map(|i| String::from(i)),
            super_class,
            body: self.body.iter().map(|p| p.as_concrete()).collect(),
        }
    }
}

pub trait AsConcrete<T> {
    fn as_concrete(&self) -> T;
}

impl<'a> AsConcrete<String> for str {
    fn as_concrete(&self) -> String {
        String::from(self)
    }
}

impl<T, U> AsConcrete<Vec<U>> for Vec<T>
where T: AsConcrete<U> {
    fn as_concrete(&self) -> Vec<U> {
        self.iter().map(|e| e.as_concrete()).collect()
    }
}

fn ref_map<T, U, F>(o: &Option<T>, f: F) -> Option<U> 
where F: Fn(&T) -> U {
    if let Some(ref inner) = o {
        Some(f(inner))
    } else {
        None
    }
}

pub mod prelude {
    pub use crate::ref_tree::{
        Identifier,
        Program,
        ProgramPart,
        Dir,
        Function,
        FunctionArg,
        FunctionBody,
        Class,
        AsConcrete,
        decl::{
            Decl,
            DefaultExportDecl,
            ExportSpecifier,
            ImportSpecifier,
            ModDecl,
            ModExport,
            ModImport,
            NamedExportDecl,
            VariableDecl
        },
        expr::{
            Expr,
            ArrayExpr,
            ObjectExpr,
            ObjectProperty,
            Property,
            PropertyKey,
            PropertyValue,
            UnaryExpr,
            UpdateExpr,
            BinaryExpr,
            AssignmentExpr,
            AssignmentLeft,
            LogicalExpr,
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
    };
    pub use crate::prelude::{
        BinaryOperator,
        LogicalOperator,
        AssignmentOperator,
        UnaryOperator,
        UpdateOperator,
        PropertyKind,
    };
}