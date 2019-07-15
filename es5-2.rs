#[test]
fn test1() {
    let js = r#"tab:for(;;)break	tab;"#;
    run_test("test1", js);
}
#[test]
fn test2() {
    let js = r#"verticalTab:for(;;)breakverticalTab;"#;
    run_test("test2", js);
}
#[test]
fn test3() {
    let js = r#"formFeed:for(;;)breakformFeed;"#;
    run_test("test3", js);
}
#[test]
fn test4() {
    let js = r#"space:for(;;)break space;"#;
    run_test("test4", js);
}
#[test]
fn test5() {
    let js = r#"nbsp:for(;;)break nbsp;"#;
    run_test("test5", js);
}
#[test]
fn test6() {
    let js = r#"bom:for(;;)break﻿bom;"#;
    run_test("test6", js);
}
#[test]
fn test7() {
    let js = r#"lineFeed:0
0;"#;
    run_test("test7", js);
}
#[test]
fn test8() {
    let js = r#"carriageReturn:00;"#;
    run_test("test8", js);
}
#[test]
fn test9() {
    let js = r#"carriageReturnLineFeed:0
0;"#;
    run_test("test9", js);
}
#[test]
fn test10() {
    let js = r#"lineSeparator:0 0;"#;
    run_test("test10", js);
}
#[test]
fn test11() {
    let js = r#"paragraphSeparator:0 0;"#;
    run_test("test11", js);
}
#[test]
fn test12() {
    let js = r#"var $, _, \u0078, x$, x_, x\u0030, xa, x0, x0a, x0123456789,
  qwertyuiopasdfghjklzxcvbnm, QWERTYUIOPASDFGHJKLZXCVBNM;"#;
    run_test("test12", js);
}
#[test]
fn test13() {
    let js = r#"var œ一, ǻ둘, ɤ〩, φ, ﬁⅷ, ユニコード, x‌‍;"#;
    run_test("test13", js);
}
#[test]
fn test14() {
    let js = r#"null; true; false;"#;
    run_test("test14", js);
}
#[test]
fn test15() {
    let js = r#"0; 00; 1234567890; 01234567;"#;
    run_test("test15", js);
}
#[test]
fn test16() {
    let js = r#"0.; 0.00; 10.00; .0; .00
0e0; 0E0; 0.e0; 0.00e+0; .00e-0;"#;
    run_test("test16", js);
}
#[test]
fn test17() {
    let js = r#"0x0; 0X0; 0x0123456789abcdefABCDEF;"#;
    run_test("test17", js);
}
#[test]
fn test18() {
    let js = r#"2e308;"#;
    run_test("test18", js);
}
#[test]
fn test19() {
    let js = r#"""; "'"; "\'\"\\\b\f\n\r\t\v\0";"#;
    run_test("test19", js);
}
#[test]
fn test20() {
    let js = r#""\1\00\400\000";"#;
    run_test("test20", js);
}
#[test]
fn test21() {
    let js = r#""\x01\x23\x45\x67\x89\xAB\xCD\xEF";"#;
    run_test("test21", js);
}
#[test]
fn test22() {
    let js = r#""\u0123\u4567\u89AB\uCDEF"; "\
";"#;
    run_test("test22", js);
}
#[test]
fn test23() {
    let js = r#"''; '"'; '\'\"\\\b\f\n\r\t\v\0';"#;
    run_test("test23", js);
}
#[test]
fn test24() {
    let js = r#"'\1\00\400\000';"#;
    run_test("test24", js);
}
#[test]
fn test25() {
    let js = r#"'\x01\x23\x45\x67\x89\xAB\xCD\xEF';"#;
    run_test("test25", js);
}
#[test]
fn test26() {
    let js = r#"'\u0123\u4567\u89AB\uCDEF'; '\
';"#;
    run_test("test26", js);
}
#[test]
fn test27() {
    let js = r#"/x/; /|/; /|||/;"#;
    run_test("test27", js);
}
#[test]
fn test28() {
    let js = r#"/^$\b\B/; /(?=(?!(?:(.))))/;"#;
    run_test("test28", js);
}
#[test]
fn test29() {
    let js = r#"/a.\f\n\r\t\v\0\[\-\/\\\x00\u0000/; /\d\D\s\S\w\W/;"#;
    run_test("test29", js);
}
#[test]
fn test30() {
    let js = r#"/\ca\cb\cc\cd\ce\cf\cg\ch\ci\cj\ck\cl\cm\cn\co\cp\cq\cr\cs\ct\cu\cv\cw\cx\cy\cz/;"#;
    run_test("test30", js);
}
#[test]
fn test31() {
    let js = r#"/\cA\cB\cC\cD\cE\cF\cG\cH\cI\cJ\cK\cL\cM\cN\cO\cP\cQ\cR\cS\cT\cU\cV\cW\cX\cY\cZ/;"#;
    run_test("test31", js);
}
#[test]
fn test32() {
    let js = r#"/[a-z-]/; /[^\b\-^]/; /[/\]\\]/;"#;
    run_test("test32", js);
}
#[test]
fn test33() {
    let js = r#"/./i; /./g; /./m; /./igm;"#;
    run_test("test33", js);
}
#[test]
fn test34() {
    let js = r#"/.*/; /.*?/; /.+/; /.+?/; /.?/; /.??/;"#;
    run_test("test34", js);
}
#[test]
fn test35() {
    let js = r#"/.{0}/; /.{0,}/; /.{0,0}/;"#;
    run_test("test35", js);
}
#[test]
fn test36() {
    let js = r#"this;"#;
    run_test("test36", js);
}
#[test]
fn test37() {
    let js = r#"x;"#;
    run_test("test37", js);
}
#[test]
fn test38() {
    let js = r#"[]; [,]; [0]; [0,]; [,0]; [0,0]; [0,0,]; [0,,0]; [,,];"#;
    run_test("test38", js);
}
#[test]
fn test39() {
    let js = r#"({}); ({x:0}); ({x:0,y:0}); ({x:0,}); ({'x':0,"y":0,var:0,});"#;
    run_test("test39", js);
}
#[test]
fn test40() {
    let js = r#"({0:0}); ({0.:0}); ({0.0:0}); ({.0:0}); ({0e0:0}); ({0x0:0});"#;
    run_test("test40", js);
}
#[test]
fn test41() {
    let js = r#"({
  get x(){}, set x(a){}, get 'y'(){}, set "y"(a){},
  get 0(){}, set 0(a){}, get var(){}, set var(x){},
});"#;
    run_test("test41", js);
}
#[test]
fn test42() {
    let js = r#"0..a;"#;
    run_test("test42", js);
}
#[test]
fn test43() {
    let js = r#"0[0];"#;
    run_test("test43", js);
}
#[test]
fn test44() {
    let js = r#"x = function f(){ return f; }; x[0] = x; x.a = x;"#;
    run_test("test44", js);
}
#[test]
fn test45() {
    let js = r#"new x(); new new x()();"#;
    run_test("test45", js);
}
#[test]
fn test46() {
    let js = r#"new x[0](); new x.a(); new x[0].a(); new x.a[0]();"#;
    run_test("test46", js);
}
#[test]
fn test47() {
    let js = r#"new x; new new x; new new x();"#;
    run_test("test47", js);
}
#[test]
fn test48() {
    let js = r#"new new x().a; new new x()[0];"#;
    run_test("test48", js);
}
#[test]
fn test49() {
    let js = r#"x(); x()(); x(x); x(x, x);"#;
    run_test("test49", js);
}
#[test]
fn test50() {
    let js = r#"x.a().a(); x[0]()[0](); x().a[0]();"#;
    run_test("test50", js);
}
#[test]
fn test51() {
    let js = r#"x++; x--;"#;
    run_test("test51", js);
}
#[test]
fn test52() {
    let js = r#"delete void typeof+-~!x; ++x; --x;"#;
    run_test("test52", js);
}
#[test]
fn test53() {
    let js = r#"0*0; 0/0; 0%0;"#;
    run_test("test53", js);
}
#[test]
fn test54() {
    let js = r#"0+0; 0-0;"#;
    run_test("test54", js);
}
#[test]
fn test55() {
    let js = r#"0<<0; 0>>0; 0>>>0;"#;
    run_test("test55", js);
}
#[test]
fn test56() {
    let js = r#"0<0; 0>0; 0<=0; 0>=0;"#;
    run_test("test56", js);
}
#[test]
fn test57() {
    let js = r#"0 instanceof function(){};"#;
    run_test("test57", js);
}
#[test]
fn test58() {
    let js = r#"0 in{};"#;
    run_test("test58", js);
}
#[test]
fn test59() {
    let js = r#"0==0; 0!=0; 0===0; 0!==0;"#;
    run_test("test59", js);
}
#[test]
fn test60() {
    let js = r#"0&0; 0^0; 0|0; 0&&0; 0||0;"#;
    run_test("test60", js);
}
#[test]
fn test61() {
    let js = r#"0?0:0; 0?0?0:0:0; 0||0?x=0:x=0;"#;
    run_test("test61", js);
}
#[test]
fn test62() {
    let js = r#"x=0; x*=0; x/=0; x%=0; x+=0; x-=0;"#;
    run_test("test62", js);
}
#[test]
fn test63() {
    let js = r#"x<<=0; x>>=0; x>>>=0; x&=0; x^=0; x|=0;"#;
    run_test("test63", js);
}
#[test]
fn test64() {
    let js = r#"0,0; 0,0,0; x=0,x=0;"#;
    run_test("test64", js);
}
#[test]
fn test65() {
    let js = r#"{} {;} {0} {0;} {0;0} {0;0;}"#;
    run_test("test65", js);
}
#[test]
fn test66() {
    let js = r#"var x; var x,y; var x,y,z;"#;
    run_test("test66", js);
}
#[test]
fn test67() {
    let js = r#"var x=0; var x=0,y; var x,y=0; var x=0,y=0;"#;
    run_test("test67", js);
}
#[test]
fn test68() {
    let js = r#";"#;
    run_test("test68", js);
}
#[test]
fn test69() {
    let js = r#"if(0); if(0);else;"#;
    run_test("test69", js);
}
#[test]
fn test70() {
    let js = r#"do;while(0);"#;
    run_test("test70", js);
}
#[test]
fn test71() {
    let js = r#"while(0);"#;
    run_test("test71", js);
}
#[test]
fn test72() {
    let js = r#"for(;;)break; for(0;0;0); for((0 in[]);0;);"#;
    run_test("test72", js);
}
#[test]
fn test73() {
    let js = r#"for(var a;;)break; for(var a,b;0;0);"#;
    run_test("test73", js);
}
#[test]
fn test74() {
    let js = r#"for(var a=0;;)break; for(var a=(0 in[]);0;);"#;
    run_test("test74", js);
}
#[test]
fn test75() {
    let js = r#"for(x in{}); for(var x in{});"#;
    run_test("test75", js);
}
#[test]
fn test76() {
    let js = r#"for(var x=[]in{}); for(var x=(0 in[])in{});"#;
    run_test("test76", js);
}
#[test]
fn test77() {
    let js = r#"for(;0;)continue; x:for(;0;)continue x;"#;
    run_test("test77", js);
}
#[test]
fn test78() {
    let js = r#"for(;;)break; x:for(;;)break x;"#;
    run_test("test78", js);
}
#[test]
fn test79() {
    let js = r#"switch(0){case 0:break;}"#;
    run_test("test79", js);
}
#[test]
fn test80() {
    let js = r#"function f(){ return; }"#;
    run_test("test80", js);
}
#[test]
fn test81() {
    let js = r#"function f(){ return 0; }"#;
    run_test("test81", js);
}
#[test]
fn test82() {
    let js = r#"with(0);"#;
    run_test("test82", js);
}
#[test]
fn test83() {
    let js = r#"switch(0){} switch(0){case 0:} switch(0){case 0:case 0:}"#;
    run_test("test83", js);
}
#[test]
fn test84() {
    let js = r#"switch(0){default:} switch(0){case 0:default:case 0:}"#;
    run_test("test84", js);
}
#[test]
fn test85() {
    let js = r#"switch(0){case 0:;} switch(0){case 0:;;}"#;
    run_test("test85", js);
}
#[test]
fn test86() {
    let js = r#"switch(0){default:;} switch(0){default:;;}"#;
    run_test("test86", js);
}
#[test]
fn test87() {
    let js = r#"x:; x:y:;"#;
    run_test("test87", js);
}
#[test]
fn test88() {
    let js = r#"try { throw 0; }catch(x){}"#;
    run_test("test88", js);
}
#[test]
fn test89() {
    let js = r#"try{}catch(x){}"#;
    run_test("test89", js);
}
#[test]
fn test90() {
    let js = r#"try{}finally{}"#;
    run_test("test90", js);
}
#[test]
fn test91() {
    let js = r#"try{}catch(x){}finally{}"#;
    run_test("test91", js);
}
#[test]
fn test92() {
    let js = r#"debugger;"#;
    run_test("test92", js);
}
#[test]
fn test93() {
    let js = r#"function f(){}"#;
    run_test("test93", js);
}
#[test]
fn test94() {
    let js = r#"function f(x){}"#;
    run_test("test94", js);
}
#[test]
fn test95() {
    let js = r#"function f(x,y){}"#;
    run_test("test95", js);
}
#[test]
fn test96() {
    let js = r#"function f(){ function f(){} }"#;
    run_test("test96", js);
}
#[test]
fn test97() {
    let js = r#"function f(){ "use strict" }"#;
    run_test("test97", js);
}
#[test]
fn test98() {
    let js = r#"function f(){ 'use strict' }"#;
    run_test("test98", js);
}
#[test]
fn test99() {
    let js = r#"function f(){ "other directive" }"#;
    run_test("test99", js);
}
#[test]
fn test100() {
    let js = r#"function f(){ 'other directive' }"#;
    run_test("test100", js);
}
#[test]
fn test101() {
    let js = r#"function f(){ ("string") }"#;
    run_test("test101", js);
}
#[test]
fn test102() {
    let js = r#"function f(){ ('string') }"#;
    run_test("test102", js);
}
#[test]
fn test103() {
    let js = r#"function f(){
  'string'
  +0
}"#;
    run_test("test103", js);
}
#[test]
fn test104() {
    let js = r#"(function(){});"#;
    run_test("test104", js);
}
#[test]
fn test105() {
    let js = r#"(function(x){});"#;
    run_test("test105", js);
}
#[test]
fn test106() {
    let js = r#"(function(x,y){});"#;
    run_test("test106", js);
}
#[test]
fn test107() {
    let js = r#"(function(){ function f(){} });"#;
    run_test("test107", js);
}
#[test]
fn test108() {
    let js = r#"(function f(){});"#;
    run_test("test108", js);
}
#[test]
fn test109() {
    let js = r#"(function f(x){});"#;
    run_test("test109", js);
}
#[test]
fn test110() {
    let js = r#"(function f(x,y){});"#;
    run_test("test110", js);
}
#[test]
fn test111() {
    let js = r#"(function f(){ function f(){} });"#;
    run_test("test111", js);
}
