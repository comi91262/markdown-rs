use translator::exec;

/// # Example 13
///***
///---
///___
///
///<hr />
///<hr />
///<hr />
#[test]
fn test_example_13() {
    let html_code = exec("***\n---\n___\n");
    assert_eq!(html_code, "<hr />\n<hr />\n<hr />\n");
}

/// # Example 14
///+++
///
///<p>+++</p>
#[test]
fn test_example_14() {
    let html_code = exec("+++\n");
    assert_eq!(html_code, "<p>+++</p>\n");
}

/// # Example 15
///===
///
///<p>===</p>
#[test]
fn test_example_15() {
    let html_code = exec("===\n");
    assert_eq!(html_code, "<p>===</p>\n");
}

/// # Example 16
///--
///**
///__
///
///<p>--
///**
///__</p>
#[test]
fn test_example_16() {
    let html_code = exec("--\n**\n__\n");
    assert_eq!(html_code, "<p>--\n**\n__</p>\n");
}

/// # Example 17
/// ***
///  ***
///   ***
///
///<hr />
///<hr />
///<hr />
#[test]
fn test_example_17() {
    let html_code = exec(" ***\n  ***\n   ***\n");
    assert_eq!(html_code, "<hr />\n<hr />\n<hr />\n");
}

/// # Example 18
///    ***
///
///<pre><code>***
///</code></pre>
#[test]
fn test_example_18() {
    let html_code = exec("    ***\n");
    assert_eq!(html_code, "<pre><code>***\n</code></pre>\n");
}

///Example 19
///Foo
///    ***
///
///<p>Foo
///***</p>
#[test]
fn test_example_19() {
    let html_code = exec("Foo\n    ***\n");
    assert_eq!(html_code, "<p>Foo\n***</p>\n");
}

/// # Example 20
///_____________________________________
///
///<hr />
#[test]
fn test_example_20() {
    let html_code = exec("_____________________________________\n");
    assert_eq!(html_code, "<hr />\n");
}

/// # Example 21
/// - - -
///
///<hr />
#[test]
fn test_example_21() {
    let html_code = exec(" - - -\n");
    assert_eq!(html_code, "<hr />\n");
}

/// # Example 22
/// **  * ** * ** * **
///  
///<hr />
#[test]
fn test_example_22() {
    let html_code = exec(" **  * ** * ** * **\n");
    assert_eq!(html_code, "<hr />\n");
}

/// # Example 23
///-     -      -      -
///
///<hr />
#[test]
fn test_example_23() {
    let html_code = exec("-     -      -      -\n");
    assert_eq!(html_code, "<hr />\n");
}

/// # Example 24
///- - - -    
///
///<hr />
#[test]
fn test_example_24() {
    let html_code = exec("- - - -    \n");
    assert_eq!(html_code, "<hr />\n");
}

/// # Example 25
///_ _ _ _ a
///
///a------
///
///---a---
///
///<p>_ _ _ _ a</p>
///<p>a------</p>
///<p>---a---</p>
#[test]
fn test_example_25() {
    let html_code = exec("_ _ _ _ a\n\na------\n\n---a---\n");
    assert_eq!(
        html_code,
        "<p>_ _ _ _ a</p>\n<p>a------</p>\n<p>---a---</p>\n"
    );
}

/// # Example 28
///Foo
///***
///bar
///
///<p>Foo</p>
///<hr />
///<p>bar</p>
#[test]
fn test_example_28() {
    let html_code = exec("Foo\n***\nbar\n");
    assert_eq!(html_code, "<p>Foo</p>\n<hr />\n<p>bar</p>\n");
}

/// # Example 32
///# foo
///## foo
///### foo
///#### foo
///##### foo
///###### foo
///
///<h1>foo</h1>
///<h2>foo</h2>
///<h3>foo</h3>
///<h4>foo</h4>
///<h5>foo</h5>
///<h6>foo</h6>
#[test]
fn test_example_32() {
    let html_code = exec("# foo\n## foo\n### foo\n#### foo\n##### foo\n###### foo\n");
    assert_eq!(
        html_code,
        "<h1>foo</h1>\n<h2>foo</h2>\n<h3>foo</h3>\n<h4>foo</h4>\n<h5>foo</h5>\n<h6>foo</h6>\n"
    );
}

/// # Example 33
///####### foo
///
///<p>####### foo</p>
#[test]
fn test_example_33() {
    let html_code = exec("####### foo\n");
    assert_eq!(html_code, "<p>####### foo</p>\n");
}

