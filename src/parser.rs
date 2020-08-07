use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{char, digit1, line_ending, not_line_ending, space1};
use nom::combinator::{map, map_res, opt};
use nom::error::ParseError;
use nom::multi::many1;
use nom::IResult;
use std::str::FromStr;
use either::*;

mod armour;
mod monster;
mod nation;
mod weapon;

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
        map(parse_string_property("#modname"), ModItem::ModName),
        map(monster::parse_monster, ModItem::Monster),
        map(armour::parse_armour, ModItem::Armour),
        map(weapon::parse_weapon, ModItem::Weapon),
        map(nation::parse_nation, ModItem::Nation),
        // This one must always be last since it'll slurp up whatever
        map(parse_unparsed_line, |line| ModItem::UnparsedLine(line)),
    ))(input)
}

fn parse_unparsed_line<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    let (input, opt_line) = opt(not_line_ending)(input)?;
    let (input, _) = line_ending(input)?;
    Ok((input, opt_line.unwrap_or("")))
}

fn parse_string_property<'a, E: ParseError<&'a str>>(
    property: &'static str,
) -> impl Fn(&'a str) -> IResult<&'a str, &'a str, E> {
    move |input| {
        let (input, _) = tag(property)(input)?;
        let (input, _) = space1(input)?;
        let (input, name) = parse_name(input)?;
        let (input, _) = parse_comment_line_end(input)?;
        Ok((input, name))
    }
}

fn parse_id_property<'a, E: ParseError<&'a str>>(
    property: &'static str,
) -> impl Fn(&'a str) -> IResult<&'a str, u32, E> {
    move |input| {
        let (input, _) = tag(property)(input)?;
        let (input, _) = space1(input)?;
        let (input, monster_id) = parse_id(input)?;
        let (input, _) = parse_comment_line_end(input)?;
        Ok((input, monster_id))
    }
}

fn parse_comment_line_end<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, (), E> {
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

fn parse_id_name_property<'a, E: ParseError<&'a str>>(
    property: &'static str,
) -> impl Fn(&'a str) -> IResult<&'a str, Either<u32, &'a str>, E> {
    move |input| {
        let (input, _) = tag(property)(input)?;
        let (input, _) = space1(input)?;
        let (input, id_or_name) = alt((
            map(parse_name, Either::Right),
            map(parse_id, Either::Left),
        ))(input)?;
        let (input, _) = parse_comment_line_end(input)?;
        Ok((input, id_or_name))
    }
}


// struct NameType { pub lines: Vec<Vec<String>> }
// struct Site { pub lines: Vec<Vec<String>> }
// struct Spell { pub lines: Vec<Vec<String>> }
// struct Item { pub lines: Vec<Vec<String>> }
// struct PopType { pub lines: Vec<Vec<String>> }
// struct Merc { pub lines: Vec<Vec<String>> }
// struct Event { pub lines: Vec<Vec<String>> }
