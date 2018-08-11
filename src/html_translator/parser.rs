use pest::Parser;
use pest::iterators::Pairs;

#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("../markdown.pest");

#[derive(Parser)]
#[grammar = "markdown.pest"]
struct MarkdownParser;

pub fn parse(line: &str) -> Pairs<Rule> {
    MarkdownParser::parse(Rule::exp, line)
        .unwrap_or_else(|e| panic!("{}", e))

}

#[test]
fn test() {
    let mut pairs = parse("# 1 1");
    let rules = pairs.next().unwrap(); //Rules
    let s = pairs.next().unwrap();

    assert_eq!(rules.as_str(), "empty");



}

