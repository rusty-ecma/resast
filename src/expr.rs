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
    /// Used for resolving possible sequence Exprs
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
    /// A class Expr see `Class`
    Class(Class),
    /// Calling a function or method
    Call(CallExpr),
    /// A ternery Expr
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
    /// Any sequence of Exprs separated with a comma
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

impl Expr {
    pub fn ident(name: &str) -> Self {
        Expr::Ident(name.to_string())
    }

    pub fn string(val: &str) -> Self {
        Expr::Literal(Literal::string(val))
    }

    pub fn number(val: &str) -> Self {
        Expr::Literal(Literal::number(val))
    }

    pub fn boolean(val: bool) -> Self {
        Expr::Literal(Literal::Boolean(val))
    }

    pub fn regex(pattern: &str, flags: &str) -> Self {
        Expr::Literal(Literal::regex(pattern, flags))
    }

    pub fn binary(left: Expr, operator: BinaryOperator, right: Expr) -> Self {
        Expr::Binary(BinaryExpr::new(left, operator, right))
    }

    pub fn call(callee: Expr, arguments: Vec<Expr>) -> Self {
        Expr::Call(CallExpr::new(callee, arguments))
    }

    pub fn member(object: Expr, property: Expr, computed: bool) -> Self {
        Expr::Member(MemberExpr::new(object, property, computed))
    }

    pub fn logical(left: Expr, operator: LogicalOperator, right: Expr) -> Self {
        Expr::Logical(LogicalExpr::new(operator, left, right))
    }

    pub fn function(
        id: Option<String>,
        params: Vec<FunctionArg>,
        body: FunctionBody,
        generator: bool,
        is_async: bool,
    ) -> Self {
        Expr::Function(Function {
            id,
            params,
            body,
            generator,
            is_async,
        })
    }

    pub fn yield_expr(arg: Option<Expr>, delegate: bool) -> Self {
        Expr::Yield(YieldExpr::new(arg, delegate))
    }

    pub fn yield_with_arg(arg: Expr, delegate: bool) -> Self {
        Expr::Yield(YieldExpr::new(Some(arg), delegate))
    }

