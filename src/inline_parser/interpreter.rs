use super::lexer::Rule;
use htmlescape::decode_html;
use pest::iterators::Pairs;

const ESCAPED_CHARACTERS: [(char, &'static str); 32] = [
    ('!', "!"),
    ('"', "&quot;"),
    ('#', "#"),
    ('$', "$"),
    ('%', "%"),
    ('&', "&amp;"),
    ('\'', "\'"),
    ('(', "("),
    (')', ")"),
    ('*', "*"),
    ('+', "+"),
    (',', ","),
    ('-', "-"),
    ('.', "."),
    ('/', "/"),
    (':', ":"),
    (';', ";"),
    ('<', "&lt;"),
    ('=', "="),
    ('>', "&gt;"),
    ('?', "?"),
    ('@', "@"),
    ('[', "["),
    ('\\', "\\"),
    (']', "]"),
    ('^', "^"),
    ('_', "_"),
    ('`', "`"),
    ('{', "{"),
    ('|', "|"),
    ('}', "}"),
    ('~', "~"),
];

fn emphasize(s: &str) -> String {
    format!("<em>{}</em>", s)
}

fn hard_line_break(_s: &str) -> String {
    String::from("<br />")
}

fn escape_backslash(s: &str) -> String {
    let mut ch = s.chars();

    // TODO error handing
    //if &ch.count() != 2 {
    //    panic!("s length should be 2");
    //    return "".to_string()
    //}

    if ch.next().unwrap() != '\\' {
        panic!("s should start with \\");
    }

    let c = ch.next().unwrap();

    match ESCAPED_CHARACTERS.binary_search_by_key(&c, |&(a, _)| a) {
        Ok(s) => {
            let (_, b) = ESCAPED_CHARACTERS[s];
            b.to_string()
        }
        Err(_) => format!("\\{}", c),
    }
}

fn escape_html_entity(s: &str) -> String {
    decode_html(s).unwrap().to_string()
}

pub fn top(tokens: Pairs<Rule>) -> String {
    let mut result = String::new();

    for token in tokens {
        match token.as_rule() {
            Rule::escaped_slash => result.push_str(&escape_backslash(token.as_str())),
            Rule::html_entity => result.push_str(&escape_html_entity(token.as_str())),
            Rule::emphasis => result.push_str(&emphasize(token.as_str())),
            Rule::hard_line_break => result.push_str(&hard_line_break(token.as_str())),
            Rule::other => result.push_str(token.as_str()),
            Rule::EOI => (),
            _ => panic!("Error: No token is parsed."),
        }
    }
    result
}

#[test]
fn test_escape_backslash() {
    let input = "\\!";
    let output = "!";
    assert_eq!(escape_backslash(input), output);
}

#[test]
fn test_escape_html_entity() {
    let input = "&amp;";
    let output = "&";
    assert_eq!(escape_html_entity(input), output);
}

#[test]
fn test_emphasize() {
    let input = "aaa";
    let output = "<em>aaa</em>";
    assert_eq!(emphasize(input), output);
}
#[test]
fn test_hard_line_break() {
    let input = "\\\n";
    let output = "<br />";
    assert_eq!(hard_line_break(input), output);

    let input = "  \n";
    let output = "<br />";
    assert_eq!(hard_line_break(input), output);
}
