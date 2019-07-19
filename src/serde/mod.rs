use serde::ser::{
    Serialize,
    Serializer,
    SerializeStruct,
};
use crate::prelude::*;
use unescape::unescape;
impl<'a> Serialize for Program<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Node", 3)?;
        state.serialize_field("type", "Program")?;
        match self {
            Program::Script(ref body) => {
                state.serialize_field("sourceType", "script")?;
                state.serialize_field("body", &body)?;
            },
            Program::Mod(ref body) => {
                state.serialize_field("sourceType", "module")?;
                state.serialize_field("body", body)?;
            }
        }
        state.end()
    }
}

impl<'a> Serialize for ProgramPart<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ProgramPart::Decl(ref d) => {
                d.serialize(serializer)
            },
            ProgramPart::Dir(ref d) => {
                d.serialize(serializer)
            },
            ProgramPart::Stmt(ref s) => {
                s.serialize(serializer)
            }
        }
    }
}
impl<'a> Serialize for Dir<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Node", 3)?;
        state.serialize_field("type", "ExpressionStatement")?;
        state.serialize_field("expression", &self.expr)?;
        if let Lit::String(ref sl) = self.expr {
            match sl {
                StringLit::Double(ref s) => if !s.is_empty() {
                    state.serialize_field("directive", &self.dir)?;
                },
                StringLit::Single(ref s) => if !s.is_empty() {
                    state.serialize_field("directive", &self.dir)?;
                }
            }
        }
        state.end()
    }
}
impl<'a> Serialize for Decl<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Decl::Func(ref f) => {
                let mut state = serializer.serialize_struct("Node", 7)?;
                state.serialize_field("type", "FunctionDeclaration")?;
                state.serialize_field("id", &f.id)?;
                state.serialize_field("body", &f.body)?;
                state.serialize_field("generator", &f.generator)?;
                state.serialize_field("async", &f.is_async)?;
                state.serialize_field("expression", &false)?;
                state.serialize_field("params", &f.params)?;
                state.end()
            },
            Decl::Class(ref c) => {
                let mut state = serializer.serialize_struct("Node", 4)?;
                state.serialize_field("type", "ClassDeclaration")?;
                state.serialize_field("body", &c.body)?;
                state.serialize_field("id", &c.id)?;
                state.serialize_field("superClass", &c.super_class)?;
                state.end()
            },
            Decl::Var(ref kind, ref vs) => {
                let mut state = serializer.serialize_struct("Node", 7)?;
                state.serialize_field("type", "VariableDeclaration")?;
                state.serialize_field("kind", kind)?;
                state.serialize_field("declarations", vs)?;
                state.end()
            }
            _ => unimplemented!(),
        }
    }
}

impl Serialize for VarKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match self {
            VarKind::Const => "const",
            VarKind::Let => "let",
            VarKind::Var => "var",
        };
        serializer.serialize_str(s)
    }
}
impl<'a> Serialize for Ident<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Node", 2)?;
        state.serialize_field("type", "Identifier")?;
        let unescaped = unescape(&self.name).unwrap_or(self.name.to_string());
        state.serialize_field("name", &unescaped)?;
        state.end()
    }
}

impl<'a> Serialize for VarDecl<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Node", 2)?;
        state.serialize_field("type", "VariableDeclarator")?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("init", &self.init)?;
        state.end()
    }
}

impl<'a> Serialize for FuncBody<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Node", 2)?;
        state.serialize_field("type", "BlockStatement")?;
        state.serialize_field("body", &self.0)?;
        state.end()
    }
}

