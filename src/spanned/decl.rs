use crate::spanned::expr::{Expr, Lit};
use crate::spanned::pat::Pat;
use crate::spanned::VarKind;
use crate::spanned::{Class, Func, Ident};

use super::tokens::{
    As, Asterisk, CloseBrace, Default, Equal, Export, From, Import, OpenBrace, Semicolon, Token,
};
use super::{ListEntry, Node, SourceLocation};

/// The declaration of a variable, function, class, import or export
#[derive(Debug, Clone, PartialEq)]
pub enum Decl<T> {
    /// A variable declaration
    /// ```js
    /// var x, b;
    /// let y, a = 0;
    /// const q = 100
    /// ```
    Var {
        decls: VarDecls<T>,
        semi_colon: Option<Semicolon>,
    },
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
    Import {
        import: Box<ModImport<T>>,
        semi_colon: Option<Semicolon>,
    },
    /// An export declaration
    /// ```js
    /// export function thing() {}
    /// ```
    Export {
        export: Box<ModExport<T>>,
        semi_colon: Option<Semicolon>,
    },
}

impl<T> Node for Decl<T> {
    fn loc(&self) -> super::SourceLocation {
        match self {
            Decl::Var { decls, semi_colon } => {
                if let Some(semi) = semi_colon {
                    return SourceLocation {
                        start: decls.loc().start,
                        end: semi.end(),
                    };
                }
                decls.loc()
            }
            Decl::Func(inner) => inner.loc(),
            Decl::Class(inner) => inner.loc(),
            Decl::Import { import, semi_colon } => {
                if let Some(semi) = semi_colon {
                    return SourceLocation {
                        start: import.loc().start,
                        end: semi.end(),
                    };
                }
                import.loc()
            }
            Decl::Export { export, semi_colon } => {
                if let Some(semi) = semi_colon {
                    return SourceLocation {
                        start: export.loc().start,
                        end: semi.end(),
                    };
                }
                export.loc()
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct VarDecls<T> {
    pub keyword: VarKind,
    pub decls: Vec<ListEntry<VarDecl<T>>>,
}

impl<T> Node for VarDecls<T> {
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
pub struct VarDecl<T> {
    pub id: Pat<T>,
    pub eq: Option<Equal>,
    pub init: Option<Expr<T>>,
}

impl<T> Node for VarDecl<T> {
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
pub enum ModDecl<T> {
    Import(ModImport<T>),
    Export(ModExport<T>),
}

impl<T> Node for ModDecl<T> {
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
pub struct ModImport<T> {
    pub keyword_import: Import,
    pub specifiers: Vec<ListEntry<ImportSpecifier<T>>>,
    pub keyword_from: Option<From>,
    pub source: Lit<T>,
}

impl<T> Node for ModImport<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.keyword_import.start(),
            end: self.source.loc().end,
        }
    }
}

/// The name of the thing being imported
#[derive(Debug, Clone, PartialEq)]
pub enum ImportSpecifier<T> {
    /// A specifier in curly braces, this might
    /// have a local alias
    ///
    /// ```js
    /// import {Thing} from './stuff.js';
    /// //     ^^^^^^^
    /// import {People as Persons, x} from './places.js';
    /// //     ^^^^^^^^^^^^^^^^^^^^^^
    /// ```
    Normal(NormalImportSpecs<T>),
    /// A specifier that has been exported with the
    /// default keyword, this should not be wrapped in
    /// curly braces.
    /// ```js
    /// import DefaultThing from './stuff/js';
    /// ```
    Default(DefaultImportSpec<T>),
    /// Import all exported members from a module
    /// in a namespace.
    ///
    /// ```js
    /// import * as Moment from 'moment.js';
    /// ```
    Namespace(NamespaceImportSpec<T>),
}

impl<T> Node for ImportSpecifier<T> {
    fn loc(&self) -> SourceLocation {
        match self {
            ImportSpecifier::Normal(inner) => inner.loc(),
            ImportSpecifier::Default(inner) => inner.loc(),
            ImportSpecifier::Namespace(inner) => inner.loc(),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct NormalImportSpecs<T> {
    pub open_brace: OpenBrace,
    pub specs: Vec<ListEntry<NormalImportSpec<T>>>,
    pub close_brace: CloseBrace,
}

impl<T> Node for NormalImportSpecs<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.open_brace.start(),
            end: self.close_brace.end(),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct NormalImportSpec<T> {
    pub imported: Ident<T>,
    pub alias: Option<Alias<T>>,
}

impl<T> Node for NormalImportSpec<T> {
    fn loc(&self) -> SourceLocation {
        if let Some(alias) = &self.alias {
            SourceLocation {
                start: self.imported.loc().start,
                end: alias.loc().end,
            }
        } else {
            self.imported.loc()
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DefaultImportSpec<T> {
    pub id: Ident<T>,
}

impl<T> Node for DefaultImportSpec<T> {
    fn loc(&self) -> SourceLocation {
        self.id.loc()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NamespaceImportSpec<T> {
    pub star: Asterisk,
    pub keyword: As,
    pub ident: Ident<T>,
}

impl<T> Node for NamespaceImportSpec<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.star.start(),
            end: self.ident.loc().end,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModExport<T> {
    pub keyword: Export,
    pub spec: ModExportSpecifier<T>,
}

impl<T> Node for ModExport<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.keyword.start(),
            end: self.spec.loc().end,
        }
    }
}

/// Something exported from this module
#[derive(Debug, Clone, PartialEq)]
pub enum ModExportSpecifier<T> {
    /// ```js
    /// export default function() {};
    /// //or
    /// export default 1;
    /// ```
    Default {
        keyword: Default,
        value: DefaultExportDeclValue<T>,
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
    Named(NamedExportDecl<T>),
    /// ```js
    /// export * from 'mod';
    /// ```
    All {
        star: Asterisk,
        alias: Option<Alias<T>>,
        keyword: From,
        name: Lit<T>,
    },
}

impl<T> Node for ModExportSpecifier<T> {
    fn loc(&self) -> SourceLocation {
        match self {
            ModExportSpecifier::Default { keyword, value } => SourceLocation {
                start: keyword.start(),
                end: value.loc().end,
            },
            ModExportSpecifier::Named(inner) => inner.loc(),
            ModExportSpecifier::All { star, name, .. } => SourceLocation {
                start: star.start(),
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
pub enum NamedExportDecl<T> {
    Decl(Decl<T>),
    Specifier(NamedExportSpec<T>),
}

impl<T> Node for NamedExportDecl<T> {
    fn loc(&self) -> SourceLocation {
        match self {
            NamedExportDecl::Decl(inner) => inner.loc(),
            NamedExportDecl::Specifier(inner) => inner.loc(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DefaultExportDecl<T> {
    pub keyword: Default,
    pub value: DefaultExportDeclValue<T>,
}

impl<T> Node for DefaultExportDecl<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.keyword.start(),
            end: self.value.loc().end,
        }
    }
}

/// A default export
/// ```js
/// export default class Thing {}
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum ExportDeclValue<T> {
    Decl(Decl<T>),
    Expr(Expr<T>),
    List(ExportList<T>),
}

impl<T> Node for ExportDeclValue<T> {
    fn loc(&self) -> SourceLocation {
        match self {
            ExportDeclValue::Decl(inner) => inner.loc(),
            ExportDeclValue::Expr(inner) => inner.loc(),
            ExportDeclValue::List(inner) => inner.loc(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DefaultExportDeclValue<T> {
    Decl(Decl<T>),
    Expr(Expr<T>),
}

impl<T> Node for DefaultExportDeclValue<T> {
    fn loc(&self) -> SourceLocation {
        match self {
            Self::Decl(inner) => inner.loc(),
            Self::Expr(inner) => inner.loc(),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct NamedExportSpec<T> {
    pub list: ExportList<T>,
    pub source: Option<NamedExportSource<T>>,
}

impl<T> Node for NamedExportSpec<T> {
    fn loc(&self) -> SourceLocation {
        if let Some(source) = &self.source {
            SourceLocation {
                start: self.list.loc().start,
                end: source.loc().end,
            }
        } else {
            self.list.loc()
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct NamedExportSource<T> {
    pub keyword_from: From,
    pub module: Lit<T>,
}

impl<T> Node for NamedExportSource<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.keyword_from.start(),
            end: self.module.loc().end,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExportList<T> {
    pub open_brace: OpenBrace,
    pub elements: Vec<ListEntry<ExportSpecifier<T>>>,
    pub close_brace: CloseBrace,
}

impl<T> Node for ExportList<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.open_brace.start(),
            end: self.close_brace.end(),
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
pub struct ExportSpecifier<T> {
    pub local: Ident<T>,
    pub alias: Option<Alias<T>>,
}

impl<T> Node for ExportSpecifier<T> {
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
pub struct Alias<T> {
    pub keyword: As,
    pub ident: Ident<T>,
}

impl<T> Node for Alias<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.keyword.start(),
            end: self.ident.loc().end,
        }
    }
}
