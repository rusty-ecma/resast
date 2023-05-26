use crate::spanned::expr::{Expr, Lit};
use crate::spanned::pat::Pat;
use crate::spanned::VarKind;
use crate::spanned::{Class, Func, Ident};

use super::{ListEntry, Node, SourceLocation, Position};

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
        semi_colon: Option<Position>,
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
        semi_colon: Option<Position>,
    },
    /// An export declaration
    /// ```js
    /// export function thing() {}
    /// ```
    Export {
        export: Box<ModExport<T>>,
        semi_colon: Option<Position>,
    },
}

// impl<T> From<Decl<T>> for crate::decl::Decl<T> {
//     fn from(other: Decl<T>) -> Self {
//         match other {
//             Decl::Var { decls, .. } => Self::Var(
//                 decls.keyword.into(),
//                 decls.decls.into_iter().map(|e| e.item.into()).collect(),
//             ),
//             Decl::Func(inner) => Self::Func(inner.into()),
//             Decl::Class(inner) => Self::Class(inner.into()),
//             Decl::Import { import, .. } => Self::Import(Box::new(From::from(*import))),
//             Decl::Export { export, .. } => Self::Export(Box::new(From::from(*export))),
//         }
//     }
// }

impl<T> Node for Decl<T> {
    fn loc(&self) -> super::SourceLocation {
        match self {
            Decl::Var { decls, semi_colon } => {
                if let Some(semi) = semi_colon {
                    return SourceLocation {
                        start: decls.loc().start,
                        end: *semi+1,
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
                        end: *semi+1,
                    };
                }
                import.loc()
            }
            Decl::Export { export, semi_colon } => {
                if let Some(semi) = semi_colon {
                    return SourceLocation {
                        start: export.loc().start,
                        end: *semi+1,
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
    pub eq: Option<Position>,
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

// impl<T> From<VarDecl<T>> for crate::decl::VarDecl<T> {
//     fn from(other: VarDecl<T>) -> Self {
//         Self {
//             id: other.id.into(),
//             init: other.init.map(From::from),
//         }
//     }
// }

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
    pub keyword_import: Position,
    pub specifiers: Vec<ListEntry<ImportSpecifier<T>>>,
    pub keyword_from: Option<Position>,
    pub source: Lit<T>,
}

impl<T> Node for ModImport<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.keyword_import,
            end: self.source.loc().end,
        }
    }
}

// impl<T> From<ModImport<T>> for crate::decl::ModImport<T> {
//     fn from(other: ModImport<T>) -> Self {
//         Self {
//             source: other.source.into(),
//             specifiers: other
//                 .specifiers
//                 .into_iter()
//                 .map(|e| e.item.into())
//                 .collect(),
//         }
//     }
// }

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

// impl<T> From<ImportSpecifier<T>> for crate::decl::ImportSpecifier<T> {
//     fn from(other: ImportSpecifier<T>) -> Self {
//         match other {
//             ImportSpecifier::Normal(inner) => {
//                 Self::Normal(inner.specs.into_iter().map(|e| e.item.into()).collect())
//             }
//             ImportSpecifier::Default(inner) => Self::Default(inner.into()),
//             ImportSpecifier::Namespace(inner) => Self::Namespace(inner.into()),
//         }
//     }
// }

#[derive(PartialEq, Debug, Clone)]
pub struct NormalImportSpecs<T> {
    pub open_brace: Position,
    pub specs: Vec<ListEntry<NormalImportSpec<T>>>,
    pub close_brace: Position,
}

impl<T> Node for NormalImportSpecs<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.open_brace,
            end: self.close_brace,
        }
    }
}

// impl<T> From<NormalImportSpec<T>> for crate::decl::NormalImportSpec<T> {
//     fn from(other: NormalImportSpec<T>) -> Self {
//         let imported: crate::Ident = other.imported.into();
//         let local: crate::Ident = if let Some(alias) = other.alias {
//             alias.into()
//         } else {
//             imported.clone()
//         };
//         Self { local, imported }
//     }
// }

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

// impl<T> From<DefaultImportSpec<T>> for crate::Ident<T> {
//     fn from(other: DefaultImportSpec<T>) -> Self {
//         other.id.into()
//     }
// }

impl<T> Node for DefaultImportSpec<T> {
    fn loc(&self) -> SourceLocation {
        self.id.loc()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NamespaceImportSpec<T> {
    pub star: Position,
    pub keyword: Position,
    pub ident: Ident<T>,
}

impl<T> Node for NamespaceImportSpec<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.star,
            end: self.ident.loc().end,
        }
    }
}

