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
        input: "***\n---\n___",
        rule: Rule::document,
        tokens: [
          thematic_break(0, 3, [
          ]),
          setext_heading_underline2(4, 7, [  // -; setext_heading > thematic_break
          ]),
          thematic_break(8, 11, [
          ]),
        ]
    };
}

#[test]
fn test_parsing_indented_code_block() {
    parses_to! {
        parser: BlockParser,
        input: "    indented code block",
        rule: Rule::document,
        tokens: [
          indented_code_block(0, 23, [
            text(4,23,[]),
          ]),
        ]
    };
}

#[test]
fn test_parsing_atx_headings() {
    parses_to! {
        parser: BlockParser,
        input: "# foo\n## foo\n### foo\n#### foo\n##### foo\n###### foo",
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
        input: "  aaa\nbbb\n\nccc\nd d d",
        rule: Rule::document,
        tokens: [
          paragraph(2, 5, [
          ]),
          paragraph(6, 9, [
          ]),
          empty(10, 10, [
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
        input: "Foo\n-------------------------\n\nFoo\n=",
        rule: Rule::document,
        tokens: [
          paragraph(0, 3, [
          ]),
          setext_heading_underline2(4, 29, [
          ]),
          empty(30, 30, [
          ]),
          paragraph(31, 34, [
          ]),
          setext_heading_underline1(35, 36, [
          ])
        ]
    };
}

// > Lorem ipsum dolor
// sit amet.
// > - Qui *quodsi iracundia*
// > - aliquando id
//-> document
//  -> block_quote
//       paragraph
//         "Lorem ipsum dolor\nsit amet."
//    -> list (type=bullet tight=true bullet_char=-)
//         list_item
//           paragraph
//             "Qui *quodsi iracundia*"
//      -> list_item
//        -> paragraph
//             "aliquando id"
//    root_block.add(BlockType::BlockQuote, "".to_string());
//
//    let block2 = Block {
//        is_closed: false,
//        block_type: BlockType::Document,
//        raw_text: "".to_string(),
//        children: vec![Block {
//            is_closed: false,
//            block_type: BlockType::BlockQuote,
//            raw_text: "".to_string(),
//            children: vec![
//                Block {
//                    is_closed: false,
//                    block_type: BlockType::Paragraph,
//                    raw_text: "Lorem ipsum dolor\nsit amet.".to_string(),
//                    children: vec![],
//                },
//                Block {
//                    is_closed: false,
//                    block_type: BlockType::List,
//                    raw_text: "".to_string(),
//                    children: vec![],
//                },
//            ],
//        }],
//    };
