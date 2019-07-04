use crate::expr::{Expr, Literal, PropertyKey, PropertyKind, PropertyValue, Property, ObjectExpr};
use crate::pat::{Pat, ObjectPatPart};
use crate::{Class, Function, Identifier};

/// The declaration of a variable, function, class, import or export
#[derive(PartialEq, Debug, Clone)]
pub enum Decl {
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
    Import(Box<ModImport>),
    /// An export declaration
    /// ```js
    /// export function thing() {}
    /// ```
    Export(Box<ModExport>),
}

impl Decl {
    pub fn variable(kind: VariableKind, decls: Vec<VariableDecl>) -> Self {
        Decl::Variable(kind, decls)
    }
    pub fn function(f: Function) -> Self {
        Decl::Function(f)
    }
    pub fn class(class: Class) -> Self {
        Decl::Class(class)
    }
    pub fn import(imp: ModImport) -> Self {
        Decl::Import(Box::new(imp))
    }
    pub fn export(exp: ModExport) -> Self {
        Decl::Export(Box::new(exp))
    }
}

/// The identifier and optional value of a variable declaration
#[derive(PartialEq, Debug, Clone)]
pub struct VariableDecl {
    pub id: Pat,
    pub init: Option<Expr>,
}

impl VariableDecl {
    pub fn new(id: Pat, init: Option<Expr>) -> Self {
        VariableDecl {
            id,
            init,
        }
    }

    pub fn uninitialized(name: &str) -> Self {
        Self {
            id: Pat::Identifier(String::from(name)),
            init: None,
        }
    }

    pub fn with_value(name: &str, value: Expr) -> Self {
        Self {
            id: Pat::Identifier(String::from(name)),
            init: Some(value),
        }
    }

    pub fn destructed(names: &[&str], value: ObjectExpr) -> Self {
        let id = Pat::Object(
            names
                .iter()
                .map(|name| {
                    ObjectPatPart::Assignment(Property {
                        key: PropertyKey::Expr(Expr::ident(&name.to_string())),
                        value: PropertyValue::None,
                        kind: PropertyKind::Init,
                        method: false,
                        short_hand: true,
                        computed: false,
                        is_static: false,
                    })
                })
                .collect(),
        );
        Self {
            id,
            init: Some(Expr::Object(value)),
        }
    }

    pub fn destructed_with_rest(names: &[&str], rest: &str, value: ObjectExpr) -> Self {
        let mut props: Vec<ObjectPatPart> = names
            .iter()
            .map(|name| {
                ObjectPatPart::Assignment(Property {
                    key: PropertyKey::Expr(Expr::Ident(String::from(*name))),
                    value: PropertyValue::None,
                    kind: PropertyKind::Init,
                    computed: false,
                    method: false,
                    short_hand: true,
                    is_static: false,
                })
            })
            .collect();
        props.push(ObjectPatPart::Rest(Box::new(Pat::RestElement(
            Box::new(Pat::Identifier(String::from(rest))),
        ))));
        let id = Pat::Object(props);
        let init = Some(Expr::Object(value));
        Self { id, init }
    }
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
pub enum ModDecl {
    Import(ModImport),
    Export(ModExport),
}

impl ModDecl {
    pub fn import(inner: ModImport) -> Self {
        ModDecl::Import(inner)
    }
    pub fn export(inner: ModExport) -> Self {
        ModDecl::Export(inner)
    }
}

/// A declaration that imports exported
/// members of another module
///
/// ```js
/// import {Thing} from './stuff.js';
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct ModImport {
    pub specifiers: Vec<ImportSpecifier>,
    pub source: Literal,
}

impl ModImport {
    pub fn new(specs: Vec<ImportSpecifier>, source: String) -> Self {
        Self {
            specifiers: specs,
            source: Literal::String(source),
        }
    }
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

impl ImportSpecifier {
    pub fn normal(ident: Identifier, module: Option<Identifier>) -> Self {
        ImportSpecifier::Normal(ident, module)
    }
    pub fn default(ident: Identifier) -> Self {
        ImportSpecifier::Default(ident)
    }
    pub fn namespace(ident: Identifier) -> Self {
        ImportSpecifier::Namespace(ident)
    }
}

/// Something exported from this module
#[derive(PartialEq, Debug, Clone)]
pub enum ModExport {
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

impl ModExport {
    pub fn default(default: DefaultExportDecl) -> Self {
        ModExport::Default(default)
    }

    pub fn named(named: NamedExportDecl) -> Self {
        ModExport::Named(named)
    }
    pub fn all(lit: Literal) -> Self {
        ModExport::All(lit)
    }
}

/// An export that has a name
/// ```js
/// export function thing() {}
/// export {stuff} from 'place';
#[derive(PartialEq, Debug, Clone)]
pub enum NamedExportDecl {
    Decl(Decl),
    Specifier(Vec<ExportSpecifier>, Option<Literal>),
}

impl NamedExportDecl {
    pub fn decl(decl: Decl) -> Self {
        NamedExportDecl::Decl(decl)
    }
    pub fn specifier(exports: Vec<ExportSpecifier>, path: Option<Literal>) -> Self {
        NamedExportDecl::Specifier(exports, path)
    }
}

/// A default export
/// ```js
/// export default class Thing {}
/// ```
#[derive(PartialEq, Debug, Clone)]
pub enum DefaultExportDecl {
    Decl(Decl),
    Expr(Expr),
}

impl DefaultExportDecl {
    pub fn decl(decl: Decl) -> Self {
        DefaultExportDecl::Decl(decl)
    }
    pub fn expr(expr: Expr) -> Self {
        DefaultExportDecl::Expr(expr)
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
pub struct ExportSpecifier {
    pub local: Identifier,
    pub exported: Option<Identifier>,
}

impl ExportSpecifier {
    pub fn new(local: Identifier, exported: Option<Identifier>) -> ExportSpecifier {
        ExportSpecifier {
            local,
            exported,
        }
    }
}