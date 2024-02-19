//! All conversions from spanned into non-spanned types
//!

use std::borrow::Cow;

use crate::spanned::{
    decl::{
        Alias, Decl, DefaultExportDeclValue, DefaultImportSpec, ExportSpecifier, ImportSpecifier,
        ModExport, ModExportSpecifier, ModImport, NamedExportDecl, NamespaceImportSpec,
        NormalImportSpec, VarDecl,
    },
    expr::{
        ArrowFuncBody, ArrowFuncExpr, AssignExpr, AssignLeft, BinaryExpr, CallExpr,
        ConditionalExpr, Expr, Lit, LogicalExpr, MemberExpr, MetaProp, NewExpr, ObjProp, Prop,
        PropInitKey, PropKey, PropMethod, PropValue, RegEx, SequenceExprEntry, StringLit,
        TaggedTemplateExpr, TemplateElement, TemplateLit, UnaryExpr, UpdateExpr, YieldExpr,
    },
    pat::{ArrayElement, ArrayPat, ArrayPatPart, AssignPat, ObjPat, ObjPatPart, Pat},
    stmt::{
        BlockStmt, CatchClause, DoWhileStmt, FinallyClause, ForInStmt, ForOfStmt, ForStmt, IfStmt,
        LabeledStmt, LoopInit, LoopLeft, Stmt, SwitchCase, SwitchStmt, TryStmt, WhileStmt,
        WithStmt,
    },
    tokens::{AssignOp, BinaryOp, LogicalOp, UnaryOp, UpdateOp},
    Class, ClassBody, Dir, Func, FuncArg, FuncArgEntry, FuncBody, Ident, Program, ProgramPart,
    Slice, VarKind,
};

mod decl {
    use super::*;

    impl<T> From<Decl<T>> for crate::decl::Decl<T> {
        fn from(other: Decl<T>) -> Self {
            match other {
                Decl::Var { decls, .. } => crate::decl::Decl::Var(
                    decls.keyword.into(),
                    decls.decls.into_iter().map(|d| d.item.into()).collect(),
                ),
                Decl::Func(inner) => crate::decl::Decl::Func(inner.into()),
                Decl::Class(c) => crate::decl::Decl::Class(c.into()),
                Decl::Import { import, .. } => {
                    crate::decl::Decl::Import(Box::new((*import).into()))
                }
                Decl::Export { export, .. } => {
                    crate::decl::Decl::Export(Box::new((*export).into()))
                }
            }
        }
    }

    impl<T> From<VarDecl<T>> for crate::decl::VarDecl<T> {
        fn from(other: VarDecl<T>) -> Self {
            Self {
                id: other.id.into(),
                init: other.init.map(From::from),
            }
        }
    }

    impl<T> From<ModImport<T>> for crate::decl::ModImport<T> {
        fn from(other: ModImport<T>) -> Self {
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

    impl<T> From<ImportSpecifier<T>> for crate::decl::ImportSpecifier<T> {
        fn from(other: ImportSpecifier<T>) -> Self {
            match other {
                ImportSpecifier::Normal(inner) => {
                    Self::Normal(inner.specs.into_iter().map(|e| e.item.into()).collect())
                }
                ImportSpecifier::Default(inner) => Self::Default(inner.into()),
                ImportSpecifier::Namespace(inner) => Self::Namespace(inner.into()),
            }
        }
    }

    impl<T> From<NormalImportSpec<T>> for crate::decl::NormalImportSpec<T> {
        fn from(other: NormalImportSpec<T>) -> Self {
            let imported: crate::Ident<T> = other.imported.into();
            let alias = other.alias.map(|a| a.ident.into());

            Self { imported, alias }
        }
    }

    impl<T> From<DefaultImportSpec<T>> for crate::Ident<T> {
        fn from(other: DefaultImportSpec<T>) -> Self {
            other.id.into()
        }
    }

    impl<T> From<NamespaceImportSpec<T>> for crate::Ident<T> {
        fn from(other: NamespaceImportSpec<T>) -> Self {
            other.ident.into()
        }
    }

    impl<T> From<ModExport<T>> for crate::decl::ModExport<T> {
        fn from(other: ModExport<T>) -> Self {
            other.spec.into()
        }
    }

    impl<T> From<ModExportSpecifier<T>> for crate::decl::ModExport<T> {
        fn from(other: ModExportSpecifier<T>) -> Self {
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
                    name: name.into(),
                },
            }
        }
    }
    impl<T> From<NamedExportDecl<T>> for crate::decl::NamedExportDecl<T> {
        fn from(other: NamedExportDecl<T>) -> Self {
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

    impl<T> From<DefaultExportDeclValue<T>> for crate::decl::DefaultExportDecl<T> {
        fn from(other: DefaultExportDeclValue<T>) -> Self {
            match other {
                DefaultExportDeclValue::Decl(inner) => Self::Decl(inner.into()),
                DefaultExportDeclValue::Expr(inner) => Self::Expr(inner.into()),
            }
        }
    }

    impl<T> From<ExportSpecifier<T>> for crate::decl::ExportSpecifier<T> {
        fn from(other: ExportSpecifier<T>) -> Self {
            let local: crate::Ident<T> = other.local.into();
            Self {
                local,
                alias: other.alias.map(|a| a.ident.into()),
            }
        }
    }

    impl<T> From<Alias<T>> for crate::Ident<T> {
        fn from(other: Alias<T>) -> Self {
            other.ident.into()
        }
    }
}

mod expr {
    use crate::spanned::{
        expr::Boolean,
        tokens::{QuasiQuote, Quote},
    };

