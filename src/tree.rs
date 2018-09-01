use block::Block;
use block::BlockType;
use block_parser::Rule;
use pest::iterators::Pairs;

pub fn to_tree(tokens: Pairs<Rule>) -> Block {
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
            Rule::break_line | Rule::empty => {
                root_block.add(BlockType::BreakLine, "".to_string());
            }
            Rule::paragraph => {
                let mut update = false;
                match root_block.get_prev() {
                    None => (),
                    Some(prev) => match prev {
                        Block {
                            block_type: BlockType::Paragraph,
                            ..
                        } => {
                            prev.update("\n");
                            prev.update(token.as_str());
                            update = true;
                        }
                        _ => (),
                    },
                }

                if !update {
                    root_block.add(BlockType::Paragraph, token.as_str().to_string());
                }
            }
            Rule::atx_heading1 => {
                let text = token.into_inner().next().unwrap();
                root_block.add(BlockType::AtxHeading1, text.as_str().to_string());
            }
            Rule::atx_heading2 => {
                let text = token.into_inner().next().unwrap();
                root_block.add(BlockType::AtxHeading2, text.as_str().to_string());
            }
            Rule::atx_heading3 => {
                let text = token.into_inner().next().unwrap();
                root_block.add(BlockType::AtxHeading3, text.as_str().to_string());
            }
            Rule::atx_heading4 => {
                let text = token.into_inner().next().unwrap();
                root_block.add(BlockType::AtxHeading4, text.as_str().to_string());
            }
            Rule::atx_heading5 => {
                let text = token.into_inner().next().unwrap();
                root_block.add(BlockType::AtxHeading5, text.as_str().to_string());
            }
            Rule::atx_heading6 => {
                let text = token.into_inner().next().unwrap();
                root_block.add(BlockType::AtxHeading6, text.as_str().to_string());
            }
            Rule::intented_code_block => {
                let text = token.into_inner().next().unwrap();
                root_block.add(BlockType::AtxHeading6, text.as_str().to_string());
            }
            _ => (),
        }
    }

    root_block
}
