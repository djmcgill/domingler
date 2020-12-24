use monster::MonsterIdOrMontagOrName;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{space0, space1};
use nom::IResult;
use nom::{branch::alt, error::VerboseError};

use crate::parser::*;

use super::monster::{MonsterId, MonsterIdOrName};
use crate::parser::monster::parse_id_name_montag_property;

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
    StartCom,
    CoastCom1,
    CoastCom2,
    AddForeignUnit,
    AddForeignCom,
    ForestRecruit,
    MountainRecruit,
    SwampRecruit,
    WasteRecruit,
    CaveRecruit,
    CoastRecruit,
    StartScout,
    ForestCommander,
    MountainCommander,
    SwampCommander,
    WasteCommander,
    CaveCommander,
    CoastCommander,
    StartUnitType1,
    StartUnitType2,
    AddRecruitUnit,
    AddRecruitCommander,
    UnderwaterRecruit,
    UnderwaterCommander,
    CoastUnit1,
    CoastUnit2,
    CoastUnit3,
    LandRecruit,
    LandCommander,
    Hero1,
    Hero2,
    Hero3,
    Hero4,
    Hero5,
    Hero6,
    Hero7,
    Hero8,
    Hero9,
    Hero10,
    MultiHero1,
    MultiHero2,
    MultiHero3,
    MultiHero4,
    MultiHero5,
    MultiHero6,
    MultiHero7,
    DefenseCommander1,
    DefenseCommander2,
    DefenseUnit1,
    DefenseUnit1b,
    DefenseUnit1c,
    DefenseUnit1d,
    DefenseUnit2,
    DefenseUnit2b,
    WallCommander,
    WallUnit,
    UnderwaterDefenseCommander1,
    UnderwaterDefenseCommander2,
    UnderwaterDefenseUnit1,
    UnderwaterDefenseUnit1b,
    UnderwaterDefenseUnit1c,
    UnderwaterDefenseUnit1d,
    UnderwaterDefenseUnit2,
    UnderwaterDefenseUnit2b,
    UnderwaterWallUnit,
    UnderwaterWallCommander,
    AddGod,
    DeleteGod,
    CheapGod20,
    CheapGod40,
    GuardSpirit,
    _Dummy(&'a ()), // unused
}
impl<'a> DominionsCommand for NationLine<'a> {
    fn line_tag(&self) -> &'static str {
        match self {
            NationLine::Name => "#name",
            NationLine::Era => "#era",
            NationLine::StartSite => "#startsite",
            NationLine::IslandSite => "#islandsite",
            NationLine::StartCom => "#startcom",
            NationLine::CoastCom1 => "#coastcom1",
            NationLine::CoastCom2 => "#coastcom2",
            NationLine::AddForeignUnit => "#addforeignunit",
            NationLine::AddForeignCom => "#addforeigncom",
            NationLine::ForestRecruit => "#forestrec",
            NationLine::MountainRecruit => "#mountainrec",
            NationLine::SwampRecruit => "#swamprec",
            NationLine::WasteRecruit => "#wasterec",
            NationLine::CaveRecruit => "#caverec",
            NationLine::CoastRecruit => "#coastrec",
            NationLine::StartScout => "#startscout",
            NationLine::ForestCommander => "#forestcom",
            NationLine::MountainCommander => "#mountaincom",
            NationLine::SwampCommander => "#swampcom",
            NationLine::WasteCommander => "#wastecom",
            NationLine::CaveCommander => "#cavecom",
            NationLine::CoastCommander => "#coastcom",
            NationLine::StartUnitType1 => "#startunittype1",
            NationLine::StartUnitType2 => "#startunittype2",
            NationLine::AddRecruitUnit => "#addrecunit",
            NationLine::AddRecruitCommander => "#addreccom",
            NationLine::UnderwaterRecruit => "#uwrec",
            NationLine::UnderwaterCommander => "#uwcom",
            NationLine::CoastUnit1 => "#coastunit1",
            NationLine::CoastUnit2 => "#coastunit2",
            NationLine::CoastUnit3 => "#coastunit3",
            NationLine::LandRecruit => "#landrec",
            NationLine::LandCommander => "#landcom",
            NationLine::Hero1 => "#hero1",
            NationLine::Hero2 => "#hero2",
            NationLine::Hero3 => "#hero3",
            NationLine::Hero4 => "#hero4",
            NationLine::Hero5 => "#hero5",
            NationLine::Hero6 => "#hero6",
            NationLine::Hero7 => "#hero7",
            NationLine::Hero8 => "#hero8",
            NationLine::Hero9 => "#hero9",
            NationLine::Hero10 => "#hero10",
            NationLine::MultiHero1 => "#multihero1",
            NationLine::MultiHero2 => "#multihero2",
            NationLine::MultiHero3 => "#multihero3",
            NationLine::MultiHero4 => "#multihero4",
            NationLine::MultiHero5 => "#multihero5",
            NationLine::MultiHero6 => "#multihero6",
            NationLine::MultiHero7 => "#multihero7",
            NationLine::DefenseCommander1 => "#defcom1",
            NationLine::DefenseCommander2 => "#defcom2",
            NationLine::DefenseUnit1 => "#defunit1",
            NationLine::DefenseUnit1b => "#defunit1b",
            NationLine::DefenseUnit1c => "#defunit1c",
            NationLine::DefenseUnit1d => "#defunit1d",
            NationLine::DefenseUnit2 => "#defunit2",
            NationLine::DefenseUnit2b => "#defunit2b",
            NationLine::WallCommander => "#wallcom",
            NationLine::WallUnit => "#wallunit",
            NationLine::UnderwaterDefenseCommander1 => "#uwdefcom1",
            NationLine::UnderwaterDefenseCommander2 => "#uwdefcom2",
            NationLine::UnderwaterDefenseUnit1 => "#uwdefunit1",
            NationLine::UnderwaterDefenseUnit1b => "#uwdefunit1b",
            NationLine::UnderwaterDefenseUnit1c => "#uwdefunit1c",
            NationLine::UnderwaterDefenseUnit1d => "#uwdefunit1d",
            NationLine::UnderwaterDefenseUnit2 => "#uwdefunit2",
            NationLine::UnderwaterDefenseUnit2b => "#uwdefunit2b",
            NationLine::UnderwaterWallUnit => "#uwwallunit",
            NationLine::UnderwaterWallCommander => "#uwwallcom",
            NationLine::AddGod => "#addgod",
            NationLine::DeleteGod => "#delgod",
            NationLine::CheapGod20 => "#cheapgod20",
            NationLine::CheapGod40 => "#cheapgod40",
            NationLine::GuardSpirit => "#guardspirit",
            NationLine::_Dummy(_) => unimplemented!(),
        }
    }
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
    let mut inner_lines = vec![];
    let (mut input, declaration) = parse_nation_declaration(input)?;

    let mut name = None;
    let mut era = None;
    let mut start_site = None;
    let mut island_site = None;
    let mut start_com = None;
    let mut coast_com_1 = vec![];
    let mut coast_com_2 = vec![];
    let mut add_foreign_unit = vec![];
    let mut add_foreign_com = vec![];
    let mut forest_recruit = vec![];
    let mut mountain_recruit = vec![];
    let mut swamp_recruit = vec![];
    let mut waste_recruit = vec![];
    let mut cave_recruit = vec![];
    let mut coast_recruit = vec![];
    let mut start_scout = None;
    let mut forest_commander = vec![];
    let mut mountain_commander = vec![];
    let mut swamp_commander = vec![];
    let mut waste_commander = vec![];
    let mut cave_commander = vec![];
    let mut coast_commander = vec![];
    let mut start_unit_type_1 = None;
    let mut start_unit_type_2 = None;
    let mut add_recruit_unit = vec![];
    let mut add_recruit_commander = vec![];
    let mut underwater_recruit = vec![];
    let mut underwater_commander = vec![];
    let mut coast_unit_1 = None;
    let mut coast_unit_2 = None;
    let mut coast_unit_3 = None;
    let mut land_recruit = vec![];
    let mut land_commander = vec![];
    let mut hero_1 = None;
    let mut hero_2 = None;
    let mut hero_3 = None;
    let mut hero_4 = None;
    let mut hero_5 = None;
    let mut hero_6 = None;
    let mut hero_7 = None;
    let mut hero_8 = None;
    let mut hero_9 = None;
    let mut hero_10 = None;
    let mut multi_hero_1 = None;
    let mut multi_hero_2 = None;
    let mut multi_hero_3 = None;
    let mut multi_hero_4 = None;
    let mut multi_hero_5 = None;
    let mut multi_hero_6 = None;
    let mut multi_hero_7 = None;
    let mut defense_commander_1 = None;
    let mut defense_commander_2 = None;
    let mut defense_unit_1 = None;
    let mut defense_unit_1b = None;
    let mut defense_unit_1c = None;
    let mut defense_unit_1d = None;
    let mut defense_unit_2 = None;
    let mut defense_unit_2b = None;
    let mut wall_commander = vec![];
    let mut wall_unit = vec![];
    let mut underwater_defense_commander_1 = None;
    let mut underwater_defense_commander_2 = None;
    let mut underwater_defense_unit_1 = None;
    let mut underwater_defense_unit_1b = None;
    let mut underwater_defense_unit_1c = None;
    let mut underwater_defense_unit_1d = None;
    let mut underwater_defense_unit_2 = None;
    let mut underwater_defense_unit_2b = None;
    let mut underwater_wall_unit = vec![];
    let mut underwater_wall_commander = vec![];
    let mut add_god = vec![];
    let mut delete_god = vec![];
    let mut cheap_god_20 = vec![];
    let mut cheap_god_40 = vec![];
    let mut guard_spirit = None;

    loop {
        if let Ok((remaining_input, found_name)) =
            parse_string_property(NationLine::Name.line_tag())(input)
        {
            if let Some(old_name) = name.replace(NationName(found_name)) {
                panic!("Nation had duplicate #name: {:?}", old_name); // fixme
            }
            input = remaining_input;
            inner_lines.push(Right(NationLine::Name));
        } else if let Ok((remaining_input, found_era_id)) =
            parse_id_property(NationLine::Era.line_tag())(input)
        {
            let found_era = match found_era_id {
                1 => Era::Early,
                2 => Era::Mid,
                3 => Era::Late,
                _ => panic!("Bad nation number: {}", found_era_id), // fixme
            };
            if let Some(old_era) = era.replace(found_era) {
                panic!("Nation had duplicate #era: {:?}", old_era);
            }
            input = remaining_input;
            inner_lines.push(Right(NationLine::Era));
        } else if parse_site_name_option(
            &mut input,
            &mut inner_lines,
            NationLine::StartSite,
            &mut start_site,
        ) {
        } else if parse_site_name_option(
            &mut input,
            &mut inner_lines,
            NationLine::IslandSite,
            &mut island_site,
        ) {
        } else if parse_monster_id_name_option(
            &mut input,
            &mut inner_lines,
            NationLine::StartCom,
            &mut start_com,
        ) {
        } else if parse_monster_id_name_list(
            &mut input,
            &mut inner_lines,
            NationLine::CoastCom1,
            &mut coast_com_1,
        ) {
        } else if parse_monster_id_name_list(
            &mut input,
            &mut inner_lines,
            NationLine::CoastCom2,
            &mut coast_com_2,
        ) {
        } else if parse_monster_id_name_list(
            &mut input,
            &mut inner_lines,
            NationLine::AddForeignUnit,
            &mut add_foreign_unit,
        ) {
        } else if parse_monster_id_name_list(
            &mut input,
            &mut inner_lines,
            NationLine::AddForeignCom,
            &mut add_foreign_com,
        ) {
        } else if parse_monster_id_name_list(
            &mut input,
            &mut inner_lines,
            NationLine::ForestRecruit,
            &mut forest_recruit,
        ) {
        } else if parse_monster_id_name_list(
            &mut input,
            &mut inner_lines,
            NationLine::MountainRecruit,
            &mut mountain_recruit,
        ) {
        } else if parse_monster_id_name_list(
            &mut input,
            &mut inner_lines,
            NationLine::SwampRecruit,
            &mut swamp_recruit,
        ) {
        } else if parse_monster_id_name_list(
            &mut input,
            &mut inner_lines,
            NationLine::WasteRecruit,
            &mut waste_recruit,
        ) {
        } else if parse_monster_id_name_list(
            &mut input,
            &mut inner_lines,
            NationLine::CaveRecruit,
            &mut cave_recruit,
        ) {
        } else if parse_monster_id_name_list(
            &mut input,
            &mut inner_lines,
            NationLine::CoastRecruit,
            &mut coast_recruit,
        ) {
        } else if parse_monster_id_name_option(
            &mut input,
            &mut inner_lines,
            NationLine::StartScout,
            &mut start_scout,
        ) {
        } else if parse_monster_id_name_list(
            &mut input,
            &mut inner_lines,
            NationLine::ForestCommander,
            &mut forest_commander,
        ) {
        } else if parse_monster_id_name_list(
            &mut input,
            &mut inner_lines,
            NationLine::MountainCommander,
            &mut mountain_commander,
        ) {
        } else if parse_monster_id_name_list(
            &mut input,
            &mut inner_lines,
            NationLine::SwampCommander,
            &mut swamp_commander,
        ) {
        } else if parse_monster_id_name_list(
            &mut input,
            &mut inner_lines,
            NationLine::WasteCommander,
            &mut waste_commander,
        ) {
        } else if parse_monster_id_name_list(
            &mut input,
            &mut inner_lines,
            NationLine::CaveCommander,
            &mut cave_commander,
        ) {
        } else if parse_monster_id_name_list(
            &mut input,
            &mut inner_lines,
            NationLine::CoastCommander,
            &mut coast_commander,
        ) {
        } else if parse_monster_id_name_option(
            &mut input,
            &mut inner_lines,
            NationLine::StartUnitType1,
            &mut start_unit_type_1,
        ) {
        } else if parse_monster_id_name_option(
            &mut input,
            &mut inner_lines,
            NationLine::StartUnitType2,
            &mut start_unit_type_2,
        ) {
        } else if parse_monster_id_name_list(
            &mut input,
            &mut inner_lines,
            NationLine::AddRecruitUnit,
            &mut add_recruit_unit,
        ) {
        } else if parse_monster_id_name_list(
            &mut input,
            &mut inner_lines,
            NationLine::AddRecruitCommander,
            &mut add_recruit_commander,
        ) {
        } else if parse_monster_id_name_list(
            &mut input,
            &mut inner_lines,
            NationLine::UnderwaterRecruit,
            &mut underwater_recruit,
        ) {
        } else if parse_monster_id_name_list(
            &mut input,
            &mut inner_lines,
            NationLine::UnderwaterCommander,
            &mut underwater_commander,
        ) {
        } else if parse_monster_id_name_option(
            &mut input,
            &mut inner_lines,
            NationLine::CoastUnit1,
            &mut coast_unit_1,
        ) {
        } else if parse_monster_id_name_option(
            &mut input,
            &mut inner_lines,
            NationLine::CoastUnit2,
            &mut coast_unit_2,
        ) {
        } else if parse_monster_id_name_option(
            &mut input,
            &mut inner_lines,
            NationLine::CoastUnit3,
            &mut coast_unit_3,
        ) {
        } else if parse_monster_id_name_list(
            &mut input,
            &mut inner_lines,
            NationLine::LandRecruit,
            &mut land_recruit,
        ) {
        } else if parse_monster_id_name_list(
            &mut input,
            &mut inner_lines,
            NationLine::LandCommander,
            &mut land_commander,
        ) {
        } else if parse_monster_id_option(
            &mut input,
            &mut inner_lines,
            NationLine::Hero1,
            &mut hero_1,
        ) {
        } else if parse_monster_id_option(
            &mut input,
            &mut inner_lines,
            NationLine::Hero2,
            &mut hero_2,
        ) {
        } else if parse_monster_id_option(
            &mut input,
            &mut inner_lines,
            NationLine::Hero3,
            &mut hero_3,
        ) {
        } else if parse_monster_id_option(
            &mut input,
            &mut inner_lines,
            NationLine::Hero4,
            &mut hero_4,
        ) {
        } else if parse_monster_id_option(
            &mut input,
            &mut inner_lines,
            NationLine::Hero5,
            &mut hero_5,
        ) {
        } else if parse_monster_id_option(
            &mut input,
            &mut inner_lines,
            NationLine::Hero6,
            &mut hero_6,
        ) {
        } else if parse_monster_id_option(
            &mut input,
            &mut inner_lines,
            NationLine::Hero7,
            &mut hero_7,
        ) {
        } else if parse_monster_id_option(
            &mut input,
            &mut inner_lines,
            NationLine::Hero8,
            &mut hero_8,
        ) {
        } else if parse_monster_id_option(
            &mut input,
            &mut inner_lines,
            NationLine::Hero9,
            &mut hero_9,
        ) {
        } else if parse_monster_id_option(
            &mut input,
            &mut inner_lines,
            NationLine::Hero10,
            &mut hero_10,
        ) {
        } else if parse_monster_id_option(
            &mut input,
            &mut inner_lines,
            NationLine::MultiHero1,
            &mut multi_hero_1,
        ) {
        } else if parse_monster_id_option(
            &mut input,
            &mut inner_lines,
            NationLine::MultiHero2,
            &mut multi_hero_2,
        ) {
        } else if parse_monster_id_option(
            &mut input,
            &mut inner_lines,
            NationLine::MultiHero3,
            &mut multi_hero_3,
        ) {
        } else if parse_monster_id_option(
            &mut input,
            &mut inner_lines,
            NationLine::MultiHero4,
            &mut multi_hero_4,
        ) {
        } else if parse_monster_id_option(
            &mut input,
            &mut inner_lines,
            NationLine::MultiHero5,
            &mut multi_hero_5,
        ) {
        } else if parse_monster_id_option(
            &mut input,
            &mut inner_lines,
            NationLine::MultiHero6,
            &mut multi_hero_6,
        ) {
        } else if parse_monster_id_option(
            &mut input,
            &mut inner_lines,
            NationLine::MultiHero7,
            &mut multi_hero_7,
        ) {
        } else if parse_monster_id_name_option(
            &mut input,
            &mut inner_lines,
            NationLine::DefenseCommander1,
            &mut defense_commander_1,
        ) {
        } else if parse_monster_id_name_option(
            &mut input,
            &mut inner_lines,
            NationLine::DefenseCommander2,
            &mut defense_commander_2,
        ) {
        } else if parse_monster_id_name_option(
            &mut input,
            &mut inner_lines,
            NationLine::DefenseUnit1,
            &mut defense_unit_1,
        ) {
        } else if parse_monster_id_name_option(
            &mut input,
            &mut inner_lines,
            NationLine::DefenseUnit1b,
            &mut defense_unit_1b,
        ) {
        } else if parse_monster_id_name_option(
            &mut input,
            &mut inner_lines,
            NationLine::DefenseUnit1c,
            &mut defense_unit_1c,
        ) {
        } else if parse_monster_id_name_option(
            &mut input,
            &mut inner_lines,
            NationLine::DefenseUnit1d,
            &mut defense_unit_1d,
        ) {
        } else if parse_monster_id_name_option(
            &mut input,
            &mut inner_lines,
            NationLine::DefenseUnit2,
            &mut defense_unit_2,
        ) {
        } else if parse_monster_id_name_option(
            &mut input,
            &mut inner_lines,
            NationLine::DefenseUnit2b,
            &mut defense_unit_2b,
        ) {
        } else if parse_monster_id_name_list(
            &mut input,
            &mut inner_lines,
            NationLine::WallCommander,
            &mut wall_commander,
        ) {
        } else if parse_monster_id_name_list(
            &mut input,
            &mut inner_lines,
            NationLine::WallUnit,
            &mut wall_unit,
        ) {
        } else if parse_monster_id_name_option(
            &mut input,
            &mut inner_lines,
            NationLine::UnderwaterDefenseCommander1,
            &mut underwater_defense_commander_1,
        ) {
        } else if parse_monster_id_name_option(
            &mut input,
            &mut inner_lines,
            NationLine::UnderwaterDefenseCommander2,
            &mut underwater_defense_commander_2,
        ) {
        } else if parse_monster_id_name_option(
            &mut input,
            &mut inner_lines,
            NationLine::UnderwaterDefenseUnit1,
            &mut underwater_defense_unit_1,
        ) {
        } else if parse_monster_id_name_option(
            &mut input,
            &mut inner_lines,
            NationLine::UnderwaterDefenseUnit1b,
            &mut underwater_defense_unit_1b,
        ) {
        } else if parse_monster_id_name_option(
            &mut input,
            &mut inner_lines,
            NationLine::UnderwaterDefenseUnit1c,
            &mut underwater_defense_unit_1c,
        ) {
        } else if parse_monster_id_name_option(
            &mut input,
            &mut inner_lines,
            NationLine::UnderwaterDefenseUnit1d,
            &mut underwater_defense_unit_1d,
        ) {
        } else if parse_monster_id_name_option(
            &mut input,
            &mut inner_lines,
            NationLine::UnderwaterDefenseUnit2,
            &mut underwater_defense_unit_2,
        ) {
        } else if parse_monster_id_name_option(
            &mut input,
            &mut inner_lines,
            NationLine::UnderwaterDefenseUnit2b,
            &mut underwater_defense_unit_2b,
        ) {
        } else if parse_monster_id_name_list(
            &mut input,
            &mut inner_lines,
            NationLine::UnderwaterWallUnit,
            &mut underwater_wall_unit,
        ) {
        } else if parse_monster_id_name_list(
            &mut input,
            &mut inner_lines,
            NationLine::UnderwaterWallCommander,
            &mut underwater_wall_commander,
        ) {
        } else if parse_monster_id_name_list(
            &mut input,
            &mut inner_lines,
            NationLine::AddGod,
            &mut add_god,
        ) {
        } else if parse_monster_id_name_list(
            &mut input,
            &mut inner_lines,
            NationLine::DeleteGod,
            &mut delete_god,
        ) {
        } else if parse_monster_id_name_list(
            &mut input,
            &mut inner_lines,
            NationLine::CheapGod20,
            &mut cheap_god_20,
        ) {
        } else if parse_monster_id_name_list(
            &mut input,
            &mut inner_lines,
            NationLine::CheapGod40,
            &mut cheap_god_40,
        ) {
        } else if parse_monster_id_name_montag_option(
            &mut input,
            &mut inner_lines,
            NationLine::GuardSpirit,
            &mut guard_spirit,
        ) {

            // These two must be last
        } else if let Ok((remaining_input, _)) = tag::<_, _, VerboseError<&'a str>>("#end")(input) {
            input = remaining_input;
            break; // we're done
        } else {
            // "cannot" fail
            let (remaining_input, unparsed_line) = parse_unparsed_line(input)?;
            input = remaining_input;
            inner_lines.push(Left(unparsed_line));
        }
    }

    let nation = Nation {
        declaration,
        name,
        era,
        start_site,
        island_site,
        start_com,
        coast_com_1,
        coast_com_2,
        add_foreign_unit,
        add_foreign_com,
        forest_recruit,
        mountain_recruit,
        swamp_recruit,
        waste_recruit,
        cave_recruit,
        coast_recruit,
        start_scout,
        forest_commander,
        mountain_commander,
        swamp_commander,
        waste_commander,
        cave_commander,
        coast_commander,
        start_unit_type_1,
        start_unit_type_2,
        add_recruit_unit,
        add_recruit_commander,
        underwater_recruit,
        underwater_commander,
        coast_unit_1,
        coast_unit_2,
        coast_unit_3,
        land_recruit,
        land_commander,
        hero_1,
        hero_2,
        hero_3,
        hero_4,
        hero_5,
        hero_6,
        hero_7,
        hero_8,
        hero_9,
        hero_10,
        multi_hero_1,
        multi_hero_2,
        multi_hero_3,
        multi_hero_4,
        multi_hero_5,
        multi_hero_6,
        multi_hero_7,
        defense_commander_1,
        defense_commander_2,
        defense_unit_1,
        defense_unit_1b,
        defense_unit_1c,
        defense_unit_1d,
        defense_unit_2,
        defense_unit_2b,
        wall_commander,
        wall_unit,
        underwater_defense_commander_1,
        underwater_defense_commander_2,
        underwater_defense_unit_1,
        underwater_defense_unit_1b,
        underwater_defense_unit_1c,
        underwater_defense_unit_1d,
        underwater_defense_unit_2,
        underwater_defense_unit_2b,
        underwater_wall_unit,
        underwater_wall_commander,
        add_god,
        delete_god,
        cheap_god_20,
        cheap_god_40,
        guard_spirit,
        inner_lines,
    };

    Ok((input, nation))
}

