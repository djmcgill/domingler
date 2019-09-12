use nom::IResult;
use std::collections::{BTreeMap, BTreeSet};
use nom::error::ParseError;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{space0, anychar, digit1};
use nom::branch::alt;
use nom::combinator::opt;
use either::{Left, Right, Either};
use nom::sequence::delimited;
use std::str::FromStr;

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

struct ParsedMod {
    mod_info: ModInfo,
    weapons: Vec<Weapon>,
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

struct Weapon {
    pub opt_either_id_name: Option<Either<u32, String>>,
    pub lines: String,
}
impl Weapon {
    pub fn parse_weapon<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Weapon, E> {
        let (input, _) = space0(input)?;
        let (input, _) =
            alt((
                tag("#newweapon"),
                tag("#selectweapon"),
            ))(input)?;
        let (input, _) = space0(input)?;
        let (input, opt_either_id_name) = opt(alt((
            parse_name,
            parse_id,
        )))(input)?;

        let (input, lines) = take_until("#end")(input)?;
        let (input, _) = tag("#end")(input)?;

        Ok((input, Weapon {
            opt_either_id_name,
            lines: lines.to_owned(),
        }))


        // parse newweapon, selectweapon
        // scan lines
        // parse #end
//        unimplemented!()
    }
}

// TODO: don't clone
// TODO: map instead of this returning either
pub fn parse_name<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Either<u32, String>, E> {
    let (input, _) = tag("\"")(input)?;
    let (input, name) = take_until("\"")(input)?;
    let (input, _) = tag("\"")(input)?;

    Ok((input, Right(name.to_owned())))
}

pub fn parse_id<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Either<u32, String>, E> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_a_name() {
        let input = "\"foo\"";
        let (remaining, name) = parse_name::<()>(input).unwrap();
        assert_eq!(remaining.len(), 0);
        assert_eq!(name.right().unwrap(), "foo");
    }

    #[test]
    fn parse_an_id() {
        let input = "39";
        let (remaining, name) = parse_id::<()>(input).unwrap();
        assert_eq!(remaining.len(), 0);
        assert_eq!(name.left().unwrap(), 39);
    }

    #[test]
    fn parse_weapon_1() {
        let input = r#"#newweapon 852
#name "Sunlight Blade"
#dmg 1
#shock
#magic
#armornegating
#nostr
#end
"#;
        let (remaining, weapon) = Weapon::parse_weapon::<()>(input).unwrap();
        assert_eq!(weapon.opt_either_id_name.unwrap().left().unwrap(), 852);
        assert_eq!(weapon.lines, r#"
#name "Sunlight Blade"
#dmg 1
#shock
#magic
#armornegating
#nostr
"#);
    }

    #[test]
    fn parse_weapon_2() {
        let input = r#"#newweapon
#copyweapon 20 -- Regular Ass Bite
#name "Magic Bite"
#magic
#end"#;

        let (remaining, weapon) = Weapon::parse_weapon::<()>(input).unwrap();
        assert!(weapon.opt_either_id_name.is_none());
        assert_eq!(weapon.lines, r#"
#copyweapon 20 -- Regular Ass Bite
#name "Magic Bite"
#magic
"#);
    }

    #[test]
    fn parse_weapon_3() {
        let input = r#"#selectweapon "Swallow"
#explspr 10259 -- Slurpy Signal
#end"#;

    }
}
