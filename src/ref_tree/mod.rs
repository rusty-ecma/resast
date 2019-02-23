pub mod decl;
pub mod expr;
pub mod pat;
pub mod stmt;
use self::decl::Decl;
use self::expr::{Expr, Property, Literal};
use self::stmt::Stmt;
use self::pat::Pat;

pub type Identifier<'a> = &'a str;
/// A fully parsed javascript program.
///
/// It is essentially a collection of `ProgramPart`s
/// with a flag denoting if the representation is
/// a ES6 Mod or a Script.
#[derive(PartialEq, Debug)]
pub enum Program<'a> {
    /// An ES6 Mod
    Mod(Vec<ProgramPart<'a>>),
    /// Not an ES6 Mod
    Script(Vec<ProgramPart<'a>>),
}

/// A single part of a Javascript program.
/// This will be either a Directive, Decl or a Stmt
#[derive(PartialEq, Debug, Clone)]
pub enum ProgramPart<'a> {
    /// A Directive like `'use strict';`
    Dir(Dir<'a>),
    /// A variable, function or module declaration
    Decl(Decl<'a>),
    /// Any other kind of statement
    Stmt(Stmt<'a>),
}

/// pretty much always `'use strict'`, this can appear at the
/// top of a file or function
#[derive(PartialEq, Debug, Clone)]
pub struct Dir<'a> {
    pub expr: Literal<'a>,
    pub dir: &'a str,
}

/// A function, this will be part of either a function
/// declaration (ID is required) or a function expression
/// (ID is optional)
/// ```js
/// //function declaration
/// function thing() {}
/// //function expressions
/// var x = function() {}
/// let y = function q() {}
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct Function<'a> {
    pub id: Option<Identifier<'a>>,
    pub params: Vec<FunctionArg<'a>>,
    pub body: FunctionBody<'a>,
    pub generator: bool,
    pub is_async: bool,
}

/// A single function argument from a function signature
#[derive(PartialEq, Debug, Clone)]
pub enum FunctionArg<'a> {
    Expr(Expr<'a>),
    Pat(Pat<'a>),
}

/// The block statement that makes up the function's body
pub type FunctionBody<'a> = Vec<ProgramPart<'a>>;
/// A way to declare object templates
/// ```js
/// class Thing {
///     constructor() {
///         this._a = 0;
///     }
///     stuff() {
///         return 'stuff'
///     }
///     set a(value) {
///         if (value > 100) {
///             this._a = 0;
///         } else {
///             this._a = value;
///         }
///     }
///     get a() {
///         return this._a;
///     }
/// }
/// let y = class {
///     constructor() {
///         this.a = 100;
///     }
/// }
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct Class<'a> {
    pub id: Option<Identifier<'a>>,
    pub super_class: Option<Box<Expr<'a>>>,
    pub body: Vec<Property<'a>>,
}

#[cfg(test)]
mod tests {}
