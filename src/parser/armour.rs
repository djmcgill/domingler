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

use crate::parser::{parse_name_either, parse_id_either, parse_comment_line_end};

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ArmourDeclaration<'a> {
    SelectId(u32),
    SelectName(&'a str),
    NewArmour(Option<u32>),
}
impl<'a> ArmourDeclaration<'a> {
    pub fn select_from_either(either_id_name: Either<u32, &'a str>) -> ArmourDeclaration<'a> {
        match either_id_name {
            Either::Left(id) => ArmourDeclaration::SelectId(id),
            Either::Right(name) => ArmourDeclaration::SelectName(name),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ArmourLine<'a> {
    Declaration,
    End,
    Unparsed(&'a str),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Armour<'a> {
    pub declaration: ArmourDeclaration<'a>,
    pub lines: Vec<ArmourLine<'a>>,
}

fn parse_select_armour<'a, E: ParseError<&'a str>>(
    input: &'a str
) -> IResult<&'a str, Either<u32, &'a str>, E> {
    let (input, _) = tag("#selectarmor")(input)?;
    let (input, _) = space0(input)?;
    let (input, either_id_name) = alt((
        parse_id_either,
        parse_name_either,
    ))(input)?;
    let (input, _) = opt(not_line_ending)(input)?;
    let (input, _) = opt(line_ending)(input)?;

    Ok((input, either_id_name))
}

fn parse_new_armour<'a, E: ParseError<&'a str>>(
    input: &'a str
) -> IResult<&'a str, Option<u32>, E> {
    let (input, _) = tag("#newarmor")(input)?;
    let (input, _) = space0(input)?;
    let (input, opt_armour_id_str) = opt(digit1)(input)?;
    let opt_armour_id = opt_armour_id_str
        .map(|armour_id_str| u32::from_str(armour_id_str).unwrap_or_else(|_| panic!("could not parse valid integer id from '{}'", armour_id_str))); // FIXME

    let (input, _) = opt(not_line_ending)(input)?;
    let (input, _) = opt(line_ending)(input)?;

    Ok((input, opt_armour_id))
}

fn parse_armour_declaration<'a, E: ParseError<&'a str>>(
    input: &'a str
) -> IResult<&'a str, ArmourDeclaration<'a>, E> {
    let (input, armour_declaration) = alt((
        map(parse_new_armour, |opt_id| ArmourDeclaration::NewArmour(opt_id)),
        map(parse_select_armour, |either_id_name| ArmourDeclaration::select_from_either(either_id_name))
    ))(input)?;

    Ok((input, armour_declaration))
}

pub fn parse_armour<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Armour<'a>, E> {
    let mut lines = vec![];
    let (input, declaration) = parse_armour_declaration(input)?;
    lines.push(ArmourLine::Declaration);

    let (input, raw_lines) = take_until("#end")(input)?;
    for line in raw_lines.lines() {
        if !line.is_empty() {
            lines.push(ArmourLine::Unparsed(line));
        }
    }

    let (input, _) = tag("#end")(input)?;
    lines.push(ArmourLine::End);

    let (input, _) = opt(not_line_ending)(input)?;
    let (input, _) = opt(line_ending)(input)?;


    Ok((input, Armour {
        declaration,
        lines,
    }))
}
