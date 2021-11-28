use std::borrow::Cow;

pub mod decl;
pub mod expr;
pub mod pat;
pub mod stmt;

use decl::Decl;
use expr::{Expr, Lit, Prop};
use pat::Pat;
use stmt::Stmt;

pub trait Node {
    fn loc(&self) -> SourceLocation;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ident<'a> {
    pub slice: Slice<'a>,
}

impl<'a> Node for Ident<'a> {
    fn loc(&self) -> SourceLocation {
        self.slice.loc
    }
}

/// A fully parsed javascript program.
///
/// It is essentially a collection of `ProgramPart`s
/// with a flag denoting if the representation is
/// a ES6 Mod or a Script.
#[derive(Debug, Clone, PartialEq)]
pub enum Program<'a> {
    /// An ES6 Mod
    Mod(Vec<ProgramPart<'a>>),
    /// Not an ES6 Mod
    Script(Vec<ProgramPart<'a>>),
}

impl<'a> Node for Program<'a> {
    fn loc(&self) -> SourceLocation {
        let start = Position { line: 1, column: 1 };
        let end = if let Some(last) = match self {
            Self::Mod(inner) => inner.last(),
            Self::Script(inner) => inner.last(),
        } {
            last.loc().end.clone()
        } else {
            start.clone()
        };
        SourceLocation { start, end }
    }
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
#[derive(Debug, Clone, PartialEq)]
pub enum ProgramPart<'a> {
    /// A Directive like `'use strict';`
    Dir(Dir<'a>),
    /// A variable, function or module declaration
    Decl(Decl<'a>),
    /// Any other kind of statement
    Stmt(Stmt<'a>),
}

impl<'a> Node for ProgramPart<'a> {
    fn loc(&self) -> SourceLocation {
        match self {
            Self::Dir(inner) => inner.loc(),
            Self::Decl(inner) => inner.loc(),
            Self::Stmt(inner) => inner.loc(),
        }
    }
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
#[derive(Debug, Clone, PartialEq)]
pub struct Dir<'a> {
    pub expr: Lit<'a>,
    pub dir: Cow<'a, str>,
}

impl<'a> Node for Dir<'a> {
    fn loc(&self) -> SourceLocation {
        self.expr.loc()
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
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub struct Func<'a> {
    pub keyword: Slice<'a>,
    pub id: Option<Ident<'a>>,
    pub params: Vec<FuncArg<'a>>,
    pub body: FuncBody<'a>,
    pub generator: bool,
    pub is_async: bool,
}

impl<'a> Node for Func<'a> {
    fn loc(&self) -> SourceLocation {
        let start = self.keyword.loc.start.clone();
        let end = self.body.close_brace.loc.end.clone();
        SourceLocation { start, end }
    }
}

impl<'a> Func<'a> {
    pub fn new(
        keyword: Slice<'a>,
        id: Option<Ident<'a>>,
        params: Vec<FuncArg<'a>>,
        body: FuncBody<'a>,
        generator: bool,
        is_async: bool,
    ) -> Self {
        Func {
            keyword,
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
pub enum FuncArg<'a> {
    Expr(Expr<'a>),
    Pat(Pat<'a>),
}

impl<'a> Node for FuncArg<'a> {
    fn loc(&self) -> SourceLocation {
        match self {
            FuncArg::Expr(inner) => inner.loc(),
            FuncArg::Pat(inner) => inner.loc(),
        }
    }
}

/// The block statement that makes up the function's body
#[derive(Debug, Clone, PartialEq)]
pub struct FuncBody<'a> {
    pub open_brace: Slice<'a>,
    pub stmts: Vec<ProgramPart<'a>>,
    pub close_brace: Slice<'a>,
}

impl<'a> Node for FuncBody<'a> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.open_brace.loc.start,
            end: self.close_brace.loc.end,
        }
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
pub struct Class<'a> {
    pub class: Slice<'a>,
    pub id: Option<Ident<'a>>,
    pub super_class: Option<Box<Expr<'a>>>,
    pub body: ClassBody<'a>,
}

impl<'a> Node for Class<'a> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.class.loc.start,
            end: self.body.close_brace.loc.end,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassBody<'a> {
    pub open_brace: Slice<'a>,
    pub props: Vec<Prop<'a>>,
    pub close_brace: Slice<'a>,
}

impl<'a> Node for ClassBody<'a> {
    fn loc(&self) -> SourceLocation {
        let start = self.open_brace.loc.start;
        let end = self.close_brace.loc.end;
        SourceLocation { start, end }
    }
}

/// The kind of variable being defined (`var`/`let`/`const`)
#[derive(Debug, Clone, PartialEq)]
pub enum VarKind<'a> {
    Var(Slice<'a>),
    Let(Slice<'a>),
    Const(Slice<'a>),
}

impl<'a> Node for VarKind<'a> {
    fn loc(&self) -> SourceLocation {
        match self {
            VarKind::Var(slice) => slice.loc,
            VarKind::Let(slice) => slice.loc,
            VarKind::Const(slice) => slice.loc,
        }
    }
}

/// The available operators for assignment Exprs
#[derive(Debug, Clone, PartialEq)]
pub enum AssignOp<'a> {
    Equal(Slice<'a>),
    PlusEqual(Slice<'a>),
    MinusEqual(Slice<'a>),
    TimesEqual(Slice<'a>),
    DivEqual(Slice<'a>),
    ModEqual(Slice<'a>),
    LeftShiftEqual(Slice<'a>),
    RightShiftEqual(Slice<'a>),
    UnsignedRightShiftEqual(Slice<'a>),
    OrEqual(Slice<'a>),
    XOrEqual(Slice<'a>),
    AndEqual(Slice<'a>),
    PowerOfEqual(Slice<'a>),
}

impl<'a> Node for AssignOp<'a> {
    fn loc(&self) -> SourceLocation {
        match self {
            AssignOp::Equal(slice) => slice.loc,
            AssignOp::PlusEqual(slice) => slice.loc,
            AssignOp::MinusEqual(slice) => slice.loc,
            AssignOp::TimesEqual(slice) => slice.loc,
            AssignOp::DivEqual(slice) => slice.loc,
            AssignOp::ModEqual(slice) => slice.loc,
            AssignOp::LeftShiftEqual(slice) => slice.loc,
            AssignOp::RightShiftEqual(slice) => slice.loc,
            AssignOp::UnsignedRightShiftEqual(slice) => slice.loc,
            AssignOp::OrEqual(slice) => slice.loc,
            AssignOp::XOrEqual(slice) => slice.loc,
            AssignOp::AndEqual(slice) => slice.loc,
            AssignOp::PowerOfEqual(slice) => slice.loc,
        }
    }
}

/// The available logical operators
#[derive(Debug, Clone, PartialEq)]
pub enum LogicalOp<'a> {
    Or(Slice<'a>),
    And(Slice<'a>),
}

impl<'a> Node for LogicalOp<'a> {
    fn loc(&self) -> SourceLocation {
        match self {
            LogicalOp::Or(slice) => slice.loc,
            LogicalOp::And(slice) => slice.loc,
        }
    }
}

/// The available operations for `Binary` Exprs
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp<'a> {
    Equal(Slice<'a>),
    NotEqual(Slice<'a>),
    StrictEqual(Slice<'a>),
    StrictNotEqual(Slice<'a>),
    LessThan(Slice<'a>),
    GreaterThan(Slice<'a>),
    LessThanEqual(Slice<'a>),
    GreaterThanEqual(Slice<'a>),
    LeftShift(Slice<'a>),
    RightShift(Slice<'a>),
    UnsignedRightShift(Slice<'a>),
    Plus(Slice<'a>),
    Minus(Slice<'a>),
    Times(Slice<'a>),
    Over(Slice<'a>),
    Mod(Slice<'a>),
    Or(Slice<'a>),
    XOr(Slice<'a>),
    And(Slice<'a>),
    In(Slice<'a>),
    InstanceOf(Slice<'a>),
    PowerOf(Slice<'a>),
}

impl<'a> Node for BinaryOp<'a> {
    fn loc(&self) -> SourceLocation {
        match self {
            BinaryOp::Equal(slice) => slice.loc,
            BinaryOp::NotEqual(slice) => slice.loc,
            BinaryOp::StrictEqual(slice) => slice.loc,
            BinaryOp::StrictNotEqual(slice) => slice.loc,
            BinaryOp::LessThan(slice) => slice.loc,
            BinaryOp::GreaterThan(slice) => slice.loc,
            BinaryOp::LessThanEqual(slice) => slice.loc,
            BinaryOp::GreaterThanEqual(slice) => slice.loc,
            BinaryOp::LeftShift(slice) => slice.loc,
            BinaryOp::RightShift(slice) => slice.loc,
            BinaryOp::UnsignedRightShift(slice) => slice.loc,
            BinaryOp::Plus(slice) => slice.loc,
            BinaryOp::Minus(slice) => slice.loc,
            BinaryOp::Times(slice) => slice.loc,
            BinaryOp::Over(slice) => slice.loc,
            BinaryOp::Mod(slice) => slice.loc,
            BinaryOp::Or(slice) => slice.loc,
            BinaryOp::XOr(slice) => slice.loc,
            BinaryOp::And(slice) => slice.loc,
            BinaryOp::In(slice) => slice.loc,
            BinaryOp::InstanceOf(slice) => slice.loc,
            BinaryOp::PowerOf(slice) => slice.loc,
        }
    }
}

/// `++` or `--`
#[derive(Debug, Clone, PartialEq)]
pub enum UpdateOp<'a> {
    Increment(Slice<'a>),
    Decrement(Slice<'a>),
}

impl<'a> Node for UpdateOp<'a> {
    fn loc(&self) -> SourceLocation {
        match self {
            UpdateOp::Increment(slice) => slice.loc,
            UpdateOp::Decrement(slice) => slice.loc,
        }
    }
}

/// The allowed operators for an Expr
/// to be `Unary`
#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp<'a> {
    Minus(Slice<'a>),
    Plus(Slice<'a>),
    Not(Slice<'a>),
    Tilde(Slice<'a>),
    TypeOf(Slice<'a>),
    Void(Slice<'a>),
    Delete(Slice<'a>),
}

impl<'a> Node for UnaryOp<'a> {
    fn loc(&self) -> SourceLocation {
        match self {
            UnaryOp::Minus(slice) => slice.loc,
            UnaryOp::Plus(slice) => slice.loc,
            UnaryOp::Not(slice) => slice.loc,
            UnaryOp::Tilde(slice) => slice.loc,
            UnaryOp::TypeOf(slice) => slice.loc,
            UnaryOp::Void(slice) => slice.loc,
            UnaryOp::Delete(slice) => slice.loc,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Slice<'a> {
    source: Cow<'a, str>,
    loc: SourceLocation,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct SourceLocation {
    start: Position,
    end: Position,
}

impl SourceLocation {
    fn zero() -> Self {
        Self {
            start: Position { line: 0, column: 0 },
            end: Position { line: 0, column: 0 },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}
