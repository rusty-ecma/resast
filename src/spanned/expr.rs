use crate::spanned::pat::Pat;
use crate::spanned::{AssignOp, BinaryOp, LogicalOp, UnaryOp, UpdateOp};
use crate::spanned::{Class, Func, FuncArg, FuncBody, Ident};
use crate::SourceText;

use super::tokens::{
    Asterisk, Async, Await, CloseBrace, CloseBracket, CloseParen, Colon, Comma, Ellipsis, False,
    FatArrow, ForwardSlash, Get, New, Null, OpenBrace, OpenBracket, OpenParen, Period, QuasiQuote,
    QuestionMark, Quote, Set, Static, Super, This, Token, True, Yield,
};
use super::{FuncArgEntry, ListEntry, Node, Slice, SourceLocation};

/// A slightly more granular program part that a statement
#[derive(Debug, Clone, PartialEq)]
pub enum Expr<T> {
    /// `[0,,]`
    Array(ArrayExpr<T>),
    /// An arrow function
    /// ```js
    /// () => console.log();
    /// x => {
    ///     return x;
    /// }
    /// ```
    ArrowFunc(ArrowFuncExpr<T>),
    /// Used for resolving possible sequence expressions
    /// that are arrow parameters
    ArrowParamPlaceHolder(ArrowParamPlaceHolder<T>),
    /// Assignment or update assignment
    /// ```js
    /// a = 0
    /// b += 1
    /// ```
    Assign(AssignExpr<T>),
    /// The `await` keyword followed by another `Expr`
    Await(Box<AwaitExpr<T>>),
    /// An operation that has two arguments
    Binary(BinaryExpr<T>),
    /// A class expression see `Class`
    Class(Box<Class<T>>),
    /// Calling a function or method
    Call(CallExpr<T>),
    /// A ternery expression
    Conditional(ConditionalExpr<T>),
    /// see `Function`
    Func(Func<T>),
    /// An identifier
    Ident(Ident<T>),
    /// A literal value, see `Literal`
    Lit(Lit<T>),
    /// A specialized `BinaryExpr` for logical evaluation
    /// ```js
    /// true && true
    /// false || true
    /// ```
    Logical(LogicalExpr<T>),
    /// Accessing the member of a value
    /// ```js
    /// b['thing'];
    /// c.stuff;
    /// ```
    Member(MemberExpr<T>),
    /// currently just `new.target`
    MetaProp(MetaProp<T>),
    /// ```js
    /// var a = true ? 'stuff' : 'things';
    /// ```
    /// `{}`
    /// Calling a constructor
    New(NewExpr<T>),
    Obj(ObjExpr<T>),
    /// Any sequence of expressions separated with a comma
    Sequence(SequenceExpr<T>),
    /// `...` followed by an `Expr`
    Spread(Box<SpreadExpr<T>>),
    /// `super`
    Super(Super),
    /// A template literal preceded by a tag function identifier
    TaggedTemplate(TaggedTemplateExpr<T>),
    /// `this`
    This(This),
    /// An operation that has one argument
    /// ```js
    /// typeof T';
    /// +9;
    /// ```
    Unary(UnaryExpr<T>),
    /// Increment or decrement
    /// ```js
    /// 1++
    /// --2
    /// ```
    Update(UpdateExpr<T>),
    Wrapped(Box<WrappedExpr<T>>),
    /// yield a value from inside of a generator function
    Yield(YieldExpr<T>),
}

