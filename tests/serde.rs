#![cfg(feature = "esprima")]
use pretty_env_logger::try_init;
use ressa::*;

use resast::prelude::*;
use serde_json::{from_str, to_string_pretty, Value};
use std::{fs::read_to_string, path::Path};
#[test]
fn serde1() {
    let ast = Program::Script(vec![ProgramPart::Decl(Decl::Func(Func {
        id: Some(Ident::from("f")),
        body: FuncBody(vec![]),
        is_async: false,
        generator: false,
        params: vec![FuncArg::Expr(Expr::Ident(Ident::from("a")))],
    }))]);
    let json = to_string_pretty(&ast).expect("Failed to serialize ast");
    let expectation = r#"{
    "type": "Program",
    "body": [
        {
            "type": "FunctionDeclaration",
            "id": {
                "type": "Identifier",
                "name": "f"
            },
            "params": [
                {
                    "type": "Identifier",
                    "name": "a"
                }
            ],
            "body": {
                "type": "BlockStatement",
                "body": []
            },
            "generator": false,
            "expression": false,
            "async": false
        }
    ],
    "sourceType": "script"
}
"#;

    let r: serde_json::Value = serde_json::from_str(&json).expect("failed to deserialize json");
    let j: serde_json::Value =
        serde_json::from_str(&expectation).expect("failed to deserialize expectation");
    assert_eq!(r, j);
}

#[test]
fn serde_es5() {
    let json = run_rs_parse("node_modules/everything.js/es5.js");
    let esparsed = esparse("node_modules/everything.js/es5.js");
    check_jsons("es5", json, esparsed);
}
#[test]
fn serde_es2015_script() {
    let json = run_rs_parse("node_modules/everything.js/es2015-script.js");
    let esparsed = esparse("node_modules/everything.js/es2015-script.js");
    check_jsons("es2015-script", json, esparsed);
}
#[test]
fn serde_es2015_module() {
    let json = run_rs_parse("node_modules/everything.js/es2015-module.js");
    let esparsed = esparse("node_modules/everything.js/es2015-module.js");
    check_jsons("es2015-module", json, esparsed);
}

fn run_rs_parse(path: impl AsRef<Path>) -> Value {
    let module = path.as_ref().ends_with("es2015-module.js");
    eprintln!("working on {:?} -> {}", path.as_ref(), module);
    let js = get_js_file(path);
    let mut b = Builder::new();
    b.set_module(module);
    let mut parser = b.js(&js).build().expect("failed to create parser");
    let parsed = parser.parse().expect("failed to parse js");
    let raw = to_string_pretty(&parsed).expect("failed to convert to json string");
    from_str(&raw).expect("failed to revert to Value")
}

fn check_jsons(name: &str, lhs: Value, rhs: Value) {
    if lhs != rhs {
        let f1 =
            ::std::fs::File::create(&format!("{}.rs.json", name)).expect("failed to write rs.json");
        serde_json::to_writer_pretty(f1, &lhs).expect("");
        let f2 =
            ::std::fs::File::create(&format!("{}.js.json", name)).expect("failed to write js.json");
        serde_json::to_writer_pretty(f2, &rhs).expect("");
        panic!("json doesn't match");
    }
}

pub fn npm_install() {
    let mut c = ::std::process::Command::new("npm");
    c.arg("i");
    c.output().expect("failed to run npm install");
}

pub fn get_js_file(path: impl AsRef<::std::path::Path>) -> String {
    let path = path.as_ref();
    if !path.exists() {
        npm_install();
        if !path.exists() {
            panic!("npm install failed to make {:?} available", path);
        }
    }
    read_to_string(path).expect(&format!("failed to read {} to a string", path.display()))
}

pub fn esparse(path: impl AsRef<::std::path::Path>) -> Value {
    let path = path.as_ref();
    if !path.exists() {
        npm_install();
        if !path.exists() {
            panic!("npm install failed to make {:?} available", path);
        }
    }
    let esparse = ::std::process::Command::new("node")
        .arg("run_es_parse.js")
        .arg(path)
        .output()
        .expect("failed to execute run_es_parse.js");
    let json = String::from_utf8_lossy(&esparse.stdout).to_string();
    from_str(&json).expect(&format!("failed  to convert {} to Value", path.display()))
}

#[test]
fn func_args() {
    let js = "function f(a, b = 0, [c,, d = 0, ...e], {f, g: h, i = 0, i: j = 0}, ...k){}";
    let mut parser = Parser::new(&js).expect("");
    let parsed = parser.parse().expect("");
    let raw = to_string_pretty(&parsed).expect("failed to convert ron to string");
    let json: Value = from_str(&raw).expect("failed to convert string to json");
    ::std::fs::write("args.js", js).expect("failed to write args.js");
    let esparsed = esparse("args.js");
    let _ = ::std::fs::remove_file("args.js");
    if json != esparsed {
        let f1 = ::std::fs::File::create("func_args.rs.json").expect("failed to create rs.json");
        serde_json::to_writer_pretty(f1, &json).expect("failed to write rs.json");
        let f2 = ::std::fs::File::create("func_args.js.json").expect("failed to create js.json");
        serde_json::to_writer_pretty(f2, &esparsed).expect("failed to write js.json");
        panic!("json doesn't match");
    }
}

#[test]
fn arrow_func_args() {
    let _ = try_init();
    let js = "({i = 0}, ...k) => {;};";
    let mut parser = Parser::new(&js).expect("");
    let parsed = parser.parse().expect("");
    let raw = to_string_pretty(&parsed).expect("failed to convert ron to string");
    let json: Value = from_str(&raw).expect("failed to convert string to json");
    ::std::fs::write("arrow-args.js", js).expect("failed to write args.js");
    let esparsed = esparse("arrow-args.js");
    let _ = ::std::fs::remove_file("arrow-args.js");
    if json != esparsed {
        let f1 =
            ::std::fs::File::create("arrow_func_args.rs.json").expect("failed to create rs.json");
        serde_json::to_writer_pretty(f1, &json).expect("failed to write rs.json");
        let f2 =
            ::std::fs::File::create("arrow_func_args.js.json").expect("failed to create js.json");
        serde_json::to_writer_pretty(f2, &esparsed).expect("failed to write js.json");
        let _ = ::std::fs::write("arrow_func_args2.ron", &format!("{:#?}", parsed));
        panic!("json doesn't match");
    }
}
