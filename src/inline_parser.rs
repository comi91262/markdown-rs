use block::Block;
use block::BlockType;

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
            let s = raw_text.trim().to_string();
            raw_text.clear();
            raw_text.push_str(&s);
        }
        _ => (),
    }
}

#[test]
fn test_inline_parser() {
    let mut root_block = Block {
        is_closed: false,
        block_type: BlockType::Document,
        raw_text: "aaa".to_string(),
        children: vec![],
    };

    root_block.add(BlockType::Paragraph, "   a  a   a   ".to_string());
    inline_parser(&mut root_block);

    assert_eq!(
        "a  a   a".to_string(),
        root_block.get_mut_prev().unwrap().raw_text
    );
}
