use crate::spanned::pat::Pat;
use crate::spanned::{AssignOp, BinaryOp, LogicalOp, UnaryOp, UpdateOp};
use crate::spanned::{Class, Func, FuncArg, FuncBody, Ident};
use crate::PropKind;

use super::{Node, Slice, SourceLocation};

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
    /// yield a value from inside of a generator function
    Yield(YieldExpr<'a>),
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
        }
    }
}
/// `[a, b, c]`
#[derive(Debug, Clone, PartialEq)]
pub struct ArrayExpr<'a> {
    open_bracket: Slice<'a>,
    elements: Vec<Option<Expr<'a>>>,
    close_bracket: Slice<'a>,
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
    open_brace: Slice<'a>,
    props: Vec<ObjProp<'a>>,
    close_brace: Slice<'a>,
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
    Spread(Expr<'a>),
}

impl<'a> Node for ObjProp<'a> {
    fn loc(&self) -> SourceLocation {
        todo!()
    }
}

/// A single part of an object literal or class
#[derive(Debug, Clone, PartialEq)]
pub struct Prop<'a> {
    pub key: PropKey<'a>,
    pub value: Option<PropValue<'a>>,
    pub kind: PropKind,
    pub method: bool,
    pub computed: bool,
    pub short_hand: bool,
    pub is_static: bool,
}

impl<'a> Node for Prop<'a> {
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

/// An object literal or class property identifier
#[derive(PartialEq, Debug, Clone)]
pub enum PropKey<'a> {
    Lit(Lit<'a>),
    Expr(Expr<'a>),
    Pat(Pat<'a>),
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
}

impl<'a> Node for PropValue<'a> {
    fn loc(&self) -> SourceLocation {
        match self {
            PropValue::Expr(inner) => inner.loc(),
            PropValue::Pat(inner) => inner.loc(),
        }
    }
}

/// An operation that takes one argument
#[derive(PartialEq, Debug, Clone)]
pub struct UnaryExpr<'a> {
    pub operator: UnaryOp<'a>,
    pub prefix: bool,
    pub argument: Box<Expr<'a>>,
}

impl<'a> Node for UnaryExpr<'a> {
    fn loc(&self) -> SourceLocation {
        let (start, end) = if self.prefix {
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
    pub prefix: bool,
}

impl<'a> Node for UpdateExpr<'a> {
    fn loc(&self) -> SourceLocation {
        let (start, end) = if self.prefix {
            (self.operator.loc().start, self.argument.loc().end)
        } else {
            (self.argument.loc().start, self.operator.loc().end)
        };
        SourceLocation { start, end }
    }
}

/// An operation that requires 2 arguments
#[derive(PartialEq, Debug, Clone)]
pub struct BinaryExpr<'a> {
    pub operator: BinaryOp<'a>,
    pub left: Box<Expr<'a>>,
    pub right: Box<Expr<'a>>,
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

impl<'a> Node for AssignExpr<'a> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.left.loc().start,
            end: self.right.loc().end,
        }
    }
}

/// The value being assigned to
#[derive(PartialEq, Debug, Clone)]
pub enum AssignLeft<'a> {
    Pat(Pat<'a>),
    Expr(Box<Expr<'a>>),
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
    pub alternate: Box<Expr<'a>>,
    pub consequent: Box<Expr<'a>>,
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
    pub arguments: Vec<Expr<'a>>,
    pub close_paren: Slice<'a>,
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
    pub open_paren: Slice<'a>,
    pub arguments: Vec<Expr<'a>>,
    pub close_paren: Slice<'a>,
}

impl<'a> Node for NewExpr<'a> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.callee.loc().start,
            end: self.close_paren.loc.end,
        }
    }
}

/// A collection of `Exprs` separated by commas
pub type SequenceExpr<'a> = Vec<Expr<'a>>;

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
    pub open_paren: Slice<'a>,
    pub args: Vec<FuncArg<'a>>,
    pub close_paren: Slice<'a>,
}

impl<'a> Node for ArrowParamPlaceHolder<'a> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.open_paren.loc.start,
            end: self.open_paren.loc.end,
        }
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
    pub open_paren: Option<Slice<'a>>,
    pub id: Option<Ident<'a>>,
    pub params: Vec<FuncArg<'a>>,
    pub close_paren: Option<Slice<'a>>,
    pub body: ArrowFuncBody<'a>,
    pub expression: bool,
    pub generator: bool,
    pub is_async: bool,
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
    pub delegate: bool,
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
    pub open_tick: Slice<'a>,
    pub quasis: Vec<TemplateElement<'a>>,
    pub expressions: Vec<Expr<'a>>,
    pub close_tick: Slice<'a>,
}

impl<'a> Node for TemplateLit<'a> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.open_tick.loc.start,
            end: self.close_tick.loc.end,
        }
    }
}

/// The text part of a `TemplateLiteral`
#[derive(Debug, Clone, PartialEq)]
pub struct TemplateElement<'a> {
    pub tail: bool,
    /// The non-quoted version
    pub cooked: Slice<'a>,
    /// The quoted version
    pub raw: Slice<'a>,
}

impl<'a> Node for TemplateElement<'a> {
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
pub struct MetaProp<'a> {
    pub meta: Ident<'a>,
    pub property: Ident<'a>,
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
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub struct StringLit<'a> {
    pub open_quote: Slice<'a>,
    pub content: Slice<'a>,
    pub close_quote: Slice<'a>,
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
