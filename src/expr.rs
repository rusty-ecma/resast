use crate::pat::Pat;
use crate::{AssignOp, BinaryOp, LogicalOp, PropKind, UnaryOp, UpdateOp, IntoAllocated};
use crate::{Class, Func, FuncArg, FuncBody, Ident};
/// A slightly more granular program part that a statement
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize)
)]
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
    ArrowParamPlaceHolder(Vec<FuncArg<T>>, bool),
    /// Assignment or update assignment
    /// ```js
    /// a = 0
    /// b += 1
    /// ```
    Assign(AssignExpr<T>),
    /// The `await` keyword followed by another `Expr`
    Await(Box<Expr<T>>),
    /// An operation that has two arguments
    Binary(BinaryExpr<T>),
    /// A class expression see `Class`
    Class(Class<T>),
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
    Spread(Box<Expr<T>>),
    /// `super`
    Super,
    /// A template literal preceded by a tag function identifier
    TaggedTemplate(TaggedTemplateExpr<T>),
    /// `this`
    This,
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
    /// yield a value from inside of a generator function
    Yield(YieldExpr<T>),
}

impl<T> IntoAllocated for Expr<T> where T: ToString {
    type Allocated = Expr<String>;

    fn into_allocated(self) -> Self::Allocated {
        match self {
            Expr::Array(inner) => Expr::Array(inner.into_iter().map(|o| o.map(|e| e.into_allocated())).collect()),
            Expr::ArrowFunc(inner) => Expr::ArrowFunc(inner.into_allocated()),
            Expr::ArrowParamPlaceHolder(args, is_async) => Expr::ArrowParamPlaceHolder(args.into_iter().map(|a| a.into_allocated()).collect(), is_async),
            Expr::Assign(inner) => Expr::Assign(inner.into_allocated()),
            Expr::Await(inner) => Expr::Await(inner.into_allocated()),
            Expr::Binary(inner) => Expr::Binary(inner.into_allocated()),
            Expr::Class(inner) => Expr::Class(inner.into_allocated()),
            Expr::Call(inner) => Expr::Call(inner.into_allocated()),
            Expr::Conditional(inner) => Expr::Conditional(inner.into_allocated()),
            Expr::Func(inner) => Expr::Func(inner.into_allocated()),
            Expr::Ident(inner) => Expr::Ident(inner.into_allocated()),
            Expr::Lit(inner) => Expr::Lit(inner.into_allocated()),
            Expr::Logical(inner) => Expr::Logical(inner.into_allocated()),
            Expr::Member(inner) => Expr::Member(inner.into_allocated()),
            Expr::MetaProp(inner) => Expr::MetaProp(inner.into_allocated()),
            Expr::New(inner) => Expr::New(inner.into_allocated()),
            Expr::Obj(inner) => Expr::Obj(inner.into_iter().map(|p| p.into_allocated()).collect()),
            Expr::Sequence(inner) => Expr::Sequence(inner.into_iter().map(|e| e.into_allocated()).collect()),
            Expr::Spread(inner) => Expr::Spread(inner.into_allocated()),
            Expr::Super => Expr::Super,
            Expr::TaggedTemplate(inner) => Expr::TaggedTemplate(inner.into_allocated()),
            Expr::This => Expr::This,
            Expr::Unary(inner) => Expr::Unary(inner.into_allocated()),
            Expr::Update(inner) => Expr::Update(inner.into_allocated()),
            Expr::Yield(inner) => Expr::Yield(inner.into_allocated()),
        }
    }
}

impl<T> Expr<T> {
    pub fn ident_from(inner: T) -> Self {
        Self::Ident(Ident { name: inner })
    }
}

/// `[a, b, c]`
pub type ArrayExpr<T> = Vec<Option<Expr<T>>>;
/// `{a: 'b', c, ...d}`
pub type ObjExpr<T> = Vec<ObjProp<T>>;
/// A single part of an object literal
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub enum ObjProp<T> {
    Prop(Prop<T>),
    Spread(Expr<T>),
}

