use crate::pattern::Pattern;
use crate::{Class, Function, FunctionArg, FunctionBody, Identifier};

/// A slightly more granular program part that a statement
#[derive(PartialEq, Debug, Clone)]
pub enum Expression {
    /// `[0,,]`
    Array(ArrayExpression),
    /// An arrow function
    /// ```js
    /// () => console.log();
    /// x => {
    ///     return x;
    /// }
    /// ```
    ArrowFunction(ArrowFunctionExpression),
    /// Used for resolving possible sequence expressions
    /// that are arrow parameters
    ArrowParamPlaceHolder(Vec<FunctionArg>, bool),
    /// Assignment or update assignment
    /// ```js
    /// a = 0
    /// b += 1
    /// ```
    Assignment(AssignmentExpression),
    /// The `await` keyword followed by another `Expression`
    Await(Box<Expression>),
    /// An operation that has two arguments
    Binary(BinaryExpression),
    /// A class expression see `Class`
    Class(Class),
    /// Calling a function or method
    Call(CallExpression),
    /// A ternery expression
    Conditional(ConditionalExpression),
    /// see `Function`
    Function(Function),
    /// An identifier
    Ident(Identifier),
    /// A literal value, see `Literal`
    Literal(Literal),
    /// A specialized `BinaryExpression` for logical evaluation
    /// ```js
    /// true && true
    /// false || true
    /// ```
    Logical(LogicalExpression),
    /// Accessing the member of a value
    /// ```js
    /// b['thing'];
    /// c.stuff;
    /// ```
    Member(MemberExpression),
    /// currently just `new.target`
    MetaProperty(MetaProperty),
    /// ```js
    /// var a = true ? 'stuff' : 'things';
    /// ```
    /// `{}`
    /// Calling a constructor
    New(NewExpression),
    Object(ObjectExpression),
    /// Any sequence of expressions separated with a comma
    Sequence(SequenceExpression),
    /// `...` followed by an `Expression`
    Spread(Box<Expression>),
    /// `super`
    SuperExpression,
    /// A template literal preceded by a tag function identifier
    TaggedTemplate(TaggedTemplateExpression),
    /// `this`
    ThisExpression,
    /// An operation that has one argument
    /// ```js
    /// typeof 'a';
    /// +9;
    /// ```
    Unary(UnaryExpression),
    /// Increment or decrement
    /// ```js
    /// 1++
    /// --2
    /// ```
    Update(UpdateExpression),
    /// yield a value from inside of a generator function
    Yield(YieldExpression),
}

/// `[a, b, c]`
pub type ArrayExpression = Vec<Option<Expression>>;
/// `{a: 'b', c, ...d}`
pub type ObjectExpression = Vec<ObjectProperty>;
/// A single part of an object literal
#[derive(PartialEq, Debug, Clone)]
pub enum ObjectProperty {
    Property(Property),
    Spread(Box<Expression>),
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
    Expr(Expression),
    Pattern(Pattern),
}
/// The value of an object literal or class property
#[derive(PartialEq, Debug, Clone)]
pub enum PropertyValue {
    Expr(Expression),
    Pattern(Pattern),
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
pub struct UnaryExpression {
    pub operator: UnaryOperator,
    pub prefix: bool,
    pub argument: Box<Expression>,
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
pub struct UpdateExpression {
    pub operator: UpdateOperator,
    pub argument: Box<Expression>,
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
pub struct BinaryExpression {
    pub operator: BinaryOperator,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
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
pub struct AssignmentExpression {
    pub operator: AssignmentOperator,
    pub left: AssignmentLeft,
    pub right: Box<Expression>,
}

/// The value being assigned to
#[derive(PartialEq, Debug, Clone)]
pub enum AssignmentLeft {
    Pattern(Pattern),
    Expr(Box<Expression>),
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

/// A specialized `BinaryExpression` for logical evaluation
/// ```js
/// true && true
/// false || true
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct LogicalExpression {
    pub operator: LogicalOperator,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
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
pub struct MemberExpression {
    pub object: Box<Expression>,
    pub property: Box<Expression>,
    pub computed: bool,
}

/// A ternery expression
/// ```js
/// var a = true ? 'stuff' : 'things';
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct ConditionalExpression {
    pub test: Box<Expression>,
    pub alternate: Box<Expression>,
    pub consequent: Box<Expression>,
}

/// Calling a function or method
/// ```js
/// Math.random()
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct CallExpression {
    pub callee: Box<Expression>,
    pub arguments: Vec<Expression>,
}

/// Calling a constructor
/// ```js
/// new Uint8Array(32);
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct NewExpression {
    pub callee: Box<Expression>,
    pub arguments: Vec<Expression>,
}

/// A collection of `Expressions` separated by commas
pub type SequenceExpression = Vec<Expression>;

/// An arrow function
/// ```js
/// let x = () => y;
/// let q = x => {
///     return x + 1;
/// }
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct ArrowFunctionExpression {
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
    Expr(Box<Expression>),
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
pub struct YieldExpression {
    pub argument: Option<Box<Expression>>,
    pub delegate: bool,
}

/// A Template literal preceded by a function identifier
/// see [MDN](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Template_literals#Tagged_templates) for more details
#[derive(PartialEq, Debug, Clone)]
pub struct TaggedTemplateExpression {
    pub tag: Box<Expression>,
    pub quasi: TemplateLiteral,
}

/// A template string literal
/// ```js
/// `I own ${0} birds`;
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct TemplateLiteral {
    pub quasis: Vec<TemplateElement>,
    pub expressions: Vec<Expression>,
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