impl<'a> Serialize for Stmt<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Stmt::Labeled(ref l) => {
                let mut state = serializer.serialize_struct("Node", 3)?;
                state.serialize_field("type", "LabeledStatement")?;
                state.serialize_field("label", &l.label)?;
                state.serialize_field("body", &l.body)?;
                state.end()
            },
            Stmt::Block(ref b) => {
                b.serialize(serializer)
            },
            Stmt::Break(ref b) => {
                let mut state = serializer.serialize_struct("Node", 2)?;
                state.serialize_field("type", "BreakStatement")?;
                state.serialize_field("label", &b)?;
                state.end()
            },
            Stmt::Continue(ref c) => {
                let mut state = serializer.serialize_struct("Node", 2)?;
                state.serialize_field("type", "ContinueStatement")?;
                state.serialize_field("label", &c)?;
                state.end()
            },
            Stmt::Debugger => {
                let mut state = serializer.serialize_struct("Node", 1)?;
                state.serialize_field("type", "DebuggerStatement")?;
                state.end()
            },
            Stmt::DoWhile(ref d) => {
                let mut state = serializer.serialize_struct("Node", 3)?;
                state.serialize_field("type", "DoWhileStatement")?;
                state.serialize_field("test", &d.test)?;
                state.serialize_field("body", &d.body)?;
                state.end()
            },
            Stmt::Empty => {
                let mut state = serializer.serialize_struct("Node", 1)?;
                state.serialize_field("type", "EmptyStatement")?;
                state.end()
            },
            Stmt::Expr(ref e) => {
                let mut state = serializer.serialize_struct("Node", 1)?;
                state.serialize_field("type", "ExpressionStatement")?;
                state.serialize_field("expression", e)?;
                state.end()
            },
            Stmt::For(ref f) => {
                let mut state = serializer.serialize_struct("Node", 5)?;
                state.serialize_field("type", "ForStatement")?;
                state.serialize_field("init", &f.init)?;
                state.serialize_field("test", &f.test)?;
                state.serialize_field("update", &f.update)?;
                state.serialize_field("body", &f.body)?;
                state.end()
            },
            Stmt::ForIn(ref f) => {
                let mut state = serializer.serialize_struct("Node", 4)?;
                state.serialize_field("type", "ForInStatement")?;
                state.serialize_field("left", &f.left)?;
                state.serialize_field("right", &f.right)?;
                state.serialize_field("body", &f.body)?;
                state.serialize_field("each", &false)?;
                state.end()
            },
            Stmt::ForOf(ref f) => {
                let mut state = serializer.serialize_struct("Node", 4)?;
                state.serialize_field("type", "ForOfStatement")?;
                state.serialize_field("left", &f.left)?;
                state.serialize_field("right", &f.right)?;
                state.serialize_field("body", &f.body)?;
                state.end()
            },
            Stmt::If(ref f) => {
                let mut state = serializer.serialize_struct("Node", 4)?;
                state.serialize_field("type", "IfStatement")?;
                state.serialize_field("test", &f.test)?;
                state.serialize_field("consequent", &f.consequent)?;
                state.serialize_field("alternate", &f.alternate)?;
                state.end()
            },
            Stmt::Return(ref r) => {
                let mut state = serializer.serialize_struct("Node", 2)?;
                state.serialize_field("type", "ReturnStatement")?;
                state.serialize_field("argument", r)?;
                state.end()
            },
            Stmt::Switch(ref s) => {
                let mut state = serializer.serialize_struct("Node", 3)?;
                state.serialize_field("type", "SwitchStatement")?;
                state.serialize_field("discriminant", &s.discriminant)?;
                state.serialize_field("cases", &s.cases)?;
                state.end()
            },
            Stmt::Throw(ref t) => {
                let mut state = serializer.serialize_struct("Node", 2)?;
                state.serialize_field("type", "ThrowStatement")?;
                state.serialize_field("argument", t)?;
                state.end()
            },
            Stmt::Try(ref t) => {
                let mut state = serializer.serialize_struct("Node", 4)?;
                state.serialize_field("type", "TryStatement")?;
                state.serialize_field("block", &t.block)?;
                state.serialize_field("handler", &t.handler)?;
                state.serialize_field("finalizer", &t.finalizer)?;
                state.end()
            },
            Stmt::Var(ref v) => {
                let mut state = serializer.serialize_struct("Node", 2)?;
                state.serialize_field("type", "VariableStatement")?;
                state.serialize_field("decls", &v)?;
                state.end()
            },
            Stmt::While(ref w) => {
                let mut state = serializer.serialize_struct("Node", 3)?;
                state.serialize_field("type", "WhileStatement")?;
                state.serialize_field("test", &w.test)?;
                state.serialize_field("body", &w.body)?;
                state.end()
            },
            Stmt::With(ref w) => {
                let mut state = serializer.serialize_struct("Node", 3)?;
                state.serialize_field("type", "WithStatement")?;
                state.serialize_field("object", &w.object)?;
                state.serialize_field("body", &w.body)?;
                state.end()
            },
        }
    }
}

