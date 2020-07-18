use nom::IResult;
use std::collections::{BTreeMap, BTreeSet};
use nom::error::ParseError;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{space0, anychar, digit1, line_ending, not_line_ending};
use nom::branch::alt;
use nom::combinator::{opt, map_res, map};
use either::{Left, Right, Either};
use nom::sequence::delimited;
use std::str::FromStr;
use nom::multi::many1;

mod weapon;
mod armour;
mod nation;
mod monster;

#[cfg(test)]
mod tests;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ModItem<'a> {
    ModName(&'a str),
    // ModDescription(&'a str),
    // ModIcon(&'a str),
    // ModVersion(&'a str),
    // ModDomVersion(&'a str),
    Weapon(weapon::Weapon<'a>),
    Armour(armour::Armour<'a>),
    Monster(monster::Monster<'a>),
    // NameType,
    // Site,
    Nation(nation::Nation<'a>),
    // Spell,
    // Item,
    // PopType,
    // Merc,
    // Event,
    UnparsedLine(&'a str),
}


pub struct ParsedMod<'a>(Vec<ModItem<'a>>);

pub fn parse_mod<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, ParsedMod<'a>, E> {
    map(many1(parse_mod_item), ParsedMod)(input)
}

// TODO: move the space0 parsing to here to speed things up
fn parse_mod_item<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, ModItem<'a>, E> {
    alt((
        map(parse_mod_name, ModItem::ModName),
        map(monster::parse_monster, ModItem::Monster),
        map(armour::parse_armour, ModItem::Armour),
        map(weapon::parse_weapon, ModItem::Weapon),
        map(nation::parse_nation, ModItem::Nation),

        // This one must always be last since it'll slurp up whatever
        map(parse_unparsed_line, |line| ModItem::UnparsedLine(line))
    ))(input)
}

fn parse_unparsed_line<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    let (input, opt_line) = opt(not_line_ending)(input)?;
    let (input, _) = line_ending(input)?;
    Ok((input, opt_line.unwrap_or("")))
}

fn parse_mod_name<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    let (input, _) = tag("#modname")(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = tag("\"")(input)?;
    let (input, name) = take_until("\"")(input)?;
    let (input, _) = tag("\"")(input)?;
    let (input, _) = parse_comment_line_end(input)?;
    Ok((input, name))
}

fn parse_comment_line_end<'a, E: ParseError<&'a str>>(
    input: &'a str
) -> IResult<&'a str, (), E> {
    let (input, _) = opt(not_line_ending)(input)?;
    let (input, _) = opt(line_ending)(input)?;
    Ok((input, ()))
}

pub fn parse_name<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    let (input, _) = tag("\"")(input)?;
    let (input, name) = take_until("\"")(input)?;
    let (input, _) = tag("\"")(input)?;

    Ok((input, name))
}

pub fn parse_id<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, u32, E> {
    map_res(digit1, u32::from_str)(input)
}

// struct NameType { pub lines: Vec<Vec<String>> }
// struct Site { pub lines: Vec<Vec<String>> }
// struct Spell { pub lines: Vec<Vec<String>> }
// struct Item { pub lines: Vec<Vec<String>> }
// struct PopType { pub lines: Vec<Vec<String>> }
// struct Merc { pub lines: Vec<Vec<String>> }
// struct Event { pub lines: Vec<Vec<String>> }
