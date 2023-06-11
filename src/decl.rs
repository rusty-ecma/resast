use crate::expr::{Expr, Lit};
use crate::pat::Pat;
use crate::{VarKind, IntoAllocated};
use crate::{Class, Func, Ident};

/// The declaration of a variable, function, class, import or export
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize)
)]
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

impl<T> IntoAllocated for Decl<T> where T: ToString {
    type Allocated = Decl<String>;

    fn into_allocated(self) -> Self::Allocated {
        match self {
            Decl::Var(k, decls) => Decl::Var(k, decls.into_iter().map(|d| d.into_allocated()).collect()),
            Decl::Func(inner) => Decl::Func(inner.into_allocated()),
            Decl::Class(inner) => Decl::Class(inner.into_allocated()),
            Decl::Import(inner) => Decl::Import(inner.into_allocated()),
            Decl::Export(inner) => Decl::Export(inner.into_allocated()),
        }
    }
}

/// The identifier and optional value of a variable declaration
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize)
)]
pub struct VarDecl<T> {
    pub id: Pat<T>,
    pub init: Option<Expr<T>>,
}

impl<T> IntoAllocated for VarDecl<T> where T: ToString {
    type Allocated = VarDecl<String>;

    fn into_allocated(self) -> Self::Allocated {
        VarDecl {
            id: self.id.into_allocated(),
            init: self.init.map(|i| i.into_allocated()),
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
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub struct ModImport<T> {
    pub specifiers: Vec<ImportSpecifier<T>>,
    pub source: Lit<T>,
}

impl<T> IntoAllocated for ModImport<T> where T: ToString {
    type Allocated = ModImport<String>;

    fn into_allocated(self) -> Self::Allocated {
        ModImport {
            specifiers: self.specifiers.into_iter().map(|s| s.into_allocated()).collect(),
            source: self.source.into_allocated(),
        }
    }
}

/// The name of the thing being imported
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize)
)]
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

impl<T> IntoAllocated for ImportSpecifier<T> where T: ToString {
    type Allocated = ImportSpecifier<String>;

    fn into_allocated(self) -> Self::Allocated {
        match self {
            ImportSpecifier::Normal(inner) => ImportSpecifier::Normal(inner.into_iter().map(|n| n.into_allocated()).collect()),
            ImportSpecifier::Default(inner) => ImportSpecifier::Default(inner.into_allocated()),
            ImportSpecifier::Namespace(inner) => ImportSpecifier::Namespace(inner.into_allocated()),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub struct NormalImportSpec<T> {
    pub alias: Option<Ident<T>>,
    pub imported: Ident<T>,
}

impl<T> IntoAllocated for NormalImportSpec<T> where T: ToString {
    type Allocated = NormalImportSpec<String>;

    fn into_allocated(self) -> Self::Allocated {
        NormalImportSpec {
            alias: self.alias.map(|i| i.into_allocated()),
            imported: self.imported.into_allocated(),
        }
    }
}

/// Something exported from this module
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize)
)]
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

impl<T> IntoAllocated for ModExport<T> where T: ToString {
    type Allocated = ModExport<String>;

    fn into_allocated(self) -> Self::Allocated {
        match self {
            ModExport::Default(inner) => ModExport::Default(inner.into_allocated()),
            ModExport::Named(inner) => ModExport::Named(inner.into_allocated()),
            ModExport::All { alias, name } => ModExport::All { alias: alias.map(|i| i.into_allocated()), name: name.into_allocated()},
        }
    }
}

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

impl<T> IntoAllocated for NamedExportDecl<T> where T: ToString {
    type Allocated = NamedExportDecl<String>;

    fn into_allocated(self) -> Self::Allocated {
        match self {
            NamedExportDecl::Decl(inner) => NamedExportDecl::Decl(inner.into_allocated()),
            NamedExportDecl::Specifier(specs, lit) => NamedExportDecl::Specifier(specs.into_iter().map(|s| s.into_allocated()).collect(), lit.map(|l| l.into_allocated())),
        }
    }
}

/// A default export
/// ```js
/// export default class Thing {}
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize)
)]
pub enum DefaultExportDecl<T> {
    Decl(Decl<T>),
    Expr(Expr<T>),
}

impl<T> IntoAllocated for DefaultExportDecl<T> where T: ToString {
    type Allocated = DefaultExportDecl<String>;

    fn into_allocated(self) -> Self::Allocated {
        match self {
            DefaultExportDecl::Decl(inner) => DefaultExportDecl::Decl(inner.into_allocated()),
            DefaultExportDecl::Expr(inner) => DefaultExportDecl::Expr(inner.into_allocated()),
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
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize)
)]
pub struct ExportSpecifier<T> {
    pub local: Ident<T>,
    pub alias: Option<Ident<T>>,
}

impl<T> IntoAllocated for ExportSpecifier<T> where T: ToString {
    type Allocated = ExportSpecifier<String>;

    fn into_allocated(self) -> Self::Allocated {
        ExportSpecifier {
            local: self.local.into_allocated(),
            alias: self.alias.map(|a| a.into_allocated()),
        }
    }
}
