mod parser;

pub fn exec(input_str: &str) -> String {
    let mut pairs = parser::parse(input_str);

    let pair = pairs.next().unwrap();

    match pair.as_rule() {
        parser::Rule::header => {
            let next = pair.into_inner().next().unwrap();
            return format!("<h1>{}</h1>", next.as_str());
        }
        //parser::Rule::thematic_break => return format!("<hr />"),
        parser::Rule::text => {
            return pair.as_str().to_string();
        }
        _ => unreachable!(),
    }
}


#[test]
fn test() {
    assert_eq!(exec("# 1"), "<h1>1</h1>");

}
