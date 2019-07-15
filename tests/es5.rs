use ressa::*;
use serde_json::{
    Value,
    to_string,
    from_str,
};

#[test]
fn test1() {
    let js = r#"tab:for(;;)break	tab"#;
    run_test(js);
}

fn run_test(js: &str) {

    let mut p = Parser::new(js).expect("couldn't create parser");
    let res = p.parse().expect("Unable to parse js");
    let raw_res = to_string(&res).expect("failed to convert it to json");
    let res_json: Value = from_str(&raw_res).expect("failed to revert back to Value");
    let mut raw_esparse = ::std::process::Command::new("node_modules/.bin/esparse")
        .arg(js)
        .output()
        .expect("failed to spawn esparse");
    let raw_js = String::from_utf8_lossy(&raw_esparse.stdout).to_string();
    println!("{}", raw_js);
    let es_json: Value = from_str(&raw_js).expect(
        "failed to convert esparse result to Value"
    );
    assert_eq!(es_json, res_json);
}
// #[test]
// fn test2 {
//     let js = r#"verticalTab:for(;;)breakverticalTab"#;
// }
// #[test]
// fn test3 {
//     let js = r#"formFeed:for(;;)breakformFeed"#;
// }
// #[test]
// fn test4 {
//     let js = r#"space:for(;;)break space"#;
// }
// #[test]
// fn test5 {
//     let js = r#"nbsp:for(;;)break nbsp"#;
// }
// #[test]
// fn test6 {
//     let js = r#"bom:for(;;)break﻿bom"#;
// }
// #[test]
// fn test7 {
//     let js = r#"lineFeed:0
// 0"#;
// }
// #[test]
// fn test8 {
//     let js = r#"carriageReturn:0
// 0"#;
// }
// #[test]
// fn test9 {
//     let js = r#"carriageReturnLineFeed:0
// 0"#;
// }
// #[test]
// fn test10 {
//     let js = r#"lineSeparator:0 0"#;
// }
// #[test]
// fn test11 {
//     let js = r#"paragraphSeparator:0 0"#;
// }
// #[test]
// fn test12 {
//     let js = r#"var $, _, \u0078, x$, x_, x\u0030, xa, x0, x0a, x0123456789,
//   qwertyuiopasdfghjklzxcvbnm, QWERTYUIOPASDFGHJKLZXCVBNM"#;
// }
// #[test]
// fn test13 {
//     let js = r#"var œ一, ǻ둘, ɤ〩, φ, ﬁⅷ, ユニコード, x‌‍"#;
// }
// #[test]
// fn test14 {
//     let js = r#"null; true; false"#;
// }
// #[test]
// fn test15 {
//     let js = r#"0; 00; 1234567890; 01234567"#;
// }
// #[test]
// fn test16 {
//     let js = r#"0.; 0.00; 10.00; .0; .00
// 0e0; 0E0; 0.e0; 0.00e+0; .00e-0"#;
// }
// #[test]
// fn test17 {
//     let js = r#"0x0; 0X0; 0x0123456789abcdefABCDEF"#;
// }
// #[test]
// fn test18 {
//     let js = r#"2e308"#;
// }
// #[test]
// fn test19 {
//     let js = r#"""; "'"; "\'\"\\\b\f\n\r\t\v\0""#;
// }
// #[test]
// fn test20 {
//     let js = r#""\1\00\400\000""#;
// }
// #[test]
// fn test21 {
//     let js = r#""\x01\x23\x45\x67\x89\xAB\xCD\xEF""#;
// }
// #[test]
// fn test22 {
//     let js = r#""\u0123\u4567\u89AB\uCDEF"; "\
// ""#;
// }
// #[test]
// fn test23 {
//     let js = r#"''; '"'; '\'\"\\\b\f\n\r\t\v\0'"#;
// }
// #[test]
// fn test24 {
//     let js = r#"'\1\00\400\000'"#;
// }
// #[test]
// fn test25 {
//     let js = r#"'\x01\x23\x45\x67\x89\xAB\xCD\xEF'"#;
// }
// #[test]
// fn test26 {
//     let js = r#"'\u0123\u4567\u89AB\uCDEF'; '\
// '"#;
// }
// #[test]
// fn test27 {
//     let js = r#"/x/; /|/; /|||/"#;
// }
// #[test]
// fn test28 {
//     let js = r#"/^$\b\B/; /(?=(?!(?:(.))))/"#;
// }
// #[test]
// fn test29 {
//     let js = r#"/a.\f\n\r\t\v\0\[\-\/\\\x00\u0000/; /\d\D\s\S\w\W/"#;
// }
// #[test]
// fn test30 {
//     let js = r#"/\ca\cb\cc\cd\ce\cf\cg\ch\ci\cj\ck\cl\cm\cn\co\cp\cq\cr\cs\ct\cu\cv\cw\cx\cy\cz/"#;
// }
// #[test]
// fn test31 {
//     let js = r#"/\cA\cB\cC\cD\cE\cF\cG\cH\cI\cJ\cK\cL\cM\cN\cO\cP\cQ\cR\cS\cT\cU\cV\cW\cX\cY\cZ/"#;
// }
// #[test]
// fn test32 {
//     let js = r#"/[a-z-]/; /[^\b\-^]/; /[/\]\\]/"#;
// }
// #[test]
// fn test33 {
//     let js = r#"/./i; /./g; /./m; /./igm"#;
// }
// #[test]
// fn test34 {
//     let js = r#"/.*/; /.*?/; /.+/; /.+?/; /.?/; /.??/"#;
// }
// #[test]
// fn test35 {
//     let js = r#"/.{0}/; /.{0,}/; /.{0,0}/"#;
// }
// #[test]
// fn test36 {
//     let js = r#"this"#;
// }
// #[test]
// fn test37 {
//     let js = r#"x"#;
// }
// #[test]
// fn test38 {
//     let js = r#"[]; [,]; [0]; [0,]; [,0]; [0,0]; [0,0,]; [0,,0]; [,,]"#;
// }
// #[test]
// fn test39 {
//     let js = r#"(); ({x:0}); ({x:0,y:0}); ({x:0,}); ({'x':0,"y":0,var:0,})"#;
// }
// #[test]
// fn test40 {
//     let js = r#"({0:0}); ({0.:0}); ({0.0:0}); ({.0:0}); ({0e0:0}); ({0x0:0})"#;
// }
// #[test]
// fn test {
//     let js = r#"({
//   get x(), set x(a), get 'y'(), set "y"(a),
//   get 0(), set 0(a), get var(), set var(x),
// })"#;
// }
// #[test]
// fn test {
//     let js = r#"0..a"#;
// }
// #[test]
// fn test {
//     let js = r#"0[0]"#;
// }
// #[test]
// fn test {
//     let js = r#"x = function f(){ return f; }; x[0] = x; x.a = x"#;
// }
// #[test]
// fn test {
//     let js = r#"new x(); new new x()()"#;
// }
// #[test]
// fn test {
//     let js = r#"new x[0](); new x.a(); new x[0].a(); new x.a[0]()"#;
// }
// #[test]
// fn test {
//     let js = r#"new x; new new x; new new x()"#;
// }
// #[test]
// fn test {
//     let js = r#"new new x().a; new new x()[0]"#;
// }
// #[test]
// fn test {
//     let js = r#"x(); x()(); x(x); x(x, x)"#;
// }
// #[test]
// fn test {
//     let js = r#"x.a().a(); x[0]()[0](); x().a[0]()"#;
// }
// #[test]
// fn test {
//     let js = r#"x++; x--"#;
// }
// #[test]
// fn test {
//     let js = r#"delete void typeof+-~!x; ++x; --x"#;
// }
// #[test]
// fn test {
//     let js = r#"0*0; 0/0; 0%0"#;
// }
// #[test]
// fn test {
//     let js = r#"0+0; 0-0"#;
// }
// #[test]
// fn test {
//     let js = r#"0<<0; 0>>0; 0>>>0"#;
// }
// #[test]
// fn test {
//     let js = r#"0<0; 0>0; 0<=0; 0>=0"#;
// }
// #[test]
// fn test {
//     let js = r#"0 instanceof function()"#;
// }
// #[test]
// fn test {
//     let js = r#"0 in"#;
// }
// #[test]
// fn test {
//     let js = r#"0==0; 0!=0; 0===0; 0!==0"#;
// }
// #[test]
// fn test {
//     let js = r#"0&0; 0^0; 0|0; 0&&0; 0||0"#;
// }
// #[test]
// fn test {
//     let js = r#"0?0:0; 0?0?0:0:0; 0||0?x=0:x=0"#;
// }
// #[test]
// fn test {
//     let js = r#"x=0; x*=0; x/=0; x%=0; x+=0; x-=0"#;
// }
// #[test]
// fn test {
//     let js = r#"x<<=0; x>>=0; x>>>=0; x&=0; x^=0; x|=0"#;
// }
// #[test]
// fn test {
//     let js = r#"0,0; 0,0,0; x=0,x=0"#;
// }
// #[test]
// fn test {
//     let js = r#"
//  {;} {0} {0;} {0;0} {0;0;}
// var x; var x,y; var x,y,z"#;
// }
// #[test]
// fn test {
//     let js = r#"var x=0; var x=0,y; var x,y=0; var x=0,y=0"#;
// }
// #[test]
// fn test {
//     let js = r#""#;
// }
// #[test]
// fn test {
//     let js = r#"if(0); if(0);else"#;
// }
// #[test]
// fn test {
//     let js = r#"do;while(0)"#;
// }
// #[test]
// fn test {
//     let js = r#"while(0)"#;
// }
// #[test]
// fn test {
//     let js = r#"for(;;)break; for(0;0;0); for((0 in[]);0;)"#;
// }
// #[test]
// fn test {
//     let js = r#"for(var a;;)break; for(var a,b;0;0)"#;
// }
// #[test]
// fn test {
//     let js = r#"for(var a=0;;)break; for(var a=(0 in[]);0;)"#;
// }
// #[test]
// fn test {
//     let js = r#"for(x in); for(var x in)"#;
// }
// #[test]
// fn test {
//     let js = r#"for(var x=[]in); for(var x=(0 in[])in)"#;
// }
// #[test]
// fn test {
//     let js = r#"for(;0;)continue; x:for(;0;)continue x"#;
// }
// #[test]
// fn test {
//     let js = r#"for(;;)break; x:for(;;)break x"#;
// }
// #[test]
// fn test {
//     let js = r#"switch(0){case 0:break;}
// function f(){ return; }
// function f(){ return 0; }
// with(0)"#;
// }
// #[test]
// fn test {
//     let js = r#"switch(0) switch(0){case 0:} switch(0){case 0:case 0:}
// switch(0){default:} switch(0){case 0:default:case 0:}
// switch(0){case 0:;} switch(0){case 0:;;}
// switch(0){default:;} switch(0){default:;;}
// x:; x:y:"#;
// }
// #[test]
// fn test {
//     let js = r#"try { throw 0; }catch(x)
// trycatch(x)
// tryfinally
// trycatch(x)finally
// debugger"#;
// }
// #[test]
// fn test {
//     let js = r#"function f()
// function f(x)
// function f(x,y)
// function f(){ function f() }
// function f(){ "use strict" }
// function f(){ 'use strict' }
// function f(){ "other directive" }
// function f(){ 'other directive' }
// function f(){ ("string") }
// function f(){ ('string') }
// function f(){
//   'string'
//   +0
// }
// (function())"#;
// }
// #[test]
// fn test {
//     let js = r#"(function(x))"#;
// }
// #[test]
// fn test {
//     let js = r#"(function(x,y))"#;
// }
// #[test]
// fn test {
//     let js = r#"(function(){ function f() })"#;
// }
// #[test]
// fn test {
//     let js = r#"(function f())"#;
// }
// #[test]
// fn test {
//     let js = r#"(function f(x))"#;
// }
// #[test]
// fn test {
//     let js = r#"(function f(x,y))"#;
// }
// #[test]
// fn test {
//     let js = r#"(function f(){ function f() })"#;
// }
