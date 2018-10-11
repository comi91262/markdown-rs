use block::Block;

mod parser;
mod tree;

pub fn top(lines: &str) -> Block {
    let pest_tree = parser::parse(lines);
    tree::to_tree(pest_tree)
}
