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
    }
}

pub fn exec(input_str: &str) -> String {
    let tree = block_parser::parse(input_str);
    print(tree)
}

#[test]
fn test_exec() {
    let tree = exec("***\n---\n___\n");
    assert_eq!(tree, "<hr /><hr /><hr />");
}
