use parser::parse_property;
use winnow::Parser;

fn main() {
    let property = parse_property.parse("name: \"Parsed World\"").unwrap();

    println!("Hello, {}!", property.value);
}
