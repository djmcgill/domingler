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
pub enum WeaponDeclaration<'a> {
    SelectId(u32),
    SelectName(&'a str),
    NewId(u32),
    NewImplicit
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum WeaponLine<'a> {
    Declaration,
    End,
    // Name,
    // CopyWeapon,
    Unparsed(&'a str),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Weapon<'a> {
    pub declaration: WeaponDeclaration<'a>,
    // pub name: Option<&'a str>, // only needed for new weapons and must be first
    // pub copy_weapon: Option<Either<u32, &'a str>>,
    pub lines: Vec<WeaponLine<'a>>,
}




fn parse_select_weapon<'a, E: ParseError<&'a str>>(
    input: &'a str
) -> IResult<&'a str, WeaponDeclaration<'a>, E> {
    let (input, _) = tag("#selectweapon")(input)?;
    let (input, _) = space0(input)?;
    let (input, either_id_name) = alt((
        map(parse_id, |id| WeaponDeclaration::SelectId(id)),
        map(parse_name, |name| WeaponDeclaration::SelectName(name)),
    ))(input)?;
    let (input, _) = parse_comment_line_end(input)?;


    Ok((input, either_id_name))
}

fn parse_new_weapon<'a, E: ParseError<&'a str>>(
    input: &'a str
) -> IResult<&'a str, WeaponDeclaration<'a>, E> {
    let (input, _) = tag("#newweapon")(input)?;
    let (input, _) = space0(input)?;
    let (input, opt_weapon_id_str) = opt(digit1)(input)?;
    let opt_weapon_id = opt_weapon_id_str
        .map(|weapon_id_str| u32::from_str(weapon_id_str).unwrap_or_else(|_| panic!("could not parse valid integer id from '{}'", weapon_id_str))); // FIXME

    let declaration = match opt_weapon_id {
        Some(weapon_id) => WeaponDeclaration::NewId(weapon_id),
        None => WeaponDeclaration::NewImplicit,
    };

    Ok((input, declaration))
}

fn parse_weapon_declaration<'a, E: ParseError<&'a str>>(
    input: &'a str
) -> IResult<&'a str, WeaponDeclaration<'a>, E> {
    let (input, _) = space0(input)?;

    let (input, weapon_declaration) = alt((
        parse_new_weapon,
        parse_select_weapon,
    ))(input)?;


    Ok((input, weapon_declaration))
}

pub fn parse_weapon<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Weapon<'a>, E> {
    let mut lines = vec![];
    let (input, declaration) = parse_weapon_declaration(input)?;
    lines.push(WeaponLine::Declaration);

    let (input, raw_lines) = take_until("#end")(input)?;
    for line in raw_lines.lines() {
        if !line.is_empty() {
            lines.push(WeaponLine::Unparsed(line));
        }
    }

    let (input, _) = tag("#end")(input)?;
    lines.push(WeaponLine::End);

    let (input, _) = parse_comment_line_end(input)?;


    Ok((input, Weapon {
       declaration,
       lines,
   }))
}
