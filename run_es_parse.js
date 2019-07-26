const esp = require('esprima');
const fs = require('fs');
async function main() {
    let js = process.argv[2];
    console.error(js);
    return new Promise((r, j) => {
        fs.readFile(js, 'utf8', (err, content) => {
            if (err) return j(err);
            let parsed;
            if (js.endsWith('module.js')) {
                parsed = esp.parseModule(content);
            } else {
                parsed = esp.parseScript(content);
            }
            let json = JSON.stringify(parsed, (key, value) => {
                if (key === 'value' && value instanceof RegExp) {
                    value = value.toString();
                }
                return value;
            }, 4);
            return r(json);
        });
    })
}

main().then((json) => console.log(json)).catch(e => console.error(e));