impl<T> IntoAllocated for ObjProp<T> where T: ToString {
    type Allocated = ObjProp<String>;

    fn into_allocated(self) -> Self::Allocated {
        match self {
            ObjProp::Prop(inner) => ObjProp::Prop(inner.into_allocated()),
            ObjProp::Spread(inner) => ObjProp::Spread(inner.into_allocated()),
        }
    }
}

/// A single part of an object literal or class
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize)
)]
pub struct Prop<T> {
    pub key: PropKey<T>,
    pub value: PropValue<T>,
    pub kind: PropKind,
    pub method: bool,
    pub computed: bool,
    pub short_hand: bool,
    pub is_static: bool,
}

impl<T> IntoAllocated for Prop<T> where T: ToString {
    type Allocated = Prop<String>;

    fn into_allocated(self) -> Self::Allocated {
        Prop {
            key: self.key.into_allocated(),
            value: self.value.into_allocated(),
            kind: self.kind,
            method: self.method,
            computed: self.computed,
            short_hand: self.short_hand,
            is_static: self.is_static,
        }
    }
}

/// An object literal or class property identifier
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub enum PropKey<T> {
    Lit(Lit<T>),
    Expr(Expr<T>),
    Pat(Pat<T>),
}

impl<T> IntoAllocated for PropKey<T> where T: ToString {
    type Allocated = PropKey<String>;

    fn into_allocated(self) -> Self::Allocated {
        match self {
            PropKey::Lit(inner) => PropKey::Lit(inner.into_allocated()),
            PropKey::Expr(inner) => PropKey::Expr(inner.into_allocated()),
            PropKey::Pat(inner) => PropKey::Pat(inner.into_allocated()),
        }
    }
}

/// The value of an object literal or class property
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub enum PropValue<T> {
    Expr(Expr<T>),
    Pat(Pat<T>),
    None,
}

impl<T> IntoAllocated for PropValue<T> where T: ToString {
    type Allocated = PropValue<String>;

    fn into_allocated(self) -> Self::Allocated {
        match self {
            PropValue::Expr(inner) => PropValue::Expr(inner.into_allocated()),
            PropValue::Pat(inner) => PropValue::Pat(inner.into_allocated()),
            PropValue::None => PropValue::None,
        }
    }
}

/// An operation that takes one argument
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub struct UnaryExpr<T> {
    pub operator: UnaryOp,
    pub prefix: bool,
    pub argument: Box<Expr<T>>,
}

impl<T> IntoAllocated for UnaryExpr<T> where T: ToString {
    type Allocated = UnaryExpr<String>;

    fn into_allocated(self) -> Self::Allocated {
        UnaryExpr {
            operator: self.operator,
            prefix: self.prefix,
            argument: self.argument.into_allocated(),
        }
    }
}

/// Increment or decrementing a value
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub struct UpdateExpr<T> {
    pub operator: UpdateOp,
    pub argument: Box<Expr<T>>,
    pub prefix: bool,
}

impl<T> IntoAllocated for UpdateExpr<T> where T: ToString {
    type Allocated = UpdateExpr<String>;

    fn into_allocated(self) -> Self::Allocated {
        UpdateExpr {
            operator: self.operator,
            argument: self.argument.into_allocated(),
            prefix: self.prefix,
        }
    }
}

/// An operation that requires 2 arguments
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub struct BinaryExpr<T> {
    pub operator: BinaryOp,
    pub left: Box<Expr<T>>,
    pub right: Box<Expr<T>>,
}

impl<T> IntoAllocated for BinaryExpr<T> where T: ToString {
    type Allocated = BinaryExpr<String>;

    fn into_allocated(self) -> Self::Allocated {
        BinaryExpr {
            operator: self.operator,
            left: self.left.into_allocated(),
            right: self.right.into_allocated(),
        }
    }
}

