use crate::ref_tree::{Function, Class, Identifier};
use crate::ref_tree::expr::{Expr, Literal};
use crate::ref_tree::pat::Pat;

/// The declaration of a variable, function, class, import or export
#[derive(PartialEq, Debug, Clone)]
pub enum Decl<'a> {
    /// A variable declaration
    /// ```js
    /// var x, b;
    /// let y, a = 0;
    /// const q = 100
    /// ```
    Variable(VariableKind, Vec<VariableDecl<'a>>),
    /// A function declaration
    /// ```js
    /// function thing() {}
    /// ```
    Function(Function<'a>),
    /// A class declaration
    /// ```js
    /// class Thing {}
    /// ```
    Class(Class<'a>),
    /// An import declaration
    /// ```js
    /// import * as moment from 'moment';
    /// import Thing, {thing} from 'stuff';
    /// ```
    Import(Box<ModImport<'a>>),
    /// An export declaration
    /// ```js
    /// export function thing() {}
    /// ```
    Export(Box<ModExport<'a>>),
}

/// The identifier and optional value of a variable declaration
#[derive(PartialEq, Debug, Clone)]
pub struct VariableDecl<'a> {
    pub id: Pat<'a>,
    pub init: Option<Expr<'a>>,
}

/// The kind of variable being defined (`var`/`let`/`const`)
#[derive(PartialEq, Clone, Debug, Copy)]
pub enum VariableKind {
    Var,
    Let,
    Const,
}

/// A module declaration, This would only be available
/// in an ES Mod, it would be either an import or
/// export at the top level
#[derive(PartialEq, Debug, Clone)]
pub enum ModDecl<'a> {
    Import(ModImport<'a>),
    Export(ModExport<'a>),
}

/// A declaration that imports exported
/// members of another module
///
/// ```js
/// import {Thing} from './stuff.js';
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct ModImport<'a> {
    pub specifiers: Vec<ImportSpecifier<'a>>,
    pub source: Literal<'a>,
}

/// The name of the thing being imported
#[derive(PartialEq, Debug, Clone)]
pub enum ImportSpecifier<'a> {
    /// A specifier in curly braces, this might
    /// have a local alias
    ///
    /// ```js
    /// import {Thing} from './stuff.js';
    /// import {People as Persons} from './places.js';
    /// ```
    Normal(Identifier<'a>, Option<Identifier<'a>>),
    /// A specifier that has been exported with the
    /// default keyword, this should not be wrapped in
    /// curly braces.
    /// ```js
    /// import DefaultThing from './stuff/js';
    /// ```
    Default(Identifier<'a>),
    /// Import all exported members from a module
    /// in a namespace.
    ///
    /// ```js
    /// import * as Moment from 'moment.js';
    /// ```
    Namespace(Identifier<'a>),
}

/// Something exported from this module
#[derive(PartialEq, Debug, Clone)]
pub enum ModExport<'a> {
    /// ```js
    /// export default function() {};
    /// //or
    /// export default 1;
    /// ```
    Default(DefaultExportDecl<'a>),
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
    Named(NamedExportDecl<'a>),
    /// ```js
    /// export * from 'mod';
    /// ```
    All(Literal<'a>),
}

/// An export that has a name
/// ```js
/// export function thing() {}
/// export {stuff} from 'place';
#[derive(PartialEq, Debug, Clone)]
pub enum NamedExportDecl<'a> {
    Decl(Decl<'a>),
    Specifier(Vec<ExportSpecifier<'a>>, Option<Literal<'a>>),
}
/// A default export
/// ```js
/// export default class Thing {}
/// ```
#[derive(PartialEq, Debug, Clone)]
pub enum DefaultExportDecl<'a> {
    Decl(Decl<'a>),
    Expr(Expr<'a>),
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
pub struct ExportSpecifier<'a> {
    pub local: Identifier<'a>,
    pub exported: Option<Identifier<'a>>,
}