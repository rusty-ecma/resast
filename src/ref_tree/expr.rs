use crate::ref_tree::pat::Pat;
use crate::ref_tree::{Class, Function, FunctionArg, FunctionBody, Identifier};

/// A slightly more granular program part that a statement
#[derive(PartialEq, Debug, Clone)]
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
    ArrowFunction(ArrowFunctionExpr<'a>),
    /// Used for resolving possible sequence expressions
    /// that are arrow parameters
    ArrowParamPlaceHolder(Vec<FunctionArg<'a>>, bool),
    /// Assignment or update assignment
    /// ```js
    /// a = 0
    /// b += 1
    /// ```
    Assignment(AssignmentExpr<'a>),
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
    Function(Function<'a>),
    /// An identifier
    Ident(Identifier<'a>),
    /// A literal value, see `Literal`
    Literal(Literal<'a>),
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
    MetaProperty(MetaProperty<'a>),
    /// ```js
    /// var a = true ? 'stuff' : 'things';
    /// ```
    /// `{}`
    /// Calling a constructor
    New(NewExpr<'a>),
    Object(ObjectExpr<'a>),
    /// Any sequence of expressions separated with a comma
    Sequence(SequenceExpr<'a>),
    /// `...` followed by an `Expr`
    Spread(Box<Expr<'a>>),
    /// `super`
    SuperExpr,
    /// A template literal preceded by a tag function identifier
    TaggedTemplate(TaggedTemplateExpr<'a>),
    /// `this`
    ThisExpr,
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

/// `[a, b, c]`
pub type ArrayExpr<'a> = Vec<Option<Expr<'a>>>;
/// `{a: 'b', c, ...d}`
pub type ObjectExpr<'a> = Vec<ObjectProperty<'a>>;
/// A single part of an object literal
#[derive(PartialEq, Debug, Clone)]
pub enum ObjectProperty<'a> {
    Property(Property<'a>),
    Spread(Box<Expr<'a>>),
}

/// A single part of an object literal or class
#[derive(PartialEq, Debug, Clone)]
pub struct Property<'a> {
    pub key: PropertyKey<'a>,
    pub value: PropertyValue<'a>,
    pub kind: PropertyKind,
    pub method: bool,
    pub computed: bool,
    pub short_hand: bool,
}
/// An object literal or class property identifier
#[derive(PartialEq, Debug, Clone)]
pub enum PropertyKey<'a> {
    Literal(Literal<'a>),
    Expr(Expr<'a>),
    Pat(Pat<'a>),
}
/// The value of an object literal or class property
#[derive(PartialEq, Debug, Clone)]
pub enum PropertyValue<'a> {
    Expr(Expr<'a>),
    Pat(Pat<'a>),
    None,
}

/// A flag for determining what kind of property
#[derive(PartialEq, Debug, Clone)]
pub enum PropertyKind {
    /// A property with a value
    Init,
    /// A method with the get keyword
    Get,
    /// A method with the set keyword
    Set,
    /// A constructor
    Ctor,
    /// A standard method
    Method,
}
/// An operation that takes one argument
#[derive(PartialEq, Debug, Clone)]
pub struct UnaryExpr<'a> {
    pub operator: UnaryOperator,
    pub prefix: bool,
    pub argument: Box<Expr<'a>>,
}

/// The allowed operators for an expression
/// to be `Unary`
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum UnaryOperator {
    Minus,
    Plus,
    Not,
    Tilde,
    TypeOf,
    Void,
    Delete,
}

/// Increment or decrementing a value
#[derive(PartialEq, Debug, Clone)]
pub struct UpdateExpr<'a> {
    pub operator: UpdateOperator,
    pub argument: Box<Expr<'a>>,
    pub prefix: bool,
}

/// `++` or `--`
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum UpdateOperator {
    Increment,
    Decrement,
}

/// An operation that requires 2 arguments
#[derive(PartialEq, Debug, Clone)]
pub struct BinaryExpr<'a> {
    pub operator: BinaryOperator,
    pub left: Box<Expr<'a>>,
    pub right: Box<Expr<'a>>,
}

/// The available operations for `Binary` expressions
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum BinaryOperator {
    Equal,
    NotEqual,
    StrictEqual,
    StrictNotEqual,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,
    LeftShift,
    RightShift,
    UnsignedRightShift,
    Plus,
    Minus,
    Times,
    Over,
    Mod,
    Or,
    XOr,
    And,
    In,
    InstanceOf,
    PowerOf,
}

/// An assignment or update + assignment operation
#[derive(PartialEq, Debug, Clone)]
pub struct AssignmentExpr<'a> {
    pub operator: AssignmentOperator,
    pub left: AssignmentLeft<'a>,
    pub right: Box<Expr<'a>>,
}

/// The value being assigned to
#[derive(PartialEq, Debug, Clone)]
pub enum AssignmentLeft<'a> {
    Pat(Pat<'a>),
    Expr(Box<Expr<'a>>),
}

/// The available operators for assignment expressions
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum AssignmentOperator {
    Equal,
    PlusEqual,
    MinusEqual,
    TimesEqual,
    DivEqual,
    ModEqual,
    LeftShiftEqual,
    RightShiftEqual,
    UnsignedRightShiftEqual,
    OrEqual,
    XOrEqual,
    AndEqual,
    PowerOfEqual,
}

/// A specialized `BinaryExpr` for logical evaluation
/// ```js
/// true && true
/// false || true
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct LogicalExpr<'a> {
    pub operator: LogicalOperator,
    pub left: Box<Expr<'a>>,
    pub right: Box<Expr<'a>>,
}

/// The available logical operators
#[derive(PartialEq, Debug, Clone)]
pub enum LogicalOperator {
    Or,
    And,
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
    pub computed: bool,
}

/// A ternery expression
/// ```js
/// var a = true ? 'stuff' : 'things';
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct ConditionalExpr<'a> {
    pub test: Box<Expr<'a>>,
    pub alternate: Box<Expr<'a>>,
    pub consequent: Box<Expr<'a>>,
}

/// Calling a function or method
/// ```js
/// Math.random()
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct CallExpr<'a> {
    pub callee: Box<Expr<'a>>,
    pub arguments: Vec<Expr<'a>>,
}

/// Calling a constructor
/// ```js
/// new Uint8Array(32);
/// ```
#[derive(PartialEq, Debug, Clone)]
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
#[derive(PartialEq, Debug, Clone)]
pub struct ArrowFunctionExpr<'a> {
    pub id: Option<Identifier<'a>>,
    pub params: Vec<FunctionArg<'a>>,
    pub body: ArrowFunctionBody<'a>,
    pub expression: bool,
    pub generator: bool,
    pub is_async: bool,
}

/// The body portion of an arrow function can be either an expression or a block of statements
#[derive(PartialEq, Debug, Clone)]
pub enum ArrowFunctionBody<'a> {
    FunctionBody(FunctionBody<'a>),
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
#[derive(PartialEq, Debug, Clone)]
pub struct YieldExpr<'a> {
    pub argument: Option<Box<Expr<'a>>>,
    pub delegate: bool,
}

/// A Template literal preceded by a function identifier
/// see [MDN](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Template_literals#Tagged_templates) for more details
#[derive(PartialEq, Debug, Clone)]
pub struct TaggedTemplateExpr<'a> {
    pub tag: Box<Expr<'a>>,
    pub quasi: TemplateLiteral<'a>,
}

/// A template string literal
/// ```js
/// `I own ${0} birds`;
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct TemplateLiteral<'a> {
    pub quasis: Vec<TemplateElement<'a>>,
    pub expressions: Vec<Expr<'a>>,
}

/// The text part of a `TemplateLiteral`
#[derive(PartialEq, Debug, Clone)]
pub struct TemplateElement<'a> {
    pub tail: bool,
    /// The non-quoted version
    pub cooked: &'a str,
    /// The quoted version
    pub raw: &'a str,
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
pub struct MetaProperty<'a> {
    pub meta: Identifier<'a>,
    pub property: Identifier<'a>,
}

/// A literal value
#[derive(PartialEq, Debug, Clone)]
pub enum Literal<'a> {
    /// `null`
    Null,
    /// `"string"`
    /// `'string'`
    String(&'a str),
    /// `0`
    /// `0.0`
    /// `.0`
    /// `0.0e1`
    /// `.0E1`
    /// `0xf`
    /// `0o7`
    /// `0b1`
    Number(&'a str),
    /// `true`
    /// `false`
    Boolean(bool),
    /// `/.+/g`
    RegEx(RegEx<'a>),
    /// ```js
    /// `I have ${0} apples`
    /// ```
    Template(TemplateLiteral<'a>),
}

/// A regular expression literal
#[derive(PartialEq, Debug, Clone)]
pub struct RegEx<'a> {
    pub pattern: &'a str,
    pub flags: &'a str,
}
