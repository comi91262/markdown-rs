use block_parser;
use block_parser::Block;
use block_parser::BlockType;

fn print(tree: block_parser::Block) -> String {
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
            result_str.push_str("<hr />");
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
        } => format!("<p>{}</p>", raw_text),
        Block {
            block_type: BlockType::AtxHeading1,
            raw_text,
            ..
        } => format!("<h1>{}</h1>", raw_text),
        _ => "".to_string(),
    }
}

pub fn exec(input_str: &str) -> String {
    let tree = block_parser::parse(input_str);
    print(tree)
}

#[test]
fn test_example_13() {
    let tree = exec("***\n---\n___\n");
    assert_eq!(tree, "<hr /><hr /><hr />");
}

//#[test]
//fn test_example_32() {
//}

//#[test]
//fn test_example_182() {
//    let tree = exec("aaa\n\nbbb\n");
//    assert_eq!(tree, "<p>aaa</p>\n<p>bbb</p>");
//}

//#[test]
//fn test_example_183() {
//    let tree = exec("aaa\n\nbbb\nccc\n\nddd\n");
//    assert_eq!(tree, "<p>aaa\nbbb</p>\n<p>ccc\nddd</p>");
//}