    use super::*;

    impl<T> From<Expr<T>> for crate::Expr<T> {
        fn from(other: Expr<T>) -> Self {
            match other {
                Expr::Array(inner) => Self::Array(
                    inner
                        .elements
                        .into_iter()
                        .map(|e| e.item.map(From::from))
                        .collect(),
                ),
                Expr::ArrowFunc(inner) => Self::ArrowFunc(inner.into()),
                Expr::ArrowParamPlaceHolder(inner) => Self::ArrowParamPlaceHolder(
                    inner.args.into_iter().map(|e| From::from(e.item)).collect(),
                    inner.keyword.is_some(),
                ),
                Expr::Assign(inner) => Self::Assign(inner.into()),
                Expr::Await(inner) => Self::Await(Box::new(inner.expr.into())),
                Expr::Binary(inner) => Self::Binary(inner.into()),
                Expr::Class(inner) => Self::Class((*inner).into()),
                Expr::Call(inner) => Self::Call(inner.into()),
                Expr::Conditional(inner) => Self::Conditional(inner.into()),
                Expr::Func(inner) => Self::Func(inner.into()),
                Expr::Ident(inner) => Self::Ident(inner.into()),
                Expr::Lit(inner) => Self::Lit(inner.into()),
                Expr::Logical(inner) => Self::Logical(inner.into()),
                Expr::Member(inner) => Self::Member(inner.into()),
                Expr::MetaProp(inner) => Self::MetaProp(inner.into()),
                Expr::New(inner) => Self::New(inner.into()),
                Expr::Obj(inner) => {
                    Self::Obj(inner.props.into_iter().map(|e| e.item.into()).collect())
                }
                Expr::Sequence(inner) => {
                    Self::Sequence(inner.into_iter().map(|e| e.item.into()).collect())
                }
                Expr::Spread(inner) => Self::Spread(Box::new(inner.expr.into())),
                Expr::Super(_) => Self::Super,
                Expr::TaggedTemplate(inner) => Self::TaggedTemplate(inner.into()),
                Expr::This(_) => Self::This,
                Expr::Unary(inner) => Self::Unary(inner.into()),
                Expr::Update(inner) => Self::Update(inner.into()),
                Expr::Yield(inner) => Self::Yield(inner.into()),
                Expr::Wrapped(inner) => inner.expr.into(),
                Expr::OptionalChain(inner) => Self::OptionalChain(Box::new((*inner.expr).into())),
            }
        }
    }

    impl<T> From<ObjProp<T>> for crate::expr::ObjProp<T> {
        fn from(other: ObjProp<T>) -> Self {
            match other {
                ObjProp::Prop(inner) => Self::Prop(inner.into()),
                ObjProp::Spread(inner) => Self::Spread(inner.expr.into()),
            }
        }
    }

