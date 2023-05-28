use crate::spanned::pat::Pat;
use crate::spanned::{AssignOp, BinaryOp, LogicalOp, UnaryOp, UpdateOp};
use crate::spanned::{Class, Func, FuncArg, FuncBody, Ident};

use super::{FuncArgEntry, ListEntry, Node, Position, Slice, SourceLocation};

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
    Super(Position),
    /// A template literal preceded by a tag function identifier
    TaggedTemplate(TaggedTemplateExpr<T>),
    /// `this`
    This(Position),
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
            Expr::Super(inner) => SourceLocation {
                start: *inner,
                end: *inner + 5,
            },
            Expr::TaggedTemplate(inner) => inner.loc(),
            Expr::This(inner) => SourceLocation {
                start: *inner,
                end: *inner + 4,
            },
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
    pub open_bracket: Position,
    pub elements: Vec<ArrayExprEntry<T>>,
    pub close_bracket: Position,
}

impl<T> Node for ArrayExpr<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.open_bracket,
            end: self.close_bracket + 1,
        }
    }
}

/// `{a: 'b', c, ...d}`
#[derive(Debug, Clone, PartialEq)]
pub struct ObjExpr<T> {
    pub open_brace: Position,
    pub props: Vec<ListEntry<ObjProp<T>>>,
    pub close_brace: Position,
}

impl<T> Node for ObjExpr<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.open_brace,
            end: self.close_brace + 1,
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
    pub dots: Position,
    pub expr: Expr<T>,
}

impl<T> Node for SpreadExpr<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.dots,
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
    pub colon: Option<Position>,
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
    pub brackets: Option<(Position, Position)>,
}

