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
fn test_example_28() {
    let input = "Foo\n***\nbar\n";
    let output = "<p>Foo</p><hr /><p>bar</p>";
    assert_eq!(exec(input), output);
}

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
fn test_example_199() {
    let input = "> # Foo\n> bar\n> baz";
    let output = "<blockquote><h1>Foo</h1><p>bar\nbaz</p></blockquote>";
    assert_eq!(exec(input), output);
}