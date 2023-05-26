use std::borrow::Cow;

pub mod decl;
pub mod expr;
pub mod pat;
pub mod stmt;

use decl::Decl;
use expr::{Expr, Lit, Prop};
use pat::Pat;
use stmt::Stmt;

use self::pat::RestPat;

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

impl<'a> From<Ident<'a>> for crate::Ident<'a> {
    fn from(other: Ident<'a>) -> Self {
        Self {
            name: other.slice.source,
        }
    }
}

impl<'a> From<Slice<'a>> for Ident<'a> {
    fn from(slice: Slice<'a>) -> Self {
        Self { slice }
    }
}

impl<'a> Ident<'a> {
    pub fn name(&self) -> Cow<'a, str> {
        self.slice.source.clone()
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

impl<'a> From<Program<'a>> for crate::Program<'a> {
    fn from(other: Program<'a>) -> Self {
        match other {
            Program::Mod(inner) => Self::Mod(inner.into_iter().map(From::from).collect()),
            Program::Script(inner) => Self::Script(inner.into_iter().map(From::from).collect()),
        }
    }
}

impl<'a> Node for Program<'a> {
    fn loc(&self) -> SourceLocation {
        match self {
            Self::Mod(inner) => inner.loc(),
            Self::Script(inner) => inner.loc(),
        }
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

impl<'a> Node for Vec<ProgramPart<'a>> {
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
pub enum ProgramPart<'a> {
    /// A Directive like `'use strict';`
    Dir(Dir<'a>),
    /// A variable, function or module declaration
    Decl(Decl<'a>),
    /// Any other kind of statement
    Stmt(Stmt<'a>),
}

impl<'a> From<ProgramPart<'a>> for crate::ProgramPart<'a> {
    fn from(other: ProgramPart<'a>) -> Self {
        match other {
            ProgramPart::Dir(inner) => Self::Dir(inner.into()),
            ProgramPart::Decl(inner) => Self::Decl(inner.into()),
            ProgramPart::Stmt(inner) => Self::Stmt(inner.into()),
        }
    }
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
    pub semi_colon: Option<Slice<'a>>,
}

impl<'a> From<Dir<'a>> for crate::Dir<'a> {
    fn from(other: Dir<'a>) -> Self {
        Self {
            expr: other.expr.into(),
            dir: other.dir,
        }
    }
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
pub struct Func<'a> {
    pub keyword: Slice<'a>,
    pub id: Option<Ident<'a>>,
    pub open_paren: Slice<'a>,
    pub params: Vec<ListEntry<'a, FuncArg<'a>>>,
    pub close_paren: Slice<'a>,
    pub body: FuncBody<'a>,
    pub star: Option<Slice<'a>>,
    pub keyword_async: Option<Slice<'a>>,
}

impl<'a> Func<'a> {
    pub fn is_async(&self) -> bool {
        self.keyword_async.is_some()
    }
    pub fn generator(&self) -> bool {
        self.star.is_some()
    }
}

impl<'a> From<Func<'a>> for crate::Func<'a> {
    fn from(other: Func<'a>) -> Self {
        Self {
            generator: other.generator(),
            is_async: other.is_async(),
            id: other.id.map(From::from),
            params: other
                .params
                .into_iter()
                .map(|e| From::from(e.item))
                .collect(),
            body: other.body.into(),
        }
    }
}

impl<'a> Node for Func<'a> {
    fn loc(&self) -> SourceLocation {
        let start = if let Some(keyword) = &self.keyword_async {
            keyword.loc.start
        } else {
            self.keyword.loc.start
        };
        let end = self.body.close_brace.loc.end.clone();
        SourceLocation { start, end }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FuncArgEntry<'a> {
    pub value: FuncArg<'a>,
    pub comma: Option<Slice<'a>>,
}

impl<'a> From<FuncArgEntry<'a>> for crate::FuncArg<'a> {
    fn from(other: FuncArgEntry<'a>) -> Self {
        other.value.into()
    }
}

impl<'a> Node for FuncArgEntry<'a> {
    fn loc(&self) -> SourceLocation {
        if let Some(comma) = &self.comma {
            return SourceLocation {
                start: self.value.loc().start,
                end: comma.loc.end,
            };
        }
        self.value.loc()
    }
}

/// A single function argument from a function signature
#[derive(Debug, Clone, PartialEq)]
pub enum FuncArg<'a> {
    Expr(Expr<'a>),
    Pat(Pat<'a>),
    Rest(Box<RestPat<'a>>),
}

impl<'a> From<FuncArg<'a>> for crate::FuncArg<'a> {
    fn from(other: FuncArg<'a>) -> Self {
        match other {
            FuncArg::Expr(inner) => Self::Expr(inner.into()),
            FuncArg::Pat(inner) => Self::Pat(inner.into()),
            FuncArg::Rest(inner) => {
                Self::Pat(crate::pat::Pat::RestElement(Box::new(inner.pat.into())))
            }
        }
    }
}

impl<'a> Node for FuncArg<'a> {
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
pub struct FuncBody<'a> {
    pub open_brace: Slice<'a>,
    pub stmts: Vec<ProgramPart<'a>>,
    pub close_brace: Slice<'a>,
}

impl<'a> From<FuncBody<'a>> for crate::FuncBody<'a> {
    fn from(other: FuncBody<'a>) -> Self {
        Self(other.stmts.into_iter().map(From::from).collect())
    }
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
    pub keyword: Slice<'a>,
    pub id: Option<Ident<'a>>,
    pub super_class: Option<SuperClass<'a>>,
    pub body: ClassBody<'a>,
}

impl<'a> From<Class<'a>> for crate::Class<'a> {
    fn from(other: Class<'a>) -> Self {
        Self {
            id: other.id.map(From::from),
            super_class: other.super_class.map(|e| Box::new(From::from(e.expr))),
            body: other.body.into(),
        }
    }
}

impl<'a> Node for Class<'a> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.keyword.loc.start,
            end: self.body.close_brace.loc.end,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct SuperClass<'a> {
    pub keyword_extends: Slice<'a>,
    pub expr: Expr<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassBody<'a> {
    pub open_brace: Slice<'a>,
    pub props: Vec<Prop<'a>>,
    pub close_brace: Slice<'a>,
}

