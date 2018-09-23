use std::fmt;

#[derive(Debug, PartialEq)]
pub enum BlockType {
    Document,
    ThematicBreaks,
    BreakLine,
    AtxHeading1,
    AtxHeading2,
    AtxHeading3,
    AtxHeading4,
    AtxHeading5,
    AtxHeading6,
    SetextHeadingUnderline1,
    SetextHeadingUnderline2,
    IndentedCodeBlock,
    FencedCodeBlock,
    BlockQuote,
    Paragraph,
    BulletListItem,
    OrderedListItem,
    //    List,
}

#[derive(Debug, PartialEq)]
pub struct Block {
    pub block_type: BlockType,
    pub children: Vec<Block>,
    pub raw_text: String,
    pub is_closed: bool,
}

impl Block {
    pub fn add(&mut self, block_type: BlockType, text: String) {
        let child = Block {
            is_closed: false,
            block_type: block_type,
            raw_text: text,
            children: vec![],
        };

        self.children.push(child);
    }

    pub fn add_block(&mut self, block: Block) {
        self.children.push(block);
    }

    pub fn get_mut_prev(&mut self) -> Option<&mut Block> {
        self.children.as_mut_slice().last_mut()
    }

    pub fn close(&mut self) {
        self.is_closed = true;
    }

    pub fn push_raw_text(&mut self, s: &str) {
        self.raw_text.push_str(s);
    }

    pub fn change_block_type(&mut self, bt: BlockType) {
        self.block_type = bt;
    }

    pub fn get_mut_last_open_block(&mut self) -> Option<&mut Block> {
        if self.children.is_empty() {
            return Some(self);
        }

        for child in self.children.iter_mut().rev() {
            if !child.is_closed {
                return child.get_mut_last_open_block();
            }
        }

        None
    }

    pub fn get_text(&self) -> &str {
        &self.raw_text
    }
}

//impl Clone for Block {
//    fn clone(&self) -> Block { *self }
//}

#[test]
fn test_get_mut_last_open_block() {
    let mut root_block = Block {
        is_closed: false,
        block_type: BlockType::Document,
        raw_text: "".to_string(),
        children: vec![],
    };

    assert_eq!(
        BlockType::Document,
        root_block.get_mut_last_open_block().unwrap().block_type
    );

    let child1 = Block {
        is_closed: true,
        block_type: BlockType::Paragraph,
        raw_text: "foo".to_string(),
        children: vec![],
    };
    root_block.add_block(child1);

    assert_eq!(None, root_block.get_mut_last_open_block());

    let child2 = Block {
        is_closed: false,
        block_type: BlockType::Paragraph,
        raw_text: "bar".to_string(),
        children: vec![],
    };
    root_block.add_block(child2);

    assert_eq!(
        BlockType::Paragraph,
        root_block.get_mut_last_open_block().unwrap().block_type
    );
    assert_eq!(
        "bar".to_string(),
        root_block.get_mut_last_open_block().unwrap().raw_text
    );
}

#[test]
fn test_get_mut_prev() {
    let mut root_block = Block {
        is_closed: false,
        block_type: BlockType::Document,
        raw_text: "".to_string(),
        children: vec![],
    };

    assert_eq!(None, root_block.get_mut_prev());

    root_block.add(BlockType::Paragraph, "aaa".to_string());

    let mut expected_block = Block {
        is_closed: false,
        block_type: BlockType::Paragraph,
        raw_text: "aaa".to_string(),
        children: vec![],
    };

    assert_eq!(Some(&mut expected_block), root_block.get_mut_prev());
}

#[test]
fn test_close() {
    let mut root_block = Block {
        is_closed: false,
        block_type: BlockType::Document,
        raw_text: "aaa".to_string(),
        children: vec![],
    };

    root_block.close();

    assert_eq!(true, root_block.is_closed);
}

#[test]
fn test_push_raw_text() {
    let mut root_block = Block {
        is_closed: false,
        block_type: BlockType::Document,
        raw_text: "aaa".to_string(),
        children: vec![],
    };

    root_block.push_raw_text("bbb");

    assert_eq!("aaabbb", root_block.raw_text);
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        result.push_str(&format!("Block: (\n  block_type: {:?}\n  raw_text: {}\n  is_closed: {}\n  children: \n", self.block_type, self.raw_text, self.is_closed));

        for v in &self.children {
            result.push_str(&format!("    {}\n", v))
        }
        write!(f, "{})", result)

    }
}