    pub fn empty_yield(delegate: bool) -> Self {
        Expr::Yield(YieldExpr::new(None, delegate))
    }
    
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

impl ObjectProperty {
    pub fn number(id: &str, value: &str) -> Self {
        let id = PropertyKey::Expr(
            Expr::ident(id)
        );
        let init = PropertyValue::Expr(Expr::Literal(Literal::Number(String::from(value))));
        ObjectProperty::Property(Property::new(id, init, PropertyKind::Init, false, false, false, false))
    }
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
    pub is_static: bool,
}

impl Property {
    pub fn new(key: PropertyKey, value: PropertyValue, kind: PropertyKind, method: bool, computed: bool, short_hand: bool, is_static: bool) -> Self {
        Self {
            key,
            value,
            kind,
            method,
            computed,
            short_hand,
            is_static,
        }
    }
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

impl UnaryExpr {
    pub fn new(operator: UnaryOperator, prefix: bool, argument: Expr) -> Self {
        Self {
            operator,
            prefix,
            argument: Box::new(argument),
        }
    }
}

/// The allowed operators for an Expr
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

impl UpdateExpr {
    pub fn new(operator: UpdateOperator, arg: Expr, prefix: bool) -> Self {
        Self {
            operator,
            argument: Box::new(arg),
            prefix,
        }
    }
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

impl BinaryExpr {
    pub fn new(left: Expr, operator: BinaryOperator, right: Expr) -> Self {
        Self {
            operator,
            left: Box::new(left),
            right: Box::new(right),
        }
    }
}

/// The available operations for `Binary` Exprs
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

impl AssignmentExpr {
    pub fn new(operator: AssignmentOperator, left: AssignmentLeft, right: Expr) -> Self {
        AssignmentExpr {
            operator,
            left,
            right: Box::new(right),
        }
    }
}

/// The value being assigned to
#[derive(PartialEq, Debug, Clone)]
pub enum AssignmentLeft {
    Pat(Pat),
    Expr(Box<Expr>),
}

impl AssignmentLeft {
    pub fn expr(expr: Expr) -> Self {
        AssignmentLeft::Expr(Box::new(expr))
    }
}

/// The available operators for assignment Exprs
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

impl LogicalExpr {
    pub fn new(operator: LogicalOperator, left: Expr, right: Expr) -> Self {
        Self {
            operator,
            left: Box::new(left),
            right: Box::new(right),
        }
    }
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

impl MemberExpr {
    pub fn new(obj: Expr, prop: Expr, computed: bool) -> Self {
        MemberExpr {
            object: Box::new(obj),
            property: Box::new(prop),
            computed,
        }
    }
}

/// A ternery Expr
/// ```js
/// var a = true ? 'stuff' : 'things';
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct ConditionalExpr {
    pub test: Box<Expr>,
    pub alternate: Box<Expr>,
    pub consequent: Box<Expr>,
}

impl ConditionalExpr {
    pub fn new(test: Expr, alternate: Expr, consequent: Expr) -> Self {
        ConditionalExpr {
            test: Box::new(test),
            alternate: Box::new(alternate),
            consequent: Box::new(consequent),
        }
    }
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

impl CallExpr {
    pub fn new(callee: Expr, arguments: Vec<Expr>) -> Self {
        CallExpr {
            callee: Box::new(callee),
            arguments
        }
    }
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

impl NewExpr {
    pub fn new(callee: Expr, arguments: Vec<Expr>) -> Self {
        NewExpr {
            callee: Box::new(callee),
            arguments,
        }
    }
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

impl ArrowFunctionExpr {
    pub fn new(id: Option<String>, params: Vec<FunctionArg>, body: ArrowFunctionBody, expression: bool, generator: bool, is_async: bool) -> Self {
        ArrowFunctionExpr {
            id,
            params,
            body,
            expression,
            generator,
            is_async,
        }
    }
}

/// The body portion of an arrow function can be either an Expr or a block of statements
#[derive(PartialEq, Debug, Clone)]
pub enum ArrowFunctionBody {
    FunctionBody(FunctionBody),
    Expr(Box<Expr>),
}

impl ArrowFunctionBody {
    pub fn function_body(bod: FunctionBody) -> Self {
        ArrowFunctionBody::FunctionBody(bod)
    }
    pub fn expr(expr: Expr) -> Self {
        ArrowFunctionBody::Expr(Box::new(expr))
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
pub struct YieldExpr {
    pub argument: Option<Box<Expr>>,
    pub delegate: bool,
}

impl YieldExpr {
    pub fn new(argument: Option<Expr>, delegate: bool) -> YieldExpr {
        YieldExpr {
            argument: argument.map(|a| Box::new(a)),
            delegate
        }
    }
}

/// A Template literal preceded by a function identifier
/// see [MDN](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Template_literals#Tagged_templates) for more details
#[derive(PartialEq, Debug, Clone)]
pub struct TaggedTemplateExpr {
    pub tag: Box<Expr>,
    pub quasi: TemplateLiteral,
}

impl TaggedTemplateExpr {
    pub fn new(tag: Expr, quasi: TemplateLiteral) -> Self {
        TaggedTemplateExpr {
            tag: Box::new(tag),
            quasi,
        }
    }
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

impl TemplateLiteral {
    pub fn new(quasis: Vec<TemplateElement>, expressions: Vec<Expr>) -> Self {
        TemplateLiteral {
            quasis,
            expressions,
        }
    }
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

impl TemplateElement {
    pub fn new(tail: bool, cooked: String, raw: String) -> Self {
        TemplateElement {
            tail,
            cooked,
            raw,
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
pub struct MetaProperty {
    pub meta: Identifier,
    pub property: Identifier,
}

impl MetaProperty {
    pub fn new(meta: Identifier, property: Identifier) -> Self {
        MetaProperty {
            meta,
            property,
        }
    }
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

impl Literal {
    pub fn string(string: &str) -> Self {
        Literal::String(string.to_string())
    }

    pub fn number(num: &str) -> Self {
        Literal::Number(num.to_string())
    }

    pub fn regex(pattern: &str, flags: &str) -> Self {
        let inner = RegEx::new(pattern, flags);
        Literal::RegEx(inner)
    }
}

/// A regular Expr literal
#[derive(PartialEq, Debug, Clone)]
pub struct RegEx {
    pub pattern: String,
    pub flags: String,
}

impl RegEx {
    pub fn new(body: &str, flags: &str) -> Self {
        RegEx {
            pattern: String::from(body),
            flags: String::from(flags),
        }
    }
}