impl<'a> Serialize for Lit<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Lit::Number(ref n) => {
                serialize_number(serializer, n)
            },
            Lit::String(ref sl) => {
                let mut state = serializer.serialize_struct("Node", 3)?;
                let (quote, value) = match sl {
                    StringLit::Double(ref s) => ('"', s),
                    StringLit::Single(ref s) => ('\'', s),
                };
                state.serialize_field("type", "Literal")?;
                let quoted = format!("{0}{1}{0}", quote, value);
                let inner = if let Some(esc) = unescaper(&quoted) {
                    esc
                } else {
                    value.to_string()
                };
                state.serialize_field("value", &inner[1..inner.len() - 1])?;
                state.serialize_field("raw", &quoted)?;
                state.end()
            },
            Lit::RegEx(ref r) => {
                let mut state = serializer.serialize_struct("Node", 4)?;
                state.serialize_field("type", "Literal")?;
                state.serialize_field("raw", &format!("/{}/{}", r.pattern, r.flags))?;
                state.serialize_field("value", &format_regex_value(r))?;
                state.serialize_field("regex", r)?;
                state.end()
            },
            Lit::Null => {
                let mut state = serializer.serialize_struct("Node", 3)?;
                state.serialize_field("type", "Literal")?;
                state.serialize_field("raw", "null")?;
                let value: Option<()> = None;
                state.serialize_field("value", &value)?;
                state.end()
            },
            Lit::Boolean(ref b) => {
                let mut state = serializer.serialize_struct("Node", 3)?;
                state.serialize_field("type", "Literal")?;
                let raw = if *b {
                    "true"
                } else {
                    "false"
                };
                state.serialize_field("raw", raw)?;
                state.serialize_field("value", b)?;
                state.end()
            },
            Lit::Template(ref t) => {
                let mut state = serializer.serialize_struct("Node", 3)?;
                state.serialize_field("type", "TemplateLiteral")?;
                state.serialize_field("quasis", &t.quasis)?;
                state.serialize_field("expressions", &t.expressions)?;
                state.end()
            },
        }
    }
}

fn format_regex_value(r: &RegEx) -> String {
    let mut ret = String::from("/");
    ret.push_str(&r.pattern.replace('/', r"\/"));
    ret.push('/');
    if r.flags.is_empty() {
        return ret;
    }
    if r.flags.contains('g') {
        ret.push('g');
    }
    if r.flags.contains('i') {
        ret.push('i');
    }
    if r.flags.contains('m') {
        ret.push('m');
    }
    if r.flags.contains('u') {
        ret.push('u')
    }
    if r.flags.contains('y') {
        ret.push('y')
    }
    ret.push_str(&r.flags.replace(|c| 
        c == 'g' 
        || c == 'i'
        || c == 'm'
        || c == 'u'
        || c == 'y', ""));
    ret
}

fn serialize_number<S>(s: S, n: &str) -> Result<S::Ok, S::Error>
where S: Serializer,
{
    let mut state = s.serialize_struct("Node", 3)?;
    state.serialize_field("type", "Literal")?;
    state.serialize_field("raw", &n)?;
    if n.starts_with("0") {
        if n.len() == 1 {
            serialize_int(&mut state, 10, n)?;
        } else if n[1..2].eq_ignore_ascii_case("x") {
            serialize_int(&mut state, 16, &n[2..])?;
        } else if n[1..2].eq_ignore_ascii_case("o") {
            serialize_int(&mut state, 8, &n[2..])?;
        } else if n[1..2].eq_ignore_ascii_case("b") {
            serialize_int(&mut state, 2, &n[2..])?;
        } else if n.chars().all(|c| c.is_digit(8)) {
            serialize_int(&mut state, 8, n)?;
        } else if n.contains('E') || n.contains('e') || n.contains('.') {
            serialize_float(&mut state, n)?;
        } else {
            serialize_int(&mut state, 10, n)?;
        }
    } else if n.contains('E') || n.contains('e') || n.contains('.') {
        serialize_float(&mut state, n)?;
    } else {
        serialize_int(&mut state, 10, n)?;
    }
    state.end()
}

