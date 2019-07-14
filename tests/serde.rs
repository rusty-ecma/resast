

use serde_json::to_string_pretty;
use resast::prelude::*;

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