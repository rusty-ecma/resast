use crate::IntoAllocated;
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


impl<T> IntoAllocated for Decl<T>
where
    T: ToString,
{
    type Allocated = Decl<String>;
    fn into_allocated(self) -> Decl<String> {
        match self {
            Decl::Var { decls, semi_colon } => Decl::Var {
                decls: decls.into_allocated(),
                semi_colon: semi_colon,
            },
            Decl::Func(f) => Decl::Func(f.into_allocated()),
            Decl::Class(c) => Decl::Class(c.into_allocated()),
            Decl::Import { import, semi_colon } => Decl::Import { import: import.into_allocated(), semi_colon },
            Decl::Export { export, semi_colon } => Decl::Export { export: export.into_allocated(), semi_colon },
        }
    }
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


impl<T> IntoAllocated for VarDecls<T>
where
    T: ToString,
{
    type Allocated = VarDecls<String>;
    fn into_allocated(self) -> VarDecls<String> {
        VarDecls {
            keyword: self.keyword,
            decls: self.decls.into_iter().map(|d| d.into_allocated()).collect(),
        }
    }
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


impl<T> IntoAllocated for VarDecl<T>
where
    T: ToString,
{
    type Allocated = VarDecl<String>;
    fn into_allocated(self) -> VarDecl<String> {
        VarDecl {
            id: self.id.into_allocated(),
            eq: self.eq,
            init: self.init.map(|i| i.into_allocated()),
        }
    }
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

impl<T> IntoAllocated for ModDecl<T> where T: ToString {
    type Allocated = ModDecl<String>;
    fn into_allocated(self) -> ModDecl<String> {
        match self {
            ModDecl::Import(inner) => ModDecl::Import(inner.into_allocated()),
            ModDecl::Export(inner) => ModDecl::Export(inner.into_allocated()),
        }
    }
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

impl<T> IntoAllocated for ModImport<T> where T: ToString {
    type Allocated = ModImport<String>;
    fn into_allocated(self) -> ModImport<String> {
        ModImport { keyword_import:self.keyword_import, specifiers: self.specifiers.into_iter().map(|s| s.into_allocated()).collect(), keyword_from: self.keyword_from, source: self.source.into_allocated() }
    }
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

impl<T> IntoAllocated for ImportSpecifier<T> where T: ToString {
    type Allocated = ImportSpecifier<String>;
    fn into_allocated(self) -> ImportSpecifier<String> {
        match self {
            ImportSpecifier::Normal(inner) => ImportSpecifier::Normal(inner.into_allocated()),
            ImportSpecifier::Default(inner) => ImportSpecifier::Default(inner.into_allocated()),
            ImportSpecifier::Namespace(inner) => ImportSpecifier::Namespace(inner.into_allocated()),
        }
    }
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

impl<T> IntoAllocated for NormalImportSpecs<T> where T: ToString {
    type Allocated = NormalImportSpecs<String>;
    fn into_allocated(self) -> NormalImportSpecs<String> {
        NormalImportSpecs {
            open_brace: self.open_brace,
            specs: self.specs.into_iter().map(|s| s.into_allocated()).collect(),
            close_brace: self.close_brace,
        }
    }
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

impl<T> IntoAllocated for NormalImportSpec<T> where T: ToString {
    type Allocated = NormalImportSpec<String>;
    fn into_allocated(self) -> NormalImportSpec<String> {
        NormalImportSpec { imported: self.imported.into_allocated(), alias: self.alias.map(|a| a.into_allocated()) }
    }
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

impl<T> IntoAllocated for DefaultImportSpec<T> where T: ToString {
    type Allocated = DefaultImportSpec<String>;
    fn into_allocated(self) -> DefaultImportSpec<String> {
        DefaultImportSpec { id: self.id.into_allocated() }
    }
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

impl<T> IntoAllocated for NamespaceImportSpec<T> where T: ToString {
    type Allocated = NamespaceImportSpec<String>;
    fn into_allocated(self) -> NamespaceImportSpec<String> {
        NamespaceImportSpec {
            star: self.star,
            keyword: self.keyword,
            ident: self.ident.into_allocated(),
        }
    }
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

impl<T> IntoAllocated for ModExport<T> where T: ToString {
    type Allocated = ModExport<String>;
    fn into_allocated(self) -> ModExport<String> {
        ModExport { keyword: self.keyword, spec: self.spec.into_allocated() }
    }
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

impl<T> IntoAllocated for ModExportSpecifier<T> where T: ToString {
    type Allocated = ModExportSpecifier<String>;
    fn into_allocated(self) -> ModExportSpecifier<String> {
        match self {
            ModExportSpecifier::Default { keyword, value } => ModExportSpecifier::Default {keyword, value: value.into_allocated()},
            ModExportSpecifier::Named(inner) => ModExportSpecifier::Named(inner.into_allocated()),
            ModExportSpecifier::All { star, alias, keyword, name } => ModExportSpecifier::All { star, alias: alias.map(|a| a.into_allocated()), keyword: keyword, name: name.into_allocated()},
        }
    }
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

impl<T> IntoAllocated for NamedExportDecl<T> where T: ToString {
    type Allocated = NamedExportDecl<String>;
    fn into_allocated(self) -> NamedExportDecl<String> {
        match self {
            NamedExportDecl::Decl(inner) => NamedExportDecl::Decl(inner.into_allocated()),
            NamedExportDecl::Specifier(inner) => NamedExportDecl::Specifier(inner.into_allocated()),
        }
    }
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

impl<T> IntoAllocated for DefaultExportDecl<T> where T: ToString {
    type Allocated = DefaultExportDecl<String>;
    fn into_allocated(self) -> DefaultExportDecl<String> {
        DefaultExportDecl {
            keyword: self.keyword,
            value: self.value.into_allocated(),
        }
    }
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

impl<T> IntoAllocated for ExportDeclValue<T> where T: ToString {
    type Allocated = ExportDeclValue<String>;
    fn into_allocated(self) -> ExportDeclValue<String> {
        match self {
            ExportDeclValue::Decl(inner) => ExportDeclValue::Decl(inner.into_allocated()),
            ExportDeclValue::Expr(inner) => ExportDeclValue::Expr(inner.into_allocated()),
            ExportDeclValue::List(inner) => ExportDeclValue::List(inner.into_allocated()),
        }
    }
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

impl<T> IntoAllocated for DefaultExportDeclValue<T> where T: ToString {
    type Allocated = DefaultExportDeclValue<String>;
    fn into_allocated(self) -> DefaultExportDeclValue<String> {
        match self {
            DefaultExportDeclValue::Decl(inner) => DefaultExportDeclValue::Decl(inner.into_allocated()),
            DefaultExportDeclValue::Expr(inner) => DefaultExportDeclValue::Expr(inner.into_allocated()),
        }
    }
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

impl<T> IntoAllocated for NamedExportSpec<T> where T: ToString {
    type Allocated = NamedExportSpec<String>;
    fn into_allocated(self) -> NamedExportSpec<String> {
        NamedExportSpec {
            list: self.list.into_allocated(),
            source: self.source.map(|s| s.into_allocated()),
        }
    }
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

impl<T> IntoAllocated for NamedExportSource<T> where T: ToString {
    type Allocated = NamedExportSource<String>;
    fn into_allocated(self) -> NamedExportSource<String> {
        NamedExportSource {
            keyword_from: self.keyword_from,
            module: self.module.into_allocated(),
        }
    }
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

impl<T> IntoAllocated for ExportList<T> where T: ToString {
    type Allocated = ExportList<String>;
    fn into_allocated(self) -> ExportList<String> {
        ExportList { open_brace: self.open_brace, elements: self.elements.into_iter().map(|e| e.into_allocated()).collect(), close_brace: self.close_brace }
    }
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

impl<T> IntoAllocated for ExportSpecifier<T> where T: ToString {
    type Allocated = ExportSpecifier<String>;

    fn into_allocated(self) -> Self::Allocated {
        ExportSpecifier {
            local: self.local.into_allocated(),
            alias: self.alias.map(|a| a.into_allocated()),
        }
    }
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

impl<T> IntoAllocated for Alias<T> where T: ToString {
    type Allocated = Alias<String>;

    fn into_allocated(self) -> Self::Allocated {
        Alias {
            keyword: self.keyword,
            ident: self.ident.into_allocated(),
        }
    }
}

impl<T> Node for Alias<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.keyword.start(),
            end: self.ident.loc().end,
        }
    }
}
