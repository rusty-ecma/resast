const fs = require('fs');
(function () {
    let es5Contents = fs.readFileSync('node_modules/everything.js/es5.js', 'utf8');
    let out = '';
    let ct = 0;
    let inStmt = false;
    let pendingTest = genTestStart(++ct);
    for (let line of es5Contents.split('\n')) {
        if (line.startsWith('//')
        || line.startsWith('/*')
        || line.trim().length === 0) {
            continue;
        }
        pendingTest += line;
        if (line.endsWith(";")
        || line.endsWith("}")) {
            out += pendingTest + genTestEnd(ct);
            pendingTest = genTestStart(++ct);
        } else {
            pendingTest += '\n'
        }
    }
    fs.writeFileSync("es5-2.rs", out);
})()

function genTestStart(ct) {
    return `#[test]
fn test${ct}() {
    let js = r#"`;
}

function genTestEnd(ct) {
    return `"#;
    run_test("test${ct}", js);
}
`
}