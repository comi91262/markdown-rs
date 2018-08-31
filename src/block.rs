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
    //    BlockQuote,
    Paragraph,
    //    List,
    //    ListItem,
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

    pub fn get_prev(&mut self) -> Option<&mut Block> {
        self.children.as_mut_slice().last_mut()
    }

    //fn get(&self) -> &Block {
    //    &self
    //}

    //fn update_prev(&mut self, s: &str) {
    //    let mut last = self.children.as_mut_slice().last_mut().unwrap();
    //    last.raw_text.push_str(s);
    //}

    pub fn update(&mut self, s: &str) {
        self.raw_text.push_str(s);
    }

    //#[test]
    //fn test_get_prev() {
    //    let mut root_block = Block {
    //        is_closed: false,
    //        block_type: BlockType::Document,
    //        raw_text: "".to_string(),
    //        children: vec![],
    //    };

    //    assert_eq!(None, root_block.get_prev());

    //    root_block.add(BlockType::Paragraph, "aaa".to_string());

    //    let mut expected_block = Block {
    //        is_closed: false,
    //        block_type: BlockType::Paragraph,
    //        raw_text: "aaa".to_string(),
    //        children: vec![],
    //    };

    //    assert_eq!(Some(&mut expected_block), root_block.get_prev());
    //}
}
