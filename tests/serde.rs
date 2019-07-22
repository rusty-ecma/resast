use ressa::*;

use serde_json::{
    to_string_pretty,
    Value,
    from_str,
};
use resast::prelude::*;
use std::fs::{
    read_to_string,
};
#[test]
fn serde1() {
    let ast = Program::Script(vec![
        ProgramPart::Decl(
            Decl::Func(
                Func {
                    id: Some(Ident::from("f")),
                    body: FuncBody(vec![]),
                    is_async: false,
                    generator: false,
                    params: vec![
                        FuncArg::Expr(
                            Expr::Ident(
                                Ident::from("a")
                            )
                        )
                    ]
                }
            )
        )
    ]);
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
    let j: serde_json::Value = serde_json::from_str(&expectation).expect("failed to deserialize expectation");
    assert_eq!(r, j);
}


#[test]
fn serde_es5() {
    let js = get_js_file("node_modules/everything.js/es5.js");
    let mut parser = Parser::new(&js).unwrap();
    let parsed = parser.parse().unwrap();
    let raw = to_string_pretty(&parsed).unwrap();
    let json: Value = from_str(&raw).unwrap();
    let es = esparse("node_modules/everything.js/es5.js");
    let esparsed: Value = from_str(&es).unwrap();
    if json != esparsed {
        let f1 = ::std::fs::File::create("1.rs.json").unwrap();
        serde_json::to_writer_pretty(f1, &json).unwrap();
        let f2 = ::std::fs::File::create("2.js.json").unwrap();
        serde_json::to_writer_pretty(f2, &esparsed).unwrap();
        panic!("json doesn't match");
    }
}
#[test]
fn serde_es2015_script() {
    let js = get_js_file("node_modules/everything.js/es2015-script.js");
    let mut parser = Parser::new(&js).unwrap();
    let parsed = parser.parse().unwrap();
    let raw = to_string_pretty(&parsed).unwrap();
    let json: Value = from_str(&raw).unwrap();
    let es = esparse("node_modules/everything.js/es2015-script.js");
    let esparsed: Value = from_str(&es).unwrap();
    if json != esparsed {
        let f1 = ::std::fs::File::create("3.rs.json").unwrap();
        serde_json::to_writer_pretty(f1, &json).unwrap();
        let f2 = ::std::fs::File::create("4.js.json").unwrap();
        serde_json::to_writer_pretty(f2, &esparsed).unwrap();
        panic!("json doesn't match");
    }
}

pub fn npm_install() {
    let mut c = ::std::process::Command::new("npm");
    c.arg("i");
    c.output().unwrap();
}

pub fn get_js_file(path: impl AsRef<::std::path::Path>) -> String {
    let path = path.as_ref();
    if !path.exists() {
        npm_install();
        if !path.exists() {
            panic!("npm install failed to make {:?} available", path);
        }
    }
    read_to_string(path).unwrap()
}

pub fn esparse(path: impl AsRef<::std::path::Path>) -> String {
    let path = path.as_ref();
    if !path.exists() {
        npm_install();
        if !path.exists() {
            panic!("npm install failed to make {:?} available", path);
        }
    }
    let esparse = ::std::process::Command::new("node_modules/.bin/esparse")
        .arg(path)
        .output()
        .unwrap();
    String::from_utf8_lossy(&esparse.stdout).to_string()       
}

#[test]
fn func_args() {
    let js = "function f(a, b = 0, [c,, d = 0, ...e], {f, g: h, i = 0, i: j = 0}, ...k){}";
     let mut parser = Parser::new(&js).unwrap();
    let parsed = parser.parse().unwrap();
    let raw = to_string_pretty(&parsed).unwrap();
    let json: Value = from_str(&raw).unwrap();
    ::std::fs::write("args.js", js).expect("failed to write args.js");
    let es = esparse("args.js");
    let esparsed: Value = from_str(&es).unwrap();
    if json != esparsed {
        let f1 = ::std::fs::File::create("func_args.rs.json").unwrap();
        serde_json::to_writer_pretty(f1, &json).unwrap();
        let f2 = ::std::fs::File::create("func_args.js.json").unwrap();
        serde_json::to_writer_pretty(f2, &esparsed).unwrap();
        panic!("json doesn't match");
    }
}