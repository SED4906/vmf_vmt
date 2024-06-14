use nom::{
    branch::alt, bytes::complete::is_not, character::complete::{multispace0, none_of, one_of}, combinator::recognize, error::ParseError, multi::{many0, many1}, sequence::{delimited, separated_pair}, IResult
};

#[derive(PartialEq,Debug)]
pub enum Valve {
    Class {
        class_name: String,
        data: Vec<Valve>,
    },
    PropertyValue(String, String),
}

pub type VmfVmt = Vec<Valve>;

pub fn build_vmf_vmt(data: VmfVmt) -> String {
    data.into_iter().map(|class| build_valve(0,class)).collect::<Vec<String>>().concat()
}

pub fn build_valve(indent: usize, data: Valve) -> String {
    match data {
        Valve::Class { class_name, data } => {
            let mut out = String::new();
            for _ in 0..indent {out.push('\t')};
            out.push_str(&class_name);
            out.push_str("\n");
            for _ in 0..indent {out.push('\t')};
            out.push_str("{\n");
            for subvalue in data {
                out.push_str(&build_valve(indent+1,subvalue));
            }
            for _ in 0..indent {out.push('\t')};
            out.push_str("}\n");
            out
        },
        Valve::PropertyValue(property, value) => {
            let mut out = String::new();
            for _ in 0..indent {out.push('\t')};
            out.push_str("\"");
            out.push_str(&property);
            out.push_str("\" \"");
            out.push_str(&value);
            out.push_str("\"\n");
            out
        },
    }
}

pub fn parse_vmf_vmt(input: &str) -> IResult<&str,VmfVmt> {
    many1(ws(parse_class))(input)
}

fn parse_valve(input: &str) -> IResult<&str, Valve> {
    alt((parse_class, parse_property_value))(input)
}

fn parse_class(input: &str) -> IResult<&str, Valve> {
    let (input, class_name) = parse_unquoted(input)?;
    let (input, data) = delimited(
        ws(one_of("{")),
        many0(ws(parse_valve)),
        ws(one_of("}")),
    )(input)?;
    Ok((input, Valve::Class { class_name, data }))
}

fn parse_property_value(input: &str) -> IResult<&str, Valve> {
    let (input, (property, value)) =
        separated_pair(parse_string, multispace0, parse_string)(input)?;
    Ok((input, Valve::PropertyValue(property, value)))
}

fn parse_string(input: &str) -> IResult<&str, String> {
    alt((parse_unquoted, parse_quoted))(input)
}

fn parse_unquoted(input: &str) -> IResult<&str, String> {
    let (input, string) = is_not(" \t\r\n\"{{}}")(input)?;
    Ok((input, string.to_string()))
}

fn parse_quoted(input: &str) -> IResult<&str, String> {
    let (input, string) = delimited(
        recognize(one_of("\"")),
        recognize(many0(none_of("\""))),
        recognize(one_of("\"")),
    )(input)?;
    Ok((input, string.to_string()))
}

fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}