    impl<T> From<Prop<T>> for crate::expr::Prop<T> {
        fn from(other: Prop<T>) -> Self {
            match other {
                Prop::Init(inner) => Self {
                    computed: inner.key.brackets.is_some(),
                    short_hand: inner.colon.is_none(),
                    key: inner.key.into(),
                    value: inner
                        .value
                        .map(From::from)
                        .unwrap_or(crate::expr::PropValue::None),
                    kind: crate::PropKind::Init,
                    method: false,
                    is_static: false,
                },
                Prop::Method(inner) => Self {
                    computed: inner.id.brackets.is_some(),
                    key: inner.id.into(),
                    value: crate::prelude::PropValue::Expr(crate::Expr::Func(crate::Func {
                        body: inner.body.into(),
                        generator: inner.star.is_some(),
                        id: None,
                        is_async: inner.keyword_async.is_some(),
                        params: inner.params.into_iter().map(|e| e.item.into()).collect(),
                    })),
                    kind: crate::PropKind::Method,
                    method: true,
                    short_hand: false,
                    is_static: inner.keyword_static.is_some(),
                },
                Prop::Ctor(inner) => Self {
                    computed: inner.keyword.brackets.is_some(),
                    key: inner.keyword.into(),
                    value: crate::prelude::PropValue::Expr(crate::Expr::Func(crate::Func {
                        body: inner.body.into(),
                        generator: false,
                        id: None,
                        is_async: false,
                        params: inner.params.into_iter().map(|e| e.item.into()).collect(),
                    })),
                    kind: crate::PropKind::Ctor,
                    is_static: false,
                    method: true,
                    short_hand: false,
                },
                Prop::Get(inner) => Self {
                    computed: inner.id.brackets.is_some(),
                    key: inner.id.into(),
                    value: crate::prelude::PropValue::Expr(crate::Expr::Func(crate::Func {
                        body: inner.body.into(),
                        generator: false,
                        id: None,
                        is_async: false,
                        params: Vec::new(),
                    })),
                    kind: crate::PropKind::Get,
                    method: false,
                    short_hand: false,
                    is_static: inner.keyword_static.is_some(),
                },
                Prop::Set(inner) => Self {
                    computed: inner.id.brackets.is_some(),
                    key: inner.id.into(),
                    value: crate::prelude::PropValue::Expr(crate::Expr::Func(crate::Func {
                        body: inner.body.into(),
                        generator: false,
                        id: None,
                        is_async: false,
                        params: vec![inner.arg.item.into()],
                    })),
                    kind: crate::PropKind::Set,
                    method: false,
                    short_hand: false,
                    is_static: inner.keyword_static.is_some(),
                },
            }
        }
    }

    impl<T> From<PropInitKey<T>> for crate::expr::PropKey<T> {
        fn from(other: PropInitKey<T>) -> Self {
            other.value.into()
        }
    }

    impl<T> From<PropMethod<T>> for crate::Func<T> {
        fn from(other: PropMethod<T>) -> Self {
            crate::Func {
                id: None,
                params: other.params.into_iter().map(|e| e.item.into()).collect(),
                body: other.body.into(),
                generator: other.star.is_some(),
                is_async: other.keyword_async.is_some(),
            }
        }
    }

    impl<T> From<PropKey<T>> for crate::expr::PropKey<T> {
        fn from(other: PropKey<T>) -> Self {
            match other {
                PropKey::Lit(inner) => Self::Lit(inner.into()),
                PropKey::Expr(inner) => Self::Expr(inner.into()),
                PropKey::Pat(inner) => Self::Pat(inner.into()),
            }
        }
    }

    impl<T> From<PropValue<T>> for crate::expr::PropValue<T> {
        fn from(other: PropValue<T>) -> Self {
            match other {
                PropValue::Expr(inner) => Self::Expr(inner.into()),
                PropValue::Pat(inner) => Self::Pat(inner.into()),
                PropValue::Method(inner) => Self::Expr(crate::expr::Expr::Func(inner.into())),
            }
        }
    }

    impl<T> From<UnaryExpr<T>> for crate::expr::UnaryExpr<T> {
        fn from(other: UnaryExpr<T>) -> Self {
            Self {
                prefix: other.prefix(),
                operator: other.operator.into(),
                argument: Box::new(From::from(*other.argument)),
            }
        }
    }

    impl<T> From<UpdateExpr<T>> for crate::expr::UpdateExpr<T> {
        fn from(other: UpdateExpr<T>) -> Self {
            Self {
                prefix: other.prefix(),
                operator: other.operator.into(),
                argument: Box::new(From::from(*other.argument)),
            }
        }
    }

    impl<T> From<BinaryExpr<T>> for crate::expr::BinaryExpr<T> {
        fn from(other: BinaryExpr<T>) -> Self {
            Self {
                operator: other.operator.into(),
                left: Box::new(From::from(*other.left)),
                right: Box::new(From::from(*other.right)),
            }
        }
    }

    impl<T> From<AssignExpr<T>> for crate::expr::AssignExpr<T> {
        fn from(other: AssignExpr<T>) -> Self {
            Self {
                operator: other.operator.into(),
                left: other.left.into(),
                right: Box::new(From::from(*other.right)),
            }
        }
    }

