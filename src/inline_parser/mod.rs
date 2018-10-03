use block::Block;
use block::BlockType;

mod interpreter;
mod lexer;

pub fn top(block_tree: &mut Block) {
    match block_tree {
        Block {
            block_type: BlockType::Document,
            children,
            ..
        } => {
            for child in children {
                top(child);
            }
        }
        Block {
            block_type: BlockType::Paragraph,
            raw_text,
            ..
        }
        | Block {
            block_type: BlockType::AtxHeading1,
            raw_text,
            ..
        }
        | Block {
            block_type: BlockType::AtxHeading2,
            raw_text,
            ..
        }
        | Block {
            block_type: BlockType::AtxHeading3,
            raw_text,
            ..
        }
        | Block {
            block_type: BlockType::AtxHeading4,
            raw_text,
            ..
        }
        | Block {
            block_type: BlockType::AtxHeading5,
            raw_text,
            ..
        }
        | Block {
            block_type: BlockType::AtxHeading6,
            raw_text,
            ..
        }
        | Block {
            block_type: BlockType::SetextHeadingUnderline1,
            raw_text,
            ..
        }
        | Block {
            block_type: BlockType::SetextHeadingUnderline2,
            raw_text,
            ..
        } => to_html(raw_text),
        Block { .. } => {}
    }
}

fn to_html(raw_text: &mut String) {
    let cloned = raw_text.to_string();
    let html = match lexer::lex(&cloned) {
        Ok(tokens) => interpreter::top(tokens),
        _ => cloned.to_string(),
    };
    raw_text.clear();
    raw_text.push_str(&html.trim_right()); //TODO
}
