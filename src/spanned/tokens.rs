//! This modules contains a collection of discrete tokens

use crate::spanned::{Node, Position, SourceLocation};

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
        impl std::convert::Into<Position> for $what {
            fn into(self) -> Position {
                self.0
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
define_token!(Var, "var");
define_token!(While, "while");
define_token!(With, "with");
define_token!(Yield, "yield");

// Punctuation
define_token!(Asterisk, "*");
define_token!(BackTick, "`");
define_token!(CloseParen, ")");
define_token!(CloseBrace, "}");
define_token!(CloseBracket, "]");
define_token!(Colon, ":");
define_token!(Comma, ",");
define_token!(DollarSignOpenBrace, "${");
define_token!(DoubleQuote, "\"");
define_token!(Ellipsis, "...");
define_token!(Equal, "=");
define_token!(FatArrow, "=>");
define_token!(ForwardSlash, "/");
define_token!(OpenBrace, "{");
define_token!(OpenBracket, "[");
define_token!(OpenParen, "(");
define_token!(Period, ".");
define_token!(QuestionMark, "?");
define_token!(Semicolon, ";");
define_token!(SingleQuote, "'");

#[derive(Debug, Clone, Copy, PartialEq)]
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