    impl<T> From<AssignLeft<T>> for crate::expr::AssignLeft<T> {
        fn from(other: AssignLeft<T>) -> Self {
            match other {
                AssignLeft::Pat(inner) => Self::Pat(inner.into()),
                AssignLeft::Expr(inner) => Self::Expr(Box::new(From::from(*inner))),
            }
        }
    }

    impl<T> From<LogicalExpr<T>> for crate::expr::LogicalExpr<T> {
        fn from(other: LogicalExpr<T>) -> Self {
            Self {
                operator: other.operator.into(),
                left: Box::new(From::from(*other.left)),
                right: Box::new(From::from(*other.right)),
            }
        }
    }

    impl<T> From<MemberExpr<T>> for crate::expr::MemberExpr<T> {
        fn from(other: MemberExpr<T>) -> Self {
            let computed = other.computed();
            Self {
                object: Box::new(From::from(*other.object)),
                property: Box::new(From::from(*other.property)),
                computed,
            }
        }
    }

    impl<T> From<ConditionalExpr<T>> for crate::expr::ConditionalExpr<T> {
        fn from(other: ConditionalExpr<T>) -> Self {
            Self {
                test: Box::new(From::from(*other.test)),
                alternate: Box::new(From::from(*other.alternate)),
                consequent: Box::new(From::from(*other.consequent)),
            }
        }
    }

    impl<T> From<CallExpr<T>> for crate::expr::CallExpr<T> {
        fn from(other: CallExpr<T>) -> Self {
            Self {
                callee: Box::new(From::from(*other.callee)),
                arguments: other.arguments.into_iter().map(|e| e.item.into()).collect(),
            }
        }
    }

    impl<T> From<NewExpr<T>> for crate::expr::NewExpr<T> {
        fn from(other: NewExpr<T>) -> Self {
            Self {
                callee: Box::new(From::from(*other.callee)),
                arguments: other.arguments.into_iter().map(|e| e.item.into()).collect(),
            }
        }
    }

    impl<T> From<ArrowFuncExpr<T>> for crate::expr::ArrowFuncExpr<T> {
        fn from(other: ArrowFuncExpr<T>) -> Self {
            let expression = matches!(&other.body, ArrowFuncBody::Expr(_));
            Self {
                id: None,
                params: other.params.into_iter().map(|e| e.item.into()).collect(),
                body: other.body.into(),
                expression,
                generator: other.star.is_some(),
                is_async: other.keyword.is_some(),
            }
        }
    }

    impl<T> From<ArrowFuncBody<T>> for crate::expr::ArrowFuncBody<T> {
        fn from(other: ArrowFuncBody<T>) -> Self {
            match other {
                ArrowFuncBody::FuncBody(inner) => Self::FuncBody(inner.into()),
                ArrowFuncBody::Expr(inner) => Self::Expr(Box::new(From::from(*inner))),
            }
        }
    }

    impl<T> From<YieldExpr<T>> for crate::expr::YieldExpr<T> {
        fn from(other: YieldExpr<T>) -> Self {
            Self {
                argument: other.argument.map(|e| Box::new(From::from(*e))),
                delegate: other.star.is_some(),
            }
        }
    }

    impl<T> From<TaggedTemplateExpr<T>> for crate::expr::TaggedTemplateExpr<T> {
        fn from(other: TaggedTemplateExpr<T>) -> Self {
            Self {
                tag: Box::new(From::from(*other.tag)),
                quasi: other.quasi.into(),
            }
        }
    }

    impl<T> From<TemplateLit<T>> for crate::expr::TemplateLit<T> {
        fn from(other: TemplateLit<T>) -> Self {
            Self {
                quasis: other.quasis.into_iter().map(From::from).collect(),
                expressions: other.expressions.into_iter().map(From::from).collect(),
            }
        }
    }

    impl From<QuasiQuote> for crate::expr::QuasiQuote {
        fn from(other: QuasiQuote) -> Self {
            match other {
                QuasiQuote::BackTick(_) => Self::BackTick,
                QuasiQuote::CloseBrace(_) => Self::CloseBrace,
                QuasiQuote::OpenBrace(_) => Self::OpenBrace,
            }
        }
    }

    impl<T> From<TemplateElement<T>> for crate::expr::TemplateElement<T> {
        fn from(other: TemplateElement<T>) -> Self {
            Self {
                open_quote: other.open_quote.into(),
                content: other.content.source,
                close_quote: other.close_quote.into(),
            }
        }
    }

    impl<T> From<MetaProp<T>> for crate::expr::MetaProp<T> {
        fn from(other: MetaProp<T>) -> Self {
            Self {
                meta: other.meta.into(),
                property: other.property.into(),
            }
        }
    }

