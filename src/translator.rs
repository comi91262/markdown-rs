use block::Block;
use block::BlockType;
use block_parser;
use inline_parser;
use tree;

fn print(tree: Block) -> String {
    match tree {
        Block {
            block_type: BlockType::Document,
            children,
            ..
        } => {
            let mut result_str = String::new();
            for v in children {
                result_str.push_str(&print(v))
            }
            result_str
        }
        Block {
            block_type: BlockType::ThematicBreaks,
            ..
        } => {
            let mut result_str = String::with_capacity(6);
            result_str.push_str("<hr />\n");
            result_str
        }
        Block {
            block_type: BlockType::BreakLine,
            ..
        } => "".to_string(),
        Block {
            block_type: BlockType::Paragraph,
            raw_text,
            ..
        } => format!("<p>{}</p>\n", raw_text),
        Block {
            block_type: BlockType::AtxHeading1,
            raw_text,
            ..
        } => format!("<h1>{}</h1>\n", raw_text),
        Block {
            block_type: BlockType::AtxHeading2,
            raw_text,
            ..
        } => format!("<h2>{}</h2>\n", raw_text),
        Block {
            block_type: BlockType::AtxHeading3,
            raw_text,
            ..
        } => format!("<h3>{}</h3>\n", raw_text),
        Block {
            block_type: BlockType::AtxHeading4,
            raw_text,
            ..
        } => format!("<h4>{}</h4>\n", raw_text),
        Block {
            block_type: BlockType::AtxHeading5,
            raw_text,
            ..
        } => format!("<h5>{}</h5>\n", raw_text),
        Block {
            block_type: BlockType::AtxHeading6,
            raw_text,
            ..
        } => format!("<h6>{}</h6>\n", raw_text),
        Block {
            block_type: BlockType::SetextHeadingUnderline1,
            raw_text,
            ..
        } => format!("<h1>{}</h1>\n", raw_text),
        Block {
            block_type: BlockType::SetextHeadingUnderline2,
            raw_text,
            ..
        } => format!("<h2>{}</h2>\n", raw_text),
        Block {
            block_type: BlockType::IndentedCodeBlock,
            raw_text,
            ..
        } => format!("<pre><code>{}\n</code></pre>\n", raw_text),
        Block {
            block_type: BlockType::BlockQuote,
            children,
            ..
        } => {
            let mut result_str = String::new();
            for v in children {
                result_str.push_str(&print(v))
            }
            format!("<blockquote>\n{}</blockquote>\n", result_str)
        }
        Block {
            block_type: BlockType::ListItem,
            children,
            ..
        } => {
            let mut result_str = String::new();
            for v in children {
                result_str.push_str(&print(v))
            }
            format!("<ol><li>\n{}</li></ol>\n", result_str)
        }
    }
}

pub fn exec(input_str: &str) -> String {
    // Add line feed.
    let mut input = String::new();
    input.push_str(input_str);
    input.push_str("\n");

    let tokens = block_parser::parse(&input);
    let tree = tree::to_tree(tokens);
    //inline_parser::inline_parser(&mut tree);
    print(tree)
}
