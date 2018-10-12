use super::parser::Rule;
use block::Block;
use block::BlockType;
use pest::iterators::Pair;
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

fn add_thematic_break(block: &mut Block) {
    block.add(BlockType::ThematicBreaks, "".to_string());
}

fn add_break_line(block: &mut Block) {
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

fn add_paragraph(token: Pair<Rule>, block: &mut Block) {
    let mut is_updated = false;
    let token_str = token.as_str().to_string();

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

fn add_atx_heading1(token: Pair<Rule>, block: &mut Block) {
    let text = token.into_inner().next().unwrap().as_str().to_string();
    block.add(BlockType::AtxHeading1, text);
}

fn add_atx_heading2(token: Pair<Rule>, block: &mut Block) {
    let text = token.into_inner().next().unwrap().as_str().to_string();
    block.add(BlockType::AtxHeading2, text);
}

fn add_atx_heading3(token: Pair<Rule>, block: &mut Block) {
    let text = token.into_inner().next().unwrap().as_str().to_string();
    block.add(BlockType::AtxHeading3, text);
}

fn add_atx_heading4(token: Pair<Rule>, block: &mut Block) {
    let text = token.into_inner().next().unwrap().as_str().to_string();
    block.add(BlockType::AtxHeading4, text);
}

fn add_atx_heading5(token: Pair<Rule>, block: &mut Block) {
    let text = token.into_inner().next().unwrap().as_str().to_string();
    block.add(BlockType::AtxHeading5, text);
}

fn add_atx_heading6(token: Pair<Rule>, block: &mut Block) {
    let text = token.into_inner().next().unwrap().as_str().to_string();
    block.add(BlockType::AtxHeading6, text);
}

fn add_setext_heading_underline1(token: Pair<Rule>, block: &mut Block) {
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

fn add_setext_heading_underline2(token: Pair<Rule>, block: &mut Block) {
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
        let token_str = token.as_str().to_string();
        if token_str == "--".to_string() {
            block.add(BlockType::Paragraph, token_str);
        } else {
            block.add(BlockType::ThematicBreaks, "".to_string());
        }
    }
}

fn add_indented_code_block(token: Pair<Rule>, block: &mut Block) {
    let mut is_updated = false;
    let text = token.into_inner().next().unwrap().as_str().to_string();

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

fn add_fenced_code_block(token: Pair<Rule>, block: &mut Block) {
    let text = token.into_inner().next().unwrap().as_str().to_string();
    block.add(BlockType::FencedCodeBlock, text);
}

fn add_block_quote(token: Pair<Rule>, block: &mut Block) {
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

fn add_bullet_list_items(token: Pair<Rule>, block: &mut Block) {
    let mut inner_token = token.into_inner();
    let text = inner_token.next().unwrap().as_str().to_string();
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

fn add_ordered_list_items(token: Pair<Rule>, block: &mut Block) {
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

fn add_reference_link(token: Pair<Rule>, block: &mut Block) {
    block.add(
        BlockType::ReferenceLink,
        token.into_inner().next().unwrap().as_str().to_string(),
    );
}

fn add_link_definition(token: Pair<Rule>, block: &mut Block) {
    let mut inner_token = token.into_inner();

    let link_label = inner_token.next().unwrap().as_str();
    let link_destination = inner_token.next().unwrap().as_str();

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

fn to_inner_tree(tokens: Pairs<Rule>, block: &mut Block) {
    for token in tokens {
        match token.as_rule() {
            Rule::thematic_break => add_thematic_break(block),
            Rule::break_line => add_break_line(block),
            Rule::paragraph => add_paragraph(token, block),
            Rule::atx_heading1 => add_atx_heading1(token, block),
            Rule::atx_heading2 => add_atx_heading2(token, block),
            Rule::atx_heading3 => add_atx_heading3(token, block),
            Rule::atx_heading4 => add_atx_heading4(token, block),
            Rule::atx_heading5 => add_atx_heading5(token, block),
            Rule::atx_heading6 => add_atx_heading6(token, block),
            Rule::setext_heading_underline1 => add_setext_heading_underline1(token, block),
            Rule::setext_heading_underline2 => add_setext_heading_underline2(token, block),
            Rule::indented_code_block => add_indented_code_block(token, block),
            Rule::fenced_code_block => add_fenced_code_block(token, block),
            Rule::block_quote => add_block_quote(token, block),
            Rule::bullet_list_items => add_bullet_list_items(token, block),
            Rule::ordered_list_items => add_ordered_list_items(token, block),
            Rule::reference_link => add_reference_link(token, block),
            Rule::link_definition => add_link_definition(token, block),
            _ => (),
        }
    }
}