fn serialize_int<T>(state: &mut T, radix: u32, n: &str) -> Result<(), T::Error>
where T: SerializeStruct,
{
    if let Ok(value) = i128::from_str_radix(n, radix) {
        if value < ::std::i32::MAX as i128 {
            state.serialize_field("value", &(value as i32))
        } else {
            state.serialize_field("value", &(value as f64))
        }
    } else {
        state.serialize_field("value", &::std::f32::NAN)
    }
}
fn serialize_float<T>(state: &mut T, n: &str) -> Result<(), T::Error>
where T: SerializeStruct,
{
    if let Ok(value) = n.parse::<f32>() {
        if value % 1.0 == 0.0 {
            state.serialize_field("value", &(value as i32))
        } else {
            state.serialize_field("value", &value)
        }
    } else {
        state.serialize_field("value", &::std::f32::NAN)
    }
}

impl<'a> Serialize for Expr<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Expr::Array(ref a) => {
                let mut state = serializer.serialize_struct("Node", 2)?;
                state.serialize_field("type", "ArrayExpression")?;
                state.serialize_field("elements", a)?;
                state.end()
            },
            Expr::ArrowFunc(ref a) => {
                let mut state = serializer.serialize_struct("Node", 6)?;
                state.serialize_field("type", "ArrorFunction")?;
                state.serialize_field("id", &a.id)?;
                state.serialize_field("expression", &a.expression)?;
                state.serialize_field("generator", &a.generator)?;
                state.serialize_field("async", &a.is_async)?;
                match a.body {
                    ArrowFuncBody::Expr(ref e) => {
                        state.serialize_field("body", e)?;
                    },
                    ArrowFuncBody::FuncBody(ref b) => {
                        state.serialize_field("body", b)?;
                    }
                }
                state.end()
            },
            Expr::ArrowParamPlaceHolder(_, _) => {
                unreachable!("ArrowParamPlaceHolder Expression should never be returned by the parsing process");
            },
            Expr::Assign(ref a) => {
                let mut state = serializer.serialize_struct("Node", 4)?;
                state.serialize_field("type", "AssignmentExpression")?;
                state.serialize_field("left", &a.left)?;
                state.serialize_field("operator", &a.operator)?;
                state.serialize_field("right", &a.right)?;
                state.end()
            },
            Expr::Await(ref a) => {
                let mut state = serializer.serialize_struct("Node", 2)?;
                state.serialize_field("type", "AwaitExpression")?;
                state.serialize_field("expression", a)?;
                state.end()
            },
            Expr::Binary(ref b) => {
                let mut state = serializer.serialize_struct("Node", 4)?;
                state.serialize_field("type", "BinaryExpression")?;
                state.serialize_field("left", &b.left)?;
                state.serialize_field("operator", &b.operator)?;
                state.serialize_field("right", &b.right)?;
                state.end()
            },
            Expr::Call(ref c) => {
                let mut state = serializer.serialize_struct("Node", 3)?;
                state.serialize_field("type", "CallExpression")?;
                state.serialize_field("callee", &c.callee)?;
                state.serialize_field("arguments", &c.arguments)?;
                state.end()
            },
            Expr::Class(ref c) => {
                let mut state = serializer.serialize_struct("Node", 4)?;
                state.serialize_field("type", "ClassExpression")?;
                state.serialize_field("id", &c.id)?;
                state.serialize_field("superClass", &c.super_class)?;
                state.serialize_field("body", &c.body)?;
                state.end()
            },
            Expr::Conditional(ref c) => {
                let mut state = serializer.serialize_struct("Node", 4)?;
                state.serialize_field("type", "ConditionalExpression")?;
                state.serialize_field("test", &c.test)?;
                state.serialize_field("consequent", &c.consequent)?;
                state.serialize_field("alternate", &c.alternate)?;
                state.end()
            },
            Expr::Func(ref f) => {
                let mut state = serializer.serialize_struct("Node", 6)?;
                state.serialize_field("type", "FunctionExpression")?;
                state.serialize_field("id", &f.id)?;
                state.serialize_field("params", &f.params)?;
                state.serialize_field("body", &f.body)?;
                state.serialize_field("generator", &f.generator)?;
                state.serialize_field("async", &f.is_async)?;
                state.serialize_field("expression", &false)?;
                state.end()
            },
            Expr::Ident(ref i) => {
                i.serialize(serializer)
            },
            Expr::Lit(ref l) => {
                l.serialize(serializer)
            },
            Expr::Logical(ref l) => {
                let mut state = serializer.serialize_struct("Node", 4)?;
                state.serialize_field("type", "LogicalExpression")?;
                state.serialize_field("left", &l.left)?;
                state.serialize_field("operator", &l.operator)?;
                state.serialize_field("right", &l.right)?;
                state.end()
            },
            Expr::Member(ref m) => {
                let mut state = serializer.serialize_struct("Node", 4)?;
                state.serialize_field("type", "MemberExpression")?;
                state.serialize_field("object", &m.object)?;
                state.serialize_field("property", &m.property)?;
                state.serialize_field("computed", &m.computed)?;
                state.end()
            },
            Expr::MetaProp(ref m) => {
                let mut state = serializer.serialize_struct("Node", 3)?;
                state.serialize_field("type", "MetaProperty")?;
                state.serialize_field("meta", &m.meta)?;
                state.serialize_field("property", &m.property)?;
                state.end()
            },
            Expr::New(ref n) => {
                let mut state = serializer.serialize_struct("Node", 3)?;
                state.serialize_field("type", "NewExpression")?;
                state.serialize_field("callee", &n.callee)?;
                state.serialize_field("arguments", &n.arguments)?;
                state.end()
            },
            Expr::Obj(ref o) => {
                let mut state = serializer.serialize_struct("Node", 3)?;
                state.serialize_field("type", "ObjectExpression")?;
                state.serialize_field("properties", o)?;
                state.end()
            },
            Expr::Sequence(ref s) => {
                let mut state = serializer.serialize_struct("Node", 3)?;
                state.serialize_field("type", "SequenceExpression")?;
                state.serialize_field("expressions", s)?;
                state.end()
            },
            Expr::Spread(ref s) => {
                let mut state = serializer.serialize_struct("Node", 3)?;
                state.serialize_field("type", "SpreadElement")?;
                state.serialize_field("argument", s)?;
                state.end()
            },
            Expr::Super => {
                let mut state = serializer.serialize_struct("Node", 1)?;
                state.serialize_field("type", "Super")?;
                state.end()
            },
            Expr::TaggedTemplate(ref t) => {
                let mut state = serializer.serialize_struct("Node", 1)?;
                state.serialize_field("type", "TaggedTemplateExpression")?;
                state.serialize_field("tag", &t.tag)?;
                state.serialize_field("quasi", &t.quasi)?;
                state.end()
            },
            Expr::This => {
                let mut state = serializer.serialize_struct("Node", 1)?;
                state.serialize_field("type", "ThisExpression")?;
                state.end()
            },
            Expr::Unary(ref u) => {
                let mut state = serializer.serialize_struct("Node", 1)?;
                state.serialize_field("type", "UnaryExpression")?;
                state.serialize_field("argument", &u.argument)?;
                state.serialize_field("operator", &u.operator)?;
                state.serialize_field("prefix", &u.prefix)?;
                state.end()
            },
            Expr::Update(ref u) => {
                let mut state = serializer.serialize_struct("Node", 1)?;
                state.serialize_field("type", "UpdateExpression")?;
                state.serialize_field("argument", &u.argument)?;
                state.serialize_field("operator", &u.operator)?;
                state.serialize_field("prefix", &u.prefix)?;
                state.end()
            },
            Expr::Yield(ref y) => {
                let mut state = serializer.serialize_struct("Node", 1)?;
                state.serialize_field("type", "YieldExpression")?;
                state.serialize_field("argument", &y.argument)?;
                state.serialize_field("delegate", &y.delegate)?;
                state.end()
            }
        }
    }
}

