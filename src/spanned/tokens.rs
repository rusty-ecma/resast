//! This modules contains a collection of discrete tokens

use crate::spanned::{Node, Position, SourceLocation};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

macro_rules! impl_token {
    ($what:ty, $s:expr) => {
        impl Token for $what {
            fn as_str(&self) -> &str {
                $s
            }
            fn start(&self) -> Position {
                self.0
            }
            fn end(&self) -> Position {
                self.0 + ($s.len() as u32)
            }
        }
        impl std::convert::From<Position> for $what {
            fn from(other: Position) -> Self {
                Self(other)
            }
        }
        impl std::convert::From<$what> for Position {
            fn from(other: $what) -> Position {
                other.0
            }
        }
        impl std::cmp::PartialEq<Position> for $what {
            fn eq(&self, other: &Position) -> bool {
                self.0 == *other
            }
        }
    };
}

/// Defines a token struct and implements the Token trait for the newly defined type.
/// The output of this macro produces something like the following.
/// ```rust
/// # use resast::spanned::{Position, tokens::Token};
/// #[derive(Debug, Clone, Copy, PartialEq)]
/// #[doc = "as"]
/// pub struct As(Position);
///
/// impl Token for As {
///     fn as_str(&self) -> &str {
///         "as"
///     }
///     fn start(&self) -> Position {
///         self.0
///     }
///     fn end(&self) -> Position {
///         self.0 + ("as".len() as u32)
///     }
/// }
/// impl std::convert::From<Position> for As {
///     fn from(other: Position) -> Self {
///         Self(other)
///     }
/// }
/// impl std::convert::Into<Position> for As {
///     fn into(self) -> Position {
///         self.0
///     }
/// }
/// impl std::cmp::PartialEq<Position> for As {
///     fn eq(&self, other: &Position) -> bool {
///         self.0 == *other
///     }
/// }
/// ```
macro_rules! define_token {
    ($name:ident, $s:expr) => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
        #[doc = $s]
        pub struct $name(Position);
        impl_token!($name, $s);
    };
}

pub trait Token {
    fn as_str(&self) -> &str;
    fn start(&self) -> Position;
    fn end(&self) -> Position;
}

impl<T> Node for T
where
    T: Token,
{
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.start(),
            end: self.end(),
        }
    }
}

// Keywords
define_token!(As, "as");
define_token!(Async, "async");
define_token!(Await, "await");
define_token!(Break, "break");
define_token!(Case, "case");
define_token!(Catch, "catch");
define_token!(Class, "class");
define_token!(Const, "const");
define_token!(Continue, "continue");
define_token!(Debugger, "debugger");
define_token!(Default, "default");
define_token!(Delete, "delete");
define_token!(Do, "do");
define_token!(Else, "else");
define_token!(Export, "export");
define_token!(Extends, "extends");
define_token!(Finally, "finally");
define_token!(From, "from");
define_token!(Get, "get");
define_token!(False, "false");
define_token!(For, "for");
define_token!(Function, "function");
define_token!(If, "if");
define_token!(Import, "import");
define_token!(In, "in");
define_token!(InstanceOf, "instanceof");
define_token!(Let, "let");
define_token!(New, "new");
define_token!(Null, "null");
define_token!(Of, "of");
define_token!(Return, "return");
define_token!(Set, "set");
define_token!(Static, "static");
define_token!(Super, "super");
define_token!(Switch, "switch");
define_token!(This, "this");
define_token!(Throw, "throw");
define_token!(True, "true");
define_token!(Try, "try");
define_token!(TypeOf, "typeof");
define_token!(Var, "var");
define_token!(Void, "void");
define_token!(While, "while");
define_token!(With, "with");
define_token!(Yield, "yield");