impl<T> Node for PropInitKey<T> {
    fn loc(&self) -> SourceLocation {
        if let Some((open, close)) = &self.brackets {
            SourceLocation {
                start: *open,
                end: *close + 1,
            }
        } else {
            self.value.loc()
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PropMethod<T> {
    pub keyword_static: Option<Position>,
    pub keyword_async: Option<Position>,
    pub id: PropInitKey<T>,
    pub star: Option<Position>,
    pub open_paren: Position,
    pub params: Vec<ListEntry<FuncArg<T>>>,
    pub close_paren: Position,
    pub body: FuncBody<T>,
}

impl<T> Node for PropMethod<T> {
    fn loc(&self) -> SourceLocation {
        let start = if let Some(keyword) = &self.keyword_async {
            *keyword
        } else if let Some(star) = &self.star {
            *star
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
    pub open_paren: Position,
    pub params: Vec<ListEntry<FuncArg<T>>>,
    pub close_paren: Position,
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
    pub keyword_static: Option<Position>,
    pub keyword_get: Position,
    pub id: PropInitKey<T>,
    pub open_paren: Position,
    pub close_paren: Position,
    pub body: FuncBody<T>,
}

impl<T> Node for PropGet<T> {
    fn loc(&self) -> SourceLocation {
        if let Some(keyword_static) = &self.keyword_static {
            return SourceLocation {
                start: *keyword_static,
                end: self.body.loc().end,
            };
        }
        SourceLocation {
            start: self.keyword_get,
            end: self.body.loc().end,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PropSet<T> {
    pub keyword_static: Option<Position>,
    pub keyword_set: Position,
    pub id: PropInitKey<T>,
    pub open_paren: Position,
    pub arg: ListEntry<FuncArg<T>>,
    pub close_paren: Position,
    pub body: FuncBody<T>,
}

impl<T> Node for PropSet<T> {
    fn loc(&self) -> SourceLocation {
        if let Some(keyword_static) = &self.keyword_static {
            return SourceLocation {
                start: *keyword_static,
                end: self.body.loc().end,
            };
        }
        SourceLocation {
            start: self.keyword_set,
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
    pub keyword: Position,
    pub expr: Expr<T>,
}

impl<T> Node for AwaitExpr<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.keyword,
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
    Period(Position),
    Computed {
        open_bracket: Position,
        close_bracket: Position,
    },
}

impl Node for MemberIndexer {
    fn loc(&self) -> SourceLocation {
        match self {
            MemberIndexer::Period(inner) => SourceLocation {
                start: *inner,
                end: *inner + 1,
            },
            MemberIndexer::Computed {
                open_bracket,
                close_bracket,
            } => SourceLocation {
                start: *open_bracket,
                end: *close_bracket + 1,
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
    pub question_mark: Position,
    pub alternate: Box<Expr<T>>,
    pub colon: Position,
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
    pub open_paren: Position,
    pub arguments: Vec<ListEntry<Expr<T>>>,
    pub close_paren: Position,
}

impl<T> Node for CallExpr<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.callee.loc().start,
            end: self.close_paren + 1,
        }
    }
}

/// Calling a constructor
/// ```js
/// new Uint8Array(32);
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct NewExpr<T> {
    pub keyword: Position,
    pub callee: Box<Expr<T>>,
    pub open_paren: Option<Position>,
    pub arguments: Vec<ListEntry<Expr<T>>>,
    pub close_paren: Option<Position>,
}

impl<T> Node for NewExpr<T> {
    fn loc(&self) -> SourceLocation {
        let end = if let Some(close) = &self.close_paren {
            *close
        } else if let Some(last) = self.arguments.last() {
            last.loc().end
        } else {
            self.callee.loc().end
        };
        SourceLocation {
            start: self.callee.loc().start,
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
    pub keyword: Option<Position>,
    pub open_paren: Option<Position>,
    pub args: Vec<ListEntry<FuncArg<T>>>,
    pub close_paren: Option<Position>,
}

impl<T> Node for ArrowParamPlaceHolder<T> {
    fn loc(&self) -> SourceLocation {
        let start = if let Some(keyword) = &self.keyword {
            *keyword
        } else if let Some(open) = &self.open_paren {
            *open
        } else if let Some(arg) = self.args.first() {
            arg.loc().start
        } else {
            Position { line: 0, column: 0 }
        };
        let end = if let Some(close) = &self.close_paren {
            *close + 1
        } else if let Some(arg) = self.args.last() {
            arg.loc().end
        } else {
            Position { line: 0, column: 0 }
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
    pub keyword: Option<Position>,
    pub star: Option<Position>,
    pub open_paren: Option<Position>,
    pub params: Vec<ListEntry<FuncArg<T>>>,
    pub close_paren: Option<Position>,
    pub arrow: Position,
    pub body: ArrowFuncBody<T>,
}

impl<T> Node for ArrowFuncExpr<T> {
    fn loc(&self) -> SourceLocation {
        let start = if let Some(slice) = &self.open_paren {
            *slice
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
    pub keyword: Position,
    pub argument: Option<Box<Expr<T>>>,
    pub star: Option<Position>,
}

impl<T> Node for YieldExpr<T> {
    fn loc(&self) -> SourceLocation {
        let end = if let Some(arg) = &self.argument {
            arg.loc().end
        } else {
            self.keyword + 5
        };
        SourceLocation {
            start: self.keyword,
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
    /// Raw quoted element
    pub raw: Slice<T>,
    pub cooked: Slice<T>,
}

impl<T> TemplateElement<T>
where
    T: AsRef<str>,
{
    pub fn is_tail(&self) -> bool {
        self.raw
            .source
            .0
            .as_ref()
            .starts_with(|c| c == '`' || c == '}')
            && self.raw.source.0.as_ref().ends_with('`')
    }
}

impl<T> Node for TemplateElement<T> {
    fn loc(&self) -> SourceLocation {
        self.raw.loc
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
    pub dot: Position,
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
    Null(Position),
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
    True(Position),
    False(Position),
}

impl Boolean {
    pub fn new_true(line: u32, column: u32) -> Self {
        Self::True(Position::new(line, column))
    }

    pub fn new_false(line: u32, column: u32) -> Self {
        Self::False(Position::new(line, column))
    }
}

impl Node for Boolean {
    fn loc(&self) -> SourceLocation {
        match self {
            Boolean::True(start) => SourceLocation {
                start: *start,
                end: *start + 4,
            },
            Boolean::False(start) => SourceLocation {
                start: *start,
                end: *start + 5,
            },
        }
    }
}

impl<T> Node for Lit<T> {
    fn loc(&self) -> SourceLocation {
        match self {
            Lit::Null(inner) => SourceLocation {
                start: *inner,
                end: *inner + 4,
            },
            Lit::String(inner) => inner.loc(),
            Lit::Number(inner) => inner.loc,
            Lit::Boolean(inner) => inner.loc(),
            Lit::RegEx(inner) => inner.loc(),
            Lit::Template(inner) => inner.loc(),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Quote {
    Double(Position),
    Single(Position),
}

impl Quote {
    pub fn position(&self) -> Position {
        match self {
            Quote::Double(inner) => *inner,
            Quote::Single(inner) => *inner,
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
            start: self.open_quote.position(),
            end: self.close_quote.position() + 1,
        }
    }
}

/// A regular expression literal
#[derive(PartialEq, Debug, Clone)]
pub struct RegEx<T> {
    pub open_slash: Position,
    pub pattern: Slice<T>,
    pub close_slash: Position,
    pub flags: Option<Slice<T>>,
}

impl<T> Node for RegEx<T> {
    fn loc(&self) -> SourceLocation {
        let end = if let Some(flags) = &self.flags {
            flags.loc.end
        } else {
            self.close_slash + 1
        };
        SourceLocation {
            start: self.open_slash,
            end,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WrappedExpr<T> {
    pub open_paren: Position,
    pub expr: Expr<T>,
    pub close_paren: Position,
}

impl<T> Node for WrappedExpr<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.open_paren,
            end: self.close_paren + 1,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SequenceExprEntry<T> {
    pub expr: Expr<T>,
    pub comma: Option<Position>,
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
                end: *comma + 1,
            };
        }
        self.expr.loc()
    }
}
