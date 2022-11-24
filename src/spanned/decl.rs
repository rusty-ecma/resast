use crate::spanned::expr::{Expr, Lit};
use crate::spanned::pat::Pat;
use crate::spanned::VarKind;
use crate::spanned::{Class, Func, Ident};

use super::{ListEntry, Node, Slice, SourceLocation};

/// The declaration of a variable, function, class, import or export
#[derive(Debug, Clone, PartialEq)]
pub enum Decl<'a> {
    /// A variable declaration
    /// ```js
    /// var x, b;
    /// let y, a = 0;
    /// const q = 100
    /// ```
    Var {
        decls: VarDecls<'a>,
        semi_colon: Option<Slice<'a>>,
    },
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
    Import {
        import: Box<ModImport<'a>>,
        semi_colon: Option<Slice<'a>>,
    },
    /// An export declaration
    /// ```js
    /// export function thing() {}
    /// ```
    Export {
        export: Box<ModExport<'a>>,
        semi_colon: Option<Slice<'a>>,
    },
}

impl<'a> From<Decl<'a>> for crate::decl::Decl<'a> {
    fn from(other: Decl<'a>) -> Self {
        match other {
            Decl::Var { decls, .. } => Self::Var(
                decls.keyword.into(),
                decls.decls.into_iter().map(|e| e.item.into()).collect(),
            ),
            Decl::Func(inner) => Self::Func(inner.into()),
            Decl::Class(inner) => Self::Class(inner.into()),
            Decl::Import { import, .. } => Self::Import(Box::new(From::from(*import))),
            Decl::Export { export, .. } => Self::Export(Box::new(From::from(*export))),
        }
    }
}

