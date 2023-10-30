use html_parser::Tag;
use winnow::Parser;

fn main() {
    let input = r#"<div width="40", height="30", height="30", height="30", height="30", height="30", height="30", height="30", height="30", height="30", height="30", height="30", height="30", height="30", height="30", height="30", height="30", height="30", height="30", height="30", height="30", height="30", height="30", height="30", height="30", height="30", height="30", height="30", height="30", height="30", height="30", height="30", height="30", height="30", height="30", height="30", height="30", height="30", height="30", height="30", height="30", height="30", height="30", height="30", height="30", height="30">"#;
    eprintln!("{:?}", Tag::parse.parse(&input).unwrap());
}
