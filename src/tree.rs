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

    to_inner_tree(tokens, &mut root_block);

    root_block
}

fn to_inner_tree(tokens: Pairs<Rule>, block: &mut Block) {
    for token in tokens {
        match token.as_rule() {
            Rule::thematic_break => {
                block.add(BlockType::ThematicBreaks, "".to_string());
            }
            Rule::break_line => {
                if let Some(prev) = block.get_mut_last_open_block() {
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
                block.add(BlockType::BreakLine, "".to_string());
            }
            Rule::paragraph => {
                let mut is_updated = false;
                let mut token_str = token.as_str().to_string();

                match block.get_mut_last_open_block() {
                    None => (),
                    Some(prev) => match prev {
                        Block {
                            block_type: BlockType::Paragraph,
                            ..
                        } => {
                            // lazy continution line
                            prev.push_raw_text("\n");
                            prev.push_raw_text(&token_str);
                            is_updated = true;
                        }
                        _ => (),
                    },
                }

                if !is_updated {
                    block.add(BlockType::Paragraph, token_str);
                }
            }
            Rule::atx_heading1 => {
                let mut text = token.into_inner().next().unwrap().as_str().to_string();
                block.add(BlockType::AtxHeading1, text);
            }
            Rule::atx_heading2 => {
                let mut text = token.into_inner().next().unwrap().as_str().to_string();
                block.add(BlockType::AtxHeading2, text);
            }
            Rule::atx_heading3 => {
                let mut text = token.into_inner().next().unwrap().as_str().to_string();
                block.add(BlockType::AtxHeading3, text);
            }
            Rule::atx_heading4 => {
                let mut text = token.into_inner().next().unwrap().as_str().to_string();
                block.add(BlockType::AtxHeading4, text);
            }
            Rule::atx_heading5 => {
                let mut text = token.into_inner().next().unwrap().as_str().to_string();
                block.add(BlockType::AtxHeading5, text);
            }
            Rule::atx_heading6 => {
                let mut text = token.into_inner().next().unwrap().as_str().to_string();
                block.add(BlockType::AtxHeading6, text);
            }
            Rule::setext_heading_underline1 => {
                let mut is_updated = false;
                match block.get_mut_prev() {
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
                    block.add(BlockType::Paragraph, token.as_str().to_string());
                }
            }
            Rule::setext_heading_underline2 => {
                let mut is_thematic_breaks = false;
                match block.get_mut_prev() {
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
                    let mut token_str = token.as_str().to_string();
                    if token_str == "--".to_string() {
                        block.add(BlockType::Paragraph, token_str);
                    } else {
                        block.add(BlockType::ThematicBreaks, "".to_string());
                    }
                }
            }
            Rule::indented_code_block => {
                let mut is_updated = false;
                let mut text = token.into_inner().next().unwrap().as_str().to_string();

                match block.get_mut_prev() {
                    None => (),
                    Some(prev) => match prev {
                        Block {
                            block_type: BlockType::Paragraph,
                            ..
                        } => {
                            // lazy continution line
                            prev.push_raw_text("\n");
                            prev.push_raw_text(text.trim_left());
                            is_updated = true;
                        }
                        Block {
                            block_type: BlockType::IndentedCodeBlock,
                            ..
                        } => {
                            // lazy continution line
                            prev.push_raw_text("\n");
                            prev.push_raw_text(&text);
                            is_updated = true;
                        }
                        _ => (),
                    },
                }

                if !is_updated {
                    block.add(BlockType::IndentedCodeBlock, text);
                }
            }
            Rule::fenced_code_block => {
                let mut text = token.into_inner().next().unwrap().as_str().to_string();
                block.add(BlockType::FencedCodeBlock, text);
            }
            Rule::block_quote => {
                let inner_token = token.into_inner();
                let mut is_updated = false;

                let mut block_quote_block = Block {
                    is_closed: false,
                    block_type: BlockType::BlockQuote,
                    raw_text: "".to_string(),
                    children: vec![],
                };

                match block.get_mut_prev() {
                    Some(mut block1) => match block1 {
                        Block {
                            block_type: BlockType::BlockQuote,
                            ..
                        } => {
                            to_inner_tree(inner_token, &mut block1);
                        }
                        _ => {
                            is_updated = true;
                            to_inner_tree(inner_token, &mut block_quote_block);
                        }
                    },
                    None => {
                        is_updated = true;
                        to_inner_tree(inner_token, &mut block_quote_block);
                    }
                }

                if is_updated {
                    block.add_block(block_quote_block);
                }
            }
            Rule::bullet_list_items => {
                let mut inner_token = token.into_inner();
                let mut text = inner_token.next().unwrap().as_str().to_string();
                let mut is_updated = false;

                let mut new_block = Block {
                    is_closed: false,
                    block_type: BlockType::BulletListItem,
                    raw_text: "".to_string(),
                    children: vec![Block {
                        is_closed: false,
                        block_type: BlockType::Paragraph,
                        raw_text: text,
                        children: vec![],
                    }],
                };

                match block.get_mut_prev() {
                    Some(mut block1) => match block1 {
                        Block {
                            block_type: BlockType::BulletListItem,
                            ..
                        } => {
                            to_inner_tree(inner_token, &mut block1);
                        }
                        _ => {
                            is_updated = true;
                            to_inner_tree(inner_token, &mut new_block);
                        }
                    },
                    None => {
                        is_updated = true;
                        to_inner_tree(inner_token, &mut new_block);
                    }
                }

                if is_updated {
                    block.add_block(new_block);
                }
            }
            Rule::ordered_list_items => {
                let inner_token = token.into_inner();
                let mut is_updated = false;

                let mut new_block = Block {
                    is_closed: false,
                    block_type: BlockType::OrderedListItem,
                    raw_text: "".to_string(),
                    children: vec![],
                };

                match block.get_mut_prev() {
                    Some(mut block1) => match block1 {
                        Block {
                            block_type: BlockType::OrderedListItem,
                            ..
                        } => {
                            to_inner_tree(inner_token, &mut block1);
                        }
                        _ => {
                            is_updated = true;
                            to_inner_tree(inner_token, &mut new_block);
                        }
                    },
                    None => {
                        is_updated = true;
                        to_inner_tree(inner_token, &mut new_block);
                    }
                }

                if is_updated {
                    block.add_block(new_block);
                }
            }
            Rule::reference_link => {
                block.add(
                    BlockType::ReferenceLink,
                    token.into_inner().next().unwrap().as_str().to_string(),
                );
            }
            Rule::link_definition => {
                let mut inner_token = token.into_inner();

                let mut link_label = inner_token.next().unwrap().as_str();
                let mut link_destination = inner_token.next().unwrap().as_str();

                let text = match inner_token.next() {
                    Some(link_title) => format!(
                        "<a href=\"/{}\" title=\"{}\">{}</a>",
                        link_destination,
                        link_title.as_str(),
                        link_label
                    ),
                    None => format!("<a href=\"{}\">{}</a>", link_destination, link_label),
                };

                block.add_block(Block {
                    is_closed: false,
                    block_type: BlockType::LinkDefinition,
                    raw_text: link_label.to_string(),
                    children: vec![Block {
                        is_closed: false,
                        block_type: BlockType::Paragraph,
                        raw_text: text,
                        children: vec![],
                    }],
                });
            }
            _ => (),
        }
    }
}


// TODO  constitute Pairs struct.
#[cfg(test)]
mod tests {
    use super::to_tree;
    use block_parser::parse;

    #[test]
    fn test_token() {
        let st = String::from("- foo\n***\n- bar\n");
        let a = to_tree(parse(&st));
        println!("{:?}", a);
    }

}
