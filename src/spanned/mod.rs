mod convert;
pub mod decl;
pub mod expr;
pub mod pat;
pub mod stmt;
pub mod tokens;

use decl::Decl;
use expr::{Expr, Lit, Prop};
use pat::Pat;
use stmt::Stmt;

use crate::IntoAllocated;

use self::{
    pat::RestPat,
    tokens::{
        AssignOp, Asterisk, Async, CloseBrace, CloseParen, Comma, Const, Extends, Function, Let,
        OpenBrace, OpenParen, Semicolon, Token, Var,
    },
};

pub trait Node {
    fn loc(&self) -> SourceLocation;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ident<T> {
    pub slice: Slice<T>,
}

impl<T> Ident<T>
where
    T: AsRef<str>,
{
    pub fn new_from_source(source: T, line: u32, start_col: u32) -> Self {
        let len = source.as_ref().len() as u32;
        Slice::new(source, line, start_col, line, start_col + len).into()
    }
}


impl<T> IntoAllocated for Ident<T>
where
    T: ToString,
{
    type Allocated = Ident<String>;
    fn into_allocated(self) -> Ident<String> {
        Ident {
            slice: self.slice.into_allocated(),
        }
    }
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
        &self.slice.source
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

impl<T> IntoAllocated for Program<T> where T: ToString {
    type Allocated = Program<String>;
    fn into_allocated(self) -> Program<String> {
        match self {
            Program::Mod(parts) => Program::Mod(parts.into_iter().map(IntoAllocated::into_allocated).collect()),
            Program::Script(parts) => Program::Script(parts.into_iter().map(IntoAllocated::into_allocated).collect()),
        }
    }
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

impl<T> IntoAllocated for ProgramPart<T> where T: ToString {
    type Allocated = ProgramPart<String>;
    fn into_allocated(self) -> ProgramPart<String> {
        match self {
            ProgramPart::Dir(part) => ProgramPart::Dir(part.into_allocated()),
            ProgramPart::Decl(part) => ProgramPart::Decl(part.into_allocated()),
            ProgramPart::Stmt(part) => ProgramPart::Stmt(part.into_allocated()),
        }
    }
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
    pub dir: T,
    pub semi_colon: Option<Semicolon>,
}

impl<T> IntoAllocated for Dir<T> where T: ToString {
    type Allocated = Dir<String>;
    fn into_allocated(self) -> Dir<String> {
        Dir {
            expr: self.expr.into_allocated(),
            dir: self.dir.to_string(),
            semi_colon: self.semi_colon,
        }
    }
}

impl<T> Node for Dir<T> {
    fn loc(&self) -> SourceLocation {
        let expr_loc = self.expr.loc();
        if let Some(semi) = &self.semi_colon {
            return SourceLocation {
                start: expr_loc.start,
                end: semi.end(),
            };
        }
        expr_loc
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
    pub keyword: Function,
    pub id: Option<Ident<T>>,
    pub open_paren: OpenParen,
    pub params: Vec<ListEntry<FuncArg<T>>>,
    pub close_paren: CloseParen,
    pub body: FuncBody<T>,
    pub star: Option<Asterisk>,
    pub keyword_async: Option<Async>,
}

impl<T> Func<T> {
    pub fn is_async(&self) -> bool {
        self.keyword_async.is_some()
    }
    pub fn generator(&self) -> bool {
        self.star.is_some()
    }
}


impl<T> IntoAllocated for Func<T>
where
    T: ToString,
{
    type Allocated = Func<String>;
    fn into_allocated(self) -> Func<String> {
        Func {
            keyword: self.keyword,
            id: self.id.map(|i| i.into_allocated()),
            open_paren: self.open_paren,
            params: self.params.into_iter().map(|p| p.into_allocated()).collect(),
            close_paren: self.close_paren,
            body: self.body.into_allocated(),
            star: self.star,
            keyword_async: self.keyword_async,
        }
    }
}

impl<T> Node for Func<T> {
    fn loc(&self) -> SourceLocation {
        let start = if let Some(keyword) = self.keyword_async {
            keyword.start()
        } else {
            self.keyword.start()
        };
        let end = self.body.close_brace.end();
        SourceLocation { start, end }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FuncArgEntry<T> {
    pub value: FuncArg<T>,
    pub comma: Option<Comma>,
}

impl<T> IntoAllocated for FuncArgEntry<T> where T: ToString {
    type Allocated = FuncArgEntry<String>;
    fn into_allocated(self) -> FuncArgEntry<String> {
        FuncArgEntry { value: self.value.into_allocated(), comma: self.comma }
    }
}

impl<T> Node for FuncArgEntry<T> {
    fn loc(&self) -> SourceLocation {
        if let Some(comma) = &self.comma {
            return SourceLocation {
                start: self.value.loc().start,
                end: comma.end(),
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

impl<T> IntoAllocated for FuncArg<T> where T: ToString {
    type Allocated = FuncArg<String>;
    fn into_allocated(self) -> FuncArg<String> {
        match self {
            FuncArg::Expr(inner) => FuncArg::Expr(inner.into_allocated()),
            FuncArg::Pat(inner) => FuncArg::Pat(inner.into_allocated()),
            FuncArg::Rest(inner) => FuncArg::Rest(inner.into_allocated()),
        }
    }
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
    pub open_brace: OpenBrace,
    pub stmts: Vec<ProgramPart<T>>,
    pub close_brace: CloseBrace,
}

impl<T> IntoAllocated for FuncBody<T> where T: ToString {
    type Allocated = FuncBody<String>;
    fn into_allocated(self) -> FuncBody<String> {
        FuncBody { open_brace: self.open_brace, stmts: self.stmts.into_iter().map(|s| s.into_allocated()).collect(), close_brace: self.close_brace }
    }
}

impl<T> Node for FuncBody<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.open_brace.start(),
            end: self.close_brace.end(),
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
    pub keyword: tokens::Class,
    pub id: Option<Ident<T>>,
    pub super_class: Option<SuperClass<T>>,
    pub body: ClassBody<T>,
}


impl<T> IntoAllocated for Class<T> where T: ToString {
    type Allocated = Class<String>;
    fn into_allocated(self) -> Class<String> {
        Class {
            keyword: self.keyword, 
            id: self.id.map(|i| i.into_allocated()),
            super_class: self.super_class.map(|s| s.into_allocated()),
            body: self.body.into_allocated(),
        }
    }
}

impl<T> Node for Class<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.keyword.start(),
            end: self.body.close_brace.end(),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct SuperClass<T> {
    pub keyword_extends: Extends,
    pub expr: Expr<T>,
}

impl<T> IntoAllocated for SuperClass<T> where T: ToString {
    type Allocated = SuperClass<String>;
    fn into_allocated(self) -> SuperClass<String> {
        SuperClass { keyword_extends: self.keyword_extends, expr: self.expr.into_allocated() }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassBody<T> {
    pub open_brace: OpenBrace,
    pub props: Vec<Prop<T>>,
    pub close_brace: CloseBrace,
}

impl<T> IntoAllocated for ClassBody<T> where T: ToString {
    type Allocated = ClassBody<String>;
    fn into_allocated(self) -> ClassBody<String> {
        ClassBody { open_brace: self.open_brace, props: self.props.into_iter().map(IntoAllocated::into_allocated).collect(), close_brace: self.close_brace }
    }
}

impl<T> Node for ClassBody<T> {
    fn loc(&self) -> SourceLocation {
        let start = self.open_brace.start();
        let end = self.close_brace.end();
        SourceLocation { start, end }
    }
}

/// The kind of variable being defined (`var`/`let`/`const`)
#[derive(Debug, Clone, PartialEq)]
pub enum VarKind {
    Var(Option<Var>),
    Let(Let),
    Const(Const),
}

impl Node for VarKind {
    fn loc(&self) -> SourceLocation {
        match self {
            VarKind::Var(Some(tok)) => tok.loc(),
            VarKind::Let(tok) => tok.loc(),
            VarKind::Const(tok) => tok.loc(),
            _ => SourceLocation::zero(),
        }
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

#[derive(Debug, Clone, PartialEq)]
pub struct Slice<T> {
    pub source: T,
    pub loc: SourceLocation,
}

impl<T> IntoAllocated for Slice<T>
where
    T: ToString,
{
    type Allocated = Slice<String>;
    fn into_allocated(self) -> Slice<String> {
        Slice {
            loc: self.loc,
            source: self.source.to_string(),
        }
    }
}

impl<T> Slice<T> {
    pub fn new(source: T, start_line: u32, start_col: u32, end_line: u32, end_column: u32) -> Self {
        Self {
            source: source,
            loc: SourceLocation::new(start_line, start_col, end_line, end_column),
        }
    }
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

impl Position {
    pub fn new(line: u32, column: u32) -> Self {
        Self { line, column }
    }
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
    pub comma: Option<Comma>,
}

impl<Item> IntoAllocated for ListEntry<Item> where Item: IntoAllocated {
    type Allocated = ListEntry<Item::Allocated>;

    fn into_allocated(self) -> Self::Allocated {
        ListEntry {
            item: self.item.into_allocated(),
            comma: self.comma,
        }
    }
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
                end: comma.end(),
            };
        }
        self.item.loc()
    }
}
