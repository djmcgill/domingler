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

use crate::parser::{parse_id, parse_comment_line_end};

#[cfg(test)]
mod tests;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum NationDeclaration {
    SelectId(u32), // select number only
    NewImplicit, // no new with number allowed
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum NationLine<'a> {
    Declaration,
    End,
    Unparsed(&'a str),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Nation<'a> {
    pub declaration: NationDeclaration,
    pub lines: Vec<NationLine<'a>>,
}

fn parse_select_nation<'a, E: ParseError<&'a str>>(
    input: &'a str
) -> IResult<&'a str, NationDeclaration, E> {
    let (input, _) = tag("#selectnation")(input)?;
    let (input, _) = space0(input)?;
    let (input, id) = parse_id(input)?;

    Ok((input, NationDeclaration::SelectId(id)))
}

fn parse_new_nation<'a, E: ParseError<&'a str>>(
    input: &'a str
) -> IResult<&'a str, NationDeclaration, E> {
    let (input, _) = tag("#newnation")(input)?;
    Ok((input, NationDeclaration::NewImplicit))
}

fn parse_nation_declaration<'a, E: ParseError<&'a str>>(
    input: &'a str
) -> IResult<&'a str, NationDeclaration, E> {
    let (input, _) = space0(input)?;

    let (input, weapon_declaration) = alt((
        parse_new_nation,
        parse_select_nation,
    ))(input)?;
    let (input, _) = parse_comment_line_end(input)?;

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

    let (input, _) = parse_comment_line_end(input)?;


    Ok((input, Nation {
        declaration,
        lines,
    }))
}