impl<'a> Serialize for Pat<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Pat::Array(ref a) => {
                let mut state = serializer.serialize_struct("Node", 2)?;
                state.serialize_field("type", "ArrayPattern")?;
                state.serialize_field("elements", a)?;
                state.end()
            },
            Pat::Assign(ref a) => {
                let mut state = serializer.serialize_struct("Node", 2)?;
                state.serialize_field("type", "AssignmentPattern")?;
                state.serialize_field("left", &a.left)?;
                state.serialize_field("right", &a.right)?;
                state.end()
            },
            Pat::Ident(ref i) => {
                i.serialize(serializer)
            },
            Pat::Obj(ref o) => {
                let mut state = serializer.serialize_struct("Node", 2)?;
                state.serialize_field("type", "ObjectPattern")?;
                state.serialize_field("properties", o)?;
                state.end()
            },
            Pat::RestElement(ref r) => {
                let mut state = serializer.serialize_struct("Node", 2)?;
                state.serialize_field("type", "RestElement")?;
                state.serialize_field("argument", r)?;
                state.end()
            }
        }
    }
}


impl<'a> Serialize for Prop<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Node", 3)?;
        state.serialize_field("type", "Property")?;
        state.serialize_field("key", &self.key)?;
        state.serialize_field("computed", &self.computed)?;
        state.serialize_field("kind", &self.kind)?;
        state.serialize_field("method", &self.method)?;
        state.serialize_field("shorthand", &self.short_hand)?;
        state.serialize_field("value", &self.value)?;
        if self.is_static {
            state.serialize_field("static", &self.is_static)?;
        }
        state.end()
    }
}

