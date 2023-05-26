mod convert;
pub mod decl;
pub mod expr;
pub mod pat;
pub mod stmt;

use decl::Decl;
use expr::{Expr, Lit, Prop};
use pat::Pat;
use stmt::Stmt;

use crate::SourceText;

use self::pat::RestPat;

pub trait Node {
    fn loc(&self) -> SourceLocation;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ident<T> {
    pub slice: Slice<T>,
}

impl<T> Node for Ident<T> {
    fn loc(&self) -> SourceLocation {
        self.slice.loc
    }
}

impl<T> From<Slice<T>> for Ident<T> {
    fn from(slice: Slice<T>) -> Self {
        Self { slice }
    }
}

impl<T> Ident<T> {
    pub fn name(&self) -> &T {
        &self.slice.source.0
    }
}

/// A fully parsed javascript program.
///
/// It is essentially a collection of `ProgramPart`s
/// with a flag denoting if the representation is
/// a ES6 Mod or a Script.
#[derive(Debug, Clone, PartialEq)]
pub enum Program<T> {
    /// An ES6 Mod
    Mod(Vec<ProgramPart<T>>),
    /// Not an ES6 Mod
    Script(Vec<ProgramPart<T>>),
}

impl<T> Node for Program<T> {
    fn loc(&self) -> SourceLocation {
        match self {
            Self::Mod(inner) => inner.loc(),
            Self::Script(inner) => inner.loc(),
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

impl<T> Node for Vec<ProgramPart<T>> {
    fn loc(&self) -> SourceLocation {
        let start = self
            .first()
            .map(|p| p.loc())
            .unwrap_or_else(SourceLocation::zero);
        let end = self.last().map(|p| p.loc()).unwrap_or(start);
        SourceLocation {
            start: start.start,
            end: end.end,
        }
    }
}

/// A single part of a Javascript program.
/// This will be either a Directive, Decl or a Stmt
#[derive(Debug, Clone, PartialEq)]
pub enum ProgramPart<T> {
    /// A Directive like `'use strict';`
    Dir(Dir<T>),
    /// A variable, function or module declaration
    Decl(Decl<T>),
    /// Any other kind of statement
    Stmt(Stmt<T>),
}

impl<T> Node for ProgramPart<T> {
    fn loc(&self) -> SourceLocation {
        match self {
            Self::Dir(inner) => inner.loc(),
            Self::Decl(inner) => inner.loc(),
            Self::Stmt(inner) => inner.loc(),
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
pub struct Dir<T> {
    pub expr: Lit<T>,
    pub dir: SourceText<T>,
    pub semi_colon: Option<Position>,
}

impl<T> Node for Dir<T> {
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
pub struct Func<T> {
    pub keyword: Position,
    pub id: Option<Ident<T>>,
    pub open_paren: Position,
    pub params: Vec<ListEntry<FuncArg<T>>>,
    pub close_paren: Position,
    pub body: FuncBody<T>,
    pub star: Option<Position>,
    pub keyword_async: Option<Position>,
}

impl<T> Func<T> {
    pub fn is_async(&self) -> bool {
        self.keyword_async.is_some()
    }
    pub fn generator(&self) -> bool {
        self.star.is_some()
    }
}

impl<T> Node for Func<T> {
    fn loc(&self) -> SourceLocation {
        let start = if let Some(keyword) = self.keyword_async {
            keyword
        } else {
            self.keyword
        };
        let end = self.body.close_brace + 1;
        SourceLocation { start, end }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FuncArgEntry<T> {
    pub value: FuncArg<T>,
    pub comma: Option<Position>,
}

impl<T> Node for FuncArgEntry<T> {
    fn loc(&self) -> SourceLocation {
        if let Some(comma) = &self.comma {
            return SourceLocation {
                start: self.value.loc().start,
                end: *comma + 1,
            };
        }
        self.value.loc()
    }
}

/// A single function argument from a function signature
#[derive(Debug, Clone, PartialEq)]
pub enum FuncArg<T> {
    Expr(Expr<T>),
    Pat(Pat<T>),
    Rest(Box<RestPat<T>>),
}

impl<T> Node for FuncArg<T> {
    fn loc(&self) -> SourceLocation {
        match self {
            FuncArg::Expr(inner) => inner.loc(),
            FuncArg::Pat(inner) => inner.loc(),
            FuncArg::Rest(inner) => inner.loc(),
        }
    }
}

/// The block statement that makes up the function's body
#[derive(Debug, Clone, PartialEq)]
pub struct FuncBody<T> {
    pub open_brace: Position,
    pub stmts: Vec<ProgramPart<T>>,
    pub close_brace: Position,
}

impl<T> Node for FuncBody<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.open_brace,
            end: self.close_brace + 1,
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
pub struct Class<T> {
    pub keyword: Position,
    pub id: Option<Ident<T>>,
    pub super_class: Option<SuperClass<T>>,
    pub body: ClassBody<T>,
}

impl<T> Node for Class<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.keyword,
            end: self.body.close_brace + 1,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct SuperClass<T> {
    pub keyword_extends: Position,
    pub expr: Expr<T>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassBody<T> {
    pub open_brace: Position,
    pub props: Vec<Prop<T>>,
    pub close_brace: Position,
}

impl<T> Node for ClassBody<T> {
    fn loc(&self) -> SourceLocation {
        let start = self.open_brace;
        let end = self.close_brace + 1;
        SourceLocation { start, end }
    }
}

/// The kind of variable being defined (`var`/`let`/`const`)
#[derive(Debug, Clone, PartialEq)]
pub enum VarKind {
    Var(Option<Position>),
    Let(Position),
    Const(Position),
}

impl Node for VarKind {
    fn loc(&self) -> SourceLocation {
        let start = match self {
            VarKind::Var(Some(slice)) => *slice,
            VarKind::Let(slice) => *slice,
            VarKind::Const(slice) => *slice,
            _ => return SourceLocation::zero(),
        };
        let end = Position {
            line: start.line,
            column: start.column + self.len(),
        };
        SourceLocation { start, end }
    }
}

impl VarKind {
    pub fn is_var(&self) -> bool {
        matches!(self, VarKind::Var(_))
    }
    pub const fn len(&self) -> u32 {
        match self {
            VarKind::Var(Some(_)) => 3,
            VarKind::Var(None) => 0,
            VarKind::Let(_) => 3,
            VarKind::Const(_) => 4,
        }
    }
}

/// The available operators for assignment Exprs
#[derive(Debug, Clone, PartialEq)]
pub enum AssignOp {
    Equal(Position),
    PlusEqual(Position),
    MinusEqual(Position),
    TimesEqual(Position),
    DivEqual(Position),
    ModEqual(Position),
    LeftShiftEqual(Position),
    RightShiftEqual(Position),
    UnsignedRightShiftEqual(Position),
    OrEqual(Position),
    XOrEqual(Position),
    AndEqual(Position),
    PowerOfEqual(Position),
}

impl Node for AssignOp {
    fn loc(&self) -> SourceLocation {
        let start = match self {
            AssignOp::Equal(start) => *start,
            AssignOp::PlusEqual(start) => *start,
            AssignOp::MinusEqual(start) => *start,
            AssignOp::TimesEqual(start) => *start,
            AssignOp::DivEqual(start) => *start,
            AssignOp::ModEqual(start) => *start,
            AssignOp::LeftShiftEqual(start) => *start,
            AssignOp::RightShiftEqual(start) => *start,
            AssignOp::UnsignedRightShiftEqual(start) => *start,
            AssignOp::OrEqual(start) => *start,
            AssignOp::XOrEqual(start) => *start,
            AssignOp::AndEqual(start) => *start,
            AssignOp::PowerOfEqual(start) => *start,
        };
        let end = Position {
            line: start.line,
            column: start.column + self.len(),
        };
        SourceLocation { start, end }
    }
}

impl AssignOp {
    pub const fn len(&self) -> u32 {
        match self {
            AssignOp::Equal(_) => 1,
            AssignOp::PlusEqual(_) => 2,
            AssignOp::MinusEqual(_) => 2,
            AssignOp::TimesEqual(_) => 2,
            AssignOp::DivEqual(_) => 2,
            AssignOp::ModEqual(_) => 2,
            AssignOp::LeftShiftEqual(_) => 3,
            AssignOp::RightShiftEqual(_) => 3,
            AssignOp::UnsignedRightShiftEqual(_) => 4,
            AssignOp::OrEqual(_) => 2,
            AssignOp::XOrEqual(_) => 2,
            AssignOp::AndEqual(_) => 2,
            AssignOp::PowerOfEqual(_) => 3,
        }
    }
}

/// The available logical operators
#[derive(Debug, Clone, PartialEq)]
pub enum LogicalOp {
    Or(Position),
    And(Position),
}

impl Node for LogicalOp {
    fn loc(&self) -> SourceLocation {
        let start = match self {
            LogicalOp::Or(start) => *start,
            LogicalOp::And(start) => *start,
        };
        let end = Position {
            line: start.line,
            column: start.column + 2,
        };
        SourceLocation { start, end }
    }
}

/// The available operations for `Binary` Exprs
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Equal(Position),
    NotEqual(Position),
    StrictEqual(Position),
    StrictNotEqual(Position),
    LessThan(Position),
    GreaterThan(Position),
    LessThanEqual(Position),
    GreaterThanEqual(Position),
    LeftShift(Position),
    RightShift(Position),
    UnsignedRightShift(Position),
    Plus(Position),
    Minus(Position),
    Times(Position),
    Over(Position),
    Mod(Position),
    Or(Position),
    XOr(Position),
    And(Position),
    In(Position),
    InstanceOf(Position),
    PowerOf(Position),
}

impl Node for BinaryOp {
    fn loc(&self) -> SourceLocation {
        let start = match self {
            BinaryOp::Equal(start) => *start,
            BinaryOp::NotEqual(start) => *start,
            BinaryOp::StrictEqual(start) => *start,
            BinaryOp::StrictNotEqual(start) => *start,
            BinaryOp::LessThan(start) => *start,
            BinaryOp::GreaterThan(start) => *start,
            BinaryOp::LessThanEqual(start) => *start,
            BinaryOp::GreaterThanEqual(start) => *start,
            BinaryOp::LeftShift(start) => *start,
            BinaryOp::RightShift(start) => *start,
            BinaryOp::UnsignedRightShift(start) => *start,
            BinaryOp::Plus(start) => *start,
            BinaryOp::Minus(start) => *start,
            BinaryOp::Times(start) => *start,
            BinaryOp::Over(start) => *start,
            BinaryOp::Mod(start) => *start,
            BinaryOp::Or(start) => *start,
            BinaryOp::XOr(start) => *start,
            BinaryOp::And(start) => *start,
            BinaryOp::In(start) => *start,
            BinaryOp::InstanceOf(start) => *start,
            BinaryOp::PowerOf(start) => *start,
        };
        let end = Position {
            line: start.line,
            column: start.column + self.len(),
        };
        SourceLocation { start, end }
    }
}

impl BinaryOp {
    pub const fn len(&self) -> u32 {
        match self {
            BinaryOp::Equal(_) => 1,
            BinaryOp::NotEqual(_) => 2,
            BinaryOp::StrictEqual(_) => 3,
            BinaryOp::StrictNotEqual(_) => 4,
            BinaryOp::LessThan(_) => 1,
            BinaryOp::GreaterThan(_) => 1,
            BinaryOp::LessThanEqual(_) => 2,
            BinaryOp::GreaterThanEqual(_) => 2,
            BinaryOp::LeftShift(_) => 2,
            BinaryOp::RightShift(_) => 2,
            BinaryOp::UnsignedRightShift(_) => 3,
            BinaryOp::Plus(_) => 1,
            BinaryOp::Minus(_) => 1,
            BinaryOp::Times(_) => 1,
            BinaryOp::Over(_) => 1,
            BinaryOp::Mod(_) => 1,
            BinaryOp::Or(_) => 1,
            BinaryOp::XOr(_) => 1,
            BinaryOp::And(_) => 1,
            BinaryOp::In(_) => 2,
            BinaryOp::InstanceOf(_) => 10,
            BinaryOp::PowerOf(_) => 2,
        }
    }
}

/// `++` or `--`
#[derive(Debug, Clone, PartialEq)]
pub enum UpdateOp {
    Increment(Position),
    Decrement(Position),
}

impl Node for UpdateOp {
    fn loc(&self) -> SourceLocation {
        let start = match self {
            UpdateOp::Increment(start) => *start,
            UpdateOp::Decrement(start) => *start,
        };
        SourceLocation {
            start,
            end: Position {
                line: start.line,
                column: start.column + 2,
            },
        }
    }
}

/// The allowed operators for an Expr
/// to be `Unary`
#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Minus(Position),
    Plus(Position),
    Not(Position),
    Tilde(Position),
    TypeOf(Position),
    Void(Position),
    Delete(Position),
}

impl Node for UnaryOp {
    fn loc(&self) -> SourceLocation {
        let start = match self {
            UnaryOp::Minus(start) => *start,
            UnaryOp::Plus(start) => *start,
            UnaryOp::Not(start) => *start,
            UnaryOp::Tilde(start) => *start,
            UnaryOp::TypeOf(start) => *start,
            UnaryOp::Void(start) => *start,
            UnaryOp::Delete(start) => *start,
        };
        let end = Position {
            line: start.line,
            column: start.column + self.len(),
        };
        SourceLocation { start, end }
    }
}

impl UnaryOp {
    pub const fn len(&self) -> u32 {
        match self {
            UnaryOp::Minus(_) => 2,
            UnaryOp::Plus(_) => 2,
            UnaryOp::Not(_) => 1,
            UnaryOp::Tilde(_) => 1,
            UnaryOp::TypeOf(_) => 6,
            UnaryOp::Void(_) => 4,
            UnaryOp::Delete(_) => 6,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Slice<T> {
    pub source: SourceText<T>,
    pub loc: SourceLocation,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct SourceLocation {
    pub start: Position,
    pub end: Position,
}

impl SourceLocation {
    pub fn new(start_line: u32, start_column: u32, end_line: u32, end_column: u32) -> Self {
        Self {
            start: Position {
                line: start_line,
                column: start_column,
            },
            end: Position {
                line: end_line,
                column: end_column,
            },
        }
    }
    fn zero() -> Self {
        Self::new(0, 0, 0, 0)
    }
}

impl core::cmp::PartialOrd for SourceLocation {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.start.partial_cmp(&other.start) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.end.partial_cmp(&other.end)
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Position {
    pub line: u32,
    pub column: u32,
}

impl std::cmp::PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let line = self.line.partial_cmp(&other.line)?;
        if matches!(line, core::cmp::Ordering::Equal) {
            return self.column.partial_cmp(&other.column);
        }
        Some(line)
    }
}

impl std::ops::Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            line: self.line + rhs.line,
            column: self.column + rhs.column,
        }
    }
}

impl std::ops::Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            line: self.line - rhs.line,
            column: self.column - rhs.column,
        }
    }
}

impl std::ops::Add<u32> for Position {
    type Output = Self;

    fn add(self, rhs: u32) -> Self::Output {
        Self {
            line: self.line,
            column: self.column + rhs,
        }
    }
}

impl std::ops::Sub<u32> for Position {
    type Output = Self;

    fn sub(self, rhs: u32) -> Self::Output {
        Self {
            line: self.line,
            column: self.column - rhs,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ListEntry<Item> {
    pub item: Item,
    pub comma: Option<Position>,
}

impl<Item> ListEntry<Item> {
    pub fn no_comma(item: Item) -> Self {
        Self { item, comma: None }
    }
}

impl<Item> Node for ListEntry<Item>
where
    Item: Node,
{
    fn loc(&self) -> SourceLocation {
        if let Some(comma) = &self.comma {
            return SourceLocation {
                start: self.item.loc().start,
                end: *comma + 1,
            };
        }
        self.item.loc()
    }
}
