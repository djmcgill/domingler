use monster::MonsterIdOrMontagOrName;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{space0, space1};
use nom::IResult;
use nom::{branch::alt, error::VerboseError};

use crate::parser::*;

use super::monster::{MonsterId, MonsterIdOrName};

#[cfg(test)]
mod tests;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum NationDeclaration {
    SelectId(u32), // select number only
    NewImplicit,   // no new with number allowed
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct NationName<'a>(&'a str);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct SiteName<'a>(&'a str);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Era {
    Early = 1,
    Mid = 2,
    Late = 3,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum NationLine<'a> {
    Name,
    Era,
    StartSite,
    IslandSite,
    _Dummy(&'a ()), // unused
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Nation<'a> {
    pub declaration: NationDeclaration,

    pub name: Option<NationName<'a>>,
    pub era: Option<Era>,

    pub start_site: Option<SiteName<'a>>,
    pub island_site: Option<SiteName<'a>>,

    pub start_com: Option<MonsterIdOrName<'a>>,
    pub coast_com_1: Vec<MonsterIdOrName<'a>>,
    pub coast_com_2: Vec<MonsterIdOrName<'a>>,
    pub add_foreign_unit: Vec<MonsterIdOrName<'a>>,
    pub add_foreign_com: Vec<MonsterIdOrName<'a>>,

    pub forest_recruit: Vec<MonsterIdOrName<'a>>,
    pub mountain_recruit: Vec<MonsterIdOrName<'a>>,
    pub swamp_recruit: Vec<MonsterIdOrName<'a>>,
    pub waste_recruit: Vec<MonsterIdOrName<'a>>,
    pub cave_recruit: Vec<MonsterIdOrName<'a>>,
    pub coast_recruit: Vec<MonsterIdOrName<'a>>,

    pub start_scout: Option<MonsterIdOrName<'a>>,

    pub forest_commander: Vec<MonsterIdOrName<'a>>,
    pub mountain_commander: Vec<MonsterIdOrName<'a>>,
    pub swamp_commander: Vec<MonsterIdOrName<'a>>,
    pub waste_commander: Vec<MonsterIdOrName<'a>>,
    pub cave_commander: Vec<MonsterIdOrName<'a>>,
    pub coast_commander: Vec<MonsterIdOrName<'a>>,

    pub start_unit_type_1: Option<MonsterIdOrName<'a>>,
    pub start_unit_type_2: Option<MonsterIdOrName<'a>>,
    pub add_recruit_unit: Vec<MonsterIdOrName<'a>>,
    pub add_recruit_commander: Vec<MonsterIdOrName<'a>>,

    pub underwater_recruit: Vec<MonsterIdOrName<'a>>,
    pub underwater_commander: Vec<MonsterIdOrName<'a>>,
    pub coast_unit_1: Option<MonsterIdOrName<'a>>,
    pub coast_unit_2: Option<MonsterIdOrName<'a>>,
    pub coast_unit_3: Option<MonsterIdOrName<'a>>,
    pub land_recruit: Vec<MonsterIdOrName<'a>>,
    pub land_commander: Vec<MonsterIdOrName<'a>>,

    pub hero_1: Option<MonsterId>,
    pub hero_2: Option<MonsterId>,
    pub hero_3: Option<MonsterId>,
    pub hero_4: Option<MonsterId>,
    pub hero_5: Option<MonsterId>,
    pub hero_6: Option<MonsterId>,
    pub hero_7: Option<MonsterId>,
    pub hero_8: Option<MonsterId>,
    pub hero_9: Option<MonsterId>,
    pub hero_10: Option<MonsterId>,
    pub multi_hero_1: Option<MonsterId>,
    pub multi_hero_2: Option<MonsterId>,
    pub multi_hero_3: Option<MonsterId>,
    pub multi_hero_4: Option<MonsterId>,
    pub multi_hero_5: Option<MonsterId>,
    pub multi_hero_6: Option<MonsterId>,
    pub multi_hero_7: Option<MonsterId>,
    
    pub defense_commander_1: Option<MonsterIdOrName<'a>>,
    pub defense_commander_2: Option<MonsterIdOrName<'a>>,
    pub defense_unit_1: Option<MonsterIdOrName<'a>>,
    pub defense_unit_1b: Option<MonsterIdOrName<'a>>,
    pub defense_unit_1c: Option<MonsterIdOrName<'a>>,
    pub defense_unit_1d: Option<MonsterIdOrName<'a>>,
    pub defense_unit_2: Option<MonsterIdOrName<'a>>,
    pub defense_unit_2b: Option<MonsterIdOrName<'a>>,
    pub wall_commander: Vec<MonsterIdOrName<'a>>,
    pub wall_unit: Vec<MonsterIdOrName<'a>>,

    pub underwater_defense_commander_1: Option<MonsterIdOrName<'a>>,
    pub underwater_defense_commander_2: Option<MonsterIdOrName<'a>>,
    pub underwater_defense_unit_1: Option<MonsterIdOrName<'a>>,
    pub underwater_defense_unit_1b: Option<MonsterIdOrName<'a>>,
    pub underwater_defense_unit_1c: Option<MonsterIdOrName<'a>>,
    pub underwater_defense_unit_1d: Option<MonsterIdOrName<'a>>,
    pub underwater_defense_unit_2: Option<MonsterIdOrName<'a>>,
    pub underwater_defense_unit_2b: Option<MonsterIdOrName<'a>>,
    pub underwater_wall_unit: Vec<MonsterIdOrName<'a>>,
    pub underwater_wall_commander: Vec<MonsterIdOrName<'a>>,

    pub add_god: Vec<MonsterIdOrName<'a>>,
    pub delete_god: Vec<MonsterIdOrName<'a>>,
    pub cheap_god_20: Vec<MonsterIdOrName<'a>>,
    pub cheap_god_40: Vec<MonsterIdOrName<'a>>,

    pub guard_spirit: Option<MonsterIdOrMontagOrName<'a>>,


    /// This field does not contain the declaration or the end
    pub inner_lines: Vec<Either<&'a str, NationLine<'a>>>,
}

fn parse_select_nation<'a>(
    input: &'a str,
) -> IResult<&'a str, NationDeclaration, VerboseError<&'a str>> {
    let (input, _) = tag("#selectnation")(input)?;
    let (input, _) = space1(input)?;
    let (input, id) = parse_id(input)?;

    Ok((input, NationDeclaration::SelectId(id)))
}