/// # Example 34
///#5 bolt
///
///#hashtag
///
///<p>#5 bolt</p>
///<p>#hashtag</p>
#[test]
fn test_example_34() {
    let html_code = exec("#5 bolt\n\n#hashtag\n");
    assert_eq!(html_code, "<p>#5 bolt</p>\n<p>#hashtag</p>\n");
}

/// # Example 37
///#                  foo
///
///<h1>foo</h1>
#[test]
fn test_example_37() {
    let html_code = exec("#                  foo                     \n");
    assert_eq!(html_code, "<h1>foo</h1>\n");
}

/// # Example 38
/// ### foo
///  ## foo
///   # foo
///
///<h3>foo</h3>
///<h2>foo</h2>
///<h1>foo</h1>
#[test]
fn test_example_38() {
    let html_code = exec(" ### foo\n  ## foo\n   # foo\n");
    assert_eq!(html_code, "<h3>foo</h3>\n<h2>foo</h2>\n<h1>foo</h1>\n");
}

/// # Example 39
///    # foo
///
///<pre><code># foo
///</code></pre>
#[test]
fn test_example_39() {
    let html_code = exec("    # foo\n");
    assert_eq!(html_code, "<pre><code># foo\n</code></pre>\n");
}

/// # Example 40
///foo
///    # bar
///
///<p>foo
///# bar</p>
#[test]
fn test_example_40() {
    let html_code = exec("foo\n    # bar\n");
    assert_eq!(html_code, "<p>foo\n# bar</p>\n");
}

/// # Example 52
///Foo
///-------------------------
///
///Foo
///=
///
///<h2>Foo</h2>
///<h1>Foo</h1>
#[test]
fn test_example_52() {
    let html_code = exec("Foo\n-------------------------\n\nFoo\n=\n");
    assert_eq!(html_code, "<h2>Foo</h2>\n<h1>Foo</h1>\n");
}

/// # Example 76
///    a simple
///      indented code block
///
///<pre><code>a simple
///  indented code block
///</code></pre>
#[test]
fn test_example_76() {
    let html_code = exec("    a simple\n      indented code block\n");
    assert_eq!(
        html_code,
        "<pre><code>a simple\n  indented code block\n</code></pre>\n"
    );
}

/// # Example 182
///aaa
///
///bbb
///  
///<p>aaa</p>
///<p>bbb</p>
#[test]
fn test_example_182() {
    let html_code = exec("aaa\n\nbbb");
    assert_eq!(html_code, "<p>aaa</p>\n<p>bbb</p>\n");
}

/// # Example 183
///aaa
///bbb
///
///ccc
///ddd
///  
///<p>aaa
///bbb</p>
///<p>ccc
///ddd</p>
#[test]
fn test_example_183() {
    let html_code = exec("aaa\nbbb\n\nccc\nddd\n");
    assert_eq!(html_code, "<p>aaa\nbbb</p>\n<p>ccc\nddd</p>\n");
}

/// # Example 184
///aaa
///
///
///bbb
///
///<p>aaa</p>
///<p>bbb</p>
#[test]
fn test_example_184() {
    let html_code = exec("aaa\n\n\nbbb\n");
    assert_eq!(html_code, "<p>aaa</p>\n<p>bbb</p>\n");
}

/// # Example 185
///  aaa
/// bbb
///
///<p>aaa
///bbb</p>
#[test]
fn test_example_185() {
    let html_code = exec("  aaa\n bbb\n");
    assert_eq!(html_code, "<p>aaa\nbbb</p>\n");
}

/// # Example 186
///aaa
///             bbb
///                                       ccc
///
///<p>aaa
///bbb
///ccc</p>
#[test]
fn test_example_186() {
    let html_code = exec("aaa\n             bbb\n                                       ccc\n");
    assert_eq!(html_code, "<p>aaa\nbbb\nccc</p>\n");
}

/// # Example 187
///   aaa
///bbb
///
///<p>aaa
///bbb</p>
#[test]
fn test_example_187() {
    let html_code = exec("   aaa\nbbb\n");
    assert_eq!(html_code, "<p>aaa\nbbb</p>\n");
}

/// # Example 199
///> # Foo
///> bar
///> baz
///
///<blockquote>
///<h1>Foo</h1>
///<p>bar
///baz</p>
///</blockquote>
#[test]
fn test_example_199() {
    let html_code = exec("> # Foo\n> bar\n> baz\n");
    assert_eq!(
        html_code,
        "<blockquote>\n<h1>Foo</h1>\n<p>bar\nbaz</p>\n</blockquote>\n"
    );
}
