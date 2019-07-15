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
        state.serialize_field("directive", &self.dir)?;
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
                let mut state = serializer.serialize_struct("Node", 2)?;
                state.serialize_field("type", "BlockStatement")?;
                state.serialize_field("body", b)?;
                state.end()
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
                state.serialize_field("value", value)?;
                state.serialize_field("raw", &format!("{0}{1}{0}", quote, value))?;
                state.end()
            },
            Lit::RegEx(ref r) => {
                let mut state = serializer.serialize_struct("Node", 4)?;
                state.serialize_field("type", "Literal")?;
                let value: ::std::collections::HashMap<(), ()> = ::std::collections::HashMap::new();
                state.serialize_field("value", &value)?;
                state.serialize_field("raw", &format!("/{}/{}", r.pattern, r.flags))?;
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
        } else {
            if n.contains('E') || n.contains('e') || n.contains('.') {
                serialize_float(&mut state, n)?;
            } else {
                serialize_int(&mut state, 10, n)?;
            }
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