use std::{collections::HashMap, hash::BuildHasher};

use winnow::{
    ascii::{alpha1, multispace0},
    combinator::{delimited, separated0, separated_pair},
    token::take_while,
    PResult, Parser,
};

/// Parse the key of a HTML attribute
fn parse_key<'i>(input: &mut &'i str) -> PResult<&'i str> {
    alpha1.parse_next(input)
}

/// Parse the value of an HTML attribute
fn parse_val<'i>(input: &mut &'i str) -> PResult<&'i str> {
    let inner = take_while(1.., |c: char| {
        c.is_alphanumeric() || c == '.' || c == '/' || c == ':'
    });
    delimited('"', inner, '"').parse_next(input)
}

/// Parses an HTML attribute.
/// Looks something like `key="val"`.
fn parse_attribute<'i>(input: &mut &'i str) -> PResult<(&'i str, &'i str)> {
    separated_pair(
        parse_key,
        delimited(multispace0, '=', multispace0),
        parse_val,
    )
    .parse_next(input)
}

/// HTML attributes
#[derive(Debug)]
pub struct Attributes<'i, S> {
    kvs: HashMap<&'i str, &'i str, S>,
}

impl<'i, S> PartialEq for Attributes<'i, S>
where
    S: BuildHasher,
{
    fn eq(&self, other: &Self) -> bool {
        self.kvs == other.kvs
    }
}

impl<'i, S> Attributes<'i, S>
where
    S: BuildHasher + Default,
{
    fn parse(input: &mut &'i str) -> PResult<Self> {
        let kvs = separated0(parse_attribute, (',', multispace0)).parse_next(input)?;
        Ok(Self { kvs })
    }
}

/// An HTML open tag, like `<a href="google.com">`.
#[derive(Debug)]
pub struct Tag<'i, S> {
    /// Like 'div'
    tag_type: &'i str,
    attributes: Attributes<'i, S>,
}

impl<'i, S> PartialEq for Tag<'i, S>
where
    S: BuildHasher,
{
    fn eq(&self, other: &Self) -> bool {
        self.tag_type == other.tag_type && self.attributes == other.attributes
    }
}

impl<'i, S> Tag<'i, S>
where
    S: BuildHasher + Default,
{
    pub fn parse(input: &mut &'i str) -> PResult<Self> {
        let parse_parts = (alpha1, ' ', Attributes::parse);
        let parse_tag = parse_parts.map(|(tag_type, _space_char, attributes)| Self {
            tag_type,
            attributes,
        });
        let tag = delimited('<', parse_tag, '>').parse_next(input)?;
        Ok(tag)
    }
}

#[cfg(test)]
mod tests {

    use std::collections::hash_map::RandomState;

    use super::*;

    #[test]
    fn test_key() {
        let input = "width";
        let actual = parse_key.parse(input).unwrap();
        let expected = "width";
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_val() {
        let input = r#""40""#;
        let actual = parse_val.parse(input).unwrap();
        let expected = "40";
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_attributes() {
        let input = r#"width="40", height = "30""#;
        let actual = Attributes::<RandomState>::parse.parse(input).unwrap();
        let expected = Attributes {
            kvs: [("width", "40"), ("height", "30")].into_iter().collect(),
        };
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_link_tag() {
        let input = r#"<a href="https://adamchalmers.com">"#;
        let expected = Tag {
            tag_type: "a",
            attributes: Attributes {
                kvs: [("href", "https://adamchalmers.com")].into_iter().collect(),
            },
        };
        let actual = Tag::<RandomState>::parse.parse(&input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_tag() {
        let input = r#"<div width="40", height="30">"#;
        let expected = Tag::<RandomState> {
            tag_type: "div",
            attributes: Attributes {
                kvs: [("width", "40"), ("height", "30")].into_iter().collect(),
            },
        };
        let actual = Tag::parse.parse(&input).unwrap();
        assert_eq!(expected, actual);
    }
}