fn parse_monster_id_name_option<'a>(
    input: &mut &'a str,
    inner_lines: &mut Vec<Either<&'a str, NationLine<'a>>>,
    desired_line: NationLine<'a>,
    value: &mut Option<MonsterIdOrName<'a>>,
) -> bool {
    let tag = desired_line.line_tag();

    if let Ok((remaining_input, found_id)) = parse_id_name_property(tag)(*input) {
        if let Some(old_id) = value.replace(found_id.into()) {
            panic!("Nation had duplicate {}: {:?}", tag, old_id); // fixme
        }
        *input = remaining_input;
        inner_lines.push(Right(desired_line));
        true // Ok(())
    } else {
        false // Err(())
    }
}

fn parse_monster_id_name_montag_option<'a>(
    input: &mut &'a str,
    inner_lines: &mut Vec<Either<&'a str, NationLine<'a>>>,
    desired_line: NationLine<'a>,
    value: &mut Option<MonsterIdOrMontagOrName<'a>>,
) -> bool {
    let tag = desired_line.line_tag();

    if let Ok((remaining_input, found_id)) = parse_id_name_montag_property(tag)(*input) {
        if let Some(old_id) = value.replace(found_id) {
            panic!("Nation had duplicate {}: {:?}", tag, old_id); // fixme
        }
        *input = remaining_input;
        inner_lines.push(Right(desired_line));
        true // Ok(())
    } else {
        false // Err(())
    }
}

