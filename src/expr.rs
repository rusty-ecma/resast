use crate::pat::Pat;
use crate::{AssignOp, BinaryOp, LogicalOp, PropKind, UnaryOp, UpdateOp, SourceText};
use crate::{Class, Func, FuncArg, FuncBody, Ident};
/// A slightly more granular program part that a statement
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    all(feature = "serde", not(feature = "esprima")),
    derive(Deserialize, Serialize)
)]
#[cfg_attr(all(feature = "serde", feature = "esprima"), derive(Deserialize))]
#[cfg_attr(all(feature = "serde", feature = "esprima"), serde(untagged))]
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

// impl<'a> Expr<&'a str> {
//     pub fn ident_from(s: &'a str) -> Self {
//         Expr::Ident(Ident::from(s))
//     }
// }

/// `[a, b, c]`
pub type ArrayExpr<T> = Vec<Option<Expr<T>>>;
/// `{a: 'b', c, ...d}`
pub type ObjExpr<T> = Vec<ObjProp<T>>;
/// A single part of an object literal
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
#[cfg_attr(all(feature = "serde", feature = "esprima"), serde(untagged))]
pub enum ObjProp<T> {
    Prop(Prop<T>),
    Spread(Expr<T>),
}

/// A single part of an object literal or class
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    all(feature = "serde", not(feature = "esprima")),
    derive(Deserialize, Serialize)
)]
#[cfg_attr(all(feature = "serde", feature = "esprima"), derive(Deserialize))]
pub struct Prop<T> {
    pub key: PropKey<T>,
    pub value: PropValue<T>,
    pub kind: PropKind,
    pub method: bool,
    pub computed: bool,
    pub short_hand: bool,
    pub is_static: bool,
}

/// An object literal or class property identifier
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
#[cfg_attr(all(feature = "serde", feature = "esprima"), serde(untagged))]
pub enum PropKey<T> {
    Lit(Lit<T>),
    Expr(Expr<T>),
    Pat(Pat<T>),
}

/// The value of an object literal or class property
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
#[cfg_attr(all(feature = "serde", feature = "esprima"), serde(untagged))]
pub enum PropValue<T> {
    Expr(Expr<T>),
    Pat(Pat<T>),
    None,
}

/// An operation that takes one argument
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub struct UnaryExpr<T> {
    pub operator: UnaryOp,
    pub prefix: bool,
    pub argument: Box<Expr<T>>,
}

/// Increment or decrementing a value
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub struct UpdateExpr<T> {
    pub operator: UpdateOp,
    pub argument: Box<Expr<T>>,
    pub prefix: bool,
}

/// An operation that requires 2 arguments
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub struct BinaryExpr<T> {
    pub operator: BinaryOp,
    pub left: Box<Expr<T>>,
    pub right: Box<Expr<T>>,
}

/// An assignment or update + assignment operation
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub struct AssignExpr<T> {
    pub operator: AssignOp,
    pub left: AssignLeft<T>,
    pub right: Box<Expr<T>>,
}

/// The value being assigned to
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
#[cfg_attr(all(feature = "serde", feature = "esprima"), serde(untagged))]
pub enum AssignLeft<T> {
    Pat(Pat<T>),
    Expr(Box<Expr<T>>),
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

/// The body portion of an arrow function can be either an expression or a block of statements
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub enum ArrowFuncBody<T> {
    FuncBody(FuncBody<T>),
    Expr(Box<Expr<T>>),
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

/// A Template literal preceded by a function identifier
/// see [MDN](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Template_literals#Tagged_templates) for more details
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub struct TaggedTemplateExpr<T> {
    pub tag: Box<Expr<T>>,
    pub quasi: TemplateLit<T>,
}

/// A template string literal
/// ```js
/// `I own ${0} birds`;
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    all(feature = "serde", not(feature = "esprima")),
    derive(Deserialize, Serialize)
)]
#[cfg_attr(all(feature = "serde", feature = "esprima"), derive(Deserialize))]
pub struct TemplateLit<T> {
    pub quasis: Vec<TemplateElement<T>>,
    pub expressions: Vec<Expr<T>>,
}

/// The text part of a `TemplateLiteral`
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    all(feature = "serde", not(feature = "esprima")),
    derive(Deserialize, Serialize)
)]
#[cfg_attr(all(feature = "serde", feature = "esprima"), derive(Deserialize))]
pub struct TemplateElement<T> {
    pub tail: bool,
    /// The non-quoted version
    pub cooked: SourceText<T>,
    /// The quoted version
    pub raw: SourceText<T>,
}

impl<T> TemplateElement<T> {
    pub fn from(tail: bool, cooked: T, raw: T) -> TemplateElement<T> {
        Self {
            tail,
            cooked: SourceText(cooked),
            raw: SourceText(raw),
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
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub struct MetaProp<T> {
    pub meta: Ident<T>,
    pub property: Ident<T>,
}

/// A literal value
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    all(feature = "serde", not(feature = "esprima")),
    derive(Deserialize, Serialize)
)]
#[cfg_attr(all(feature = "serde", feature = "esprima"), derive(Deserialize))]
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
    Number(SourceText<T>),
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

impl<T> Lit<T> {
    pub fn number_from(s: T) -> Self {
        Lit::Number(SourceText(s))
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
    Double(SourceText<T>),
    Single(SourceText<T>),
}

impl<T> StringLit<T> {
    pub fn double_from(s: T) -> StringLit<T> {
        StringLit::Double(SourceText(s))
    }
    pub fn single_from(s: T) -> StringLit<T> {
        StringLit::Single(SourceText(s))
    }
    // pub fn clone_inner(&self) -> Cow<T> {
    //     match self {
    //         StringLit::Single(ref s) => s.clone(),
    //         StringLit::Double(ref s) => s.clone(),
    //     }
    // }
    // pub fn inner_matches(&self, o: &str) -> bool {
    //     match self {
    //         StringLit::Single(ref s) => o.eq(s.as_ref()),
    //         StringLit::Double(ref d) => o.eq(d.as_ref()),
    //     }
    // }
}
/// A regular expression literal
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
#[cfg_attr(all(feature = "serialization"), serde(rename_all = "camelCase"))]
pub struct RegEx<T> {
    pub pattern: SourceText<T>,
    pub flags: Option<SourceText<T>>,
}

impl<T> RegEx<T> {
    pub fn from(p: T, f: Option<T>) -> Self {
        RegEx {
            pattern: SourceText(p),
            flags: f.map(SourceText),
        }
    }
}
