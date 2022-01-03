use std::borrow::Cow;

use crate::spanned::pat::Pat;
use crate::spanned::{AssignOp, BinaryOp, LogicalOp, UnaryOp, UpdateOp};
use crate::spanned::{Class, Func, FuncArg, FuncBody, Ident};

use super::{FuncArgEntry, ListEntry, Node, Position, Slice, SourceLocation};

/// A slightly more granular program part that a statement
#[derive(Debug, Clone, PartialEq)]
pub enum Expr<'a> {
    /// `[0,,]`
    Array(ArrayExpr<'a>),
    /// An arrow function
    /// ```js
    /// () => console.log();
    /// x => {
    ///     return x;
    /// }
    /// ```
    ArrowFunc(ArrowFuncExpr<'a>),
    /// Used for resolving possible sequence expressions
    /// that are arrow parameters
    ArrowParamPlaceHolder(ArrowParamPlaceHolder<'a>),
    /// Assignment or update assignment
    /// ```js
    /// a = 0
    /// b += 1
    /// ```
    Assign(AssignExpr<'a>),
    /// The `await` keyword followed by another `Expr`
    Await(Box<AwaitExpr<'a>>),
    /// An operation that has two arguments
    Binary(BinaryExpr<'a>),
    /// A class expression see `Class`
    Class(Box<Class<'a>>),
    /// Calling a function or method
    Call(CallExpr<'a>),
    /// A ternery expression
    Conditional(ConditionalExpr<'a>),
    /// see `Function`
    Func(Func<'a>),
    /// An identifier
    Ident(Ident<'a>),
    /// A literal value, see `Literal`
    Lit(Lit<'a>),
    /// A specialized `BinaryExpr` for logical evaluation
    /// ```js
    /// true && true
    /// false || true
    /// ```
    Logical(LogicalExpr<'a>),
    /// Accessing the member of a value
    /// ```js
    /// b['thing'];
    /// c.stuff;
    /// ```
    Member(MemberExpr<'a>),
    /// currently just `new.target`
    MetaProp(MetaProp<'a>),
    /// ```js
    /// var a = true ? 'stuff' : 'things';
    /// ```
    /// `{}`
    /// Calling a constructor
    New(NewExpr<'a>),
    Obj(ObjExpr<'a>),
    /// Any sequence of expressions separated with a comma
    Sequence(SequenceExpr<'a>),
    /// `...` followed by an `Expr`
    Spread(Box<SpreadExpr<'a>>),
    /// `super`
    Super(Slice<'a>),
    /// A template literal preceded by a tag function identifier
    TaggedTemplate(TaggedTemplateExpr<'a>),
    /// `this`
    This(Slice<'a>),
    /// An operation that has one argument
    /// ```js
    /// typeof 'a';
    /// +9;
    /// ```
    Unary(UnaryExpr<'a>),
    /// Increment or decrement
    /// ```js
    /// 1++
    /// --2
    /// ```
    Update(UpdateExpr<'a>),
    Wrapped(Box<WrappedExpr<'a>>),
    /// yield a value from inside of a generator function
    Yield(YieldExpr<'a>),
}

impl<'a> From<Expr<'a>> for crate::Expr<'a> {
    fn from(other: Expr<'a>) -> Self {
        match other {
            Expr::Array(inner) => Self::Array(
                inner
                    .elements
                    .into_iter()
                    .map(|e| e.item.map(From::from))
                    .collect(),
            ),
            Expr::ArrowFunc(inner) => Self::ArrowFunc(inner.into()),
            Expr::ArrowParamPlaceHolder(inner) => Self::ArrowParamPlaceHolder(
                inner.args.into_iter().map(|e| From::from(e.item)).collect(),
                inner.keyword.is_some(),
            ),
            Expr::Assign(inner) => Self::Assign(inner.into()),
            Expr::Await(inner) => Self::Await(Box::new(inner.expr.into())),
            Expr::Binary(inner) => Self::Binary(inner.into()),
            Expr::Class(inner) => Self::Class((*inner).into()),
            Expr::Call(inner) => Self::Call(inner.into()),
            Expr::Conditional(inner) => Self::Conditional(inner.into()),
            Expr::Func(inner) => Self::Func(inner.into()),
            Expr::Ident(inner) => Self::Ident(inner.into()),
            Expr::Lit(inner) => Self::Lit(inner.into()),
            Expr::Logical(inner) => Self::Logical(inner.into()),
            Expr::Member(inner) => Self::Member(inner.into()),
            Expr::MetaProp(inner) => Self::MetaProp(inner.into()),
            Expr::New(inner) => Self::New(inner.into()),
            Expr::Obj(inner) => Self::Obj(inner.props.into_iter().map(|e| e.item.into()).collect()),
            Expr::Sequence(inner) => {
                Self::Sequence(inner.into_iter().map(|e| e.item.into()).collect())
            }
            Expr::Spread(inner) => Self::Spread(Box::new(inner.expr.into())),
            Expr::Super(_) => Self::Super,
            Expr::TaggedTemplate(inner) => Self::TaggedTemplate(inner.into()),
            Expr::This(_) => Self::This,
            Expr::Unary(inner) => Self::Unary(inner.into()),
            Expr::Update(inner) => Self::Update(inner.into()),
            Expr::Yield(inner) => Self::Yield(inner.into()),
            Expr::Wrapped(inner) => inner.expr.into(),
        }
    }
}

impl<'a> Node for Expr<'a> {
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
            Expr::Super(inner) => inner.loc,
            Expr::TaggedTemplate(inner) => inner.loc(),
            Expr::This(inner) => inner.loc,
            Expr::Unary(inner) => inner.loc(),
            Expr::Update(inner) => inner.loc(),
            Expr::Yield(inner) => inner.loc(),
            Expr::Wrapped(inner) => inner.loc(),
        }
    }
}

type ArrayExprEntry<'a> = ListEntry<'a, Option<Expr<'a>>>;

/// `[a, b, c]`
#[derive(Debug, Clone, PartialEq)]
pub struct ArrayExpr<'a> {
    pub open_bracket: Slice<'a>,
    pub elements: Vec<ArrayExprEntry<'a>>,
    pub close_bracket: Slice<'a>,
}

impl<'a> Node for ArrayExpr<'a> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.open_bracket.loc.start,
            end: self.close_bracket.loc.end,
        }
    }
}

/// `{a: 'b', c, ...d}`
#[derive(Debug, Clone, PartialEq)]
pub struct ObjExpr<'a> {
    pub open_brace: Slice<'a>,
    pub props: Vec<ListEntry<'a, ObjProp<'a>>>,
    pub close_brace: Slice<'a>,
}

impl<'a> Node for ObjExpr<'a> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.open_brace.loc.start,
            end: self.close_brace.loc.end,
        }
    }
}

/// A single part of an object literal
#[derive(PartialEq, Debug, Clone)]
pub enum ObjProp<'a> {
    Prop(Prop<'a>),
    Spread(SpreadExpr<'a>),
}

impl<'a> From<ObjProp<'a>> for crate::expr::ObjProp<'a> {
    fn from(other: ObjProp<'a>) -> Self {
        match other {
            ObjProp::Prop(inner) => Self::Prop(inner.into()),
            ObjProp::Spread(inner) => Self::Spread(inner.expr.into()),
        }
    }
}

impl<'a> Node for ObjProp<'a> {
    fn loc(&self) -> SourceLocation {
        match self {
            ObjProp::Prop(inner) => inner.loc(),
            ObjProp::Spread(inner) => inner.loc(),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct SpreadExpr<'a> {
    pub dots: Slice<'a>,
    pub expr: Expr<'a>,
}

impl<'a> Node for SpreadExpr<'a> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.dots.loc.start,
            end: self.expr.loc().end,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Prop<'a> {
    Init(PropInit<'a>),
    Method(PropMethod<'a>),
    Ctor(PropCtor<'a>),
    Get(PropGet<'a>),
    Set(PropSet<'a>),
}

impl<'a> From<Prop<'a>> for crate::expr::Prop<'a> {
    fn from(other: Prop<'a>) -> Self {
        match other {
            Prop::Init(inner) => Self {
                computed: inner.key.brackets.is_some(),
                short_hand: inner.colon.is_none(),
                key: inner.key.into(),
                value: inner
                    .value
                    .map(From::from)
                    .unwrap_or(crate::expr::PropValue::None),
                kind: crate::PropKind::Init,
                method: false,
                is_static: false,
            },
            Prop::Method(inner) => Self {
                computed: inner.id.brackets.is_some(),
                key: inner.id.into(),
                value: crate::prelude::PropValue::Expr(crate::Expr::Func(crate::Func {
                    body: inner.body.into(),
                    generator: inner.star.is_some(),
                    id: None,
                    is_async: inner.keyword_async.is_some(),
                    params: inner.params.into_iter().map(|e| e.item.into()).collect(),
                })),
                kind: crate::PropKind::Method,
                method: true,
                short_hand: false,
                is_static: inner.keyword_static.is_some(),
            },
            Prop::Ctor(inner) => Self {
                computed: inner.keyword.brackets.is_some(),
                key: inner.keyword.into(),
                value: crate::prelude::PropValue::Expr(crate::Expr::Func(crate::Func {
                    body: inner.body.into(),
                    generator: false,
                    id: None,
                    is_async: false,
                    params: inner.params.into_iter().map(|e| e.item.into()).collect(),
                })),
                kind: crate::PropKind::Ctor,
                is_static: false,
                method: true,
                short_hand: false,
            },
            Prop::Get(inner) => Self {
                computed: inner.id.brackets.is_some(),
                key: inner.id.into(),
                value: crate::prelude::PropValue::Expr(crate::Expr::Func(crate::Func {
                    body: inner.body.into(),
                    generator: false,
                    id: None,
                    is_async: false,
                    params: Vec::new(),
                })),
                kind: crate::PropKind::Get,
                method: false,
                short_hand: false,
                is_static: inner.keyword_static.is_some(),
            },
            Prop::Set(inner) => Self {
                computed: inner.id.brackets.is_some(),
                key: inner.id.into(),
                value: crate::prelude::PropValue::Expr(crate::Expr::Func(crate::Func {
                    body: inner.body.into(),
                    generator: false,
                    id: None,
                    is_async: false,
                    params: vec![inner.arg.item.into()],
                })),
                kind: crate::PropKind::Set,
                method: false,
                short_hand: false,
                is_static: inner.keyword_static.is_some(),
            },
        }
    }
}

impl<'a> Node for Prop<'a> {
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

impl<'a> Prop<'a> {
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
pub struct PropInit<'a> {
    pub key: PropInitKey<'a>,
    pub colon: Option<Slice<'a>>,
    pub value: Option<PropValue<'a>>,
}

impl<'a> Node for PropInit<'a> {
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

impl<'a> PropInit<'a> {
    pub fn computed(&self) -> bool {
        self.key.brackets.is_some()
    }
    pub fn short_hand(&self) -> bool {
        self.value.is_none()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PropInitKey<'a> {
    pub value: PropKey<'a>,
    pub brackets: Option<(Slice<'a>, Slice<'a>)>,
}

impl<'a> From<PropInitKey<'a>> for crate::expr::PropKey<'a> {
    fn from(other: PropInitKey<'a>) -> Self {
        other.value.into()
    }
}

impl<'a> Node for PropInitKey<'a> {
    fn loc(&self) -> SourceLocation {
        if let Some((open, close)) = &self.brackets {
            SourceLocation {
                start: open.loc.start,
                end: close.loc.end,
            }
        } else {
            self.value.loc()
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PropMethod<'a> {
    pub keyword_static: Option<Slice<'a>>,
    pub keyword_async: Option<Slice<'a>>,
    pub id: PropInitKey<'a>,
    pub star: Option<Slice<'a>>,
    pub open_paren: Slice<'a>,
    pub params: Vec<ListEntry<'a, FuncArg<'a>>>,
    pub close_paren: Slice<'a>,
    pub body: FuncBody<'a>,
}

impl<'a> Node for PropMethod<'a> {
    fn loc(&self) -> SourceLocation {
        let start = if let Some(keyword) = &self.keyword_async {
            keyword.loc.start
        } else if let Some(star) = &self.star {
            star.loc.start
        } else {
            self.id.loc().start
        };
        SourceLocation {
            start,
            end: self.body.loc().end,
        }
    }
}

impl<'a> From<PropMethod<'a>> for crate::Func<'a> {
    fn from(other: PropMethod<'a>) -> Self {
        crate::Func {
            id: None,
            params: other.params.into_iter().map(|e| e.item.into()).collect(),
            body: other.body.into(),
            generator: other.star.is_some(),
            is_async: other.keyword_async.is_some(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PropCtor<'a> {
    pub keyword: PropInitKey<'a>,
    pub open_paren: Slice<'a>,
    pub params: Vec<ListEntry<'a, FuncArg<'a>>>,
    pub close_paren: Slice<'a>,
    pub body: FuncBody<'a>,
}

impl<'a> Node for PropCtor<'a> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.keyword.loc().start,
            end: self.body.loc().end,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PropGet<'a> {
    pub keyword_static: Option<Slice<'a>>,
    pub keyword_get: Slice<'a>,
    pub id: PropInitKey<'a>,
    pub open_paren: Slice<'a>,
    pub close_paren: Slice<'a>,
    pub body: FuncBody<'a>,
}

impl<'a> Node for PropGet<'a> {
    fn loc(&self) -> SourceLocation {
        if let Some(keyword_static) = &self.keyword_static {
            return SourceLocation {
                start: keyword_static.loc.start,
                end: self.body.loc().end,
            };
        }
        SourceLocation {
            start: self.keyword_get.loc.start,
            end: self.body.loc().end,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PropSet<'a> {
    pub keyword_static: Option<Slice<'a>>,
    pub keyword_set: Slice<'a>,
    pub id: PropInitKey<'a>,
    pub open_paren: Slice<'a>,
    pub arg: ListEntry<'a, FuncArg<'a>>,
    pub close_paren: Slice<'a>,
    pub body: FuncBody<'a>,
}

impl<'a> Node for PropSet<'a> {
    fn loc(&self) -> SourceLocation {
        if let Some(keyword_static) = &self.keyword_static {
            return SourceLocation {
                start: keyword_static.loc.start,
                end: self.body.loc().end,
            };
        }
        SourceLocation {
            start: self.keyword_set.loc.start,
            end: self.body.loc().end,
        }
    }
}

/// An object literal or class property identifier
#[derive(PartialEq, Debug, Clone)]
pub enum PropKey<'a> {
    Lit(Lit<'a>),
    Expr(Expr<'a>),
    Pat(Pat<'a>),
}

impl<'a> From<PropKey<'a>> for crate::expr::PropKey<'a> {
    fn from(other: PropKey<'a>) -> Self {
        match other {
            PropKey::Lit(inner) => Self::Lit(inner.into()),
            PropKey::Expr(inner) => Self::Expr(inner.into()),
            PropKey::Pat(inner) => Self::Pat(inner.into()),
        }
    }
}

impl<'a> Node for PropKey<'a> {
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
pub enum PropValue<'a> {
    Expr(Expr<'a>),
    Pat(Pat<'a>),
    Method(PropMethod<'a>),
}

impl<'a> From<PropValue<'a>> for crate::expr::PropValue<'a> {
    fn from(other: PropValue<'a>) -> Self {
        match other {
            PropValue::Expr(inner) => Self::Expr(inner.into()),
            PropValue::Pat(inner) => Self::Pat(inner.into()),
            PropValue::Method(inner) => Self::Expr(crate::expr::Expr::Func(inner.into())),
        }
    }
}

impl<'a> Node for PropValue<'a> {
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
pub struct UnaryExpr<'a> {
    pub operator: UnaryOp<'a>,
    pub argument: Box<Expr<'a>>,
}

impl<'a> UnaryExpr<'a> {
    pub fn prefix(&self) -> bool {
        self.operator.loc() < self.argument.loc()
    }
}

impl<'a> From<UnaryExpr<'a>> for crate::expr::UnaryExpr<'a> {
    fn from(other: UnaryExpr<'a>) -> Self {
        Self {
            prefix: other.prefix(),
            operator: other.operator.into(),
            argument: Box::new(From::from(*other.argument)),
        }
    }
}

impl<'a> Node for UnaryExpr<'a> {
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
pub struct UpdateExpr<'a> {
    pub operator: UpdateOp<'a>,
    pub argument: Box<Expr<'a>>,
}

impl<'a> UpdateExpr<'a> {
    pub fn prefix(&self) -> bool {
        self.operator.loc().start < self.argument.loc().start
    }
}

impl<'a> From<UpdateExpr<'a>> for crate::expr::UpdateExpr<'a> {
    fn from(other: UpdateExpr<'a>) -> Self {
        let ret = Self {
            prefix: other.prefix(),
            operator: other.operator.into(),
            argument: Box::new(From::from(*other.argument)),
        };
        ret
    }
}

impl<'a> Node for UpdateExpr<'a> {
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
pub struct BinaryExpr<'a> {
    pub operator: BinaryOp<'a>,
    pub left: Box<Expr<'a>>,
    pub right: Box<Expr<'a>>,
}

impl<'a> From<BinaryExpr<'a>> for crate::expr::BinaryExpr<'a> {
    fn from(other: BinaryExpr<'a>) -> Self {
        Self {
            operator: other.operator.into(),
            left: Box::new(From::from(*other.left)),
            right: Box::new(From::from(*other.right)),
        }
    }
}

impl<'a> Node for BinaryExpr<'a> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.left.loc().start,
            end: self.right.loc().end,
        }
    }
}

/// An assignment or update + assignment operation
#[derive(PartialEq, Debug, Clone)]
pub struct AssignExpr<'a> {
    pub operator: AssignOp<'a>,
    pub left: AssignLeft<'a>,
    pub right: Box<Expr<'a>>,
}

impl<'a> From<AssignExpr<'a>> for crate::expr::AssignExpr<'a> {
    fn from(other: AssignExpr<'a>) -> Self {
        Self {
            operator: other.operator.into(),
            left: other.left.into(),
            right: Box::new(From::from(*other.right)),
        }
    }
}

impl<'a> Node for AssignExpr<'a> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.left.loc().start,
            end: self.right.loc().end,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct AwaitExpr<'a> {
    pub keyword: Slice<'a>,
    pub expr: Expr<'a>,
}

impl<'a> Node for AwaitExpr<'a> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.keyword.loc.start,
            end: self.expr.loc().end,
        }
    }
}

/// The value being assigned to
#[derive(PartialEq, Debug, Clone)]
pub enum AssignLeft<'a> {
    Pat(Pat<'a>),
    Expr(Box<Expr<'a>>),
}

impl<'a> From<AssignLeft<'a>> for crate::expr::AssignLeft<'a> {
    fn from(other: AssignLeft<'a>) -> Self {
        match other {
            AssignLeft::Pat(inner) => Self::Pat(inner.into()),
            AssignLeft::Expr(inner) => Self::Expr(Box::new(From::from(*inner))),
        }
    }
}

impl<'a> Node for AssignLeft<'a> {
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
pub struct LogicalExpr<'a> {
    pub operator: LogicalOp<'a>,
    pub left: Box<Expr<'a>>,
    pub right: Box<Expr<'a>>,
}

impl<'a> From<LogicalExpr<'a>> for crate::expr::LogicalExpr<'a> {
    fn from(other: LogicalExpr<'a>) -> Self {
        Self {
            operator: other.operator.into(),
            left: Box::new(From::from(*other.left)),
            right: Box::new(From::from(*other.right)),
        }
    }
}

impl<'a> Node for LogicalExpr<'a> {
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
pub struct MemberExpr<'a> {
    pub object: Box<Expr<'a>>,
    pub property: Box<Expr<'a>>,
    pub indexer: MemberIndexer<'a>,
}

impl<'a> From<MemberExpr<'a>> for crate::expr::MemberExpr<'a> {
    fn from(other: MemberExpr<'a>) -> Self {
        let computed = other.computed();
        Self {
            object: Box::new(From::from(*other.object)),
            property: Box::new(From::from(*other.property)),
            computed,
        }
    }
}

impl<'a> MemberExpr<'a> {
    pub fn computed(&self) -> bool {
        matches!(self.indexer, MemberIndexer::Computed { .. })
    }
}

impl<'a> Node for MemberExpr<'a> {
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
pub enum MemberIndexer<'a> {
    Period(Slice<'a>),
    Computed {
        open_bracket: Slice<'a>,
        close_bracket: Slice<'a>,
    },
}

impl<'a> Node for MemberIndexer<'a> {
    fn loc(&self) -> SourceLocation {
        match self {
            MemberIndexer::Period(inner) => inner.loc,
            MemberIndexer::Computed {
                open_bracket,
                close_bracket,
            } => SourceLocation {
                start: open_bracket.loc.start,
                end: close_bracket.loc.end,
            },
        }
    }
}

/// A ternery expression
/// ```js
/// var a = true ? 'stuff' : 'things';
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct ConditionalExpr<'a> {
    pub test: Box<Expr<'a>>,
    pub question_mark: Slice<'a>,
    pub alternate: Box<Expr<'a>>,
    pub colon: Slice<'a>,
    pub consequent: Box<Expr<'a>>,
}

impl<'a> From<ConditionalExpr<'a>> for crate::expr::ConditionalExpr<'a> {
    fn from(other: ConditionalExpr<'a>) -> Self {
        Self {
            test: Box::new(From::from(*other.test)),
            alternate: Box::new(From::from(*other.alternate)),
            consequent: Box::new(From::from(*other.consequent)),
        }
    }
}

impl<'a> Node for ConditionalExpr<'a> {
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
pub struct CallExpr<'a> {
    pub callee: Box<Expr<'a>>,
    pub open_paren: Slice<'a>,
    pub arguments: Vec<ListEntry<'a, Expr<'a>>>,
    pub close_paren: Slice<'a>,
}

impl<'a> From<CallExpr<'a>> for crate::expr::CallExpr<'a> {
    fn from(other: CallExpr<'a>) -> Self {
        Self {
            callee: Box::new(From::from(*other.callee)),
            arguments: other.arguments.into_iter().map(|e| e.item.into()).collect(),
        }
    }
}

impl<'a> Node for CallExpr<'a> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.callee.loc().start,
            end: self.close_paren.loc.end,
        }
    }
}

/// Calling a constructor
/// ```js
/// new Uint8Array(32);
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct NewExpr<'a> {
    pub callee: Box<Expr<'a>>,
    pub open_paren: Option<Slice<'a>>,
    pub arguments: Vec<ListEntry<'a, Expr<'a>>>,
    pub close_paren: Option<Slice<'a>>,
}

impl<'a> From<NewExpr<'a>> for crate::expr::NewExpr<'a> {
    fn from(other: NewExpr<'a>) -> Self {
        Self {
            callee: Box::new(From::from(*other.callee)),
            arguments: other.arguments.into_iter().map(|e| e.item.into()).collect(),
        }
    }
}

impl<'a> Node for NewExpr<'a> {
    fn loc(&self) -> SourceLocation {
        let end = if let Some(close) = &self.close_paren {
            close.loc.end
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
pub type SequenceExpr<'a> = Vec<ListEntry<'a, Expr<'a>>>;

impl<'a> Node for SequenceExpr<'a> {
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
pub struct ArrowParamPlaceHolder<'a> {
    // async keyword
    pub keyword: Option<Slice<'a>>,
    pub open_paren: Option<Slice<'a>>,
    pub args: Vec<ListEntry<'a, FuncArg<'a>>>,
    pub close_paren: Option<Slice<'a>>,
}

impl<'a> Node for ArrowParamPlaceHolder<'a> {
    fn loc(&self) -> SourceLocation {
        let start = if let Some(keyword) = &self.keyword {
            keyword.loc.start
        } else if let Some(open) = &self.open_paren {
            open.loc.start
        } else if let Some(arg) = self.args.first() {
            arg.loc().start
        } else {
            Position { line: 0, column: 0 }
        };
        let end = if let Some(close) = &self.close_paren {
            close.loc.end
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
pub struct ArrowFuncExpr<'a> {
    pub keyword: Option<Slice<'a>>,
    pub star: Option<Slice<'a>>,
    pub open_paren: Option<Slice<'a>>,
    pub params: Vec<ListEntry<'a, FuncArg<'a>>>,
    pub close_paren: Option<Slice<'a>>,
    pub arrow: Slice<'a>,
    pub body: ArrowFuncBody<'a>,
}

impl<'a> From<ArrowFuncExpr<'a>> for crate::expr::ArrowFuncExpr<'a> {
    fn from(other: ArrowFuncExpr<'a>) -> Self {
        let expression = matches!(&other.body, ArrowFuncBody::Expr(_));
        Self {
            id: None,
            params: other.params.into_iter().map(|e| e.item.into()).collect(),
            body: other.body.into(),
            expression,
            generator: other.star.is_some(),
            is_async: other.keyword.is_some(),
        }
    }
}

impl<'a> Node for ArrowFuncExpr<'a> {
    fn loc(&self) -> SourceLocation {
        let start = if let Some(slice) = &self.open_paren {
            slice.loc.start
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
pub enum ArrowFuncBody<'a> {
    FuncBody(FuncBody<'a>),
    Expr(Box<Expr<'a>>),
}

impl<'a> From<ArrowFuncBody<'a>> for crate::expr::ArrowFuncBody<'a> {
    fn from(other: ArrowFuncBody<'a>) -> Self {
        match other {
            ArrowFuncBody::FuncBody(inner) => Self::FuncBody(inner.into()),
            ArrowFuncBody::Expr(inner) => Self::Expr(Box::new(From::from(*inner))),
        }
    }
}

impl<'a> Node for ArrowFuncBody<'a> {
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
pub struct YieldExpr<'a> {
    pub keyword: Slice<'a>,
    pub argument: Option<Box<Expr<'a>>>,
    pub star: Option<Slice<'a>>,
}

impl<'a> From<YieldExpr<'a>> for crate::expr::YieldExpr<'a> {
    fn from(other: YieldExpr<'a>) -> Self {
        Self {
            argument: other.argument.map(|e| Box::new(From::from(*e))),
            delegate: other.star.is_some(),
        }
    }
}

impl<'a> Node for YieldExpr<'a> {
    fn loc(&self) -> SourceLocation {
        let end = if let Some(arg) = &self.argument {
            arg.loc().end
        } else {
            self.keyword.loc.end
        };
        SourceLocation {
            start: self.keyword.loc.start,
            end,
        }
    }
}

/// A Template literal preceded by a function identifier
/// see [MDN](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Template_literals#Tagged_templates) for more details
#[derive(PartialEq, Debug, Clone)]
pub struct TaggedTemplateExpr<'a> {
    pub tag: Box<Expr<'a>>,
    pub quasi: TemplateLit<'a>,
}

impl<'a> From<TaggedTemplateExpr<'a>> for crate::expr::TaggedTemplateExpr<'a> {
    fn from(other: TaggedTemplateExpr<'a>) -> Self {
        Self {
            tag: Box::new(From::from(*other.tag)),
            quasi: other.quasi.into(),
        }
    }
}

impl<'a> Node for TaggedTemplateExpr<'a> {
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
pub struct TemplateLit<'a> {
    pub quasis: Vec<TemplateElement<'a>>,
    pub expressions: Vec<Expr<'a>>,
}

impl<'a> From<TemplateLit<'a>> for crate::expr::TemplateLit<'a> {
    fn from(other: TemplateLit<'a>) -> Self {
        Self {
            quasis: other.quasis.into_iter().map(From::from).collect(),
            expressions: other.expressions.into_iter().map(From::from).collect(),
        }
    }
}

impl<'a> Node for TemplateLit<'a> {
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
pub struct TemplateElement<'a> {
    /// Raw quoted element
    pub raw: Slice<'a>,
    pub cooked: Slice<'a>,
}

impl<'a> From<TemplateElement<'a>> for crate::expr::TemplateElement<'a> {
    fn from(other: TemplateElement<'a>) -> Self {
        let tail = other.is_tail();
        Self {
            tail,
            cooked: other.cooked.source,
            raw: other.raw.source,
        }
    }
}

impl<'a> Node for TemplateElement<'a> {
    fn loc(&self) -> SourceLocation {
        self.raw.loc
    }
}

impl<'a> TemplateElement<'a> {
    pub fn is_tail(&self) -> bool {
        self.raw.source.starts_with(|c| c == '`' || c == '}') && self.raw.source.ends_with('`')
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
pub struct MetaProp<'a> {
    pub meta: Ident<'a>,
    pub dot: Slice<'a>,
    pub property: Ident<'a>,
}

impl<'a> From<MetaProp<'a>> for crate::expr::MetaProp<'a> {
    fn from(other: MetaProp<'a>) -> Self {
        Self {
            meta: other.meta.into(),
            property: other.property.into(),
        }
    }
}

impl<'a> Node for MetaProp<'a> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.meta.loc().start,
            end: self.property.loc().end,
        }
    }
}

/// A literal value
#[derive(Debug, Clone, PartialEq)]
pub enum Lit<'a> {
    /// `null`
    Null(Slice<'a>),
    /// `"string"`
    /// `'string'`
    String(StringLit<'a>),
    /// `0`
    /// `0.0`
    /// `.0`
    /// `0.0e1`
    /// `.0E1`
    /// `0xf`
    /// `0o7`
    /// `0b1`
    Number(Slice<'a>),
    /// `true`
    /// `false`
    Boolean(Slice<'a>),
    /// `/.+/g`
    RegEx(RegEx<'a>),
    /// ```js
    /// `I have ${0} apples`
    /// ```
    Template(TemplateLit<'a>),
}

impl<'a> From<Lit<'a>> for crate::expr::Lit<'a> {
    fn from(other: Lit<'a>) -> Self {
        match other {
            Lit::Null(_inner) => Self::Null,
            Lit::String(inner) => Self::String(inner.into()),
            Lit::Number(inner) => Self::Number(inner.source),
            Lit::Boolean(inner) => Self::Boolean(inner.source == "true"),
            Lit::RegEx(inner) => Self::RegEx(inner.into()),
            Lit::Template(inner) => Self::Template(inner.into()),
        }
    }
}

impl<'a> Node for Lit<'a> {
    fn loc(&self) -> SourceLocation {
        match self {
            Lit::Null(inner) => inner.loc,
            Lit::String(inner) => inner.loc(),
            Lit::Number(inner) => inner.loc,
            Lit::Boolean(inner) => inner.loc,
            Lit::RegEx(inner) => inner.loc(),
            Lit::Template(inner) => inner.loc(),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct StringLit<'a> {
    pub open_quote: Slice<'a>,
    pub content: Slice<'a>,
    pub close_quote: Slice<'a>,
}

impl<'a> From<StringLit<'a>> for crate::expr::StringLit<'a> {
    fn from(other: StringLit<'a>) -> Self {
        if other.open_quote.source == "\"" {
            Self::Double(other.content.source)
        } else {
            Self::Single(other.content.source)
        }
    }
}

impl<'a> Node for StringLit<'a> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.open_quote.loc.start,
            end: self.close_quote.loc.end,
        }
    }
}

impl<'a> StringLit<'a> {
    pub fn inner_matches(&self, o: &str) -> bool {
        self.content.source == o
    }
}
/// A regular expression literal
#[derive(PartialEq, Debug, Clone)]
pub struct RegEx<'a> {
    pub open_slash: Slice<'a>,
    pub pattern: Slice<'a>,
    pub close_slash: Slice<'a>,
    pub flags: Option<Slice<'a>>,
}

impl<'a> From<RegEx<'a>> for crate::expr::RegEx<'a> {
    fn from(other: RegEx<'a>) -> Self {
        Self {
            pattern: other.pattern.source,
            flags: other
                .flags
                .map(|f| f.source)
                .unwrap_or_else(|| Cow::Borrowed("")),
        }
    }
}

impl<'a> Node for RegEx<'a> {
    fn loc(&self) -> SourceLocation {
        let end = if let Some(flags) = &self.flags {
            flags.loc.end
        } else {
            self.close_slash.loc.end
        };
        SourceLocation {
            start: self.open_slash.loc.start,
            end,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WrappedExpr<'a> {
    pub open_paren: Slice<'a>,
    pub expr: Expr<'a>,
    pub close_paren: Slice<'a>,
}

impl<'a> Node for WrappedExpr<'a> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.open_paren.loc.start,
            end: self.close_paren.loc.end,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SequenceExprEntry<'a> {
    pub expr: Expr<'a>,
    pub comma: Option<Slice<'a>>,
}

impl<'a> SequenceExprEntry<'a> {
    pub fn no_comma(expr: Expr<'a>) -> Self {
        Self { expr, comma: None }
    }
}

impl<'a> From<SequenceExprEntry<'a>> for FuncArgEntry<'a> {
    fn from(other: SequenceExprEntry<'a>) -> Self {
        Self {
            value: FuncArg::Expr(other.expr),
            comma: other.comma,
        }
    }
}

impl<'a> Node for SequenceExprEntry<'a> {
    fn loc(&self) -> SourceLocation {
        if let Some(comma) = &self.comma {
            return SourceLocation {
                start: self.expr.loc().start,
                end: comma.loc.end,
            };
        }
        self.expr.loc()
    }
}

impl<'a> From<SequenceExprEntry<'a>> for crate::expr::Expr<'a> {
    fn from(other: SequenceExprEntry<'a>) -> Self {
        other.expr.into()
    }
}