/// An assignment or update + assignment operation
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub struct AssignExpr<T> {
    pub operator: AssignOp,
    pub left: AssignLeft<T>,
    pub right: Box<Expr<T>>,
}

impl<T> IntoAllocated for AssignExpr<T> where T: ToString {
    type Allocated = AssignExpr<String>;

    fn into_allocated(self) -> Self::Allocated {
        AssignExpr {
            operator: self.operator,
            left: self.left.into_allocated(),
            right: self.right.into_allocated(),
        }
    }
}

/// The value being assigned to
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub enum AssignLeft<T> {
    Pat(Pat<T>),
    Expr(Box<Expr<T>>),
}

impl<T> IntoAllocated for AssignLeft<T> where T: ToString {
    type Allocated = AssignLeft<String>;

    fn into_allocated(self) -> Self::Allocated {
        match self {
            AssignLeft::Pat(inner) => AssignLeft::Pat(inner.into_allocated()),
            AssignLeft::Expr(inner) => AssignLeft::Expr(inner.into_allocated()),
        }
    }
}

/// A specialized `BinaryExpr` for logical evaluation
/// ```js
/// true && true
/// false || true
/// ```
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub struct LogicalExpr<T> {
    pub operator: LogicalOp,
    pub left: Box<Expr<T>>,
    pub right: Box<Expr<T>>,
}

impl<T> IntoAllocated for LogicalExpr<T> where T: ToString {
    type Allocated = LogicalExpr<String>;

    fn into_allocated(self) -> Self::Allocated {
        LogicalExpr {
            operator: self.operator,
            left: self.left.into_allocated(),
            right: self.right.into_allocated(),
        }
    }
}

/// Accessing the member of a value
/// ```js
/// b['thing'];
/// c.stuff;
/// ```
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub struct MemberExpr<T> {
    pub object: Box<Expr<T>>,
    pub property: Box<Expr<T>>,
    pub computed: bool,
}

impl<T> IntoAllocated for MemberExpr<T> where T: ToString {
    type Allocated = MemberExpr<String>;

    fn into_allocated(self) -> Self::Allocated {
        MemberExpr {
            object: self.object.into_allocated(),
            property: self.property.into_allocated(),
            computed: self.computed,
        }
    }
}

/// A ternery expression
/// ```js
/// var a = true ? 'stuff' : 'things';
/// ```
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub struct ConditionalExpr<T> {
    pub test: Box<Expr<T>>,
    pub alternate: Box<Expr<T>>,
    pub consequent: Box<Expr<T>>,
}

impl<T> IntoAllocated for ConditionalExpr<T> where T: ToString {
    type Allocated = ConditionalExpr<String>;

    fn into_allocated(self) -> Self::Allocated {
        ConditionalExpr {
            test: self.test.into_allocated(),
            alternate: self.alternate.into_allocated(),
            consequent: self.consequent.into_allocated(),
        }
    }
}

/// Calling a function or method
/// ```js
/// Math.random()
/// ```
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub struct CallExpr<T> {
    pub callee: Box<Expr<T>>,
    pub arguments: Vec<Expr<T>>,
}

impl<T> IntoAllocated for CallExpr<T> where T: ToString {
    type Allocated = CallExpr<String>;

    fn into_allocated(self) -> Self::Allocated {
        CallExpr {
            callee: self.callee.into_allocated(),
            arguments: self.arguments.into_iter().map(IntoAllocated::into_allocated).collect(),
        }
    }
}

/// Calling a constructor
/// ```js
/// new Uint8Array(32);
/// ```
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub struct NewExpr<T> {
    pub callee: Box<Expr<T>>,
    pub arguments: Vec<Expr<T>>,
}

impl<T> IntoAllocated for NewExpr<T> where T: ToString {
    type Allocated = NewExpr<String>;

    fn into_allocated(self) -> Self::Allocated {
        NewExpr {
            callee: self.callee.into_allocated(),
            arguments: self.arguments.into_iter().map(|a| a.into_allocated()).collect(),
        }
    }
}

