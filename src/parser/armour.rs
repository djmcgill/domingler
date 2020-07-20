use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{digit1, space0};
use nom::combinator::{map, opt};
use nom::error::ParseError;
use nom::IResult;
use std::str::FromStr;

use crate::parser::{parse_comment_line_end, parse_id, parse_name};

#[cfg(test)]
mod tests;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ArmourDeclaration<'a> {
    SelectId(u32),
    SelectName(&'a str),
    NewId(u32),
    NewImplicit,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ArmourLine<'a> {
    Unparsed(&'a str),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Armour<'a> {
    pub declaration: ArmourDeclaration<'a>,
    /// This field does not contain the declaration or the end
    pub inner_lines: Vec<ArmourLine<'a>>,
}

fn parse_select_armour<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, ArmourDeclaration<'a>, E> {
    let (input, _) = tag("#selectarmor")(input)?;
    let (input, _) = space0(input)?;
    let (input, either_id_name) = alt((
        map(parse_id, |id| ArmourDeclaration::SelectId(id)),
        map(parse_name, |name| ArmourDeclaration::SelectName(name)),
    ))(input)?;

    Ok((input, either_id_name))
}

fn parse_new_armour<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, ArmourDeclaration<'a>, E> {
    let (input, _) = tag("#newarmor")(input)?;
    let (input, _) = space0(input)?;
    let (input, opt_armour_id_str) = opt(digit1)(input)?;
    let opt_armour_id = opt_armour_id_str.map(|armour_id_str| {
        u32::from_str(armour_id_str)
            .unwrap_or_else(|_| panic!("could not parse valid integer id from '{}'", armour_id_str))
    }); // FIXME

    let declaration = match opt_armour_id {
        Some(armour_id) => ArmourDeclaration::NewId(armour_id),
        None => ArmourDeclaration::NewImplicit,
    };

    Ok((input, declaration))
}

fn parse_armour_declaration<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, ArmourDeclaration<'a>, E> {
    let (input, _) = space0(input)?;
    let (input, armour_declaration) = alt((parse_new_armour, parse_select_armour))(input)?;
    let (input, _) = parse_comment_line_end(input)?;

    Ok((input, armour_declaration))
}

pub fn parse_armour<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Armour<'a>, E> {
    let mut inner_lines = vec![];
    let (input, declaration) = parse_armour_declaration(input)?;

    let (input, raw_lines) = take_until("#end")(input)?;
    for line in raw_lines.lines() {
        if !line.is_empty() {
            inner_lines.push(ArmourLine::Unparsed(line));
        }
    }

    let (input, _) = tag("#end")(input)?;

    let (input, _) = parse_comment_line_end(input)?;

    Ok((
        input,
        Armour {
            declaration,
            inner_lines,
        },
    ))
}
