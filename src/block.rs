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
    BlockQuote,
    Paragraph,
    ListItem,
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