// impl<T> From<NamespaceImportSpec<T>> for crate::Ident<T> {
//     fn from(other: NamespaceImportSpec<T>) -> Self {
//         other.ident.into()
//     }
// }

#[derive(Debug, Clone, PartialEq)]
pub struct ModExport<T> {
    pub keyword: Position,
    pub spec: ModExportSpecifier<T>,
}

impl<T> Node for ModExport<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.keyword,
            end: self.spec.loc().end,
        }
    }
}

// impl<T> From<ModExport<T>> for crate::decl::ModExport<T> {
//     fn from(other: ModExport<T>) -> Self {
//         other.spec.into()
//     }
// }

/// Something exported from this module
#[derive(Debug, Clone, PartialEq)]
pub enum ModExportSpecifier<T> {
    /// ```js
    /// export default function() {};
    /// //or
    /// export default 1;
    /// ```
    Default {
        keyword: Position,
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
        star: Position,
        alias: Option<Alias<T>>,
        keyword: Position,
        name: Lit<T>,
    },
}

impl<T> Node for ModExportSpecifier<T> {
    fn loc(&self) -> SourceLocation {
        match self {
            ModExportSpecifier::Default { keyword, value } => SourceLocation {
                start: *keyword,
                end: value.loc().end,
            },
            ModExportSpecifier::Named(inner) => inner.loc(),
            ModExportSpecifier::All { star, name, .. } => SourceLocation {
                start: *star,
                end: name.loc().end,
            },
        }
    }
}

// impl<T> From<ModExportSpecifier<T>> for crate::decl::ModExport<T> {
//     fn from(other: ModExportSpecifier<T>) -> Self {
//         match other {
//             ModExportSpecifier::Default { keyword: _, value } => Self::Default(value.into()),
//             ModExportSpecifier::Named(inner) => Self::Named(inner.into()),
//             ModExportSpecifier::All {
//                 star: _,
//                 alias,
//                 keyword: _,
//                 name,
//             } => Self::All {
//                 alias: alias.map(|a| a.ident.into()),
//                 name: name.into()
//             },
//         }
//     }
// }

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

// impl<T> From<NamedExportDecl<T>> for crate::decl::NamedExportDecl<T> {
//     fn from(other: NamedExportDecl<T>) -> Self {
//         match other {
//             NamedExportDecl::Decl(inner) => Self::Decl(inner.into()),
//             NamedExportDecl::Specifier(inner) => Self::Specifier(
//                 inner
//                     .list
//                     .elements
//                     .into_iter()
//                     .map(|e| e.item.into())
//                     .collect(),
//                 inner.source.map(|s| s.module.into()),
//             ),
//         }
//     }
// }

#[derive(Debug, Clone, PartialEq)]
pub struct DefaultExportDecl<T> {
    pub keyword: Position,
    pub value: DefaultExportDeclValue<T>,
}

impl<T> Node for DefaultExportDecl<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.keyword,
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

// impl<T> From<DefaultExportDeclValue<T>> for crate::decl::DefaultExportDecl<T> {
//     fn from(other: DefaultExportDeclValue<T>) -> Self {
//         match other {
//             DefaultExportDeclValue::Decl(inner) => Self::Decl(inner.into()),
//             DefaultExportDeclValue::Expr(inner) => Self::Expr(inner.into()),
//         }
//     }
// }

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
    pub keyword_from: Position,
    pub module: Lit<T>,
}

impl<T> Node for NamedExportSource<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.keyword_from,
            end: self.module.loc().end,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExportList<T> {
    pub open_brace: Position,
    pub elements: Vec<ListEntry<ExportSpecifier<T>>>,
    pub close_brace: Position,
}

impl<T> Node for ExportList<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.open_brace,
            end: self.close_brace,
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

// impl<T> From<ExportSpecifier<T>> for crate::decl::ExportSpecifier<T> {
//     fn from(other: ExportSpecifier<T>) -> Self {
//         let local: crate::Ident = other.local.into();
//         Self {
//             local: local.clone(),
//             exported: other.alias.map(|a| a.ident.into()).unwrap_or(local),
//         }
//     }
// }

#[derive(Debug, Clone, PartialEq)]
pub struct Alias<T> {
    pub keyword: Position,
    pub ident: Ident<T>,
}

impl<T> Node for Alias<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.keyword,
            end: self.ident.loc().end,
        }
    }
}

// impl<T> From<Alias<T>> for crate::Ident<T> {
//     fn from(other: Alias<T>) -> Self {
//         other.ident.into()
//     }
// }