impl<T> Node for Expr<T> {
    fn loc(&self) -> SourceLocation {
        match self {
            Expr::Array(inner) => inner.loc(),
            Expr::ArrowFunc(inner) => inner.loc(),
            Expr::ArrowParamPlaceHolder(inner) => inner.loc(),
            Expr::Assign(inner) => inner.loc(),
            Expr::Await(inner) => inner.loc(),
            Expr::Binary(inner) => inner.loc(),
            Expr::Class(inner) => inner.loc(),
            Expr::Call(inner) => inner.loc(),
            Expr::Conditional(inner) => inner.loc(),
            Expr::Func(inner) => inner.loc(),
            Expr::Ident(inner) => inner.loc(),
            Expr::Lit(inner) => inner.loc(),
            Expr::Logical(inner) => inner.loc(),
            Expr::Member(inner) => inner.loc(),
            Expr::MetaProp(inner) => inner.loc(),
            Expr::New(inner) => inner.loc(),
            Expr::Obj(inner) => inner.loc(),
            Expr::Sequence(inner) => inner.loc(),
            Expr::Spread(inner) => inner.loc(),
            Expr::Super(inner) => inner.loc(),
            Expr::TaggedTemplate(inner) => inner.loc(),
            Expr::This(inner) => inner.loc(),
            Expr::Unary(inner) => inner.loc(),
            Expr::Update(inner) => inner.loc(),
            Expr::Yield(inner) => inner.loc(),
            Expr::Wrapped(inner) => inner.loc(),
        }
    }
}

type ArrayExprEntry<T> = ListEntry<Option<Expr<T>>>;

/// `[a, b, c]`
#[derive(Debug, Clone, PartialEq)]
pub struct ArrayExpr<T> {
    pub open_bracket: OpenBracket,
    pub elements: Vec<ArrayExprEntry<T>>,
    pub close_bracket: CloseBracket,
}

impl<T> Node for ArrayExpr<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.open_bracket.start(),
            end: self.close_bracket.end(),
        }
    }
}

/// `{a: 'b', c, ...d}`
#[derive(Debug, Clone, PartialEq)]
pub struct ObjExpr<T> {
    pub open_brace: OpenBrace,
    pub props: Vec<ListEntry<ObjProp<T>>>,
    pub close_brace: CloseBrace,
}

impl<T> Node for ObjExpr<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.open_brace.start(),
            end: self.close_brace.end(),
        }
    }
}

/// A single part of an object literal
#[derive(PartialEq, Debug, Clone)]
pub enum ObjProp<T> {
    Prop(Prop<T>),
    Spread(SpreadExpr<T>),
}

