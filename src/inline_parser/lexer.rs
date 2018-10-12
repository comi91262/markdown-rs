use pest::error::Error;
use pest::iterators::Pairs;
use pest::Parser;

#[derive(Parser)]
#[grammar = "inline_parser/inline.pest"]
struct InlineParser;

pub fn lex(line: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    InlineParser::parse(Rule::inline, line)
}

#[test]
fn test_parsing_hard_line_break() {
    parses_to! {
        parser: InlineParser,
        input: "  \n",
        rule: Rule::inline,
        tokens: [
            hard_line_break(0, 3, [])
        ]
    };

    parses_to! {
        parser: InlineParser,
        input: "        \n",
        rule: Rule::inline,
        tokens: [
            hard_line_break(0, 9, [])
        ]
    };

    parses_to! {
        parser: InlineParser,
        input: "\\\n",
        rule: Rule::inline,
        tokens: [
            hard_line_break(0, 2, [])
        ]
    };
}

#[test]
fn test_html_entity() {
    parses_to! {
        parser: InlineParser,
        input: "&amp;",
        rule: Rule::inline,
        tokens: [
            html_entity(0, 5, [])
        ]
    };
}

#[test]
fn test_escaped_slash() {
    parses_to! {
        parser: InlineParser,
        input: "\\&",
        rule: Rule::inline,
        tokens: [
            escaped_slash(0, 2, []),
        ]
    };
}

#[test]
fn test_parse_sequence() {
    parses_to! {
        parser: InlineParser,
        input: "\\& &yopf;  \n\\$",
        rule: Rule::inline,
        tokens: [
            escaped_slash(0, 2, []),
            other(2, 3, []),
            html_entity(3, 9, []),
            hard_line_break(9, 12, []),
            escaped_slash(12, 14, []),
        ]
    };
}

#[test]
fn test_emphasis_rule1() {
    parses_to! {
        parser: InlineParser,
        input: "foo*bar*",
        rule: Rule::inline,
        tokens: [
          other(0, 1, [
          ]),
          other(1, 2, [
          ]),
          other(2, 3, [
          ]),
          emphasis(4, 7, [
          ]),
        ]
    };
}

#[test]
fn test_parse_empty_sequence() {
    parses_to! {
        parser: InlineParser,
        input: "",
        rule: Rule::inline,
        tokens: [
        ]
    };
}
