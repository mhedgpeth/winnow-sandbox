use winnow::{
    ascii::multispace0,
    combinator::{delimited, separated_pair},
    error::ParserError,
    token::take_while,
    LocatingSlice, Parser, Result,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Property {
    key: String,
    pub value: String,
    span: std::ops::Range<usize>,
}

fn parse_identifier(input: &mut LocatingSlice<&str>) -> Result<String> {
    let identifier =
        take_while(0.., |c: char| c.is_alphanumeric() || c == '-' || c == '_').parse_next(input)?;
    Ok(identifier.to_owned())
}

fn parse_string_value(input: &mut LocatingSlice<&str>) -> Result<String> {
    let result = delimited('"', take_while(0.., |c: char| c != '"'), '"').parse_next(input)?;
    Ok(result.to_owned())
}

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
fn ws<'a, F, O, E: ParserError<&'a str>>(inner: F) -> impl Parser<&'a str, O, E>
where
    F: Parser<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

pub fn parse_property(input: &mut LocatingSlice<&str>) -> Result<Property> {
    let ((key, value), span) = separated_pair(parse_identifier, ws(':'), parse_string_value)
        .with_span()
        .parse_next(input)?;
    Ok(Property { key, value, span })
}

#[cfg(test)]
mod tests {
    mod parse_identifier_fn {
        use rstest::rstest;

        use crate::parse_identifier;
        use winnow::Parser;

        #[rstest]
        #[case("valid")]
        #[case("with-dashes")]
        #[case("with_underscores")]
        #[case("with-numbers-23")]
        #[case("ValidIdentifier")]
        fn should_parse_valid_identifier(#[case] identifier: &str) {
            let result = parse_identifier.parse(identifier);

            assert_eq!(result.unwrap(), identifier);
        }

        #[rstest]
        #[case("?invalid")]
        #[case("not-including-colon:")]
        fn should_fail_to_parse_invalid_identifier(#[case] input: &str) {
            let result = parse_identifier.parse(input);

            assert!(result.is_err());
        }
    }

    mod string_value_parser_fn {
        use rstest::rstest;

        use crate::parse_string_value;
        use winnow::Parser;

        #[rstest]
        #[case("\"Michael\"", "Michael")]
        #[case("\"$\"", "$")]
        #[case("\"1234\"", "1234")]
        fn should_parse_valid_string(#[case] input: &str, #[case] expected: &str) {
            let result = parse_string_value.parse(input);

            assert_eq!(result.unwrap(), expected.to_string())
        }

        #[rstest]
        #[case("No begging quote\"")]
        #[case("\"No ending quote")]
        fn should_not_parse_invalid_string(#[case] input: &str) {
            let result = parse_string_value.parse(input);

            assert!(result.is_err());
        }
    }

    mod parse_property_fn {
        use rstest::rstest;

        use crate::parse_property;
        use crate::Property;
        use winnow::Parser;

        #[rstest]
        #[case(
            "first_name: \"Michael\"",
            Property { key: String::from("first_name"), value: String::from("Michael") }
        )]
        #[case(
            "name:\t\"Jack\"",
            Property { key: String::from("name"), value: String::from("Jack") }
        )]
        fn should_parse_valid_property(#[case] input: &str, #[case] expected: Property) {
            let result = parse_property.parse(input);

            assert_eq!(result.unwrap(), expected)
        }
    }
}
