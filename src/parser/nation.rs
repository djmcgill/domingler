use nom::error::ParseError;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{space0, anychar, digit1, line_ending, not_line_ending};
use nom::branch::alt;
use nom::combinator::{opt, map};
use either::{Left, Right, Either};
use nom::sequence::delimited;
use std::str::FromStr;
use nom::IResult;
use nom::sequence::tuple;

use crate::parser::{parse_id};

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum NationDeclaration {
    SelectId(u32), // select number only
    NewNation, // no new with number allowed
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum NationLine<'a> {
    Declaration,
    End,
    Unparsed(&'a str),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Nation<'a> {
    pub declaration: NationDeclaration,
    pub lines: Vec<NationLine<'a>>,
}

fn parse_select_nation<'a, E: ParseError<&'a str>>(
    input: &'a str
) -> IResult<&'a str, u32, E> {
    let (input, _) = tag("#selectnation")(input)?;
    let (input, _) = space0(input)?;
    let (input, id) = parse_id(input)?;
    let (input, _) = opt(not_line_ending)(input)?;
    let (input, _) = opt(line_ending)(input)?;

    Ok((input, id))
}

fn parse_new_nation<'a, E: ParseError<&'a str>>(
    input: &'a str
) -> IResult<&'a str, (), E> {
    let (input, _) = tag("#newnation")(input)?;
    let (input, _) = space0(input)?;

    let (input, _) = opt(not_line_ending)(input)?;
    let (input, _) = opt(line_ending)(input)?;

    Ok((input, ()))
}

fn parse_nation_declaration<'a, E: ParseError<&'a str>>(
    input: &'a str
) -> IResult<&'a str, NationDeclaration, E> {
    let (input, weapon_declaration) = alt((
        map(parse_new_nation, |()| NationDeclaration::NewNation),
        map(parse_select_nation, |id| NationDeclaration::SelectId(id)),
    ))(input)?;

    Ok((input, weapon_declaration))
}

pub fn parse_nation<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Nation<'a>, E> {
    let mut lines = vec![];
    let (input, declaration) = parse_nation_declaration(input)?;
    lines.push(NationLine::Declaration);

    let (input, raw_lines) = take_until("#end")(input)?;
    for line in raw_lines.lines() {
        if !line.is_empty() {
            lines.push(NationLine::Unparsed(line));

        }
    }

    let (input, _) = tag("#end")(input)?;
    lines.push(NationLine::End);

    let (input, _) = opt(not_line_ending)(input)?;
    let (input, _) = opt(line_ending)(input)?;


    Ok((input, Nation {
        declaration,
        lines,
    }))
}