impl<T> Node for ObjProp<T> {
    fn loc(&self) -> SourceLocation {
        match self {
            ObjProp::Prop(inner) => inner.loc(),
            ObjProp::Spread(inner) => inner.loc(),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct SpreadExpr<T> {
    pub dots: Ellipsis,
    pub expr: Expr<T>,
}

impl<T> Node for SpreadExpr<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.dots.start(),
            end: self.expr.loc().end,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Prop<T> {
    Init(PropInit<T>),
    Method(PropMethod<T>),
    Ctor(PropCtor<T>),
    Get(PropGet<T>),
    Set(PropSet<T>),
}

impl<T> Node for Prop<T> {
    fn loc(&self) -> SourceLocation {
        match self {
            Prop::Init(inner) => inner.loc(),
            Prop::Method(inner) => inner.loc(),
            Prop::Ctor(inner) => inner.loc(),
            Prop::Get(inner) => inner.loc(),
            Prop::Set(inner) => inner.loc(),
        }
    }
}

impl<T> Prop<T> {
    pub fn computed(&self) -> bool {
        if let Self::Init(init) = self {
            init.computed()
        } else {
            false
        }
    }
    pub fn short_hand(&self) -> bool {
        if let Self::Init(init) = self {
            init.short_hand()
        } else {
            false
        }
    }
    pub fn is_async(&self) -> bool {
        if let Self::Method(meth) = self {
            meth.keyword_async.is_some()
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PropInit<T> {
    pub key: PropInitKey<T>,
    pub colon: Option<Colon>,
    pub value: Option<PropValue<T>>,
}

impl<T> Node for PropInit<T> {
    fn loc(&self) -> SourceLocation {
        if let Some(value) = &self.value {
            SourceLocation {
                start: self.key.loc().start,
                end: value.loc().end,
            }
        } else {
            self.key.loc()
        }
    }
}

impl<T> PropInit<T> {
    pub fn computed(&self) -> bool {
        self.key.brackets.is_some()
    }
    pub fn short_hand(&self) -> bool {
        self.value.is_none()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PropInitKey<T> {
    pub value: PropKey<T>,
    pub brackets: Option<(OpenBracket, CloseBracket)>,
}

impl<T> Node for PropInitKey<T> {
    fn loc(&self) -> SourceLocation {
        if let Some((open, close)) = &self.brackets {
            SourceLocation {
                start: open.start(),
                end: close.end(),
            }
        } else {
            self.value.loc()
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PropMethod<T> {
    pub keyword_static: Option<Static>,
    pub keyword_async: Option<Async>,
    pub id: PropInitKey<T>,
    pub star: Option<Asterisk>,
    pub open_paren: OpenParen,
    pub params: Vec<ListEntry<FuncArg<T>>>,
    pub close_paren: CloseParen,
    pub body: FuncBody<T>,
}

impl<T> Node for PropMethod<T> {
    fn loc(&self) -> SourceLocation {
        let start = if let Some(keyword) = &self.keyword_async {
            keyword.start()
        } else if let Some(star) = &self.star {
            star.start()
        } else {
            self.id.loc().start
        };
        SourceLocation {
            start,
            end: self.body.loc().end,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PropCtor<T> {
    pub keyword: PropInitKey<T>,
    pub open_paren: OpenParen,
    pub params: Vec<ListEntry<FuncArg<T>>>,
    pub close_paren: CloseParen,
    pub body: FuncBody<T>,
}

impl<T> Node for PropCtor<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.keyword.loc().start,
            end: self.body.loc().end,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PropGet<T> {
    pub keyword_static: Option<Static>,
    pub keyword_get: Get,
    pub id: PropInitKey<T>,
    pub open_paren: OpenParen,
    pub close_paren: CloseParen,
    pub body: FuncBody<T>,
}

impl<T> Node for PropGet<T> {
    fn loc(&self) -> SourceLocation {
        if let Some(keyword_static) = &self.keyword_static {
            return SourceLocation {
                start: keyword_static.start(),
                end: self.body.loc().end,
            };
        }
        SourceLocation {
            start: self.keyword_get.start(),
            end: self.body.loc().end,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PropSet<T> {
    pub keyword_static: Option<Static>,
    pub keyword_set: Set,
    pub id: PropInitKey<T>,
    pub open_paren: OpenParen,
    pub arg: ListEntry<FuncArg<T>>,
    pub close_paren: CloseParen,
    pub body: FuncBody<T>,
}

impl<T> Node for PropSet<T> {
    fn loc(&self) -> SourceLocation {
        if let Some(keyword_static) = &self.keyword_static {
            return SourceLocation {
                start: keyword_static.start(),
                end: self.body.loc().end,
            };
        }
        SourceLocation {
            start: self.keyword_set.start(),
            end: self.body.loc().end,
        }
    }
}

/// An object literal or class property identifier
#[derive(PartialEq, Debug, Clone)]
pub enum PropKey<T> {
    Lit(Lit<T>),
    Expr(Expr<T>),
    Pat(Pat<T>),
}

impl<T> Node for PropKey<T> {
    fn loc(&self) -> SourceLocation {
        match self {
            PropKey::Lit(inner) => inner.loc(),
            PropKey::Expr(inner) => inner.loc(),
            PropKey::Pat(inner) => inner.loc(),
        }
    }
}

/// The value of an object literal or class property
#[derive(PartialEq, Debug, Clone)]
pub enum PropValue<T> {
    Expr(Expr<T>),
    Pat(Pat<T>),
    Method(PropMethod<T>),
}

impl<T> Node for PropValue<T> {
    fn loc(&self) -> SourceLocation {
        match self {
            PropValue::Expr(inner) => inner.loc(),
            PropValue::Pat(inner) => inner.loc(),
            PropValue::Method(inner) => inner.loc(),
        }
    }
}

/// An operation that takes one argument
#[derive(PartialEq, Debug, Clone)]
pub struct UnaryExpr<T> {
    pub operator: UnaryOp,
    pub argument: Box<Expr<T>>,
}

impl<T> UnaryExpr<T> {
    pub fn prefix(&self) -> bool {
        self.operator.loc() < self.argument.loc()
    }
}

impl<T> Node for UnaryExpr<T> {
    fn loc(&self) -> SourceLocation {
        let (start, end) = if self.prefix() {
            (self.operator.loc().start, self.argument.loc().end)
        } else {
            (self.argument.loc().start, self.operator.loc().end)
        };
        SourceLocation { start, end }
    }
}

/// Increment or decrementing a value
#[derive(PartialEq, Debug, Clone)]
pub struct UpdateExpr<T> {
    pub operator: UpdateOp,
    pub argument: Box<Expr<T>>,
}

impl<T> UpdateExpr<T> {
    pub fn prefix(&self) -> bool {
        self.operator.loc().start < self.argument.loc().start
    }
}

impl<T> Node for UpdateExpr<T> {
    fn loc(&self) -> SourceLocation {
        let op = self.operator.loc();
        let arg = self.argument.loc();
        if op < arg {
            SourceLocation {
                start: op.start,
                end: arg.end,
            }
        } else {
            SourceLocation {
                start: arg.start,
                end: op.end,
            }
        }
    }
}

/// An operation that requires 2 arguments
#[derive(PartialEq, Debug, Clone)]
pub struct BinaryExpr<T> {
    pub operator: BinaryOp,
    pub left: Box<Expr<T>>,
    pub right: Box<Expr<T>>,
}

impl<T> Node for BinaryExpr<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.left.loc().start,
            end: self.right.loc().end,
        }
    }
}

/// An assignment or update + assignment operation
#[derive(PartialEq, Debug, Clone)]
pub struct AssignExpr<T> {
    pub operator: AssignOp,
    pub left: AssignLeft<T>,
    pub right: Box<Expr<T>>,
}

impl<T> Node for AssignExpr<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.left.loc().start,
            end: self.right.loc().end,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct AwaitExpr<T> {
    pub keyword: Await,
    pub expr: Expr<T>,
}

impl<T> Node for AwaitExpr<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.keyword.start(),
            end: self.expr.loc().end,
        }
    }
}

/// The value being assigned to
#[derive(PartialEq, Debug, Clone)]
pub enum AssignLeft<T> {
    Pat(Pat<T>),
    Expr(Box<Expr<T>>),
}

impl<T> Node for AssignLeft<T> {
    fn loc(&self) -> SourceLocation {
        match self {
            AssignLeft::Pat(inner) => inner.loc(),
            AssignLeft::Expr(inner) => inner.loc(),
        }
    }
}

/// A specialized `BinaryExpr` for logical evaluation
/// ```js
/// true && true
/// false || true
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct LogicalExpr<T> {
    pub operator: LogicalOp,
    pub left: Box<Expr<T>>,
    pub right: Box<Expr<T>>,
}

impl<T> Node for LogicalExpr<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.left.loc().start,
            end: self.right.loc().end,
        }
    }
}

/// Accessing the member of a value
/// ```js
/// b['thing'];
/// c.stuff;
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct MemberExpr<T> {
    pub object: Box<Expr<T>>,
    pub property: Box<Expr<T>>,
    pub indexer: MemberIndexer,
}

impl<T> MemberExpr<T> {
    pub fn computed(&self) -> bool {
        matches!(self.indexer, MemberIndexer::Computed { .. })
    }
}

impl<T> Node for MemberExpr<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.object.loc().start,
            end: if self.computed() {
                self.indexer.loc().end
            } else {
                self.property.loc().end
            },
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum MemberIndexer {
    Period(Period),
    Computed {
        open_bracket: OpenBracket,
        close_bracket: CloseBracket,
    },
}

impl Node for MemberIndexer {
    fn loc(&self) -> SourceLocation {
        match self {
            MemberIndexer::Period(inner) => inner.loc(),
            MemberIndexer::Computed {
                open_bracket,
                close_bracket,
            } => SourceLocation {
                start: open_bracket.start(),
                end: close_bracket.end(),
            },
        }
    }
}

/// A ternery expression
/// ```js
/// var a = true ? 'stuff' : 'things';
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct ConditionalExpr<T> {
    pub test: Box<Expr<T>>,
    pub question_mark: QuestionMark,
    pub alternate: Box<Expr<T>>,
    pub colon: Colon,
    pub consequent: Box<Expr<T>>,
}

impl<T> Node for ConditionalExpr<T> {
    fn loc(&self) -> SourceLocation {
        let start = self.test.loc().start;
        let end = self.alternate.loc().end;
        SourceLocation { start, end }
    }
}

/// Calling a function or method
/// ```js
/// Math.random()
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct CallExpr<T> {
    pub callee: Box<Expr<T>>,
    pub open_paren: OpenParen,
    pub arguments: Vec<ListEntry<Expr<T>>>,
    pub close_paren: CloseParen,
}

impl<T> Node for CallExpr<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.callee.loc().start,
            end: self.close_paren.end(),
        }
    }
}

/// Calling a constructor
/// ```js
/// new Uint8Array(32);
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct NewExpr<T> {
    pub keyword: New,
    pub callee: Box<Expr<T>>,
    pub open_paren: Option<OpenParen>,
    pub arguments: Vec<ListEntry<Expr<T>>>,
    pub close_paren: Option<CloseParen>,
}

impl<T> Node for NewExpr<T> {
    fn loc(&self) -> SourceLocation {
        let end = if let Some(close) = &self.close_paren {
            close.end()
        } else if let Some(last) = self.arguments.last() {
            last.loc().end
        } else {
            self.callee.loc().end
        };
        SourceLocation {
            start: self.keyword.start(),
            end: end,
        }
    }
}

/// A collection of `Exprs` separated by commas
pub type SequenceExpr<T> = Vec<ListEntry<Expr<T>>>;

impl<T> Node for SequenceExpr<T> {
    fn loc(&self) -> SourceLocation {
        let first_loc = if let Some(first) = self.first() {
            first.loc()
        } else {
            SourceLocation::zero()
        };
        let last_loc = if let Some(last) = self.last() {
            last.loc()
        } else {
            SourceLocation::zero()
        };
        SourceLocation {
            start: first_loc.start,
            end: last_loc.end,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrowParamPlaceHolder<T> {
    // async keyword
    pub keyword: Option<Async>,
    pub open_paren: Option<OpenParen>,
    pub args: Vec<ListEntry<FuncArg<T>>>,
    pub close_paren: Option<CloseParen>,
}

impl<T> Node for ArrowParamPlaceHolder<T> {
    fn loc(&self) -> SourceLocation {
        let start = if let Some(keyword) = &self.keyword {
            keyword.start()
        } else if let Some(open) = &self.open_paren {
            open.start()
        } else if let Some(arg) = self.args.first() {
            arg.loc().start
        } else {
            crate::spanned::Position { line: 0, column: 0 }
        };
        let end = if let Some(close) = &self.close_paren {
            close.end()
        } else if let Some(arg) = self.args.last() {
            arg.loc().end
        } else {
            crate::spanned::Position { line: 0, column: 0 }
        };
        SourceLocation { start, end }
    }
}

/// An arrow function
/// ```js
/// let x = () => y;
/// let q = x => {
///     return x + 1;
/// }
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct ArrowFuncExpr<T> {
    pub keyword: Option<Async>,
    pub star: Option<Asterisk>,
    pub open_paren: Option<OpenParen>,
    pub params: Vec<ListEntry<FuncArg<T>>>,
    pub close_paren: Option<CloseParen>,
    pub arrow: FatArrow,
    pub body: ArrowFuncBody<T>,
}

impl<T> Node for ArrowFuncExpr<T> {
    fn loc(&self) -> SourceLocation {
        let start = if let Some(keyword) = &self.keyword {
            keyword.start()
        } else if let Some(slice) = &self.open_paren {
            slice.start()
        } else if let Some(first) = self.params.first() {
            first.loc().start
        } else {
            SourceLocation::zero().start
        };
        SourceLocation {
            start,
            end: self.body.loc().end,
        }
    }
}

/// The body portion of an arrow function can be either an expression or a block of statements
#[derive(PartialEq, Debug, Clone)]
pub enum ArrowFuncBody<T> {
    FuncBody(FuncBody<T>),
    Expr(Box<Expr<T>>),
}

impl<T> Node for ArrowFuncBody<T> {
    fn loc(&self) -> SourceLocation {
        match self {
            ArrowFuncBody::FuncBody(inner) => inner.loc(),
            ArrowFuncBody::Expr(inner) => inner.loc(),
        }
    }
}

/// yield a value from inside of a generator function
/// ```js
/// function *gen() {
///     while ((new Date() / 1000) < Number.MAX_VALUE) {
///         yield new Date() / 1000;
///     }
/// }
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct YieldExpr<T> {
    pub keyword: Yield,
    pub argument: Option<Box<Expr<T>>>,
    pub star: Option<Asterisk>,
}

impl<T> Node for YieldExpr<T> {
    fn loc(&self) -> SourceLocation {
        let end = if let Some(arg) = &self.argument {
            arg.loc().end
        } else {
            self.keyword.end()
        };
        SourceLocation {
            start: self.keyword.start(),
            end,
        }
    }
}

/// A Template literal preceded by a function identifier
/// see [MDN](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Template_literals#Tagged_templates) for more details
#[derive(PartialEq, Debug, Clone)]
pub struct TaggedTemplateExpr<T> {
    pub tag: Box<Expr<T>>,
    pub quasi: TemplateLit<T>,
}

impl<T> Node for TaggedTemplateExpr<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.tag.loc().start,
            end: self.quasi.loc().end,
        }
    }
}

/// A template string literal
/// ```js
/// `I own ${0} birds`;
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct TemplateLit<T> {
    pub quasis: Vec<TemplateElement<T>>,
    pub expressions: Vec<Expr<T>>,
}

impl<T> Node for TemplateLit<T> {
    fn loc(&self) -> SourceLocation {
        let start = self
            .quasis
            .first()
            .map(|q| q.loc())
            .unwrap_or_else(SourceLocation::zero);
        let end = self
            .quasis
            .last()
            .map(|q| q.loc())
            .unwrap_or_else(SourceLocation::zero);
        SourceLocation {
            start: start.start,
            end: end.end,
        }
    }
}

/// The text part of a `TemplateLiteral`
#[derive(Debug, Clone, PartialEq)]
pub struct TemplateElement<T> {
    pub open_quote: QuasiQuote,
    pub content: SourceText<T>,
    pub close_quote: QuasiQuote,
}

impl<T> TemplateElement<T>
where
    T: AsRef<str>,
{
    pub fn is_tail(&self) -> bool {
        matches!(
            self.open_quote,
            QuasiQuote::BackTick(_) | QuasiQuote::CloseBrace(_)
        ) && matches!(self.close_quote, QuasiQuote::BackTick(_))
    }
}

impl<T> Node for TemplateElement<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.open_quote.start(),
            end: self.close_quote.end(),
        }
    }
}

/// pretty much just `new.target`
/// ```js
/// function Thing(one, two) {
///     if (!new.target) {
///         return new Thing(one, two);
///     }
///     this.one = one;
///     this.two = two;
/// }
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct MetaProp<T> {
    pub meta: Ident<T>,
    pub dot: Period,
    pub property: Ident<T>,
}

impl<T> Node for MetaProp<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.meta.loc().start,
            end: self.property.loc().end,
        }
    }
}