    impl<T> From<Lit<T>> for crate::expr::Lit<T> {
        fn from(other: Lit<T>) -> Self {
            match other {
                Lit::Null(_inner) => Self::Null,
                Lit::String(inner) => Self::String(inner.into()),
                Lit::Number(inner) => Self::Number(inner.source),
                Lit::Boolean(inner) => Self::Boolean(inner.into()),
                Lit::RegEx(inner) => Self::RegEx(inner.into()),
                Lit::Template(inner) => Self::Template(inner.into()),
            }
        }
    }

    impl From<Boolean> for bool {
        fn from(other: Boolean) -> Self {
            match other {
                Boolean::True(_) => true,
                Boolean::False(_) => false,
            }
        }
    }

    impl<T> From<StringLit<T>> for crate::expr::StringLit<T> {
        fn from(other: StringLit<T>) -> Self {
            if matches!(other.open_quote, Quote::Double(_)) {
                Self::Double(other.content.source)
            } else {
                Self::Single(other.content.source)
            }
        }
    }

    impl<T> From<RegEx<T>> for crate::expr::RegEx<T> {
        fn from(other: RegEx<T>) -> Self {
            Self {
                pattern: other.pattern.source,
                flags: other.flags.map(|f| f.source),
            }
        }
    }

    impl<T> From<SequenceExprEntry<T>> for crate::expr::Expr<T> {
        fn from(other: SequenceExprEntry<T>) -> Self {
            other.expr.into()
        }
    }
}

impl<T> From<Ident<T>> for crate::Ident<T> {
    fn from(other: Ident<T>) -> Self {
        Self {
            name: other.slice.source,
        }
    }
}

impl<T> From<Dir<T>> for crate::Dir<T> {
    fn from(other: Dir<T>) -> Self {
        Self {
            expr: other.expr.into(),
            dir: other.dir,
        }
    }
}

impl<T> From<Func<T>> for crate::Func<T> {
    fn from(other: Func<T>) -> Self {
        Self {
            generator: other.generator(),
            is_async: other.is_async(),
            id: other.id.map(From::from),
            params: other
                .params
                .into_iter()
                .map(|e| From::from(e.item))
                .collect(),
            body: other.body.into(),
        }
    }
}

impl<T> From<FuncArgEntry<T>> for crate::FuncArg<T> {
    fn from(other: FuncArgEntry<T>) -> Self {
        other.value.into()
    }
}

impl<T> From<FuncArg<T>> for crate::FuncArg<T> {
    fn from(other: FuncArg<T>) -> Self {
        match other {
            FuncArg::Expr(inner) => Self::Expr(inner.into()),
            FuncArg::Pat(inner) => Self::Pat(inner.into()),
            FuncArg::Rest(inner) => {
                Self::Pat(crate::pat::Pat::RestElement(Box::new(inner.pat.into())))
            }
        }
    }
}

impl<T> From<Program<T>> for crate::Program<T> {
    fn from(other: Program<T>) -> Self {
        match other {
            Program::Mod(inner) => Self::Mod(inner.into_iter().map(From::from).collect()),
            Program::Script(inner) => Self::Script(inner.into_iter().map(From::from).collect()),
        }
    }
}

impl<T> From<ProgramPart<T>> for crate::ProgramPart<T> {
    fn from(other: ProgramPart<T>) -> Self {
        match other {
            ProgramPart::Dir(inner) => Self::Dir(inner.into()),
            ProgramPart::Decl(inner) => Self::Decl(inner.into()),
            ProgramPart::Stmt(inner) => Self::Stmt(inner.into()),
        }
    }
}

impl<T> From<FuncBody<T>> for crate::FuncBody<T> {
    fn from(other: FuncBody<T>) -> Self {
        Self(other.stmts.into_iter().map(From::from).collect())
    }
}

impl<T> From<Class<T>> for crate::Class<T> {
    fn from(other: Class<T>) -> Self {
        Self {
            id: other.id.map(From::from),
            super_class: other.super_class.map(|e| Box::new(From::from(e.expr))),
            body: other.body.into(),
        }
    }
}

impl<T> From<ClassBody<T>> for crate::ClassBody<T> {
    fn from(other: ClassBody<T>) -> Self {
        Self(other.props.into_iter().map(From::from).collect())
    }
}

impl From<VarKind> for crate::VarKind {
    fn from(other: VarKind) -> Self {
        match other {
            VarKind::Var(_) => Self::Var,
            VarKind::Let(_) => Self::Let,
            VarKind::Const(_) => Self::Const,
        }
    }
}

impl From<AssignOp> for crate::AssignOp {
    fn from(other: AssignOp) -> Self {
        match other {
            AssignOp::Equal(_) => Self::Equal,
            AssignOp::PlusEqual(_) => Self::PlusEqual,
            AssignOp::MinusEqual(_) => Self::MinusEqual,
            AssignOp::TimesEqual(_) => Self::TimesEqual,
            AssignOp::DivEqual(_) => Self::DivEqual,
            AssignOp::ModEqual(_) => Self::ModEqual,
            AssignOp::LeftShiftEqual(_) => Self::LeftShiftEqual,
            AssignOp::RightShiftEqual(_) => Self::RightShiftEqual,
            AssignOp::UnsignedRightShiftEqual(_) => Self::UnsignedRightShiftEqual,
            AssignOp::OrEqual(_) => Self::OrEqual,
            AssignOp::XOrEqual(_) => Self::XOrEqual,
            AssignOp::AndEqual(_) => Self::AndEqual,
            AssignOp::PowerOfEqual(_) => Self::PowerOfEqual,
            AssignOp::DoubleAmpersandEqual(_) => Self::DoubleAmpersandEqual,
            AssignOp::DoublePipeEqual(_) => Self::DoublePipeEqual,
            AssignOp::DoubleQuestionmarkEqual(_) => Self::DoubleQuestionmarkEqual,
        }
    }
}

impl From<LogicalOp> for crate::LogicalOp {
    fn from(other: LogicalOp) -> Self {
        match other {
            LogicalOp::Or(_) => Self::Or,
            LogicalOp::And(_) => Self::And,
            LogicalOp::NullishCoalescing(_) => Self::NullishCoalescing,
        }
    }
}

impl From<BinaryOp> for crate::BinaryOp {
    fn from(other: BinaryOp) -> Self {
        match other {
            BinaryOp::Equal(_) => Self::Equal,
            BinaryOp::NotEqual(_) => Self::NotEqual,
            BinaryOp::StrictEqual(_) => Self::StrictEqual,
            BinaryOp::StrictNotEqual(_) => Self::StrictNotEqual,
            BinaryOp::LessThan(_) => Self::LessThan,
            BinaryOp::GreaterThan(_) => Self::GreaterThan,
            BinaryOp::LessThanEqual(_) => Self::LessThanEqual,
            BinaryOp::GreaterThanEqual(_) => Self::GreaterThanEqual,
            BinaryOp::LeftShift(_) => Self::LeftShift,
            BinaryOp::RightShift(_) => Self::RightShift,
            BinaryOp::UnsignedRightShift(_) => Self::UnsignedRightShift,
            BinaryOp::Plus(_) => Self::Plus,
            BinaryOp::Minus(_) => Self::Minus,
            BinaryOp::Times(_) => Self::Times,
            BinaryOp::Over(_) => Self::Over,
            BinaryOp::Mod(_) => Self::Mod,
            BinaryOp::Or(_) => Self::Or,
            BinaryOp::XOr(_) => Self::XOr,
            BinaryOp::And(_) => Self::And,
            BinaryOp::In(_) => Self::In,
            BinaryOp::InstanceOf(_) => Self::InstanceOf,
            BinaryOp::PowerOf(_) => Self::PowerOf,
        }
    }
}

impl From<UpdateOp> for crate::UpdateOp {
    fn from(other: UpdateOp) -> Self {
        match other {
            UpdateOp::Increment(_) => Self::Increment,
            UpdateOp::Decrement(_) => Self::Decrement,
        }
    }
}

impl From<UnaryOp> for crate::UnaryOp {
    fn from(other: UnaryOp) -> Self {
        match other {
            UnaryOp::Minus(_) => Self::Minus,
            UnaryOp::Plus(_) => Self::Plus,
            UnaryOp::Not(_) => Self::Not,
            UnaryOp::Tilde(_) => Self::Tilde,
            UnaryOp::TypeOf(_) => Self::TypeOf,
            UnaryOp::Void(_) => Self::Void,
            UnaryOp::Delete(_) => Self::Delete,
        }
    }
}

mod pat {
    use super::*;

