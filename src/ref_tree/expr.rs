use crate::expr::{
    AssignmentOperator, BinaryOperator, LogicalOperator, PropertyKind, UnaryOperator,
    UpdateOperator,
};
use crate::ref_tree::pat::Pat;
use crate::ref_tree::{AsConcrete, Class, Function, FunctionArg, FunctionBody, Identifier, ref_map};
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

impl<'a> AsConcrete<crate::expr::Expr> for Expr<'a> {
    fn as_concrete(&self) -> crate::expr::Expr {
        match self {
            Expr::Array(ref a) => crate::expr::Expr::Array(
                a.iter()
                    .map(|e| ref_map(e, |i| i.as_concrete()))
                    .collect(),
            ),
            Expr::ArrowFunction(ref a) => crate::expr::Expr::ArrowFunction(a.as_concrete()),
            Expr::ArrowParamPlaceHolder(ref a, ref b) => crate::expr::Expr::ArrowParamPlaceHolder(
                a.iter().map(|a| a.as_concrete()).collect(),
                *b,
            ),
            Expr::Assignment(ref a) => crate::expr::Expr::Assignment(a.as_concrete()),
            Expr::Await(ref a) => crate::expr::Expr::Await(Box::new((*a).as_concrete())),
            Expr::Binary(ref b) => crate::expr::Expr::Binary(b.as_concrete()),
            Expr::Call(ref c) => crate::expr::Expr::Call(c.as_concrete()),
            Expr::Class(ref c) => crate::expr::Expr::Class(c.as_concrete()),
            Expr::Conditional(ref c) => crate::expr::Expr::Conditional(c.as_concrete()),
            Expr::Function(ref f) => crate::expr::Expr::Function(f.as_concrete()),
            Expr::Ident(ref i) => crate::expr::Expr::Ident(String::from(*i)),
            Expr::Literal(ref l) => crate::expr::Expr::Literal(l.as_concrete()),
            Expr::Logical(ref l) => crate::expr::Expr::Logical(l.as_concrete()),
            Expr::Member(ref m) => crate::expr::Expr::Member(m.as_concrete()),
            Expr::MetaProperty(ref m) => crate::expr::Expr::MetaProperty(m.as_concrete()),
            Expr::New(ref n) => crate::expr::Expr::New(n.as_concrete()),
            Expr::Object(ref o) => crate::expr::Expr::Object(o.as_concrete()),
            Expr::Sequence(ref s) => crate::expr::Expr::Sequence(s.as_concrete()),
            Expr::Spread(ref s) => crate::expr::Expr::Spread(Box::new(s.as_concrete())),
            Expr::Super => crate::expr::Expr::Super,
            Expr::TaggedTemplate(ref t) => crate::expr::Expr::TaggedTemplate(t.as_concrete()),
            Expr::This => crate::expr::Expr::This,
            Expr::Unary(ref u) => crate::expr::Expr::Unary(u.as_concrete()),
            Expr::Update(ref u) => crate::expr::Expr::Update(u.as_concrete()),
            Expr::Yield(ref y) => crate::expr::Expr::Yield(y.as_concrete()),
        }
    }
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

impl<'a> AsConcrete<crate::expr::ObjectProperty> for ObjectProperty<'a> {
    fn as_concrete(&self) -> crate::expr::ObjectProperty {
        match self {
            ObjectProperty::Property(ref p) => {
                crate::expr::ObjectProperty::Property(p.as_concrete())
            }
            ObjectProperty::Spread(ref e) => {
                crate::expr::ObjectProperty::Spread(Box::new(e.as_concrete()))
            }
        }
    }
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
    pub is_static: bool,
}

impl<'a> AsConcrete<crate::expr::Property> for Property<'a> {
    fn as_concrete(&self) -> crate::expr::Property {
        crate::expr::Property {
            key: self.key.as_concrete(),
            value: self.value.as_concrete(),
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
pub enum PropertyKey<'a> {
    Literal(Literal<'a>),
    Expr(Expr<'a>),
    Pat(Pat<'a>),
}

impl<'a> AsConcrete<crate::expr::PropertyKey> for PropertyKey<'a> {
    fn as_concrete(&self) -> crate::expr::PropertyKey {
        match self {
            PropertyKey::Literal(ref l) => crate::expr::PropertyKey::Literal(l.as_concrete()),
            PropertyKey::Expr(ref e) => crate::expr::PropertyKey::Expr(e.as_concrete()),
            PropertyKey::Pat(ref p) => crate::expr::PropertyKey::Pat(p.as_concrete()),
        }
    }
}

/// The value of an object literal or class property
#[derive(PartialEq, Debug, Clone)]
pub enum PropertyValue<'a> {
    Expr(Expr<'a>),
    Pat(Pat<'a>),
    None,
}

impl<'a> AsConcrete<crate::expr::PropertyValue> for PropertyValue<'a> {
    fn as_concrete(&self) -> crate::expr::PropertyValue {
        match self {
            PropertyValue::Expr(ref e) => crate::expr::PropertyValue::Expr(e.as_concrete()),
            PropertyValue::Pat(ref p) => crate::expr::PropertyValue::Pat(p.as_concrete()),
            PropertyValue::None => crate::expr::PropertyValue::None,
        }
    }
}

/// An operation that takes one argument
#[derive(PartialEq, Debug, Clone)]
pub struct UnaryExpr<'a> {
    pub operator: UnaryOperator,
    pub prefix: bool,
    pub argument: Box<Expr<'a>>,
}

impl<'a> AsConcrete<crate::expr::UnaryExpr> for UnaryExpr<'a> {
    fn as_concrete(&self) -> crate::expr::UnaryExpr {
        crate::expr::UnaryExpr {
            operator: self.operator,
            prefix: self.prefix,
            argument: Box::new(self.argument.as_concrete()),
        }
    }
}

/// Increment or decrementing a value
#[derive(PartialEq, Debug, Clone)]
pub struct UpdateExpr<'a> {
    pub operator: UpdateOperator,
    pub argument: Box<Expr<'a>>,
    pub prefix: bool,
}

impl<'a> AsConcrete<crate::expr::UpdateExpr> for UpdateExpr<'a> {
    fn as_concrete(&self) -> crate::expr::UpdateExpr {
        crate::expr::UpdateExpr {
            operator: self.operator,
            argument: Box::new(self.argument.as_concrete()),
            prefix: self.prefix,
        }
    }
}

/// An operation that requires 2 arguments
#[derive(PartialEq, Debug, Clone)]
pub struct BinaryExpr<'a> {
    pub operator: BinaryOperator,
    pub left: Box<Expr<'a>>,
    pub right: Box<Expr<'a>>,
}

impl<'a> AsConcrete<crate::expr::BinaryExpr> for BinaryExpr<'a> {
    fn as_concrete(&self) -> crate::expr::BinaryExpr {
        crate::expr::BinaryExpr {
            operator: self.operator,
            left: Box::new(self.left.as_concrete()),
            right: Box::new(self.right.as_concrete()),
        }
    }
}

/// An assignment or update + assignment operation
#[derive(PartialEq, Debug, Clone)]
pub struct AssignmentExpr<'a> {
    pub operator: AssignmentOperator,
    pub left: AssignmentLeft<'a>,
    pub right: Box<Expr<'a>>,
}

impl<'a> AsConcrete<crate::expr::AssignmentExpr> for AssignmentExpr<'a> {
    fn as_concrete(&self) -> crate::expr::AssignmentExpr {
        crate::expr::AssignmentExpr {
            operator: self.operator,
            left: self.left.as_concrete(),
            right: Box::new(self.right.as_concrete()),
        }
    }
}

/// The value being assigned to
#[derive(PartialEq, Debug, Clone)]
pub enum AssignmentLeft<'a> {
    Pat(Pat<'a>),
    Expr(Box<Expr<'a>>),
}

impl<'a> AsConcrete<crate::expr::AssignmentLeft> for AssignmentLeft<'a> {
    fn as_concrete(&self) -> crate::expr::AssignmentLeft {
        match self {
            AssignmentLeft::Expr(ref e) => {
                crate::expr::AssignmentLeft::Expr(Box::new(e.as_concrete()))
            }
            AssignmentLeft::Pat(ref p) => crate::expr::AssignmentLeft::Pat(p.as_concrete()),
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
    pub operator: LogicalOperator,
    pub left: Box<Expr<'a>>,
    pub right: Box<Expr<'a>>,
}

impl<'a> AsConcrete<crate::expr::LogicalExpr> for LogicalExpr<'a> {
    fn as_concrete(&self) -> crate::expr::LogicalExpr {
        crate::expr::LogicalExpr {
            operator: self.operator,
            left: Box::new(self.left.as_concrete()),
            right: Box::new(self.right.as_concrete()),
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
    pub computed: bool,
}

impl<'a> AsConcrete<crate::expr::MemberExpr> for MemberExpr<'a> {
    fn as_concrete(&self) -> crate::expr::MemberExpr {
        crate::expr::MemberExpr {
            object: Box::new(self.object.as_concrete()),
            property: Box::new(self.property.as_concrete()),
            computed: self.computed,
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
    pub alternate: Box<Expr<'a>>,
    pub consequent: Box<Expr<'a>>,
}

impl<'a> AsConcrete<crate::expr::ConditionalExpr> for ConditionalExpr<'a> {
    fn as_concrete(&self) -> crate::expr::ConditionalExpr {
        crate::expr::ConditionalExpr {
            test: Box::new(self.test.as_concrete()),
            alternate: Box::new(self.alternate.as_concrete()),
            consequent: Box::new(self.consequent.as_concrete()),
        }
    }
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

impl<'a> AsConcrete<crate::expr::CallExpr> for CallExpr<'a> {
    fn as_concrete(&self) -> crate::expr::CallExpr {
        crate::expr::CallExpr {
            callee: Box::new(self.callee.as_concrete()),
            arguments: self.arguments.as_concrete(),
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
    pub arguments: Vec<Expr<'a>>,
}

impl<'a> AsConcrete<crate::expr::NewExpr> for NewExpr<'a> {
    fn as_concrete(&self) -> crate::expr::NewExpr {
        crate::expr::NewExpr {
            callee: Box::new(self.callee.as_concrete()),
            arguments: self.arguments.as_concrete(),
        }
    }
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

impl<'a> AsConcrete<crate::expr::ArrowFunctionExpr> for ArrowFunctionExpr<'a> {
    fn as_concrete(&self) -> crate::expr::ArrowFunctionExpr {
        crate::expr::ArrowFunctionExpr {
            id: ref_map(&self.id, |i| i.as_concrete()),
            params: self.params.iter().map(|p| p.as_concrete()).collect(),
            body: self.body.as_concrete(),
            expression: self.expression,
            generator: self.generator,
            is_async: self.is_async,
        }
    }
}

/// The body portion of an arrow function can be either an expression or a block of statements
#[derive(PartialEq, Debug, Clone)]
pub enum ArrowFunctionBody<'a> {
    FunctionBody(FunctionBody<'a>),
    Expr(Box<Expr<'a>>),
}

impl<'a> AsConcrete<crate::expr::ArrowFunctionBody> for ArrowFunctionBody<'a> {
    fn as_concrete(&self) -> crate::expr::ArrowFunctionBody {
        match self {
            ArrowFunctionBody::FunctionBody(ref bod) => {
                crate::expr::ArrowFunctionBody::FunctionBody(bod.as_concrete())
            }
            ArrowFunctionBody::Expr(ref expr) => {
                crate::expr::ArrowFunctionBody::Expr(Box::new(expr.as_concrete()))
            }
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
    pub argument: Option<Box<Expr<'a>>>,
    pub delegate: bool,
}

impl<'a> AsConcrete<crate::expr::YieldExpr> for YieldExpr<'a> {
    fn as_concrete(&self) -> crate::expr::YieldExpr {
        let argument = if let Some(ref a) = self.argument {
            Some(Box::new(a.as_concrete()))
        } else {
            None
        };
        crate::expr::YieldExpr {
            argument,
            delegate: self.delegate,
        }
    }
}

/// A Template literal preceded by a function identifier
/// see [MDN](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Template_literals#Tagged_templates) for more details
#[derive(PartialEq, Debug, Clone)]
pub struct TaggedTemplateExpr<'a> {
    pub tag: Box<Expr<'a>>,
    pub quasi: TemplateLiteral<'a>,
}

impl<'a> AsConcrete<crate::expr::TaggedTemplateExpr> for TaggedTemplateExpr<'a> {
    fn as_concrete(&self) -> crate::expr::TaggedTemplateExpr {
        crate::expr::TaggedTemplateExpr {
            tag: Box::new((*self.tag).as_concrete()),
            quasi: self.quasi.as_concrete(),
        }
    }
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

impl<'a> AsConcrete<crate::expr::TemplateLiteral> for TemplateLiteral<'a> {
    fn as_concrete(&self) -> crate::expr::TemplateLiteral {
        crate::expr::TemplateLiteral {
            quasis: self.quasis.as_concrete(),
            expressions: self.expressions.as_concrete(),
        }
    }
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

impl<'a> AsConcrete<crate::expr::TemplateElement> for TemplateElement<'a> {
    fn as_concrete(&self) -> crate::expr::TemplateElement {
        crate::expr::TemplateElement {
            tail: self.tail,
            cooked: String::from(self.cooked),
            raw: String::from(self.raw),
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
pub struct MetaProperty<'a> {
    pub meta: Identifier<'a>,
    pub property: Identifier<'a>,
}

impl<'a> AsConcrete<crate::expr::MetaProperty> for MetaProperty<'a> {
    fn as_concrete(&self) -> crate::expr::MetaProperty {
        crate::expr::MetaProperty {
            meta: String::from(self.meta),
            property: String::from(self.property),
        }
    }
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

impl<'a> AsConcrete<crate::expr::Literal> for Literal<'a> {
    fn as_concrete(&self) -> crate::expr::Literal {
        match self {
            Literal::Null => crate::Literal::Null,
            Literal::String(ref s) => crate::Literal::String(String::from(*s)),
            Literal::Number(ref n) => crate::Literal::Number(String::from(*n)),
            Literal::Boolean(ref b) => crate::Literal::Boolean(*b),
            Literal::RegEx(ref r) => crate::Literal::RegEx(r.as_concrete()),
            Literal::Template(ref t) => crate::Literal::Template(t.as_concrete()),
        }
    }
}

/// A regular expression literal
#[derive(PartialEq, Debug, Clone)]
pub struct RegEx<'a> {
    pub pattern: &'a str,
    pub flags: &'a str,
}

impl<'a> AsConcrete<crate::expr::RegEx> for RegEx<'a> {
    fn as_concrete(&self) -> crate::expr::RegEx {
        crate::expr::RegEx {
            pattern: String::from(self.pattern),
            flags: String::from(self.flags),
        }
    }
}
