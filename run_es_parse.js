const esp = require('esprima');

(function() {
    let js = process.argv[2];
    let parsed = esp.parseScript(js);
    console.log(JSON.stringify(parsed));
})()