use std::borrow::Cow;
use crate::{
    AssignOp, BinaryOp, LogicalOp, PropKind, UnaryOp,
    UpdateOp,
};
use crate::pat::Pat;
use crate::{Class, Func, FuncArg, FuncBody, Ident};
/// A slightly more granular program part that a statement
#[derive(PartialEq, Debug, Clone, Deserialize)]
#[serde(untagged)]
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
    ArrowParamPlaceHolder(Vec<FuncArg<'a>>, bool),
    /// Assignment or update assignment
    /// ```js
    /// a = 0
    /// b += 1
    /// ```
    Assign(AssignExpr<'a>),
    /// The `await` keyword followed by another `Expr`
    Await(Box<Expr<'a>>),
    /// An operation that has two arguments
    Binary(BinaryExpr<'a>),
    /// A class expression see `Class`
    Class(Class<'a>),
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
    Spread(Box<Expr<'a>>),
    /// `super`
    Super,
    /// A template literal preceded by a tag function identifier
    TaggedTemplate(TaggedTemplateExpr<'a>),
    /// `this`
    This,
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
    /// yield a value from inside of a generator function
    Yield(YieldExpr<'a>),
}

impl<'a> Expr<'a> {
    pub fn ident_from(s: &'a str) -> Self {
        Expr::Ident(
            Ident::from(s)
        )
    }
}

/// `[a, b, c]`
pub type ArrayExpr<'a> = Vec<Option<Expr<'a>>>;
/// `{a: 'b', c, ...d}`
pub type ObjExpr<'a> = Vec<ObjProp<'a>>;
/// A single part of an object literal
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ObjProp<'a> {
    Prop(Prop<'a>),
    Spread(Expr<'a>),
}

/// A single part of an object literal or class
#[derive(PartialEq, Debug, Clone, Deserialize)]
pub struct Prop<'a> {
    pub key: PropKey<'a>,
    pub value: PropValue<'a>,
    pub kind: PropKind,
    pub method: bool,
    pub computed: bool,
    pub short_hand: bool,
    pub is_static: bool,
}

/// An object literal or class property identifier
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PropKey<'a> {
    Lit(Lit<'a>),
    Expr(Expr<'a>),
    Pat(Pat<'a>),
}

/// The value of an object literal or class property
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PropValue<'a> {
    Expr(Expr<'a>),
    Pat(Pat<'a>),
    None,
}

/// An operation that takes one argument
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct UnaryExpr<'a> {
    pub operator: UnaryOp,
    pub prefix: bool,
    pub argument: Box<Expr<'a>>,
}

/// Increment or decrementing a value
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct UpdateExpr<'a> {
    pub operator: UpdateOp,
    pub argument: Box<Expr<'a>>,
    pub prefix: bool,
}

/// An operation that requires 2 arguments
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct BinaryExpr<'a> {
    pub operator: BinaryOp,
    pub left: Box<Expr<'a>>,
    pub right: Box<Expr<'a>>,
}

/// An assignment or update + assignment operation
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct AssignExpr<'a> {
    pub operator: AssignOp,
    pub left: AssignLeft<'a>,
    pub right: Box<Expr<'a>>,
}

/// The value being assigned to
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AssignLeft<'a> {
    Pat(Pat<'a>),
    Expr(Box<Expr<'a>>),
}

/// A specialized `BinaryExpr` for logical evaluation
/// ```js
/// true && true
/// false || true
/// ```
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct LogicalExpr<'a> {
    pub operator: LogicalOp,
    pub left: Box<Expr<'a>>,
    pub right: Box<Expr<'a>>,
}

/// Accessing the member of a value
/// ```js
/// b['thing'];
/// c.stuff;
/// ```
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct MemberExpr<'a> {
    pub object: Box<Expr<'a>>,
    pub property: Box<Expr<'a>>,
    pub computed: bool,
}

/// A ternery expression
/// ```js
/// var a = true ? 'stuff' : 'things';
/// ```
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct ConditionalExpr<'a> {
    pub test: Box<Expr<'a>>,
    pub alternate: Box<Expr<'a>>,
    pub consequent: Box<Expr<'a>>,
}

/// Calling a function or method
/// ```js
/// Math.random()
/// ```
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct CallExpr<'a> {
    pub callee: Box<Expr<'a>>,
    pub arguments: Vec<Expr<'a>>,
}

/// Calling a constructor
/// ```js
/// new Uint8Array(32);
/// ```
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct NewExpr<'a> {
    pub callee: Box<Expr<'a>>,
    pub arguments: Vec<Expr<'a>>,
}

/// A collection of `Exprs` separated by commas
pub type SequenceExpr<'a> = Vec<Expr<'a>>;

/// An arrow function
/// ```js
/// let x = () => y;
/// let q = x => {
///     return x + 1;
/// }
/// ```
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct ArrowFuncExpr<'a> {
    pub id: Option<Ident<'a>>,
    pub params: Vec<FuncArg<'a>>,
    pub body: ArrowFuncBody<'a>,
    pub expression: bool,
    pub generator: bool,
    pub is_async: bool,
}

/// The body portion of an arrow function can be either an expression or a block of statements
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum ArrowFuncBody<'a> {
    FuncBody(FuncBody<'a>),
    Expr(Box<Expr<'a>>),
}

/// yield a value from inside of a generator function
/// ```js
/// function *gen() {
///     while ((new Date() / 1000) < Number.MAX_VALUE) {
///         yield new Date() / 1000;
///     }
/// }
/// ```
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct YieldExpr<'a> {
    pub argument: Option<Box<Expr<'a>>>,
    pub delegate: bool,
}

/// A Template literal preceded by a function identifier
/// see [MDN](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Template_literals#Tagged_templates) for more details
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct TaggedTemplateExpr<'a> {
    pub tag: Box<Expr<'a>>,
    pub quasi: TemplateLit<'a>,
}

