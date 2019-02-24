pub mod decl;
pub mod expr;
pub mod pat;
pub mod ref_tree;
pub mod stmt;
use decl::Decl;
use expr::{Expr, Literal, Property};
use pat::Pat;
use stmt::Stmt;

pub type Identifier = String;
/// A fully parsed javascript program.
///
/// It is essentially a collection of `ProgramPart`s
/// with a flag denoting if the representation is
/// a ES6 Mod or a Script.
#[derive(PartialEq, Debug)]
pub enum Program {
    /// An ES6 Mod
    Mod(Vec<ProgramPart>),
    /// Not an ES6 Mod
    Script(Vec<ProgramPart>),
}

/// A single part of a Javascript program.
/// This will be either a Directive, Decl or a Stmt
#[derive(PartialEq, Debug, Clone)]
pub enum ProgramPart {
    /// A Directive like `'use strict';`
    Dir(Dir),
    /// A variable, function or module declaration
    Decl(Decl),
    /// Any other kind of statement
    Stmt(Stmt),
}

/// pretty much always `'use strict'`, this can appear at the
/// top of a file or function
#[derive(PartialEq, Debug, Clone)]
pub struct Dir {
    pub expr: Literal,
    pub dir: String,
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
pub struct Function {
    pub id: Option<String>,
    pub params: Vec<FunctionArg>,
    pub body: FunctionBody,
    pub generator: bool,
    pub is_async: bool,
}

/// A single function argument from a function signature
#[derive(PartialEq, Debug, Clone)]
pub enum FunctionArg {
    Expr(Expr),
    Pat(Pat),
}

/// The block statement that makes up the function's body
pub type FunctionBody = Vec<ProgramPart>;
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
pub struct Class {
    pub id: Option<Identifier>,
    pub super_class: Option<Box<Expr>>,
    pub body: Vec<Property>,
}

#[cfg(test)]
mod tests {}
