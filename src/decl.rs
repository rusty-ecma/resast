use crate::expr::{Expr, Lit};
use crate::pat::Pat;
use crate::VarKind;
use crate::{Class, Func, Ident};

/// The declaration of a variable, function, class, import or export
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    all(feature = "serde", not(feature = "esprima")),
    derive(Deserialize, Serialize)
)]
#[cfg_attr(all(feature = "serde", feature = "esprima"), derive(Deserialize))]
pub enum Decl<T> {
    /// A variable declaration
    /// ```js
    /// var x, b;
    /// let y, a = 0;
    /// const q = 100
    /// ```
    Var(VarKind, Vec<VarDecl<T>>),
    /// A function declaration
    /// ```js
    /// function thing() {}
    /// ```
    Func(Func<T>),
    /// A class declaration
    /// ```js
    /// class Thing {}
    /// ```
    Class(Class<T>),
    /// An import declaration
    /// ```js
    /// import * as moment from 'moment';
    /// import Thing, {thing} from 'stuff';
    /// ```
    Import(Box<ModImport<T>>),
    /// An export declaration
    /// ```js
    /// export function thing() {}
    /// ```
    Export(Box<ModExport<T>>),
}

/// The identifier and optional value of a variable declaration
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    all(feature = "serde", not(feature = "esprima")),
    derive(Deserialize, Serialize)
)]
#[cfg_attr(all(feature = "serde", feature = "esprima"), derive(Deserialize))]
pub struct VarDecl<T> {
    pub id: Pat<T>,
    pub init: Option<Expr<T>>,
}

/// A module declaration, This would only be available
/// in an ES Mod, it would be either an import or
/// export at the top level
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub enum ModDecl<T> {
    Import(ModImport<T>),
    Export(ModExport<T>),
}

/// A declaration that imports exported
/// members of another module
///
/// ```js
/// import {Thing} from './stuff.js';
/// ```
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub struct ModImport<T> {
    pub specifiers: Vec<ImportSpecifier<T>>,
    pub source: Lit<T>,
}

/// The name of the thing being imported
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    all(feature = "serde", not(feature = "esprima")),
    derive(Deserialize, Serialize)
)]
#[cfg_attr(all(feature = "serde", feature = "esprima"), derive(Deserialize))]
pub enum ImportSpecifier<T> {
    /// A specifier in curly braces, this might
    /// have a local alias
    ///
    /// ```js
    /// import {Thing} from './stuff.js';
    /// import {People as Persons} from './places.js';
    /// ```
    Normal(Vec<NormalImportSpec<T>>),
    /// A specifier that has been exported with the
    /// default keyword, this should not be wrapped in
    /// curly braces.
    /// ```js
    /// import DefaultThing from './stuff/js';
    /// ```
    Default(Ident<T>),
    /// Import all exported members from a module
    /// in a namespace.
    ///
    /// ```js
    /// import * as Moment from 'moment.js';
    /// ```
    Namespace(Ident<T>),
}
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub struct NormalImportSpec<T> {
    pub local: Ident<T>,
    pub imported: Ident<T>,
}

/// Something exported from this module
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    all(feature = "serde", not(feature = "esprima")),
    derive(Deserialize, Serialize)
)]
#[cfg_attr(all(feature = "serde", feature = "esprima"), derive(Deserialize))]
pub enum ModExport<T> {
    /// ```js
    /// export default function() {};
    /// //or
    /// export default 1;
    /// ```
    Default(DefaultExportDecl<T>),
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
    Named(NamedExportDecl<T>),
    /// ```js
    /// export * from 'mod';
    /// ```
    All {
        alias: Option<Ident<T>>,
        name: Lit<T>,
    },
}

// pub struct NamedExportDecl<T> {
//     decl: Option<Box<Decl<T>>>,
//     specs: Vec<ExportSpecifier<T>>,
//     source: Option<Cow<T, str>>
// }
/// An export that has a name
/// ```js
/// export function thing() {}
/// export {stuff} from 'place';
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub enum NamedExportDecl<T> {
    Decl(Decl<T>),
    Specifier(Vec<ExportSpecifier<T>>, Option<Lit<T>>),
}

/// A default export
/// ```js
/// export default class Thing {}
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    all(feature = "serde", not(feature = "esprima")),
    derive(Deserialize, Serialize)
)]
#[cfg_attr(all(feature = "serde", feature = "esprima"), derive(Deserialize))]
pub enum DefaultExportDecl<T> {
    Decl(Decl<T>),
    Expr(Expr<T>),
}

/// The name of the thing being exported
/// this might include an alias
/// ```js
/// //no-alias
/// export {Thing} from 'place';
/// //aliased
/// export {Stuff as NewThing} from 'place'
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    all(feature = "serde", not(feature = "esprima")),
    derive(Deserialize, Serialize)
)]
#[cfg_attr(all(feature = "serde", feature = "esprima"), derive(Deserialize))]
pub struct ExportSpecifier<T> {
    pub local: Ident<T>,
    pub exported: Ident<T>,
}
