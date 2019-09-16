use nom::IResult;
use std::collections::{BTreeMap, BTreeSet};
use nom::error::ParseError;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{space0, anychar, digit1, line_ending};
use nom::branch::alt;
use nom::combinator::opt;
use either::{Left, Right, Either};
use nom::sequence::delimited;
use std::str::FromStr;

mod weapon;
use weapon::Weapon;

enum ParseBlockType {
    Weapon,
    Armour,
    Monster,
    NameType,
    Site,
    Nation,
    Spell,
    Item,
    PopType,
    Merc,
    Event,
}

struct ParseState {
    in_block: Option<ParseBlockType>,
    in_multiline_text: bool,
}

struct ParsedMod<'a> {
    mod_info: ModInfo,
    weapons: Vec<Weapon<'a>>,
    armours: Vec<Armour>,
    monsters: Vec<Monster>,
    name_types: Vec<NameType>,
    sites: Vec<Site>,
    nations: Vec<Nation>,
    spells: Vec<Spell>,
    items: Vec<Item>,
    pop_type: Vec<PopType>,
    mercs: Vec<Merc>,
    events: Vec<Event>,
}

struct ModInfo { pub lines: Vec<Vec<String>> }

fn parse_comment_line_end<'a, E: ParseError<&'a str>>(
    input: &'a str
) -> IResult<&'a str, (), E> {
    let (input, _) = take_until(line_ending)(input)?;
    let (input, _) = line_ending(input)?;
    Ok((input, ()))
}


// TODO: map instead of this returning either
pub fn parse_name<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Either<u32, &'a str>, E> {
    let (input, _) = tag("\"")(input)?;
    let (input, name) = take_until("\"")(input)?;
    let (input, _) = tag("\"")(input)?;

    Ok((input, Right(name)))
}

pub fn parse_id<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Either<u32, &'a str>, E> {
    let (input, number) = digit1(input)?;
    // FIXME
    Ok((input, Left(u32::from_str(number).unwrap())))
}

struct Armour { pub lines: Vec<Vec<String>> }
struct Monster { pub lines: Vec<Vec<String>> }
struct NameType { pub lines: Vec<Vec<String>> }
struct Site { pub lines: Vec<Vec<String>> }
struct Nation { pub lines: Vec<Vec<String>> }
struct Spell { pub lines: Vec<Vec<String>> }
struct Item { pub lines: Vec<Vec<String>> }
struct PopType { pub lines: Vec<Vec<String>> }
struct Merc { pub lines: Vec<Vec<String>> }
struct Event { pub lines: Vec<Vec<String>> }