    impl<T> From<Pat<T>> for crate::pat::Pat<T> {
        fn from(other: Pat<T>) -> Self {
            match other {
                Pat::Ident(inner) => Self::Ident(inner.into()),
                Pat::Obj(inner) => Self::Obj(inner.into()),
                Pat::Array(inner) => Self::Array(inner.into()),
                Pat::Assign(inner) => Self::Assign(inner.into()),
            }
        }
    }

    impl<T> From<ArrayPat<T>> for Vec<Option<crate::pat::ArrayPatPart<T>>> {
        fn from(other: ArrayPat<T>) -> Self {
            other
                .elements
                .into_iter()
                .map(|e| e.item.map(Into::into))
                .collect()
        }
    }

    impl<T> From<ArrayElement<T>> for Option<crate::pat::ArrayPatPart<T>> {
        fn from(other: ArrayElement<T>) -> Self {
            other.part.map(From::from)
        }
    }

    impl<T> From<ArrayPatPart<T>> for crate::pat::ArrayPatPart<T> {
        fn from(other: ArrayPatPart<T>) -> Self {
            match other {
                ArrayPatPart::Pat(inner) => Self::Pat(inner.into()),
                ArrayPatPart::Expr(inner) => Self::Expr(inner.into()),
                ArrayPatPart::Rest(inner) => {
                    Self::Pat(crate::pat::Pat::RestElement(Box::new(inner.pat.into())))
                }
            }
        }
    }