/// A collection of `Exprs` separated by commas
pub type SequenceExpr<T> = Vec<Expr<T>>;

/// An arrow function
/// ```js
/// let x = () => y;
/// let q = x => {
///     return x + 1;
/// }
/// ```
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub struct ArrowFuncExpr<T> {
    pub id: Option<Ident<T>>,
    pub params: Vec<FuncArg<T>>,
    pub body: ArrowFuncBody<T>,
    pub expression: bool,
    pub generator: bool,
    pub is_async: bool,
}

impl<T> IntoAllocated for ArrowFuncExpr<T> where T: ToString {
    type Allocated = ArrowFuncExpr<String>;

    fn into_allocated(self) -> Self::Allocated {
        ArrowFuncExpr {
            id: self.id.map(|i| i.into_allocated()),
            params: self.params.into_iter().map(|p| p.into_allocated()).collect(),
            body: self.body.into_allocated(),
            expression: self.expression,
            generator: self.generator,
            is_async: self.is_async,
        }
    }
}

/// The body portion of an arrow function can be either an expression or a block of statements
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub enum ArrowFuncBody<T> {
    FuncBody(FuncBody<T>),
    Expr(Box<Expr<T>>),
}

impl<T> IntoAllocated for ArrowFuncBody<T> where T: ToString {
    type Allocated = ArrowFuncBody<String>;

    fn into_allocated(self) -> Self::Allocated {
        match self {
            ArrowFuncBody::FuncBody(inner) => ArrowFuncBody::FuncBody(inner.into_allocated()),
            ArrowFuncBody::Expr(inner) => ArrowFuncBody::Expr(inner.into_allocated()),
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
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub struct YieldExpr<T> {
    pub argument: Option<Box<Expr<T>>>,
    pub delegate: bool,
}

impl<T> IntoAllocated for YieldExpr<T> where T: ToString {
    type Allocated = YieldExpr<String>;

    fn into_allocated(self) -> Self::Allocated {
        YieldExpr {
            delegate: self.delegate,
            argument: self.argument.map(IntoAllocated::into_allocated),
        }
    }
}
/// A Template literal preceded by a function identifier
/// see [MDN](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Template_literals#Tagged_templates) for more details
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub struct TaggedTemplateExpr<T> {
    pub tag: Box<Expr<T>>,
    pub quasi: TemplateLit<T>,
}

impl<T> IntoAllocated for TaggedTemplateExpr<T> where T: ToString {
    type Allocated = TaggedTemplateExpr<String>;

    fn into_allocated(self) -> Self::Allocated {
        TaggedTemplateExpr {
            tag: self.tag.into_allocated(),
            quasi: self.quasi.into_allocated(),
        }
    }
}

/// A template string literal
/// ```js
/// `I own ${0} birds`;
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize)
)]
pub struct TemplateLit<T> {
    pub quasis: Vec<TemplateElement<T>>,
    pub expressions: Vec<Expr<T>>,
}

impl<T> IntoAllocated for TemplateLit<T> where T: ToString {
    type Allocated = TemplateLit<String>;

    fn into_allocated(self) -> Self::Allocated {
        TemplateLit {
            quasis: self.quasis.into_iter().map(|e| e.into_allocated()).collect(),
            expressions: self.expressions.into_iter().map(|e| e.into_allocated()).collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize)
)]
pub enum QuasiQuote {
    /// `
    BackTick,
    /// ${
    OpenBrace,
    /// }
    CloseBrace,
}

/// The text part of a `TemplateLiteral`
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize)
)]
pub struct TemplateElement<T> {
    pub open_quote: QuasiQuote,
    /// The non-quoted version
    pub content: T,
    pub close_quote: QuasiQuote,
}

impl<T> IntoAllocated for TemplateElement<T> where T: ToString {
    type Allocated = TemplateElement<String>;

    fn into_allocated(self) -> Self::Allocated {
        TemplateElement {
            open_quote: self.open_quote,
            content: self.content.to_string(),
            close_quote: self.close_quote,
        }
    }
}

