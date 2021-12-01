use crate::spanned::expr::{Expr, Lit};
use crate::spanned::pat::Pat;
use crate::spanned::VarKind;
use crate::spanned::{Class, Func, Ident};

use super::{Node, Slice, SourceLocation};

/// The declaration of a variable, function, class, import or export
#[derive(Debug, Clone, PartialEq)]
pub enum Decl<'a> {
    /// A variable declaration
    /// ```js
    /// var x, b;
    /// let y, a = 0;
    /// const q = 100
    /// ```
    Var(VarDecls<'a>),
    /// A function declaration
    /// ```js
    /// function thing() {}
    /// ```
    Func(Func<'a>),
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

impl<'a> From<Decl<'a>> for crate::decl::Decl<'a> {
    fn from(_: Decl<'a>) -> Self {
        todo!()
    }
}

impl<'a> Node for Decl<'a> {
    fn loc(&self) -> super::SourceLocation {
        match self {
            Decl::Var(inner) => inner.loc(),
            Decl::Func(inner) => inner.loc(),
            Decl::Class(inner) => inner.loc(),
            Decl::Import(inner) => inner.loc(),
            Decl::Export(inner) => inner.loc(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct VarDecls<'a> {
    pub keyword: VarKind<'a>,
    pub decls: Vec<VarDecl<'a>>,
}

impl<'a> Node for VarDecls<'a> {
    fn loc(&self) -> SourceLocation {
        if let Some(last) = self.decls.last() {
            SourceLocation {
                start: self.keyword.loc().start,
                end: last.loc().end,
            }
        } else {
            self.keyword.loc()
        }
    }
}

/// The identifier and optional value of a variable declaration
#[derive(Debug, Clone, PartialEq)]
pub struct VarDecl<'a> {
    pub id: Pat<'a>,
    pub init: Option<Expr<'a>>,
}

impl<'a> Node for VarDecl<'a> {
    fn loc(&self) -> SourceLocation {
        if let Some(init) = &self.init {
            SourceLocation {
                start: self.id.loc().start,
                end: init.loc().end,
            }
        } else {
            self.id.loc()
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

impl<'a> Node for ModDecl<'a> {
    fn loc(&self) -> SourceLocation {
        match self {
            ModDecl::Import(inner) => inner.loc(),
            ModDecl::Export(inner) => inner.loc(),
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
    pub keyword: Slice<'a>,
    pub specifiers: Vec<ImportSpecifier<'a>>,
    pub source: Lit<'a>,
}

impl<'a> Node for ModImport<'a> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.keyword.loc.start,
            end: self.source.loc().end,
        }
    }
}

/// The name of the thing being imported
#[derive(Debug, Clone, PartialEq)]
pub enum ImportSpecifier<'a> {
    /// A specifier in curly braces, this might
    /// have a local alias
    ///
    /// ```js
    /// import {Thing} from './stuff.js';
    /// import {People as Persons} from './places.js';
    /// ```
    Normal(NormalImportSpecs<'a>),
    /// A specifier that has been exported with the
    /// default keyword, this should not be wrapped in
    /// curly braces.
    /// ```js
    /// import DefaultThing from './stuff/js';
    /// ```
    Default(DefaultImportSpec<'a>),
    /// Import all exported members from a module
    /// in a namespace.
    ///
    /// ```js
    /// import * as Moment from 'moment.js';
    /// ```
    Namespace(NamespaceImportSpec<'a>),
}

impl<'a> Node for ImportSpecifier<'a> {
    fn loc(&self) -> SourceLocation {
        match self {
            ImportSpecifier::Normal(inner) => inner.loc(),
            ImportSpecifier::Default(inner) => inner.loc(),
            ImportSpecifier::Namespace(inner) => inner.loc(),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct NormalImportSpecs<'a> {
    pub open_brace: Slice<'a>,
    pub specs: Vec<NormalImportSpec<'a>>,
    pub close_brace: Slice<'a>,
}

impl<'a> Node for NormalImportSpecs<'a> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.open_brace.loc.start,
            end: self.close_brace.loc.end,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct NormalImportSpec<'a> {
    pub local: Ident<'a>,
    pub alias: Option<Alias<'a>>,
}

impl<'a> Node for NormalImportSpec<'a> {
    fn loc(&self) -> SourceLocation {
        if let Some(alias) = &self.alias {
            SourceLocation {
                start: self.local.loc().start,
                end: alias.loc().end,
            }
        } else {
            self.local.loc()
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DefaultImportSpec<'a> {
    pub id: Ident<'a>,
    pub keyword: Slice<'a>,
    pub module: Lit<'a>,
}

impl<'a> Node for DefaultImportSpec<'a> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.id.loc().start,
            end: self.module.loc().end,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NamespaceImportSpec<'a> {
    pub star: Slice<'a>,
    pub keyword: Slice<'a>,
    pub ident: Ident<'a>,
}

impl<'a> Node for NamespaceImportSpec<'a> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.star.loc.start,
            end: self.ident.loc().end,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModExport<'a> {
    pub keyword: Slice<'a>,
    pub spec: ModExportSpecifier<'a>,
}

impl<'a> Node for ModExport<'a> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.keyword.loc.start,
            end: self.spec.loc().end,
        }
    }
}

/// Something exported from this module
#[derive(Debug, Clone, PartialEq)]
pub enum ModExportSpecifier<'a> {
    /// ```js
    /// export default function() {};
    /// //or
    /// export default 1;
    /// ```
    Default {
        keyword: Slice<'a>,
        value: ExportDeclValue<'a>,
    },
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
    All {
        star: Slice<'a>,
        keyword: Slice<'a>,
        name: Lit<'a>,
    },
}

impl<'a> Node for ModExportSpecifier<'a> {
    fn loc(&self) -> SourceLocation {
        match self {
            ModExportSpecifier::Default { keyword, value } => SourceLocation {
                start: keyword.loc.start,
                end: value.loc().end,
            },
            ModExportSpecifier::Named(inner) => inner.loc(),
            ModExportSpecifier::All { star, name, .. } => SourceLocation {
                start: star.loc.start,
                end: name.loc().end,
            },
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
    Specifier(ExportList<'a>, Option<Slice<'a>>, Option<Lit<'a>>),
}

impl<'a> Node for NamedExportDecl<'a> {
    fn loc(&self) -> SourceLocation {
        match self {
            NamedExportDecl::Decl(inner) => inner.loc(),
            NamedExportDecl::Specifier(list, _, module) => {
                if let Some(module) = module {
                    SourceLocation {
                        start: list.loc().start,
                        end: module.loc().end,
                    }
                } else {
                    list.loc()
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DefaultExportDecl<'a> {
    pub keyword: Slice<'a>,
    pub value: ExportDeclValue<'a>,
}

impl<'a> Node for DefaultExportDecl<'a> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.keyword.loc.start,
            end: self.value.loc().end,
        }
    }
}

/// A default export
/// ```js
/// export default class Thing {}
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum ExportDeclValue<'a> {
    Decl(Decl<'a>),
    Expr(Expr<'a>),
    List(ExportList<'a>),
}

impl<'a> Node for ExportDeclValue<'a> {
    fn loc(&self) -> SourceLocation {
        match self {
            ExportDeclValue::Decl(inner) => inner.loc(),
            ExportDeclValue::Expr(inner) => inner.loc(),
            ExportDeclValue::List(inner) => inner.loc(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExportList<'a> {
    pub open_brace: Slice<'a>,
    pub elements: Vec<ExportSpecifier<'a>>,
    pub close_brace: Slice<'a>,
}

impl<'a> Node for ExportList<'a> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.open_brace.loc.start,
            end: self.close_brace.loc.end,
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
pub struct ExportSpecifier<'a> {
    pub local: Ident<'a>,
    pub alias: Option<Alias<'a>>,
}

impl<'a> Node for ExportSpecifier<'a> {
    fn loc(&self) -> SourceLocation {
        if let Some(alias) = &self.alias {
            SourceLocation {
                start: self.local.loc().start,
                end: alias.loc().end,
            }
        } else {
            self.local.loc()
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Alias<'a> {
    keyword: Slice<'a>,
    ident: Ident<'a>,
}

impl<'a> Node for Alias<'a> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.keyword.loc.start,
            end: self.ident.loc().end,
        }
    }
}