impl<'a> From<ClassBody<'a>> for crate::ClassBody<'a> {
    fn from(other: ClassBody<'a>) -> Self {
        Self(other.props.into_iter().map(From::from).collect())
    }
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
    Var(Option<Slice<'a>>),
    Let(Slice<'a>),
    Const(Slice<'a>),
}

impl<'a> From<VarKind<'a>> for crate::VarKind {
    fn from(other: VarKind<'a>) -> Self {
        match other {
            VarKind::Var(_) => Self::Var,
            VarKind::Let(_) => Self::Let,
            VarKind::Const(_) => Self::Const,
        }
    }
}

impl<'a> Node for VarKind<'a> {
    fn loc(&self) -> SourceLocation {
        match self {
            VarKind::Var(Some(slice)) => slice.loc,
            VarKind::Let(slice) => slice.loc,
            VarKind::Const(slice) => slice.loc,
            _ => SourceLocation::zero(),
        }
    }
}

impl<'a> VarKind<'a> {
    pub fn is_var(&self) -> bool {
        matches!(self, VarKind::Var(_))
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

impl<'a> From<AssignOp<'a>> for crate::AssignOp {
    fn from(other: AssignOp<'a>) -> Self {
        match other {
            AssignOp::Equal(_) => Self::Equal,
            AssignOp::PlusEqual(_) => Self::PlusEqual,
            AssignOp::MinusEqual(_) => Self::MinusEqual,
            AssignOp::TimesEqual(_) => Self::TimesEqual,
            AssignOp::DivEqual(_) => Self::DivEqual,
            AssignOp::ModEqual(_) => Self::ModEqual,
            AssignOp::LeftShiftEqual(_) => Self::LeftShiftEqual,
            AssignOp::RightShiftEqual(_) => Self::RightShiftEqual,
            AssignOp::UnsignedRightShiftEqual(_) => Self::UnsignedRightShiftEqual,
            AssignOp::OrEqual(_) => Self::OrEqual,
            AssignOp::XOrEqual(_) => Self::XOrEqual,
            AssignOp::AndEqual(_) => Self::AndEqual,
            AssignOp::PowerOfEqual(_) => Self::PowerOfEqual,
        }
    }
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

impl<'a> From<LogicalOp<'a>> for crate::LogicalOp {
    fn from(other: LogicalOp<'a>) -> Self {
        match other {
            LogicalOp::Or(_) => Self::Or,
            LogicalOp::And(_) => Self::And,
        }
    }
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

impl<'a> From<BinaryOp<'a>> for crate::BinaryOp {
    fn from(other: BinaryOp<'a>) -> Self {
        match other {
            BinaryOp::Equal(_) => Self::Equal,
            BinaryOp::NotEqual(_) => Self::NotEqual,
            BinaryOp::StrictEqual(_) => Self::StrictEqual,
            BinaryOp::StrictNotEqual(_) => Self::StrictNotEqual,
            BinaryOp::LessThan(_) => Self::LessThan,
            BinaryOp::GreaterThan(_) => Self::GreaterThan,
            BinaryOp::LessThanEqual(_) => Self::LessThanEqual,
            BinaryOp::GreaterThanEqual(_) => Self::GreaterThanEqual,
            BinaryOp::LeftShift(_) => Self::LeftShift,
            BinaryOp::RightShift(_) => Self::RightShift,
            BinaryOp::UnsignedRightShift(_) => Self::UnsignedRightShift,
            BinaryOp::Plus(_) => Self::Plus,
            BinaryOp::Minus(_) => Self::Minus,
            BinaryOp::Times(_) => Self::Times,
            BinaryOp::Over(_) => Self::Over,
            BinaryOp::Mod(_) => Self::Mod,
            BinaryOp::Or(_) => Self::Or,
            BinaryOp::XOr(_) => Self::XOr,
            BinaryOp::And(_) => Self::And,
            BinaryOp::In(_) => Self::In,
            BinaryOp::InstanceOf(_) => Self::InstanceOf,
            BinaryOp::PowerOf(_) => Self::PowerOf,
        }
    }
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

impl<'a> From<UpdateOp<'a>> for crate::UpdateOp {
    fn from(other: UpdateOp<'a>) -> Self {
        match other {
            UpdateOp::Increment(_) => Self::Increment,
            UpdateOp::Decrement(_) => Self::Decrement,
        }
    }
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

impl<'a> From<UnaryOp<'a>> for crate::UnaryOp {
    fn from(other: UnaryOp<'a>) -> Self {
        match other {
            UnaryOp::Minus(_) => Self::Minus,
            UnaryOp::Plus(_) => Self::Plus,
            UnaryOp::Not(_) => Self::Not,
            UnaryOp::Tilde(_) => Self::Tilde,
            UnaryOp::TypeOf(_) => Self::TypeOf,
            UnaryOp::Void(_) => Self::Void,
            UnaryOp::Delete(_) => Self::Delete,
        }
    }
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
    pub source: Cow<'a, str>,
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

#[derive(Debug, Clone, PartialEq)]
pub struct ListEntry<'a, T> {
    pub item: T,
    pub comma: Option<Slice<'a>>,
}

impl<'a, T> ListEntry<'a, T> {
    pub fn no_comma(item: T) -> Self {
        Self { item, comma: None }
    }
}

impl<'a, T> Node for ListEntry<'a, T>
where
    T: Node,
{
    fn loc(&self) -> SourceLocation {
        if let Some(comma) = &self.comma {
            return SourceLocation {
                start: self.item.loc().start,
                end: comma.loc.end,
            };
        }
        self.item.loc()
    }
}