impl<T> TemplateElement<T> {
    pub fn is_tail(&self) -> bool {
        matches!(
            self.open_quote,
            QuasiQuote::BackTick | QuasiQuote::CloseBrace
        ) && matches!(self.close_quote, QuasiQuote::BackTick)
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
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub struct MetaProp<T> {
    pub meta: Ident<T>,
    pub property: Ident<T>,
}

impl<T> IntoAllocated for MetaProp<T> where T: ToString {
    type Allocated = MetaProp<String>;

    fn into_allocated(self) -> Self::Allocated {
        MetaProp {
            meta: self.meta.into_allocated(),
            property: self.property.into_allocated(),
        }
    }
}

/// A literal value
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize)
)]
pub enum Lit<T> {
    /// `null`
    Null,
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
    Number(T),
    /// `true`
    /// `false`
    Boolean(bool),
    /// `/.+/g`
    RegEx(RegEx<T>),
    /// ```js
    /// `I have ${0} apples`
    /// ```
    Template(TemplateLit<T>),
}

impl<T> IntoAllocated for Lit<T> where T: ToString {
    type Allocated = Lit<String>;

    fn into_allocated(self) -> Self::Allocated {
        match self {
            Lit::Null => Lit::Null,
            Lit::String(inner) => Lit::String(inner.into_allocated()),
            Lit::Number(inner) => Lit::Number(inner.to_string()),
            Lit::Boolean(inner) => Lit::Boolean(inner),
            Lit::RegEx(inner) => Lit::RegEx(inner.into_allocated()),
            Lit::Template(inner) => Lit::Template(inner.into_allocated()),
        }
    }
}

impl<T> Lit<T> {
    pub fn number_from(s: T) -> Self {
        Lit::Number(s)
    }
    pub fn single_string_from(s: T) -> Self {
        Lit::String(StringLit::single_from(s))
    }
    pub fn double_string_from(s: T) -> Self {
        Lit::String(StringLit::double_from(s))
    }
}

#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub enum StringLit<T> {
    Double(T),
    Single(T),
}

impl<T> IntoAllocated for StringLit<T> where T: ToString {
    type Allocated = StringLit<String>;

    fn into_allocated(self) -> Self::Allocated {
        match self {
            StringLit::Double(inner) => StringLit::Double(inner.to_string()),
            StringLit::Single(inner) => StringLit::Single(inner.to_string()),
        }
    }
}

impl<T> StringLit<T> {
    pub fn double_from(s: T) -> StringLit<T> {
        StringLit::Double(s)
    }
    pub fn single_from(s: T) -> StringLit<T> {
        StringLit::Single(s)
    }
}
impl<T> StringLit<T>
where
    T: Clone,
{
    pub fn clone_inner(&self) -> T {
        match self {
            StringLit::Single(ref s) => s.clone(),
            StringLit::Double(ref s) => s.clone(),
        }
    }
}

impl<T> StringLit<T>
where
    T: AsRef<str>,
{
    pub fn inner_matches(&self, o: &str) -> bool {
        match self {
            StringLit::Single(ref s) => o.eq(s.as_ref()),
            StringLit::Double(ref d) => o.eq(d.as_ref()),
        }
    }
}
/// A regular expression literal
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
#[cfg_attr(all(feature = "serialization"), serde(rename_all = "camelCase"))]
pub struct RegEx<T> {
    pub pattern: T,
    pub flags: Option<T>,
}

impl<T> IntoAllocated for RegEx<T> where T: ToString {
    type Allocated = RegEx<String>;

    fn into_allocated(self) -> Self::Allocated {
        RegEx {
            pattern: self.pattern.to_string(),
            flags: self.flags.map(|f| f.to_string())
        }
    }
}

impl<T> RegEx<T> {
    pub fn from(p: T, f: Option<T>) -> Self {
        RegEx {
            pattern: p,
            flags: f,
        }
    }
}