/// A literal value
#[derive(Debug, Clone, PartialEq)]
pub enum Lit<T> {
    /// `null`
    Null(Null),
    /// `"string"`
    /// `'string'`
    String(StringLit<T>),
    /// `0`
    /// `0.0`
    /// `.0`
    /// `0.0e1`
    /// `.0E1`
    /// `0xf`
    /// `0o7`
    /// `0b1`
    Number(Slice<T>),
    /// `true`
    /// `false`
    Boolean(Boolean),
    /// `/.+/g`
    RegEx(RegEx<T>),
    /// ```js
    /// `I have ${0} apples`
    /// ```
    Template(TemplateLit<T>),
}

impl<T> Lit<T> {
    pub fn new_true(line: u32, column: u32) -> Self {
        Self::Boolean(Boolean::new_true(line, column))
    }

    pub fn new_false(line: u32, column: u32) -> Self {
        Self::Boolean(Boolean::new_false(line, column))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Boolean {
    True(True),
    False(False),
}

impl Boolean {
    pub fn new_true(line: u32, column: u32) -> Self {
        Self::True(crate::spanned::Position::new(line, column).into())
    }

    pub fn new_false(line: u32, column: u32) -> Self {
        Self::False(crate::spanned::Position::new(line, column).into())
    }
}

impl Node for Boolean {
    fn loc(&self) -> SourceLocation {
        match self {
            Boolean::True(inner) => inner.loc(),
            Boolean::False(inner) => inner.loc(),
        }
    }
}

impl<T> Node for Lit<T> {
    fn loc(&self) -> SourceLocation {
        match self {
            Lit::Null(inner) => inner.loc(),
            Lit::String(inner) => inner.loc(),
            Lit::Number(inner) => inner.loc,
            Lit::Boolean(inner) => inner.loc(),
            Lit::RegEx(inner) => inner.loc(),
            Lit::Template(inner) => inner.loc(),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct StringLit<T> {
    pub open_quote: Quote,
    pub content: Slice<T>,
    pub close_quote: Quote,
}

impl<T> Node for StringLit<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.open_quote.start(),
            end: self.close_quote.end(),
        }
    }
}

/// A regular expression literal
#[derive(PartialEq, Debug, Clone)]
pub struct RegEx<T> {
    pub open_slash: ForwardSlash,
    pub pattern: Slice<T>,
    pub close_slash: ForwardSlash,
    pub flags: Option<Slice<T>>,
}

impl<T> Node for RegEx<T> {
    fn loc(&self) -> SourceLocation {
        let end = if let Some(flags) = &self.flags {
            flags.loc.end
        } else {
            self.close_slash.end()
        };
        SourceLocation {
            start: self.open_slash.start(),
            end,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WrappedExpr<T> {
    pub open_paren: OpenParen,
    pub expr: Expr<T>,
    pub close_paren: CloseParen,
}

impl<T> Node for WrappedExpr<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.open_paren.start(),
            end: self.close_paren.end(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SequenceExprEntry<T> {
    pub expr: Expr<T>,
    pub comma: Option<Comma>,
}

impl<T> SequenceExprEntry<T> {
    pub fn no_comma(expr: Expr<T>) -> Self {
        Self { expr, comma: None }
    }
}

impl<T> From<SequenceExprEntry<T>> for FuncArgEntry<T> {
    fn from(other: SequenceExprEntry<T>) -> Self {
        Self {
            value: FuncArg::Expr(other.expr),
            comma: other.comma,
        }
    }
}

impl<T> Node for SequenceExprEntry<T> {
    fn loc(&self) -> SourceLocation {
        if let Some(comma) = &self.comma {
            return SourceLocation {
                start: self.expr.loc().start,
                end: comma.end(),
            };
        }
        self.expr.loc()
    }
}