impl<'a> Serialize for BlockStmt<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Node", 2)?;
        state.serialize_field("type", "BlockStatement")?;
        state.serialize_field("body", &self.0)?;
        state.end()
    }
}

impl<'a> Serialize for CatchClause<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Node", 2)?;
        state.serialize_field("type", "CatchClause")?;
        state.serialize_field("param", &self.param)?;
        state.serialize_field("body", &self.body)?;
        state.end()
    }
}
impl<'a> Serialize for SwitchCase<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Node", 3)?;
        state.serialize_field("type", "SwitchCase")?;
        state.serialize_field("test", &self.test)?;
        state.serialize_field("consequent", &self.consequent)?;
        state.end()
    }
}

impl Serialize for AssignOp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match self {
            AssignOp::Equal => "=",
            AssignOp::PlusEqual => "+=",
            AssignOp::MinusEqual => "-=",
            AssignOp::TimesEqual => "*=",
            AssignOp::DivEqual => "/=",
            AssignOp::ModEqual => "%=",
            AssignOp::LeftShiftEqual => "<<=",
            AssignOp::RightShiftEqual => ">>=",
            AssignOp::UnsignedRightShiftEqual => ">>>=",
            AssignOp::OrEqual => "|=",
            AssignOp::XOrEqual => "^=",
            AssignOp::AndEqual => "&=",
            AssignOp::PowerOfEqual => "**=",
        };
        serializer.serialize_str(s)
    }
}
impl Serialize for BinaryOp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match self {
            BinaryOp::Equal => "==",
            BinaryOp::NotEqual => "!=",
            BinaryOp::StrictEqual => "===",
            BinaryOp::StrictNotEqual => "!==",
            BinaryOp::LessThan => "<",
            BinaryOp::GreaterThan => ">",
            BinaryOp::LessThanEqual => "<=",
            BinaryOp::GreaterThanEqual => ">=",
            BinaryOp::LeftShift => "<<",
            BinaryOp::RightShift => ">>",
            BinaryOp::UnsignedRightShift => ">>>",
            BinaryOp::Plus => "+",
            BinaryOp::Minus => "-",
            BinaryOp::Times => "*",
            BinaryOp::Over => "/",
            BinaryOp::Mod => "%",
            BinaryOp::Or => "|",
            BinaryOp::XOr => "^",
            BinaryOp::And => "&",
            BinaryOp::In => "in",
            BinaryOp::InstanceOf => "instanceof",
            BinaryOp::PowerOf => "**",
        };
        serializer.serialize_str(s)
    }
}
impl Serialize for LogicalOp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match self {
            LogicalOp::And => "&&",
            LogicalOp::Or => "||",
        };
        serializer.serialize_str(s)
    }
}
impl Serialize for UnaryOp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match self {
            UnaryOp::Minus => "-",
            UnaryOp::Plus => "+",
            UnaryOp::Not => "!",
            UnaryOp::Tilde => "~",
            UnaryOp::TypeOf => "typeof",
            UnaryOp::Void => "void",
            UnaryOp::Delete => "delete",
        };
        serializer.serialize_str(s)
    }
}
impl Serialize for UpdateOp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match self {
            UpdateOp::Increment => "++",
            UpdateOp::Decrement => "--",
        };
        serializer.serialize_str(s)
    }
}
impl<'a> Serialize for LoopLeft<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            LoopLeft::Expr(ref e) => e.serialize(serializer),
            LoopLeft::Pat(ref p) => p.serialize(serializer),
            LoopLeft::Variable(ref kind, ref v) => {
                let mut state = serializer.serialize_struct("Node", 3)?;
                state.serialize_field("type", "VariableDeclaration")?;
                state.serialize_field("kind", kind)?;
                state.serialize_field("declarations", &[v])?;
                state.end()
            },
        }
    }
}
impl<'a> Serialize for LoopInit<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            LoopInit::Expr(ref e) => e.serialize(serializer),
            LoopInit::Variable(ref kind, ref v) => {
                let mut state = serializer.serialize_struct("Node", 3)?;
                state.serialize_field("type", "VariableDeclaration")?;
                state.serialize_field("kind", kind)?;
                state.serialize_field("declarations", v)?;
                state.end()
            },
        }
    }
}