/// A template string literal
/// ```js
/// `I own ${0} birds`;
/// ```
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct TemplateLit<'a> {
    pub quasis: Vec<TemplateElement<'a>>,
    pub expressions: Vec<Expr<'a>>,
}

/// The text part of a `TemplateLiteral`
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct TemplateElement<'a> {
    pub tail: bool,
    /// The non-quoted version
    pub cooked: Cow<'a, str>,
    /// The quoted version
    pub raw: Cow<'a, str>,
}

impl<'a> TemplateElement<'a> {
    pub fn from(tail: bool, cooked: &'a str, raw: &'a str) -> TemplateElement<'a> {
        Self {
            tail,
            cooked: Cow::Borrowed(cooked),
            raw: Cow::Borrowed(raw),
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
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct MetaProp<'a> {
    pub meta: Ident<'a>,
    pub property: Ident<'a>,
}

/// A literal value
#[derive(PartialEq, Debug, Clone, Deserialize)]
pub enum Lit<'a> {
    /// `null`
    Null,
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
    Number(Cow<'a, str>),
    /// `true`
    /// `false`
    Boolean(bool),
    /// `/.+/g`
    RegEx(RegEx<'a>),
    /// ```js
    /// `I have ${0} apples`
    /// ```
    Template(TemplateLit<'a>),
}

impl<'a> Lit<'a> {
    pub fn number_from(s: &'a str) -> Self {
        Lit::Number(
            Cow::Borrowed(s)
        )
    }
    pub fn single_string_from(s: &'a str) -> Self {
        Lit::String(
            StringLit::single_from(s)
        )
    }
    pub fn double_string_from(s: &'a str) -> Self {
        Lit::String(
            StringLit::double_from(s)
        )
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum StringLit<'a> {
    Double(Cow<'a, str>),
    Single(Cow<'a, str>),
}

impl<'a> StringLit<'a> {
    pub fn double_from(s: &'a str) -> StringLit<'a> {
        StringLit::Double(
            Cow::Borrowed(s)
        )
    }
    pub fn single_from(s: &'a str) -> StringLit<'a> {
        StringLit::Single(
            Cow::Borrowed(s)
        )
    }
    pub fn clone_inner(&self) -> Cow<'a, str> {
        match self {
            StringLit::Single(ref s) => s.clone(),
            StringLit::Double(ref s) => s.clone(),
        }
    }
    pub fn inner_matches(&self, o: &str) -> bool {
        match self {
            StringLit::Single(ref s) => s == o,
            StringLit::Double(ref d) => d == o,
        }
    }
}
/// A regular expression literal
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegEx<'a> {
    pub pattern: Cow<'a, str>,
    pub flags: Cow<'a, str>,
}

impl<'a> RegEx<'a> {
    pub fn from(p: &'a str, f: &'a str) -> Self {
        RegEx {
            pattern: Cow::Borrowed(p),
            flags: Cow::Borrowed(f),
        }
    }
}