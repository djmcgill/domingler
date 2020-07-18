
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

use crate::parser::{parse_name, parse_id, parse_comment_line_end};

#[cfg(test)]
mod tests;


#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MonsterDeclaration<'a> {
    SelectId(u32),
    SelectName(&'a str),
    NewId(u32),
    NewImplicit,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MonsterLine<'a> {
    Declaration,
    End,
    Unparsed(&'a str),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Monster<'a> {
    pub declaration: MonsterDeclaration<'a>,
    pub lines: Vec<MonsterLine<'a>>,
}


fn parse_select_monster<'a, E: ParseError<&'a str>>(
    input: &'a str
) -> IResult<&'a str, MonsterDeclaration<'a>, E> {
    let (input, _) = tag("#selectmonster")(input)?;
    let (input, _) = space0(input)?;
    let (input, either_id_name) = alt((
        map(parse_id, |id| MonsterDeclaration::SelectId(id)),
        map(parse_name, |name| MonsterDeclaration::SelectName(name)),
    ))(input)?;

    Ok((input, either_id_name))
}

fn parse_new_monster<'a, E: ParseError<&'a str>>(
    input: &'a str
) -> IResult<&'a str, MonsterDeclaration<'a>, E> {
    let (input, _) = tag("#newmonster")(input)?;
    let (input, _) = space0(input)?;
    let (input, opt_id_str) = opt(digit1)(input)?;
    let opt_id = opt_id_str
        .map(|id_str| u32::from_str(id_str).unwrap_or_else(|_| panic!("could not parse valid integer id from '{}'", id_str))); // FIXME

    let declaration = match opt_id {
        Some(id) => MonsterDeclaration::NewId(id),
        None => MonsterDeclaration::NewImplicit,
    };


    Ok((input, declaration))
}

fn parse_monster_declaration<'a, E: ParseError<&'a str>>(
    input: &'a str
) -> IResult<&'a str, MonsterDeclaration<'a>, E> {
    let (input, _) = space0(input)?;

    let (input, declaration) = alt((
        parse_new_monster,
        parse_select_monster,
    ))(input)?;
    let (input, _) = parse_comment_line_end(input)?;

    Ok((input, declaration))
}


pub fn parse_monster<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Monster<'a>, E> {
    let mut lines = vec![];
    let (input, declaration) = parse_monster_declaration(input)?;
    lines.push(MonsterLine::Declaration);

    let (input, raw_lines) = take_until("#end")(input)?;
    for line in raw_lines.lines() {
        if !line.is_empty() {
            lines.push(MonsterLine::Unparsed(line));
        }
    }

    let (input, _) = tag("#end")(input)?;
    lines.push(MonsterLine::End);

    let (input, _) = parse_comment_line_end(input)?;


    Ok((input, Monster {
        declaration,
        lines,
    }))
}