impl<'a> Node for Decl<'a> {
    fn loc(&self) -> super::SourceLocation {
        match self {
            Decl::Var { decls, semi_colon } => {
                if let Some(semi) = semi_colon {
                    return SourceLocation {
                        start: decls.loc().start,
                        end: semi.loc.end,
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
                        end: semi.loc.end,
                    };
                }
                import.loc()
            }
            Decl::Export { export, semi_colon } => {
                if let Some(semi) = semi_colon {
                    return SourceLocation {
                        start: export.loc().start,
                        end: semi.loc.end,
                    };
                }
                export.loc()
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct VarDecls<'a> {
    pub keyword: VarKind<'a>,
    pub decls: Vec<ListEntry<'a, VarDecl<'a>>>,
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
    pub eq: Option<Slice<'a>>,
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

impl<'a> From<VarDecl<'a>> for crate::decl::VarDecl<'a> {
    fn from(other: VarDecl<'a>) -> Self {
        Self {
            id: other.id.into(),
            init: other.init.map(From::from),
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
    pub keyword_import: Slice<'a>,
    pub specifiers: Vec<ListEntry<'a, ImportSpecifier<'a>>>,
    pub keyword_from: Option<Slice<'a>>,
    pub source: Lit<'a>,
}

impl<'a> Node for ModImport<'a> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.keyword_import.loc.start,
            end: self.source.loc().end,
        }
    }
}

impl<'a> From<ModImport<'a>> for crate::decl::ModImport<'a> {
    fn from(other: ModImport<'a>) -> Self {
        Self {
            source: other.source.into(),
            specifiers: other
                .specifiers
                .into_iter()
                .map(|e| e.item.into())
                .collect(),
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
    /// //     ^^^^^^^
    /// import {People as Persons, x} from './places.js';
    /// //     ^^^^^^^^^^^^^^^^^^^^^^
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

impl<'a> From<ImportSpecifier<'a>> for crate::decl::ImportSpecifier<'a> {
    fn from(other: ImportSpecifier<'a>) -> Self {
        match other {
            ImportSpecifier::Normal(inner) => {
                Self::Normal(inner.specs.into_iter().map(|e| e.item.into()).collect())
            }
            ImportSpecifier::Default(inner) => Self::Default(inner.into()),
            ImportSpecifier::Namespace(inner) => Self::Namespace(inner.into()),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct NormalImportSpecs<'a> {
    pub open_brace: Slice<'a>,
    pub specs: Vec<ListEntry<'a, NormalImportSpec<'a>>>,
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

impl<'a> From<NormalImportSpec<'a>> for crate::decl::NormalImportSpec<'a> {
    fn from(other: NormalImportSpec<'a>) -> Self {
        let imported: crate::Ident = other.imported.into();
        let local: crate::Ident = if let Some(alias) = other.alias {
            alias.into()
        } else {
            imported.clone()
        };
        Self { local, imported }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct NormalImportSpec<'a> {
    pub imported: Ident<'a>,
    pub alias: Option<Alias<'a>>,
}

impl<'a> Node for NormalImportSpec<'a> {
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
pub struct DefaultImportSpec<'a> {
    pub id: Ident<'a>,
}

impl<'a> From<DefaultImportSpec<'a>> for crate::Ident<'a> {
    fn from(other: DefaultImportSpec<'a>) -> Self {
        other.id.into()
    }
}

impl<'a> Node for DefaultImportSpec<'a> {
    fn loc(&self) -> SourceLocation {
        self.id.loc()
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

impl<'a> From<NamespaceImportSpec<'a>> for crate::Ident<'a> {
    fn from(other: NamespaceImportSpec<'a>) -> Self {
        other.ident.into()
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

impl<'a> From<ModExport<'a>> for crate::decl::ModExport<'a> {
    fn from(other: ModExport<'a>) -> Self {
        other.spec.into()
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
        value: DefaultExportDeclValue<'a>,
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
        alias: Option<Alias<'a>>,
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

impl<'a> From<ModExportSpecifier<'a>> for crate::decl::ModExport<'a> {
    fn from(other: ModExportSpecifier<'a>) -> Self {
        match other {
            ModExportSpecifier::Default { keyword: _, value } => Self::Default(value.into()),
            ModExportSpecifier::Named(inner) => Self::Named(inner.into()),
            ModExportSpecifier::All {
                star: _,
                alias,
                keyword: _,
                name,
            } => Self::All {
                alias: alias.map(|a| a.ident.into()),
                name: name.into()
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
    Specifier(NamedExportSpec<'a>),
}

impl<'a> Node for NamedExportDecl<'a> {
    fn loc(&self) -> SourceLocation {
        match self {
            NamedExportDecl::Decl(inner) => inner.loc(),
            NamedExportDecl::Specifier(inner) => inner.loc(),
        }
    }
}

impl<'a> From<NamedExportDecl<'a>> for crate::decl::NamedExportDecl<'a> {
    fn from(other: NamedExportDecl<'a>) -> Self {
        match other {
            NamedExportDecl::Decl(inner) => Self::Decl(inner.into()),
            NamedExportDecl::Specifier(inner) => Self::Specifier(
                inner
                    .list
                    .elements
                    .into_iter()
                    .map(|e| e.item.into())
                    .collect(),
                inner.source.map(|s| s.module.into()),
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DefaultExportDecl<'a> {
    pub keyword: Slice<'a>,
    pub value: DefaultExportDeclValue<'a>,
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
pub enum DefaultExportDeclValue<'a> {
    Decl(Decl<'a>),
    Expr(Expr<'a>),
}

impl<'a> Node for DefaultExportDeclValue<'a> {
    fn loc(&self) -> SourceLocation {
        match self {
            Self::Decl(inner) => inner.loc(),
            Self::Expr(inner) => inner.loc(),
        }
    }
}

impl<'a> From<DefaultExportDeclValue<'a>> for crate::decl::DefaultExportDecl<'a> {
    fn from(other: DefaultExportDeclValue<'a>) -> Self {
        match other {
            DefaultExportDeclValue::Decl(inner) => Self::Decl(inner.into()),
            DefaultExportDeclValue::Expr(inner) => Self::Expr(inner.into()),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct NamedExportSpec<'a> {
    pub list: ExportList<'a>,
    pub source: Option<NamedExportSource<'a>>,
}

impl<'a> Node for NamedExportSpec<'a> {
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
pub struct NamedExportSource<'a> {
    pub keyword_from: Slice<'a>,
    pub module: Lit<'a>,
}

impl<'a> Node for NamedExportSource<'a> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.keyword_from.loc.start,
            end: self.module.loc().end,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExportList<'a> {
    pub open_brace: Slice<'a>,
    pub elements: Vec<ListEntry<'a, ExportSpecifier<'a>>>,
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

impl<'a> From<ExportSpecifier<'a>> for crate::decl::ExportSpecifier<'a> {
    fn from(other: ExportSpecifier<'a>) -> Self {
        let local: crate::Ident = other.local.into();
        Self {
            local: local.clone(),
            exported: other.alias.map(|a| a.ident.into()).unwrap_or(local),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Alias<'a> {
    pub keyword: Slice<'a>,
    pub ident: Ident<'a>,
}

impl<'a> Node for Alias<'a> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.keyword.loc.start,
            end: self.ident.loc().end,
        }
    }
}

impl<'a> From<Alias<'a>> for crate::Ident<'a> {
    fn from(other: Alias<'a>) -> Self {
        other.ident.into()
    }
}
