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
        Block { raw_text, .. } => {
            let mut s = raw_text.to_string();
            let s = decode_html(&s).unwrap();
            raw_text.clear();
            raw_text.push_str(&s);
        }
    }
}

#[test]
fn test_inline_parser() {
    //    let mut root_block = Block {
    //        is_closed: false,
    //        block_type: BlockType::Document,
    //        raw_text: "aaa".to_string(),
    //        children: vec![],
    //    };

    //    assert_eq!(
    //        "a  a   a".to_string(),
    //        root_block.get_mut_prev().unwrap().raw_text
    //    );
}
