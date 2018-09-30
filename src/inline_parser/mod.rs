use pest::iterators::Pairs;
use pest::Error;
use pest::Parser;

#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("inline.pest");

#[derive(Parser)]
#[grammar = "inline_parser/inline.pest"]
struct InlineParser;

pub fn parse(line: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    InlineParser::parse(Rule::inline, line)
}

use block::Block;
use block::BlockType;
use htmlescape::decode_html;

pub fn inline_parser(block_tree: &mut Block) {
    match block_tree {
        Block {
            block_type: BlockType::Document,
            children,
            ..
        } => {
            for child in children {
                inline_parser(child);
            }
        }
        Block {
            block_type: BlockType::Paragraph,
            raw_text,
            ..
        } => {
            let mut s = raw_text.to_string();
            let s = match decode_html(&s) {
                Ok(actual) => actual,
                Err(_) => s,
            };

            let s = escape_backslash(&s);

            let s = match parse(&s) {
                Ok(tokens) => interpret(tokens),
                _ => s.to_string(),
            };
            raw_text.clear();
            raw_text.push_str(&s);
        }
        Block {
            block_type: BlockType::AtxHeading1, // TODO inplement h2,3,4,5 and 6
            raw_text,
            ..
        } => {
            let mut s = raw_text.to_string();
            let s = decode_html(&s).unwrap();

            let s = match parse(&s) {
                Ok(tokens) => interpret(tokens),
                _ => s.to_string(),
            };
            raw_text.clear();
            raw_text.push_str(&s);
        }
        Block {
            block_type: BlockType::SetextHeadingUnderline1,
            raw_text,
            ..
        } => {
            let mut s = raw_text.to_string();
            let s = decode_html(&s).unwrap();

            let s = match parse(&s) {
                Ok(tokens) => interpret(tokens),
                _ => s.to_string(),
            };
            raw_text.clear();
            raw_text.push_str(&s);
        }
        Block {
            block_type: BlockType::SetextHeadingUnderline2,
            raw_text,
            ..
        } => {
            let mut s = raw_text.to_string();
            let s = decode_html(&s).unwrap();

            let s = match parse(&s) {
                Ok(tokens) => interpret(tokens),
                _ => s.to_string(),
            };
            raw_text.clear();
            raw_text.push_str(&s);
        }
        Block { raw_text, .. } => {
            let mut s = raw_text.to_string();
            let s = decode_html(&s).unwrap();
            raw_text.clear();
            raw_text.push_str(&s);
        }
    }
}

fn interpret(tokens: Pairs<Rule>) -> String {
    let mut result = String::new();

    for token in tokens {
        match token.as_rule() {
            Rule::text => result.push_str(token.as_str()),
            Rule::emphasis => {
                result.push_str("<em>");
                result.push_str(token.as_str());
                result.push_str("</em>");
            }
            _ => panic!("Error: inte"),
        }
    }

    result
}

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

fn escape_backslash(s: &str) -> String {
    enum Status {
        SLASH,
        NOSLASH,
    };

    let mut status = Status::NOSLASH;

    s.chars()
        .map(|c| match status {
            Status::SLASH => match ESCAPED_CHARACTERS.binary_search_by_key(&c, |&(a, _)| a) {
                Ok(s) => {
                    status = Status::NOSLASH;
                    let (_, b) = ESCAPED_CHARACTERS[s];
                    b.to_string()
                }
                Err(_) => {
                    status = Status::NOSLASH;
                    format!("\\{}", c)
                }
            },
            Status::NOSLASH if c == '\\' => {
                status = Status::SLASH;
                "".to_string()
            }
            Status::NOSLASH => c.to_string(),
        }).collect()
}

#[test]
fn test_emphasis_rule1() {
    parses_to! {
        parser: InlineParser,
        input: "foo*bar*",
        rule: Rule::inline,
        tokens: [
          text(0, 3, [
          ]),
          emphasis(4, 7, [
          ]),
          text(8, 8, [
          ]),
        ]
    };
}