// Punctuation
define_token!(Ampersand, "&");
define_token!(AmpersandEqual, "&=");
define_token!(Asterisk, "*");
define_token!(AsteriskEqual, "*=");
define_token!(BackTick, "`");
define_token!(Bang, "!");
define_token!(BangEqual, "!=");
define_token!(BangDoubleEqual, "!==");
define_token!(Caret, "^");
define_token!(CaretEqual, "^=");
define_token!(CloseParen, ")");
define_token!(CloseBrace, "}");
define_token!(CloseBracket, "]");
define_token!(Colon, ":");
define_token!(Comma, ",");
define_token!(DoubleAmpersand, "&&");
define_token!(DoubleAmpersandEqual, "&&=");
define_token!(DoubleAsterisk, "**");
define_token!(DoubleAsteriskEqual, "**=");
define_token!(DoubleEqual, "==");
define_token!(DollarSignOpenBrace, "${");
define_token!(DoubleGreaterThan, ">>");
define_token!(DoubleGreaterThanEqual, ">>=");
define_token!(DoubleLessThan, "<<");
define_token!(DoubleLessThanEqual, "<<=");
define_token!(DoublePipe, "||");
define_token!(DoublePipeEqual, "||=");
define_token!(DoubleQuestionmark, "??");
define_token!(DoubleQuestionmarkEqual, "??=");
define_token!(DoubleQuote, "\"");
define_token!(Ellipsis, "...");
define_token!(Equal, "=");
define_token!(FatArrow, "=>");
define_token!(ForwardSlash, "/");
define_token!(ForwardSlashEqual, "/=");
define_token!(GreaterThan, ">");
define_token!(GreaterThanEqual, ">=");
define_token!(LessThan, "<");
define_token!(LessThanEqual, "<=");
define_token!(Minus, "-");
define_token!(MinusEqual, "-=");
define_token!(OpenBrace, "{");
define_token!(OpenBracket, "[");
define_token!(OpenParen, "(");
define_token!(Percent, "%");
define_token!(PercentEqual, "%=");
define_token!(Period, ".");
define_token!(Pipe, "|");
define_token!(PipeEqual, "|=");
define_token!(Plus, "+");
define_token!(PlusEqual, "+=");
define_token!(QuestionMark, "?");
define_token!(QuestionMarkDot, "?.");
define_token!(Semicolon, ";");
define_token!(SingleQuote, "'");
define_token!(Tilde, "~");
define_token!(TripleEqual, "===");
define_token!(TripleGreaterThan, ">>>");
define_token!(TripleGreaterThanEqual, ">>>=");

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Quote {
    Double(DoubleQuote),
    Single(SingleQuote),
}

impl Token for Quote {
    fn as_str(&self) -> &str {
        match self {
            Quote::Double(inner) => inner.as_str(),
            Quote::Single(inner) => inner.as_str(),
        }
    }

    fn start(&self) -> Position {
        match self {
            Quote::Double(inner) => inner.start(),
            Quote::Single(inner) => inner.start(),
        }
    }

