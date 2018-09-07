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
                if let Some(prev) = root_block.get_mut_prev() {
                    match prev {
                        Block {
                            block_type: BlockType::Paragraph,
                            ..
                        } => {
                            prev.close();
                        }
                        _ => (),
                    }
                }
                root_block.add(BlockType::BreakLine, "".to_string());
            }
            Rule::paragraph => {
                let mut is_updated = false;
                match root_block.get_mut_prev() {
                    None => (),
                    Some(prev) => match prev {
                        Block {
                            block_type: BlockType::Paragraph,
                            ..
                        } => {
                            // lazy continution line
                            prev.push_raw_text("\n");
                            prev.push_raw_text(token.as_str());
                            is_updated = true;
                        }
                        _ => (),
                    },
                }

                if !is_updated {
                    root_block.add(BlockType::Paragraph, token.as_str().to_string());
                }
            }
            Rule::atx_heading1 => {
                let text = token.into_inner().next().unwrap().as_str().to_string();
                root_block.add(BlockType::AtxHeading1, text);
            }
            Rule::atx_heading2 => {
                let text = token.into_inner().next().unwrap().as_str().to_string();
                root_block.add(BlockType::AtxHeading2, text);
            }
            Rule::atx_heading3 => {
                let text = token.into_inner().next().unwrap().as_str().to_string();
                root_block.add(BlockType::AtxHeading3, text);
            }
            Rule::atx_heading4 => {
                let text = token.into_inner().next().unwrap().as_str().to_string();
                root_block.add(BlockType::AtxHeading4, text);
            }
            Rule::atx_heading5 => {
                let text = token.into_inner().next().unwrap().as_str().to_string();
                root_block.add(BlockType::AtxHeading5, text);
            }
            Rule::atx_heading6 => {
                let text = token.into_inner().next().unwrap().as_str().to_string();
                root_block.add(BlockType::AtxHeading6, text);
            }
            Rule::setext_heading_underline1 => {
                let mut is_updated = false;
                match root_block.get_mut_prev() {
                    None => (),
                    Some(prev) => match prev {
                        Block {
                            block_type: BlockType::Paragraph,
                            ..
                        } => {
                            prev.change_block_type(BlockType::SetextHeadingUnderline1);
                            is_updated = true;
                        }
                        _ => (),
                    },
                }
                if !is_updated {
                    root_block.add(BlockType::Paragraph, token.as_str().to_string());
                }
            }
            Rule::setext_heading_underline2 => {
                let mut is_thematic_breaks = false;
                match root_block.get_mut_prev() {
                    None => is_thematic_breaks = true,
                    Some(prev) => match prev {
                        Block {
                            block_type: BlockType::Paragraph,
                            ..
                        } => {
                            prev.change_block_type(BlockType::SetextHeadingUnderline2);
                        }
                        _ => is_thematic_breaks = true,
                    },
                }
                if is_thematic_breaks {
                    let text = token.as_str().to_string();
                    if text == "--".to_string() {
                        root_block.add(BlockType::Paragraph, text);
                    } else {
                        root_block.add(BlockType::ThematicBreaks, "".to_string());
                    }
                }
            }
            Rule::indented_code_block => {
                let mut is_updated = false;
                let text = token.into_inner().next().unwrap().as_str();

                match root_block.get_mut_prev() {
                    None => (),
                    Some(prev) => match prev {
                        Block {
                            block_type: BlockType::Paragraph,
                            ..
                        } => {
                            // lazy continution line
                            prev.push_raw_text("\n");
                            prev.push_raw_text(&skip_space(text.to_string()));
                            is_updated = true;
                        }
                        Block {
                            block_type: BlockType::IndentedCodeBlock,
                            ..
                        } => {
                            // lazy continution line
                            prev.push_raw_text("\n");
                            prev.push_raw_text(text);
                            is_updated = true;
                        }
                        _ => (),
                    },
                }

                if !is_updated {
                    root_block.add(BlockType::IndentedCodeBlock, text.to_string());
                }
            }
            Rule::block_quote => {
                let text = token.into_inner().next().unwrap().as_str().to_string();
                let mut block_quote_block = Block {
                    is_closed: false,
                    block_type: BlockType::BlockQuote,
                    raw_text: "".to_string(),
                    children: vec![],
                };
                block_quote_block.add(BlockType::Paragraph, text);

                root_block.add_block(block_quote_block);
            }
            Rule::list_item => {
                let text = token.into_inner().next().unwrap().as_str().to_string();
                let mut block_quote_block = Block {
                    is_closed: false,
                    block_type: BlockType::ListItem,
                    raw_text: "".to_string(),
                    children: vec![],
                };
                block_quote_block.add(BlockType::Paragraph, text);

                root_block.add_block(block_quote_block);
            }
            _ => (),
        }
    }

    root_block
}

fn skip_space(mut s: String) -> String {
    let mut i = 0;
    for ss in s.chars() {
        if ss != '\u{0020}' {
            break;
        }
        i = i + 1;
    }
    s.split_off(i)
}

#[test]
fn test_skip_space() {
    let s = String::from("");
    assert_eq!("", skip_space(s));

    let s = String::from(" ");
    assert_eq!("", skip_space(s));

    let s = String::from("  ");
    assert_eq!("", skip_space(s));

    let s = String::from("  aaa");
    assert_eq!("aaa", skip_space(s));

    let s = String::from("  aaa   ");
    assert_eq!("aaa   ", skip_space(s));
}

// TODO  constitute Pairs struct.
#[cfg(test)]
mod tests {
    use super::to_tree;
    use block_parser::parse;

    #[test]
    fn test_token() {
        println!("{:?}", to_tree(parse(&String::from("> aaa\n"))));
    }

}
