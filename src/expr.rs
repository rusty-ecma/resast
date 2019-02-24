use crate::pat::Pat;
use crate::{Class, Function, FunctionArg, FunctionBody, Identifier};

/// A slightly more granular program part that a statement
#[derive(PartialEq, Debug, Clone)]
pub enum Expr {
    /// `[0,,]`
    Array(ArrayExpr),
    /// An arrow function
    /// ```js
    /// () => console.log();
    /// x => {
    ///     return x;
    /// }
    /// ```
    ArrowFunction(ArrowFunctionExpr),
    /// Used for resolving possible sequence expressions
    /// that are arrow parameters
    ArrowParamPlaceHolder(Vec<FunctionArg>, bool),
    /// Assignment or update assignment
    /// ```js
    /// a = 0
    /// b += 1
    /// ```
    Assignment(AssignmentExpr),
    /// The `await` keyword followed by another `Expr`
    Await(Box<Expr>),
    /// An operation that has two arguments
    Binary(BinaryExpr),
    /// A class expression see `Class`
    Class(Class),
    /// Calling a function or method
    Call(CallExpr),
    /// A ternery expression
    Conditional(ConditionalExpr),
    /// see `Function`
    Function(Function),
    /// An identifier
    Ident(Identifier),
    /// A literal value, see `Literal`
    Literal(Literal),
    /// A specialized `BinaryExpr` for logical evaluation
    /// ```js
    /// true && true
    /// false || true
    /// ```
    Logical(LogicalExpr),
    /// Accessing the member of a value
    /// ```js
    /// b['thing'];
    /// c.stuff;
    /// ```
    Member(MemberExpr),
    /// currently just `new.target`
    MetaProperty(MetaProperty),
    /// ```js
    /// var a = true ? 'stuff' : 'things';
    /// ```
    /// `{}`
    /// Calling a constructor
    New(NewExpr),
    Object(ObjectExpr),
    /// Any sequence of expressions separated with a comma
    Sequence(SequenceExpr),
    /// `...` followed by an `Expr`
    Spread(Box<Expr>),
    /// `super`
    Super,
    /// A template literal preceded by a tag function identifier
    TaggedTemplate(TaggedTemplateExpr),
    /// `this`
    This,
    /// An operation that has one argument
    /// ```js
    /// typeof 'a';
    /// +9;
    /// ```
    Unary(UnaryExpr),
    /// Increment or decrement
    /// ```js
    /// 1++
    /// --2
    /// ```
    Update(UpdateExpr),
    /// yield a value from inside of a generator function
    Yield(YieldExpr),
}

/// `[a, b, c]`
pub type ArrayExpr = Vec<Option<Expr>>;
/// `{a: 'b', c, ...d}`
pub type ObjectExpr = Vec<ObjectProperty>;
/// A single part of an object literal
#[derive(PartialEq, Debug, Clone)]
pub enum ObjectProperty {
    Property(Property),
    Spread(Box<Expr>),
}

/// A single part of an object literal or class
#[derive(PartialEq, Debug, Clone)]
pub struct Property {
    pub key: PropertyKey,
    pub value: PropertyValue,
    pub kind: PropertyKind,
    pub method: bool,
    pub computed: bool,
    pub short_hand: bool,
}
/// An object literal or class property identifier
#[derive(PartialEq, Debug, Clone)]
pub enum PropertyKey {
    Literal(Literal),
    Expr(Expr),
    Pat(Pat),
}
/// The value of an object literal or class property
#[derive(PartialEq, Debug, Clone)]
pub enum PropertyValue {
    Expr(Expr),
    Pat(Pat),
    None,
}

/// A flag for determining what kind of property
#[derive(PartialEq, Debug, Clone, Copy)]
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
pub struct UnaryExpr {
    pub operator: UnaryOperator,
    pub prefix: bool,
    pub argument: Box<Expr>,
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
pub struct UpdateExpr {
    pub operator: UpdateOperator,
    pub argument: Box<Expr>,
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
pub struct BinaryExpr {
    pub operator: BinaryOperator,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
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
pub struct AssignmentExpr {
    pub operator: AssignmentOperator,
    pub left: AssignmentLeft,
    pub right: Box<Expr>,
}

/// The value being assigned to
#[derive(PartialEq, Debug, Clone)]
pub enum AssignmentLeft {
    Pat(Pat),
    Expr(Box<Expr>),
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
pub struct LogicalExpr {
    pub operator: LogicalOperator,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

/// The available logical operators
#[derive(PartialEq, Debug, Clone, Copy)]
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
pub struct MemberExpr {
    pub object: Box<Expr>,
    pub property: Box<Expr>,
    pub computed: bool,
}

/// A ternery expression
/// ```js
/// var a = true ? 'stuff' : 'things';
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct ConditionalExpr {
    pub test: Box<Expr>,
    pub alternate: Box<Expr>,
    pub consequent: Box<Expr>,
}

/// Calling a function or method
/// ```js
/// Math.random()
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct CallExpr {
    pub callee: Box<Expr>,
    pub arguments: Vec<Expr>,
}

/// Calling a constructor
/// ```js
/// new Uint8Array(32);
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct NewExpr {
    pub callee: Box<Expr>,
    pub arguments: Vec<Expr>,
}

/// A collection of `Exprs` separated by commas
pub type SequenceExpr = Vec<Expr>;

/// An arrow function
/// ```js
/// let x = () => y;
/// let q = x => {
///     return x + 1;
/// }
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct ArrowFunctionExpr {
    pub id: Option<String>,
    pub params: Vec<FunctionArg>,
    pub body: ArrowFunctionBody,
    pub expression: bool,
    pub generator: bool,
    pub is_async: bool,
}

/// The body portion of an arrow function can be either an expression or a block of statements
#[derive(PartialEq, Debug, Clone)]
pub enum ArrowFunctionBody {
    FunctionBody(FunctionBody),
    Expr(Box<Expr>),
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
pub struct YieldExpr {
    pub argument: Option<Box<Expr>>,
    pub delegate: bool,
}

/// A Template literal preceded by a function identifier
/// see [MDN](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Template_literals#Tagged_templates) for more details
#[derive(PartialEq, Debug, Clone)]
pub struct TaggedTemplateExpr {
    pub tag: Box<Expr>,
    pub quasi: TemplateLiteral,
}

/// A template string literal
/// ```js
/// `I own ${0} birds`;
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct TemplateLiteral {
    pub quasis: Vec<TemplateElement>,
    pub expressions: Vec<Expr>,
}

/// The text part of a `TemplateLiteral`
#[derive(PartialEq, Debug, Clone)]
pub struct TemplateElement {
    pub tail: bool,
    /// The non-quoted version
    pub cooked: String,
    /// The quoted version
    pub raw: String,
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
pub struct MetaProperty {
    pub meta: Identifier,
    pub property: Identifier,
}

/// A literal value
#[derive(PartialEq, Debug, Clone)]
pub enum Literal {
    /// `null`
    Null,
    /// `"string"`
    /// `'string'`
    String(String),
    /// `0`
    /// `0.0`
    /// `.0`
    /// `0.0e1`
    /// `.0E1`
    /// `0xf`
    /// `0o7`
    /// `0b1`
    Number(String),
    /// `true`
    /// `false`
    Boolean(bool),
    /// `/.+/g`
    RegEx(RegEx),
    /// ```js
    /// `I have ${0} apples`
    /// ```
    Template(TemplateLiteral),
}

/// A regular expression literal
#[derive(PartialEq, Debug, Clone)]
pub struct RegEx {
    pub pattern: String,
    pub flags: String,
}