fn parse_monster_id_name_list<'a>(
    input: &mut &'a str,
    inner_lines: &mut Vec<Either<&'a str, NationLine<'a>>>,
    desired_line: NationLine<'a>,
    values: &mut Vec<MonsterIdOrName<'a>>,
) -> bool {
    let tag = desired_line.line_tag();
    if let Ok((remaining_input, found_value)) = parse_id_name_property(tag)(*input) {
        values.push(found_value.into());
        *input = remaining_input;
        inner_lines.push(Right(desired_line));
        true // Ok(())
    } else {
        false // Err(())
    }
}

fn parse_monster_id_option<'a>(
    input: &mut &'a str,
    inner_lines: &mut Vec<Either<&'a str, NationLine<'a>>>,
    desired_line: NationLine<'a>,
    value: &mut Option<MonsterId>,
) -> bool {
    let tag = desired_line.line_tag();

    if let Ok((remaining_input, found_id)) = parse_id_property(tag)(*input) {
        if let Some(old_id) = value.replace(MonsterId(found_id)) {
            panic!("Nation had duplicate {}: {:?}", tag, old_id); // fixme
        }
        *input = remaining_input;
        inner_lines.push(Right(desired_line));
        true // Ok(())
    } else {
        false // Err(())
    }
}

fn parse_site_name_option<'a>(
    input: &mut &'a str,
    inner_lines: &mut Vec<Either<&'a str, NationLine<'a>>>,
    desired_line: NationLine<'a>,
    value: &mut Option<SiteName<'a>>,
) -> bool {
    let tag = desired_line.line_tag();
    if let Ok((remaining_input, found_value)) = parse_string_property(tag)(*input) {
        if let Some(old_value) = value.replace(SiteName(found_value)) {
            panic!("Nation had duplicate {}: {:?}", tag, old_value); // fixme
        }
        *input = remaining_input;
        inner_lines.push(Right(desired_line));
        true // Ok(())
    } else {
        false // Err(())
    }
}