    impl<T> From<ObjPat<T>> for crate::pat::ObjPat<T> {
        fn from(other: ObjPat<T>) -> Self {
            other.props.into_iter().map(|e| e.item.into()).collect()
        }
    }

    impl<T> From<ObjPatPart<T>> for crate::pat::ObjPatPart<T> {
        fn from(other: ObjPatPart<T>) -> Self {
            match other {
                ObjPatPart::Assign(prop) => Self::Assign(prop.into()),
                ObjPatPart::Rest(inner) => Self::Rest(Box::new(From::from(inner.pat))),
            }
        }
    }

    impl<T> From<AssignPat<T>> for crate::pat::AssignPat<T> {
        fn from(other: AssignPat<T>) -> Self {
            Self {
                left: Box::new(From::from(*other.left)),
                right: Box::new(From::from(*other.right)),
            }
        }
    }
}
mod stmt {
    use super::*;

    impl<T> From<Stmt<T>> for crate::stmt::Stmt<T> {
        fn from(other: Stmt<T>) -> Self {
            match other {
                Stmt::Expr { expr, .. } => Self::Expr(expr.into()),
                Stmt::Block(inner) => Self::Block(inner.into()),
                Stmt::Empty(_) => Self::Empty,
                Stmt::Debugger { .. } => Self::Debugger,
                Stmt::With(inner) => Self::With(inner.into()),
                Stmt::Return { value, .. } => Self::Return(value.map(From::from)),
                Stmt::Labeled(inner) => Self::Labeled(inner.into()),
                Stmt::Break { label, .. } => Self::Break(label.map(From::from)),
                Stmt::Continue { label, .. } => Self::Continue(label.map(From::from)),
                Stmt::If(inner) => Self::If(inner.into()),
                Stmt::Switch(inner) => Self::Switch(inner.into()),
                Stmt::Throw { expr, .. } => Self::Throw(expr.into()),
                Stmt::Try(inner) => Self::Try(inner.into()),
                Stmt::While(inner) => Self::While(inner.into()),
                Stmt::DoWhile(inner) => Self::DoWhile(inner.into()),
                Stmt::For(inner) => Self::For(inner.into()),
                Stmt::ForIn(inner) => Self::ForIn(inner.into()),
                Stmt::ForOf(inner) => Self::ForOf(inner.into()),
                Stmt::Var { decls, .. } => {
                    Self::Var(decls.decls.into_iter().map(|e| e.item.into()).collect())
                }
            }
        }
    }

    impl<T> From<WithStmt<T>> for crate::stmt::WithStmt<T> {
        fn from(other: WithStmt<T>) -> Self {
            Self {
                object: other.object.into(),
                body: Box::new(From::from(*other.body)),
            }
        }
    }

    impl<T> From<LabeledStmt<T>> for crate::stmt::LabeledStmt<T> {
        fn from(other: LabeledStmt<T>) -> Self {
            Self {
                label: other.label.into(),
                body: Box::new(From::from(*other.body)),
            }
        }
    }

    impl<T> From<IfStmt<T>> for crate::stmt::IfStmt<T> {
        fn from(other: IfStmt<T>) -> Self {
            Self {
                test: other.test.into(),
                consequent: Box::new(From::from(*other.consequent)),
                alternate: other.alternate.map(|s| Box::new(From::from(s.body))),
            }
        }
    }

    impl<T> From<SwitchStmt<T>> for crate::stmt::SwitchStmt<T> {
        fn from(other: SwitchStmt<T>) -> Self {
            Self {
                discriminant: other.discriminant.into(),
                cases: other.cases.into_iter().map(From::from).collect(),
            }
        }
    }

