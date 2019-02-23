use crate::{Function, Class, Identifier};
use crate::expression::{Expression, Literal};
use crate::pattern::Pattern;

/// The declaration of a variable, function, class, import or export
#[derive(PartialEq, Debug, Clone)]
pub enum Declaration {
    /// A variable declaration
    /// ```js
    /// var x, b;
    /// let y, a = 0;
    /// const q = 100
    /// ```
    Variable(VariableKind, Vec<VariableDecl>),
    /// A function declaration
    /// ```js
    /// function thing() {}
    /// ```
    Function(Function),
    /// A class declaration
    /// ```js
    /// class Thing {}
    /// ```
    Class(Class),
    /// An import declaration
    /// ```js
    /// import * as moment from 'moment';
    /// import Thing, {thing} from 'stuff';
    /// ```
    Import(Box<ModuleImport>),
    /// An export declaration
    /// ```js
    /// export function thing() {}
    /// ```
    Export(Box<ModuleExport>),
}

/// The identifier and optional value of a variable declaration
#[derive(PartialEq, Debug, Clone)]
pub struct VariableDecl {
    pub id: Pattern,
    pub init: Option<Expression>,
}

/// The kind of variable being defined (`var`/`let`/`const`)
#[derive(PartialEq, Clone, Debug, Copy)]
pub enum VariableKind {
    Var,
    Let,
    Const,
}

/// A module declaration, This would only be available
/// in an ES Module, it would be either an import or
/// export at the top level
#[derive(PartialEq, Debug, Clone)]
pub enum ModuleDecl {
    Import(ModuleImport),
    Export(ModuleExport),
}

/// A declaration that imports exported
/// members of another module
///
/// ```js
/// import {Thing} from './stuff.js';
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct ModuleImport {
    pub specifiers: Vec<ImportSpecifier>,
    pub source: Literal,
}

/// The name of the thing being imported
#[derive(PartialEq, Debug, Clone)]
pub enum ImportSpecifier {
    /// A specifier in curly braces, this might
    /// have a local alias
    ///
    /// ```js
    /// import {Thing} from './stuff.js';
    /// import {People as Persons} from './places.js';
    /// ```
    Normal(Identifier, Option<Identifier>),
    /// A specifier that has been exported with the
    /// default keyword, this should not be wrapped in
    /// curly braces.
    /// ```js
    /// import DefaultThing from './stuff/js';
    /// ```
    Default(Identifier),
    /// Import all exported members from a module
    /// in a namespace.
    ///
    /// ```js
    /// import * as Moment from 'moment.js';
    /// ```
    Namespace(Identifier),
}

/// Something exported from this module
#[derive(PartialEq, Debug, Clone)]
pub enum ModuleExport {
    /// ```js
    /// export default function() {};
    /// //or
    /// export default 1;
    /// ```
    Default(DefaultExportDecl),
    ///```js
    /// export {foo} from 'mod';
    /// //or
    /// export {foo as bar} from 'mod';
    /// //or
    /// export var foo = 1;
    /// //or
    /// export function bar() {
    /// }
    /// ```
    Named(NamedExportDecl),
    /// ```js
    /// export * from 'mod';
    /// ```
    All(Literal),
}

/// An export that has a name
/// ```js
/// export function thing() {}
/// export {stuff} from 'place';
#[derive(PartialEq, Debug, Clone)]
pub enum NamedExportDecl {
    Decl(Declaration),
    Specifier(Vec<ExportSpecifier>, Option<Literal>),
}
/// A default export
/// ```js
/// export default class Thing {}
/// ```
#[derive(PartialEq, Debug, Clone)]
pub enum DefaultExportDecl {
    Decl(Declaration),
    Expr(Expression),
}
/// The name of the thing being exported
/// this might include an alias
/// ```js
/// //no-alias
/// export {Thing} from 'place';
/// //aliased
/// export {Stuff as NewThing} from 'place'
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct ExportSpecifier {
    pub local: Identifier,
    pub exported: Option<Identifier>,
}