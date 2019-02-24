use crate::decl::VariableKind;
use crate::ref_tree::expr::{Expr, Literal};
use crate::ref_tree::pat::Pat;
use crate::ref_tree::{AsConcrete, Class, Function, Identifier};

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

impl<'a> AsConcrete<crate::decl::Decl> for Decl<'a> {
    fn as_concrete(&self) -> crate::decl::Decl {
        match self {
            Decl::Class(ref c) => crate::decl::Decl::Class(c.as_concrete()),
            Decl::Export(ref e) => crate::decl::Decl::Export(Box::new(e.as_concrete())),
            Decl::Function(ref f) => crate::decl::Decl::Function(f.as_concrete()),
            Decl::Import(ref i) => crate::decl::Decl::Import(Box::new(i.as_concrete())),
            Decl::Variable(k, ref v) => crate::decl::Decl::Variable(*k, v.as_concrete()),
        }
    }
}

/// The identifier and optional value of a variable declaration
#[derive(PartialEq, Debug, Clone)]
pub struct VariableDecl<'a> {
    pub id: Pat<'a>,
    pub init: Option<Expr<'a>>,
}

impl<'a> AsConcrete<crate::decl::VariableDecl> for VariableDecl<'a> {
    fn as_concrete(&self) -> crate::decl::VariableDecl {
        let init = if let Some(ref i) = self.init {
            Some(i.as_concrete())
        } else {
            None
        };
        crate::decl::VariableDecl {
            id: self.id.as_concrete(),
            init,
        }
    }
}

/// A module declaration, This would only be available
/// in an ES Mod, it would be either an import or
/// export at the top level
#[derive(PartialEq, Debug, Clone)]
pub enum ModDecl<'a> {
    Import(ModImport<'a>),
    Export(ModExport<'a>),
}

impl<'a> AsConcrete<crate::decl::ModDecl> for ModDecl<'a> {
    fn as_concrete(&self) -> crate::decl::ModDecl {
        match self {
            ModDecl::Import(ref i) => crate::decl::ModDecl::Import(i.as_concrete()),
            ModDecl::Export(ref e) => crate::decl::ModDecl::Export(e.as_concrete()),
        }
    }
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

impl<'a> AsConcrete<crate::decl::ModImport> for ModImport<'a> {
    fn as_concrete(&self) -> crate::decl::ModImport {
        crate::decl::ModImport {
            specifiers: self.specifiers.as_concrete(),
            source: self.source.as_concrete(),
        }
    }
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

impl<'a> AsConcrete<crate::decl::ImportSpecifier> for ImportSpecifier<'a> {
    fn as_concrete(&self) -> crate::decl::ImportSpecifier {
        match self {
            ImportSpecifier::Default(ref d) => crate::decl::ImportSpecifier::Default(String::from(*d)),
            ImportSpecifier::Namespace(ref n) => crate::decl::ImportSpecifier::Namespace(String::from(*n)),
            ImportSpecifier::Normal(ref n, ref i) => {
                let import = if let Some(ref i) = i {
                    Some(String::from(*i))
                } else {
                    None
                };
                crate::decl::ImportSpecifier::Normal(String::from(*n), import)
            },
        }
    }
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

impl<'a> AsConcrete<crate::decl::ModExport> for ModExport<'a> {
    fn as_concrete(&self) -> crate::decl::ModExport {
        match self {
            ModExport::All(ref a) => crate::decl::ModExport::All(a.as_concrete()),
            ModExport::Default(ref d) => crate::decl::ModExport::Default(d.as_concrete()),
            ModExport::Named(ref n) => crate::decl::ModExport::Named(n.as_concrete()),
        }
    }
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

impl<'a> AsConcrete<crate::decl::NamedExportDecl> for NamedExportDecl<'a> {
    fn as_concrete(&self) -> crate::decl::NamedExportDecl {
        match self {
            NamedExportDecl::Decl(ref d) => crate::decl::NamedExportDecl::Decl(d.as_concrete()),
            NamedExportDecl::Specifier(ref s, ref l) => {
                let lit = if let Some(ref lit) = l {
                    Some(lit.as_concrete())
                } else {
                    None
                };
                let specs = s.as_concrete();
                crate::decl::NamedExportDecl::Specifier(specs, lit)
            },
        }
    }
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

impl<'a> AsConcrete<crate::decl::DefaultExportDecl> for DefaultExportDecl<'a> {
    fn as_concrete(&self) -> crate::decl::DefaultExportDecl {
        match self {
            DefaultExportDecl::Decl(ref d) => crate::decl::DefaultExportDecl::Decl(d.as_concrete()),
            DefaultExportDecl::Expr(ref e) => crate::decl::DefaultExportDecl::Expr(e.as_concrete()),
        }
    }
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

impl<'a> AsConcrete<crate::decl::ExportSpecifier> for ExportSpecifier<'a> {
    fn as_concrete(&self) -> crate::decl::ExportSpecifier {
        let exported = if let Some(ref ident) = self.exported {
            Some(String::from(*ident))
        } else {
            None
        };
        crate::decl::ExportSpecifier {
            local: String::from(self.local),
            exported,
        }
    }
}
