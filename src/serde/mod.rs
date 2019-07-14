use serde::ser::{
    Serialize,
    Serializer,
    SerializeStruct,
};
use crate::prelude::*;

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
                state.serialize_field("body", body)?;
            },
            Program::Mod(ref body) => {
                state.serialize_field("sourceType", "module")?;
                state.serialize_field("body", body)?;
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

impl<'a> Serialize for Ident<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Node", 2)?;
        state.serialize_field("type", "Identifier")?;
        state.serialize_field("name", &self.name)?;
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