use std::collections::VecDeque;
fn unescaper(s: &str) -> Option<String> {
    
    let mut queue : VecDeque<_> = String::from(s).chars().collect();
    let mut s = String::new();

    while let Some(c) = queue.pop_front() {
        if c != '\\' {
            s.push(c);
            continue;
        }

        match queue.pop_front() {
            Some('b') => s.push('\u{0008}'),
            Some('f') => s.push('\u{000C}'),
            Some('n') => s.push('\n'),
            Some('r') => s.push('\r'),
            Some('t') => s.push('\t'),
            Some('\'') => s.push('\''),
            Some('\"') => s.push('\"'),
            Some('\\') => s.push('\\'),
            Some('u') => if let Some(x) = unescape_unicode(&mut queue) {
                s.push(x);
            } else {
                return None;
            },
            Some('x') => if let Some(x) = unescape_byte(&mut queue) {
                s.push(x)
            } else {
                return None;
            },
            Some(c) if c.is_digit(8) => {
                if let Some(x) = unescape_octal(c, &mut queue) {
                    s.push(x);
                } else {
                    return None;
                }
            },
            _ => return None
        };
    }

    Some(s)
}

fn unescape_unicode(queue: &mut VecDeque<char>) -> Option<char> {
    let mut s = String::new();

    for _ in 0..4 {
        if let Some(x) = queue.pop_front() {
            s.push(x)
        } else {
            return None;
        }
    }
    match u32::from_str_radix(&s, 16) {
        Ok(u) => ::std::char::from_u32(u),
        Err(e) => {
            eprintln!("error parsing char {}", e);
            None
        }
    }
}

fn unescape_byte(queue: &mut VecDeque<char>) -> Option<char> {
    let mut s = String::new();

    for _ in 0..2 {
        if let Some(c) = queue.pop_front() {
            s.push(c)
        } else {
            return None
        }
    }
    match u32::from_str_radix(&s, 16) {
        Ok(u) => ::std::char::from_u32(u),
        Err(e) => {
            panic!("{}", e);
            None
        }
    }
    
}

fn unescape_octal(c: char, queue: &mut VecDeque<char>) -> Option<char> {
    match unescape_octal_leading(c, queue) {
        Some(ch) => {
            let _ = queue.pop_front();
            let _ = queue.pop_front();
            Some(ch)
        }
        None => unescape_octal_no_leading(c, queue)
    }
}

fn unescape_octal_leading(c: char, queue: &VecDeque<char>) -> Option<char> {
    if c != '0' && c != '1' && c != '2' && c != '3' {
        return None;
    }

    let mut s = String::new();
    s.push(c);
    if let (Some(one), Some(two)) = (queue.get(0), queue.get(1)) {
        s.push(*one);
        s.push(*two);
    } else {
        return None;
    }
    match u32::from_str_radix(&s, 8) {
        Ok(u) => ::std::char::from_u32(u),
        Err(e) => {
            panic!("{}: {}", e, s);
            None
        }
    }
}

fn unescape_octal_no_leading(c: char, queue: &mut VecDeque<char>) -> Option<char> {
    let mut s = String::new();
    s.push(c);
    if let Some(one) = queue.pop_front() {
        s.push(one);
    } else {
        return None;
    }

    match u32::from_str_radix(&s, 8) {
        Ok(u) => ::std::char::from_u32(u),
        Err(e) => {
            panic!("{}", e);
            None
        }
    }
}