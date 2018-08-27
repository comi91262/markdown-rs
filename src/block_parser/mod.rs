use pest::Parser;

#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("block.pest");

#[derive(Parser)]
#[grammar = "block_parser/block.pest"]
struct BlockParser;

#[derive(Debug)]
pub enum BlockType {
    Document,
    ThematicBreaks,
    BreakLine,
    //    AtxHeadings,
    //    BlockQuote,
    //    Paragraph,
    //    List,
    //    ListItem,
}

#[derive(Debug)]
pub struct Block {
    pub block_type: BlockType,
    pub children: Vec<Block>,
    is_closed: bool,
    raw_text: String,
}

impl Block {
    fn add(&mut self, block_type: BlockType, text: String) {
        let child = Block {
            is_closed: false,
            block_type: block_type,
            raw_text: text,
            children: vec![],
        };

        self.children.push(child);
    }

    fn get(&self) -> &Block {
        &self
    }
}

pub fn parse(line: &str) -> Block {
    let tokens = BlockParser::parse(Rule::document, line).unwrap_or_else(|e| panic!("{}", e));

    let mut root_block = Block {
        is_closed: false,
        block_type: BlockType::Document,
        raw_text: "".to_string(),
        children: vec![],
    };

    for token in tokens {
        match token.as_rule() {
            Rule::thematic_break => {
                root_block.add(BlockType::ThematicBreaks, "".to_string());
            }
            Rule::break_line => {
                root_block.add(BlockType::BreakLine, "".to_string());
            }
            _ => (),
        }
    }
    println!("{:?}", root_block);
    root_block
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

#[test]
fn test_example_13() {
    parses_to! {
        parser: BlockParser,
        input: "***\n---\n___\n",
        rule: Rule::document,
        tokens: [
          thematic_break(0, 3, [
          ]),
          thematic_break(4, 7, [
          ]),
          thematic_break(8, 11, [
          ])
        ]
    };
}

#[test]
fn test_example_xx() {
    parses_to! {
        parser: BlockParser,
        input: " \n",
        rule: Rule::document,
        tokens: [
          break_line(0, 1, [
          ]),
        ]
    };
}
//fn test_s() {
//    let mut root_block = Block {
//        is_closed: false,
//        block_type: BlockType::Document,
//        raw_text: "".to_string(),
//        children: vec![],
//    };
//
//    for token in parse("***\n---\n___\n") {
//        match token {
//            thematic_break => root_block.add(BlockType::AtxHeadings, "".to_string()),
//            _ => println!("{:?}", token),
//
//        }
//
//    }
//
//     println!("{:?}", root_block);
//
//}
//

//# foo
//## foo
//### foo
//#### foo
//##### foo
//###### foo
//####### foo
//
//<p>####### foo</p>
//
//<h1>foo</h1>
//<h2>foo</h2>
//<h3>foo</h3>
//<h4>foo</h4>
//<h5>foo</h5>
//<h6>foo</h6>
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
//
//    println!("{:?}", root_block);
//
//pest::parses_to! {
//    parser: MarkdownParser,
//    input: "# 1",
//    rule: Rule::exp,
//    tokens: [
//    ]
//};