    impl<T> From<SwitchCase<T>> for crate::stmt::SwitchCase<T> {
        fn from(other: SwitchCase<T>) -> Self {
            Self {
                test: other.test.map(From::from),
                consequent: other.consequent.into_iter().map(From::from).collect(),
            }
        }
    }

    impl<T> From<BlockStmt<T>> for crate::stmt::BlockStmt<T> {
        fn from(other: BlockStmt<T>) -> Self {
            Self(other.stmts.into_iter().map(From::from).collect())
        }
    }

    impl<T> From<TryStmt<T>> for crate::stmt::TryStmt<T> {
        fn from(other: TryStmt<T>) -> Self {
            Self {
                block: other.block.into(),
                handler: other.handler.map(From::from),
                finalizer: other.finalizer.map(From::from),
            }
        }
    }

    impl<T> From<CatchClause<T>> for crate::stmt::CatchClause<T> {
        fn from(other: CatchClause<T>) -> Self {
            Self {
                param: other.param.map(|a| a.param.into()),
                body: other.body.into(),
            }
        }
    }

    impl<T> From<FinallyClause<T>> for crate::stmt::BlockStmt<T> {
        fn from(other: FinallyClause<T>) -> Self {
            other.body.into()
        }
    }

    impl<T> From<WhileStmt<T>> for crate::stmt::WhileStmt<T> {
        fn from(other: WhileStmt<T>) -> Self {
            Self {
                test: other.test.into(),
                body: Box::new(From::from(*other.body)),
            }
        }
    }

    impl<T> From<DoWhileStmt<T>> for crate::stmt::DoWhileStmt<T> {
        fn from(other: DoWhileStmt<T>) -> Self {
            Self {
                test: other.test.into(),
                body: Box::new(From::from(*other.body)),
            }
        }
    }

    impl<T> From<ForStmt<T>> for crate::stmt::ForStmt<T> {
        fn from(other: ForStmt<T>) -> Self {
            Self {
                init: other.init.map(From::from),
                test: other.test.map(From::from),
                update: other.update.map(From::from),
                body: Box::new(From::from(*other.body)),
            }
        }
    }

    impl<T> From<LoopInit<T>> for crate::stmt::LoopInit<T> {
        fn from(other: LoopInit<T>) -> Self {
            match other {
                LoopInit::Expr(inner) => Self::Expr(inner.into()),
                LoopInit::Variable(kind, decls) => Self::Variable(
                    kind.into(),
                    decls.into_iter().map(|e| e.item.into()).collect(),
                ),
            }
        }
    }

    impl<T> From<ForInStmt<T>> for crate::stmt::ForInStmt<T> {
        fn from(other: ForInStmt<T>) -> Self {
            Self {
                left: other.left.into(),
                right: other.right.into(),
                body: Box::new(From::from(*other.body)),
            }
        }
    }

    impl<T> From<ForOfStmt<T>> for crate::stmt::ForOfStmt<T> {
        fn from(other: ForOfStmt<T>) -> Self {
            Self {
                left: other.left.into(),
                right: other.right.into(),
                body: Box::new(From::from(*other.body)),
                is_await: other.is_await,
            }
        }
    }

    impl<T> From<LoopLeft<T>> for crate::stmt::LoopLeft<T> {
        fn from(other: LoopLeft<T>) -> Self {
            match other {
                LoopLeft::Expr(inner) => Self::Expr(inner.into()),
                LoopLeft::Variable(kind, decl) => Self::Variable(kind.into(), decl.into()),
                LoopLeft::Pat(inner) => Self::Pat(inner.into()),
            }
        }
    }
}

impl From<Slice<String>> for String {
    fn from(other: Slice<String>) -> Self {
        other.source
    }
}

impl<'a> From<Slice<&'a str>> for &'a str {
    fn from(other: Slice<&'a str>) -> &'a str {
        other.source
    }
}

impl<'a> From<Slice<Cow<'a, str>>> for Cow<'a, str> {
    fn from(other: Slice<Cow<'a, str>>) -> Cow<'a, str> {
        other.source
    }
}

impl<'a> From<Slice<&'a [u8]>> for &'a [u8] {
    fn from(other: Slice<&'a [u8]>) -> &'a [u8] {
        other.source
    }
}

impl<'a> From<Slice<Cow<'a, [u8]>>> for Cow<'a, [u8]> {
    fn from(other: Slice<Cow<'a, [u8]>>) -> Cow<'a, [u8]> {
        other.source
    }
}

impl From<Slice<Vec<u8>>> for Vec<u8> {
    fn from(other: Slice<Vec<u8>>) -> Self {
        other.source
    }
}
