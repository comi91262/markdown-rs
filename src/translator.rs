use block::Block;
use block::BlockType;
use block_parser;

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
    }
}

pub fn exec(input_str: &str) -> String {
    let tree = block_parser::parse(input_str);
    print(tree)
}

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
    let tree = exec("***\n---\n___\n");
    assert_eq!(tree, "<hr />\n<hr />\n<hr />\n");
}

/// # Example 182
/// aaa
///
/// bbb
///  
/// <p>aaa</p>
/// <p>bbb</p>
#[test]
fn test_example_182() {
    let tree = exec("aaa\n\nbbb");
    assert_eq!(tree, "<p>aaa</p>\n<p>bbb</p>\n");
}

/// # Example 183
/// aaa
/// bbb
///
/// ccc
/// ddd
///  
/// <p>aaa
/// bbb</p>
/// <p>ccc
/// ddd</p>
#[test]
fn test_example_183() {
    let tree = exec("aaa\nbbb\n\nccc\nddd\n");
    assert_eq!(tree, "<p>aaa\nbbb</p>\n<p>ccc\nddd</p>\n");
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
    let tree = exec("aaa\n\n\nbbb\n");
    assert_eq!(tree, "<p>aaa</p>\n<p>bbb</p>\n");
}

// # Example 185
//  aaa
// bbb
//
//<p>aaa
//bbb</p>
//#[test]
//fn test_example_185() {
//    let tree = exec("  aaa\n bbb\n");
//    assert_eq!(tree, "<p>aaa\nbbb</p>\n");
//}