    fn end(&self) -> Position {
        match self {
            Quote::Double(inner) => inner.end(),
            Quote::Single(inner) => inner.end(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum QuasiQuote {
    BackTick(BackTick),
    CloseBrace(CloseBrace),
    OpenBrace(DollarSignOpenBrace),
}

impl Token for QuasiQuote {
    fn as_str(&self) -> &str {
        match self {
            Self::BackTick(inner) => inner.as_str(),
            Self::CloseBrace(inner) => inner.as_str(),
            Self::OpenBrace(inner) => inner.as_str(),
        }
    }

    fn start(&self) -> Position {
        match self {
            Self::BackTick(inner) => inner.start(),
            Self::CloseBrace(inner) => inner.start(),
            Self::OpenBrace(inner) => inner.start(),
        }
    }

    fn end(&self) -> Position {
        match self {
            Self::BackTick(inner) => inner.end(),
            Self::CloseBrace(inner) => inner.end(),
            Self::OpenBrace(inner) => inner.end(),
        }
    }
}

/// The available operators for assignment Exprs
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum AssignOp {
    Equal(Equal),
    PlusEqual(PlusEqual),
    MinusEqual(MinusEqual),
    TimesEqual(AsteriskEqual),
    DivEqual(ForwardSlashEqual),
    ModEqual(PercentEqual),
    LeftShiftEqual(DoubleLessThanEqual),
    RightShiftEqual(DoubleGreaterThanEqual),
    UnsignedRightShiftEqual(TripleGreaterThanEqual),
    OrEqual(PipeEqual),
    XOrEqual(CaretEqual),
    AndEqual(AmpersandEqual),
    PowerOfEqual(DoubleAsteriskEqual),
    DoubleAmpersandEqual(DoubleAmpersandEqual),
    DoublePipeEqual(DoublePipeEqual),
    DoubleQuestionmarkEqual(DoubleQuestionmarkEqual),
}

impl Node for AssignOp {
    fn loc(&self) -> SourceLocation {
        match self {
            AssignOp::Equal(tok) => tok.loc(),
            AssignOp::PlusEqual(tok) => tok.loc(),
            AssignOp::MinusEqual(tok) => tok.loc(),
            AssignOp::TimesEqual(tok) => tok.loc(),
            AssignOp::DivEqual(tok) => tok.loc(),
            AssignOp::ModEqual(tok) => tok.loc(),
            AssignOp::LeftShiftEqual(tok) => tok.loc(),
            AssignOp::RightShiftEqual(tok) => tok.loc(),
            AssignOp::UnsignedRightShiftEqual(tok) => tok.loc(),
            AssignOp::OrEqual(tok) => tok.loc(),
            AssignOp::XOrEqual(tok) => tok.loc(),
            AssignOp::AndEqual(tok) => tok.loc(),
            AssignOp::PowerOfEqual(tok) => tok.loc(),
            AssignOp::DoubleAmpersandEqual(tok) => tok.loc(),
            AssignOp::DoublePipeEqual(tok) => tok.loc(),
            AssignOp::DoubleQuestionmarkEqual(tok) => tok.loc(),
        }
    }
}

/// The available logical operators
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum LogicalOp {
    Or(DoublePipe),
    And(DoubleAmpersand),
    NullishCoalescing(DoubleQuestionmark),
}

impl Node for LogicalOp {
    fn loc(&self) -> SourceLocation {
        match self {
            LogicalOp::Or(tok) => tok.loc(),
            LogicalOp::And(tok) => tok.loc(),
            LogicalOp::NullishCoalescing(tok) => tok.loc(),
        }
    }
}

/// The available operations for `Binary` Exprs
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum BinaryOp {
    Equal(DoubleEqual),
    NotEqual(BangEqual),
    StrictEqual(TripleEqual),
    StrictNotEqual(BangDoubleEqual),
    LessThan(LessThan),
    GreaterThan(GreaterThan),
    LessThanEqual(LessThanEqual),
    GreaterThanEqual(GreaterThanEqual),
    LeftShift(DoubleLessThan),
    RightShift(DoubleGreaterThan),
    UnsignedRightShift(TripleGreaterThan),
    Plus(Plus),
    Minus(Minus),
    Times(Asterisk),
    Over(ForwardSlash),
    Mod(Percent),
    Or(Pipe),
    XOr(Caret),
    And(Ampersand),
    In(In),
    InstanceOf(InstanceOf),
    PowerOf(DoubleAsterisk),
}

impl Node for BinaryOp {
    fn loc(&self) -> SourceLocation {
        match self {
            BinaryOp::Equal(tok) => tok.loc(),
            BinaryOp::NotEqual(tok) => tok.loc(),
            BinaryOp::StrictEqual(tok) => tok.loc(),
            BinaryOp::StrictNotEqual(tok) => tok.loc(),
            BinaryOp::LessThan(tok) => tok.loc(),
            BinaryOp::GreaterThan(tok) => tok.loc(),
            BinaryOp::LessThanEqual(tok) => tok.loc(),
            BinaryOp::GreaterThanEqual(tok) => tok.loc(),
            BinaryOp::LeftShift(tok) => tok.loc(),
            BinaryOp::RightShift(tok) => tok.loc(),
            BinaryOp::UnsignedRightShift(tok) => tok.loc(),
            BinaryOp::Plus(tok) => tok.loc(),
            BinaryOp::Minus(tok) => tok.loc(),
            BinaryOp::Times(tok) => tok.loc(),
            BinaryOp::Over(tok) => tok.loc(),
            BinaryOp::Mod(tok) => tok.loc(),
            BinaryOp::Or(tok) => tok.loc(),
            BinaryOp::XOr(tok) => tok.loc(),
            BinaryOp::And(tok) => tok.loc(),
            BinaryOp::In(tok) => tok.loc(),
            BinaryOp::InstanceOf(tok) => tok.loc(),
            BinaryOp::PowerOf(tok) => tok.loc(),
        }
    }
}
define_token!(DoublePlus, "++");
define_token!(DoubleMinus, "--");
/// `++` or `--`
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum UpdateOp {
    Increment(DoublePlus),
    Decrement(DoubleMinus),
}

impl Node for UpdateOp {
    fn loc(&self) -> SourceLocation {
        match self {
            Self::Increment(tok) => tok.loc(),
            Self::Decrement(tok) => tok.loc(),
        }
    }
}

/// The allowed operators for an Expr
/// to be `Unary`
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum UnaryOp {
    Minus(Minus),
    Plus(Plus),
    Not(Bang),
    Tilde(Tilde),
    TypeOf(TypeOf),
    Void(Void),
    Delete(Delete),
}

impl Node for UnaryOp {
    fn loc(&self) -> SourceLocation {
        match self {
            Self::Minus(tok) => tok.loc(),
            Self::Plus(tok) => tok.loc(),
            Self::Not(tok) => tok.loc(),
            Self::Tilde(tok) => tok.loc(),
            Self::TypeOf(tok) => tok.loc(),
            Self::Void(tok) => tok.loc(),
            Self::Delete(tok) => tok.loc(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum SwitchCaseKeyword {
    Case(Case),
    Default(Default),
}

impl Token for SwitchCaseKeyword {
    fn as_str(&self) -> &str {
        match self {
            SwitchCaseKeyword::Case(inner) => inner.as_str(),
            SwitchCaseKeyword::Default(inner) => inner.as_str(),
        }
    }

    fn start(&self) -> Position {
        match self {
            SwitchCaseKeyword::Case(inner) => inner.start(),
            SwitchCaseKeyword::Default(inner) => inner.start(),
        }
    }

    fn end(&self) -> Position {
        match self {
            SwitchCaseKeyword::Case(inner) => inner.end(),
            SwitchCaseKeyword::Default(inner) => inner.end(),
        }
    }
}
