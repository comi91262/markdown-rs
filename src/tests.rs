use translator::exec;

#[test]
fn test_example_1() {
    let input = "\tfoo\tbaz\t\tbim";
    let output = "<pre><code>foo\tbaz\t\tbim</code></pre>";

    assert_eq!(exec(input), output);
}

#[test]
fn test_example_2() {
    let input = "  \tfoo\tbaz\t\tbim";
    let output = "<pre><code>foo\tbaz\t\tbim</code></pre>";

    assert_eq!(exec(input), output);
}

#[test]
fn test_example_3() {
    let input = "    a\ta\n    ὐ\ta";
    let output = "<pre><code>a\ta\nὐ\ta</code></pre>";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_13() {
    let input = "***\n---\n___";
    let output = "<hr /><hr /><hr />";

    assert_eq!(exec(input), output);
}

#[test]
fn test_example_14() {
    let input = "+++";
    let output = "<p>+++</p>";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_15() {
    let input = "===";
    let output = "<p>===</p>";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_16() {
    let input = "--\n**\n__";
    let output = "<p>--\n**\n__</p>";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_17() {
    let input = " ***\n  ***\n   ***";
    let output = "<hr /><hr /><hr />";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_18() {
    let input = "    ***";
    let output = "<pre><code>***</code></pre>";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_19() {
    let input = "Foo\n    ***";
    let output = "<p>Foo\n***</p>";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_20() {
    let input = "_____________________________________";
    let output = "<hr />";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_21() {
    let input = " - - -";
    let output = "<hr />";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_22() {
    let input = " **  * ** * ** * **";
    let output = "<hr />";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_23() {
    let input = "-     -      -      -";
    let output = "<hr />";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_24() {
    let input = "- - - -    ";
    let output = "<hr />";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_25() {
    let input = "_ _ _ _ a\n\na------\n\n---a---";
    let output = "<p>_ _ _ _ a</p><p>a------</p><p>---a---</p>";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_26() {
    let input = "*-*";
    let output = "<p><em>-</em></p>";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_27() {
    let input = "- foo\n***\n- bar";
    let output = "<ul><li>foo</li></ul><hr /><ul><li>bar</li></ul>";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_28() {
    let input = "Foo\n***\nbar";
    let output = "<p>Foo</p><hr /><p>bar</p>";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_29() {
    let input = "Foo\n---\nbar";
    let output = "<h2>Foo</h2><p>bar</p>";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_30() {
    let input = "* Foo\n* * *\n* Bar";
    let output = "<ul><li>Foo</li></ul><hr /><ul><li>Bar</li></ul>";
    assert_eq!(exec(input), output);
}

//#[test]
//fn test_example_31() {
//    let input = "- Foo\n- * * *";
//    let output = "<ul><li>Foo</li></ul><li><hr /></li></ul>";
//    assert_eq!(exec(input), output);
//}

#[test]
fn test_example_32() {
    let input = "# foo\n## foo\n### foo\n#### foo\n##### foo\n###### foo";
    let output = "<h1>foo</h1><h2>foo</h2><h3>foo</h3><h4>foo</h4><h5>foo</h5><h6>foo</h6>";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_33() {
    let input = "####### foo";
    let output = "<p>####### foo</p>";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_34() {
    let input = "#5 bolt\n\n#hashtag";
    let output = "<p>#5 bolt</p><p>#hashtag</p>";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_37() {
    let input = "#                  foo                     ";
    let output = "<h1>foo</h1>";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_38() {
    let input = " ### foo\n  ## foo\n   # foo";
    let output = "<h3>foo</h3><h2>foo</h2><h1>foo</h1>";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_39() {
    let input = "    # foo";
    let output = "<pre><code># foo</code></pre>";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_40() {
    let input = "foo\n    # bar";
    let output = "<p>foo\n# bar</p>";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_50() {
    let input = "Foo *bar*\n=========\n\nFoo *bar*\n---------";
    let output = "<h1>Foo <em>bar</em></h1><h2>Foo <em>bar</em></h2>";

    assert_eq!(exec(input), output);
}

#[test]
fn test_example_51() {
    let input = "Foo *bar\nbaz*\n====";
    let output = "<h1>Foo <em>bar\nbaz</em></h1>";

    assert_eq!(exec(input), output);
}

#[test]
fn test_example_52() {
    let input = "Foo\n-------------------------\n\nFoo\n=";
    let output = "<h2>Foo</h2><h1>Foo</h1>";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_76() {
    let input = "    a simple\n      indented code block";
    let output = "<pre><code>a simple\n  indented code block</code></pre>";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_91() {
    let input = "```\naaa\n~~~\n```";
    let output = "<pre><code>aaa\n~~~</code></pre>";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_92() {
    let input = "~~~\naaa\n```\n~~~";
    let output = "<pre><code>aaa\n```</code></pre>";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_182() {
    let input = "aaa\n\nbbb";
    let output = "<p>aaa</p><p>bbb</p>";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_183() {
    let input = "aaa\nbbb\n\nccc\nddd";
    let output = "<p>aaa\nbbb</p><p>ccc\nddd</p>";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_184() {
    let input = "aaa\n\n\nbbb";
    let output = "<p>aaa</p><p>bbb</p>";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_185() {
    let input = "  aaa\n bbb";
    let output = "<p>aaa\nbbb</p>";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_186() {
    let input = "aaa\n             bbb\n                                       ccc";
    let output = "<p>aaa\nbbb\nccc</p>";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_187() {
    let input = "   aaa\nbbb";
    let output = "<p>aaa\nbbb</p>";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_190() {
    let input = "  \n\naaa\n  \n\n# aaa\n\n  ";
    let output = "<p>aaa</p><h1>aaa</h1>";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_199() {
    let input = "> # Foo\n> bar\n> baz";
    let output = "<blockquote><h1>Foo</h1><p>bar\nbaz</p></blockquote>";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_200() {
    let input = "># Foo\n>bar\n> baz";
    let output = "<blockquote><h1>Foo</h1><p>bar\nbaz</p></blockquote>";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_201() {
    let input = "   > # Foo\n   > bar\n > baz";
    let output = "<blockquote><h1>Foo</h1><p>bar\nbaz</p></blockquote>";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_202() {
    let input = " > # Foo\n> bar\nbaz";
    let output = "<blockquote><h1>Foo</h1><p>bar\nbaz</p></blockquote>";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_204() {
    let input = "> bar\nbaz\n> foo";
    let output = "<blockquote><p>bar\nbaz\nfoo</p></blockquote>";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_205() {
    let input = "> foo\n---";
    let output = "<blockquote><p>foo</p></blockquote><hr />";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_210() {
    let input = ">";
    let output = "<blockquote></blockquote>";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_211() {
    let input = ">\n>  \n> ";
    let output = "<blockquote></blockquote>";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_212() {
    let input = ">\n> foo\n>  ";
    let output = "<blockquote><p>foo</p></blockquote>";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_213() {
    let input = "> foo\n\n> bar";
    let output = "<blockquote><p>foo</p></blockquote><blockquote><p>bar</p></blockquote>";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_214() {
    let input = "> foo\n> bar";
    let output = "<blockquote><p>foo\nbar</p></blockquote>";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_215() {
    let input = "> foo\n>\n> bar";
    let output = "<blockquote><p>foo</p><p>bar</p></blockquote>";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_216() {
    let input = "foo\n> bar";
    let output = "<p>foo</p><blockquote><p>bar</p></blockquote>";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_217() {
    let input = "> aaa\n***\n> bbb";
    let output = "<blockquote><p>aaa</p></blockquote><hr /><blockquote><p>bbb</p></blockquote>";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_218() {
    let input = "> bar\nbaz";
    let output = "<blockquote><p>bar\nbaz</p></blockquote>";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_219() {
    let input = "> bar\n\nbaz";
    let output = "<blockquote><p>bar</p></blockquote><p>baz</p>";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_220() {
    let input = "> bar\n>\nbaz";
    let output = "<blockquote><p>bar</p></blockquote><p>baz</p>";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_224() {
    let input = "a paragraph\nwith two lines.\n\n    indented code\n\n> A block quote.";
    let output = "<p>a paragraph\nwith two lines.</p><pre><code>indented code</code></pre><blockquote><p>A block quote.</p></blockquote>";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_225() {
    let input =
        "1.  a paragraph\n    with two lines.\n\n        indented code\n\n    > A block quote.";
    let output = "<ol><li><p>a paragraph\nwith two lines.</p><pre><code>indented code</code></pre><blockquote><p>A block quote.</p></blockquote></li></ol>";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_226() {
    let input = "- one\n\n two\n";
    let output = "<ul><li>one</li></ul><p>two</p>";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_227() {
    let input = "- one\n\n  two\n";
    let output = "<ul><li><p>one</p><p>two</p></li></ul>";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_228() {
    let input = " -    one\n\n     two\n";
    let output = "<ul><li>one</li></ul><pre><code> two</code></pre>";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_229() {
    let input = " -    one\n\n      two";
    let output = "<ul><li><p>one</p><p>two</p></li></ul>";
    assert_eq!(exec(input), output);
}

//#[test]
//fn test_example_274() {
//    let input = "- foo\n- bar\n+ baz";
//    let output = "<ul><li>foo</li><li>bar</li></ul><ul><li>baz</li></ul>";
//    assert_eq!(exec(input), output);
//}

#[test]
fn test_example_314() {
    let input = "&nbsp; &amp; &copy; &AElig; &Dcaron;\n&frac34; &HilbertSpace; &DifferentialD;\n&ClockwiseContourIntegral; &ngE;";
    let output = "<p>  & © Æ Ď\n¾ ℋ ⅆ\n∲ ≧̸</p>"; // &amp?
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_343() {
    let input = "*foo bar*";
    let output = "<p><em>foo bar</em></p>";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_344() {
    let input = "a * foo bar*";
    let output = "<p>a * foo bar*</p>";
    assert_eq!(exec(input), output);
}

//#[test]
//fn test_example_346() {
//    let input = "* a *";
//    let output = "<p>* a *</p>";        ->  should be <ul><li>a *</li></ul>  ?
//    assert_eq!(exec(input), output);
//}

#[test]
fn test_example_347() {
    let input = "foo*bar*";
    let output = "<p>foo<em>bar</em></p>";
    assert_eq!(exec(input), output);
}

#[test]
fn test_example_348() {
    let input = "5*6*78";
    let output = "<p>5<em>6</em>78</p>";
    assert_eq!(exec(input), output);
}