fn parse_new_nation<'a>(
    input: &'a str,
) -> IResult<&'a str, NationDeclaration, VerboseError<&'a str>> {
    let (input, _) = tag("#newnation")(input)?;
    Ok((input, NationDeclaration::NewImplicit))
}

fn parse_nation_declaration<'a>(
    input: &'a str,
) -> IResult<&'a str, NationDeclaration, VerboseError<&'a str>> {
    let (input, _) = space0(input)?;

    let (input, weapon_declaration) = alt((parse_new_nation, parse_select_nation))(input)?;
    let (input, _) = parse_comment_line_end(input)?;

    Ok((input, weapon_declaration))
}

pub fn parse_nation<'a>(input: &'a str) -> IResult<&'a str, Nation<'a>, VerboseError<&'a str>> {
    // let mut inner_lines = vec![];
    // let (input, declaration) = parse_nation_declaration(input)?;

    // FIXME
    // let (input, raw_lines) = take_until("#end")(input)?;
    // for line in raw_lines.lines() {
    //     if !line.is_empty() {
    //         inner_lines.push(NationLine::Unparsed(line));
    //     }
    // }

    // let (input, _) = tag("#end")(input)?;

    // let (input, _) = parse_comment_line_end(input)?;
    // Ok((
    //     input,
    //     Nation {
    //         declaration,
    //         inner_lines,
    //     },
    // ))
    todo!()
}
