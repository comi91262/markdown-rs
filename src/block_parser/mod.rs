use pest::iterators::Pairs;
use pest::Parser;

#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("block.pest");

#[derive(Parser)]
#[grammar = "block_parser/block.pest"]
struct BlockParser;

pub fn parse(line: &str) -> Pairs<Rule> {
    BlockParser::parse(Rule::document, line).unwrap_or_else(|e| panic!("{}", e))
}

#[test]
fn test_parsing_themantic_break() {
    parses_to! {
        parser: BlockParser,
        input: "*     *      *      *\n",
        rule: Rule::document,
        tokens: [
          thematic_break(0, 21, [
          ]),
        ]
    };
}

#[test]
fn test_parsing_paragraph_underlines_as_setext_heading_underlines() {
    parses_to! {
        parser: BlockParser,
        input: "-- a\n",
        rule: Rule::document,
        tokens: [
          paragraph(0, 4, [
          ]),
        ]
    };
}

#[test]
fn test_parsing_paragraph_as_themantic_break() {
    parses_to! {
        parser: BlockParser,
        input: "_ _ _ _ a\n",
        rule: Rule::document,
        tokens: [
          paragraph(0, 9, [
          ]),
        ]
    };
}

#[test]
fn test_parsing_indented_code_block() {
    parses_to! {
        parser: BlockParser,
        input: "    indented code block\n",
        rule: Rule::document,
        tokens: [
          indented_code_block(0, 23, [
            text(4,23,[]),
          ]),
        ]
    };
}

#[test]
fn test_parsing_fenced_code_block() {
    parses_to! {
        parser: BlockParser,
        input: "```\naaa\n~~~\n```\n",
        rule: Rule::document,
        tokens: [
          fenced_code_block(0, 15, [
            fenced_text1(4, 12, [])
          ]),
        ]
    };
}

#[test]
fn test_parsing_atx_headings() {
    parses_to! {
        parser: BlockParser,
        input: "# foo\n## foo\n### foo\n#### foo\n##### foo\n###### foo\n",
        rule: Rule::document,
        tokens: [
          atx_heading1(0, 5, [
            text(2,5,[]),
          ]),
          atx_heading2(6, 12, [
            text(9,12,[]),
          ]),
          atx_heading3(13, 20, [
            text(17,20,[]),
          ]),
          atx_heading4(21, 29, [
            text(26,29,[]),
          ]),
          atx_heading5(30, 39, [
            text(36,39,[]),
          ]),
          atx_heading6(40, 50, [
            text(47,50,[]),
          ]),
        ]
    };
}

#[test]
fn test_parsing_paragraph() {
    parses_to! {
        parser: BlockParser,
        input: "  aaa\nbbb\n\nccc\nd d d\n",
        rule: Rule::document,
        tokens: [
          paragraph(0, 5, [
          ]),
          paragraph(6, 9, [
          ]),
          break_line(10, 10, [
          ]),
          paragraph(11, 14, [
          ]),
          paragraph(15, 20, [
          ])
        ]
    };
}

#[test]
fn test_parsing_setext_heading_underlines() {
    parses_to! {
        parser: BlockParser,
        input: "Foo\n-------------------------\n\nFoo\n=\n",
        rule: Rule::document,
        tokens: [
          paragraph(0, 3, [
          ]),
          setext_heading_underline2(4, 29, [
          ]),
          break_line(30, 30, [
          ]),
          paragraph(31, 34, [
          ]),
          setext_heading_underline1(35, 36, [
          ])
        ]
    };
}

#[test]
fn test_parsing_empty() {
    parses_to! {
        parser: BlockParser,
        input: "\n",
        rule: Rule::document,
        tokens: [
          break_line(0, 0, [
          ]),
        ]
    };
}

#[test]
fn test_parsing_block_quote() {
    parses_to! {
        parser: BlockParser,
        input: "> # Foo\n> bar\n> baz\n",
        rule: Rule::document,
        tokens: [
          block_quote(0, 7, [
            atx_heading1(2, 7, [
               text(4, 7, []),
            ])
          ]),
          block_quote(8, 13, [
            paragraph(10, 13, []),
          ]),
          block_quote(14, 19, [
            paragraph(16, 19, []),
          ]),
        ]
    };
}
