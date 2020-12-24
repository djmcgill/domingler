use either::{Either, Left, Right};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space0, space1}; // todo: inspect all usages of space0
use nom::combinator::{map, opt};
use nom::IResult;
use std::str::FromStr;

use crate::parser::*;

#[cfg(test)]
mod tests;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct MonsterId(pub u32);
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct MonsterName<'a>(&'a str);
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct MonsterIdOrMontag(pub i32);
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct MonsterIdOrMontagOrName<'a>(pub Either<MonsterIdOrMontag, MonsterName<'a>>);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct MonsterIdOrName<'a>(pub Either<MonsterId, MonsterName<'a>>);
impl<'a> From<Either<u32, &'a str>> for MonsterIdOrName<'a> {
    fn from(either: Either<u32, &'a str>) -> Self {
        match either {
            Either::Left(id) => MonsterIdOrName(Either::Left(MonsterId(id))),
            Either::Right(name) => MonsterIdOrName(Either::Right(MonsterName(name))),
        }
    }
}

pub fn parse_id_or_montag<'a>(
    input: &'a str,
) -> IResult<&'a str, MonsterIdOrMontag, VerboseError<&'a str>> {
    let (input, opt_minus) = opt(char('-'))(input)?;
    let (input, pos_number) = map_res(digit1, i32::from_str)(input)?;
    let number = if opt_minus.is_some() {
        pos_number * -1
    } else {
        pos_number
    };
    Ok((input, MonsterIdOrMontag(number)))
}

pub fn parse_id_name_montag_property<'a>(
    property: &'static str,
) -> impl Fn(&'a str) -> IResult<&'a str, MonsterIdOrMontagOrName, VerboseError<&'a str>> {
    move |input| {
        let (input, _) = tag(property)(input)?;
        let (input, _) = space1(input)?;
        let (input, monster_id) = alt((
            map(parse_name, |n| Either::Right(MonsterName(n))),
            map(parse_id_or_montag, Either::Left),
        ))(input)?;
        let (input, _) = parse_comment_line_end(input)?;
        Ok((input, MonsterIdOrMontagOrName(monster_id)))
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MonsterDeclaration<'a> {
    SelectId(MonsterId),
    SelectName(MonsterName<'a>),
    NewId(MonsterId),
    NewImplicit,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MonsterLine<'a> {
    Name,
    MonPresentRec,
    OwnsMonRec,
    CopyStats,
    CopySpr,
    Shapechange,
    ProphetShape,
    FirstShape,
    SecondShape,
    SecondTmpShape,
    ForestShape,
    PlainShape,
    ForeignShape,
    HomeShape,
    DomShape,
    NotDomShape,
    SpringShape,
    SummerShape,
    AutumnShape,
    WinterShape,
    LandShape,
    WaterShape,
    Twiceborn,
    // fixme: should I make this parsing more robust to check that the command doesn't continue after?
    DomSummon,
    DomSummon2,
    DomSummon20,
    RareDomSummon,
    TempleTrainer,
    MakeMonsters1,
    MakeMonsters2,
    MakeMonsters3,
    MakeMonsters4,
    MakeMonsters5,
    Summon1,
    Summon2,
    Summon3,
    Summon4,
    Summon5,
    BattleSum1,
    BattleSum2,
    BattleSum3,
    BattleSum4,
    BattleSum5,
    BatStartSum1,
    BatStartSum2,
    BatStartSum3,
    BatStartSum4,
    BatStartSum5,
    BatStartSum1d3,
    BatStartSum1d6,
    BatStartSum2d6,
    BatStartSum3d6,
    BatStartSum4d6,
    BatStartSum5d6,
    BatStartSum6d6,
    BatStartSum7d6,
    BatStartSum8d6,
    BatStartSum9d6,
    Slaver,
    RaiseShape,
    GrowHp,         // TODO: <hit points>
    ShrinkHp,       // TODO: <hit points>
    XpShape,        // TODO: <xp value>
    _Dummy(&'a ()), // never used
}

impl<'a> MonsterLine<'a> {
    pub fn line_tag(&self) -> &'static str {
        match self {
            MonsterLine::CopySpr => "#copyspr",
            MonsterLine::CopyStats => "#copystats",
            MonsterLine::Name => "#name",
            MonsterLine::OwnsMonRec => "#ownsmonrec",
            MonsterLine::MonPresentRec => "#monpresentrec",
            MonsterLine::RaiseShape => "#raiseshape",
            MonsterLine::Shapechange => "#shapechange",
            MonsterLine::ProphetShape => "#prophetshape",
            MonsterLine::FirstShape => "#firstshape",
            MonsterLine::SecondShape => "#secondshape",
            MonsterLine::SecondTmpShape => "#secondtmpshape",
            MonsterLine::ForestShape => "#forestshape",
            MonsterLine::PlainShape => "#plainshape",
            MonsterLine::ForeignShape => "#foreignshape",
            MonsterLine::HomeShape => "#homeshape",
            MonsterLine::DomShape => "#domshape",
            MonsterLine::NotDomShape => "#notdomshape",
            MonsterLine::SpringShape => "#springshape",
            MonsterLine::SummerShape => "#summershape",
            MonsterLine::AutumnShape => "#autumnshape",
            MonsterLine::WinterShape => "#wintershape",
            MonsterLine::LandShape => "#landshape",
            MonsterLine::WaterShape => "#watershape",
            MonsterLine::Twiceborn => "#twiceborn",
            MonsterLine::DomSummon => "#domsummon",
            MonsterLine::DomSummon2 => "#domsummon2",
            MonsterLine::DomSummon20 => "#domsummon20",
            MonsterLine::RareDomSummon => "#raredomsummon",
            MonsterLine::TempleTrainer => "#templetrainer",
            MonsterLine::MakeMonsters1 => "#makemonsters1",
            MonsterLine::MakeMonsters2 => "#makemonsters2",
            MonsterLine::MakeMonsters3 => "#makemonsters3",
            MonsterLine::MakeMonsters4 => "#makemonsters4",
            MonsterLine::MakeMonsters5 => "#makemonsters5",
            MonsterLine::Summon1 => "#summon1",
            MonsterLine::Summon2 => "#summon2",
            MonsterLine::Summon3 => "#summon3",
            MonsterLine::Summon4 => "#summon4",
            MonsterLine::Summon5 => "#summon5",
            MonsterLine::BattleSum1 => "#battlesum1",
            MonsterLine::BattleSum2 => "#battlesum2",
            MonsterLine::BattleSum3 => "#battlesum3",
            MonsterLine::BattleSum4 => "#battlesum4",
            MonsterLine::BattleSum5 => "#battlesum5",
            MonsterLine::BatStartSum1 => "#batstartsum1",
            MonsterLine::BatStartSum2 => "#batstartsum2",
            MonsterLine::BatStartSum3 => "#batstartsum3",
            MonsterLine::BatStartSum4 => "#batstartsum4",
            MonsterLine::BatStartSum5 => "#batstartsum5",
            MonsterLine::BatStartSum1d3 => "#batstartsum1d3",
            MonsterLine::BatStartSum1d6 => "#batstartsum1d6",
            MonsterLine::BatStartSum2d6 => "#batstartsum2d6",
            MonsterLine::BatStartSum3d6 => "#batstartsum3d6",
            MonsterLine::BatStartSum4d6 => "#batstartsum4d6",
            MonsterLine::BatStartSum5d6 => "#batstartsum5d6",
            MonsterLine::BatStartSum6d6 => "#batstartsum6d6",
            MonsterLine::BatStartSum7d6 => "#batstartsum7d6",
            MonsterLine::BatStartSum8d6 => "#batstartsum8d6",
            MonsterLine::BatStartSum9d6 => "#batstartsum9d6",
            MonsterLine::Slaver => "#slaver",
            MonsterLine::GrowHp => "#growhp",
            MonsterLine::ShrinkHp => "#shrinkhp",
            MonsterLine::XpShape => "#xpshape",
            MonsterLine::_Dummy(_) => unimplemented!(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Monster<'a> {
    pub declaration: MonsterDeclaration<'a>,
    pub name: Option<MonsterName<'a>>,
    pub copy_stats: Option<MonsterId>,
    pub copy_spr: Option<MonsterId>,

    pub mon_present_rec: Option<MonsterIdOrMontagOrName<'a>>,
    pub owns_mon_rec: Option<MonsterIdOrMontagOrName<'a>>,
    pub raise_shape: Option<MonsterIdOrMontagOrName<'a>>,
    pub shapechange: Option<MonsterIdOrMontagOrName<'a>>,
    pub prophet_shape: Option<MonsterIdOrMontagOrName<'a>>,
    pub first_shape: Option<MonsterIdOrMontagOrName<'a>>,
    pub second_shape: Option<MonsterIdOrMontagOrName<'a>>,
    pub second_tmp_shape: Option<MonsterIdOrMontagOrName<'a>>,
    pub forest_shape: Option<MonsterIdOrMontagOrName<'a>>,
    pub plain_shape: Option<MonsterIdOrMontagOrName<'a>>,
    pub foreign_shape: Option<MonsterIdOrMontagOrName<'a>>,
    pub home_shape: Option<MonsterIdOrMontagOrName<'a>>,
    pub dom_shape: Option<MonsterIdOrMontagOrName<'a>>,
    pub not_dom_shape: Option<MonsterIdOrMontagOrName<'a>>,
    pub spring_shape: Option<MonsterIdOrMontagOrName<'a>>,
    pub summer_shape: Option<MonsterIdOrMontagOrName<'a>>,
    pub autumn_shape: Option<MonsterIdOrMontagOrName<'a>>,
    pub winter_shape: Option<MonsterIdOrMontagOrName<'a>>,
    pub land_shape: Option<MonsterIdOrMontagOrName<'a>>,
    pub water_shape: Option<MonsterIdOrMontagOrName<'a>>,
    pub twiceborn: Option<MonsterIdOrMontagOrName<'a>>,
    pub dom_summon: Option<MonsterIdOrMontagOrName<'a>>,
    pub dom_summon_2: Option<MonsterIdOrMontagOrName<'a>>,
    pub dom_summon_20: Option<MonsterIdOrMontagOrName<'a>>,
    pub rare_dom_summon: Option<MonsterIdOrMontagOrName<'a>>,
    pub temple_trainer: Option<MonsterIdOrMontagOrName<'a>>,
    pub make_monsters_1: Option<MonsterIdOrMontagOrName<'a>>,
    pub make_monsters_2: Option<MonsterIdOrMontagOrName<'a>>,
    pub make_monsters_3: Option<MonsterIdOrMontagOrName<'a>>,
    pub make_monsters_4: Option<MonsterIdOrMontagOrName<'a>>,
    pub make_monsters_5: Option<MonsterIdOrMontagOrName<'a>>,
    pub summon_1: Option<MonsterIdOrMontagOrName<'a>>,
    pub summon_2: Option<MonsterIdOrMontagOrName<'a>>,
    pub summon_3: Option<MonsterIdOrMontagOrName<'a>>,
    pub summon_4: Option<MonsterIdOrMontagOrName<'a>>,
    pub summon_5: Option<MonsterIdOrMontagOrName<'a>>,
    pub battle_sum_1: Option<MonsterIdOrMontagOrName<'a>>,
    pub battle_sum_2: Option<MonsterIdOrMontagOrName<'a>>,
    pub battle_sum_3: Option<MonsterIdOrMontagOrName<'a>>,
    pub battle_sum_4: Option<MonsterIdOrMontagOrName<'a>>,
    pub battle_sum_5: Option<MonsterIdOrMontagOrName<'a>>,
    pub bat_start_sum_1: Option<MonsterIdOrMontagOrName<'a>>,
    pub bat_start_sum_2: Option<MonsterIdOrMontagOrName<'a>>,
    pub bat_start_sum_3: Option<MonsterIdOrMontagOrName<'a>>,
    pub bat_start_sum_4: Option<MonsterIdOrMontagOrName<'a>>,
    pub bat_start_sum_5: Option<MonsterIdOrMontagOrName<'a>>,
    pub bat_start_sum1d3: Option<MonsterIdOrMontagOrName<'a>>,
    pub bat_start_sum1d6: Option<MonsterIdOrMontagOrName<'a>>,
    pub bat_start_sum2d6: Option<MonsterIdOrMontagOrName<'a>>,
    pub bat_start_sum3d6: Option<MonsterIdOrMontagOrName<'a>>,
    pub bat_start_sum4d6: Option<MonsterIdOrMontagOrName<'a>>,
    pub bat_start_sum5d6: Option<MonsterIdOrMontagOrName<'a>>,
    pub bat_start_sum6d6: Option<MonsterIdOrMontagOrName<'a>>,
    pub bat_start_sum7d6: Option<MonsterIdOrMontagOrName<'a>>,
    pub bat_start_sum8d6: Option<MonsterIdOrMontagOrName<'a>>,
    pub bat_start_sum9d6: Option<MonsterIdOrMontagOrName<'a>>,
    pub slaver: Option<MonsterIdOrMontagOrName<'a>>,

    /// This field does not contain the declaration or the end
    pub inner_lines: Vec<Either<&'a str, MonsterLine<'a>>>,
}
impl<'a> Monster<'a> {
    pub fn referenced_monster_ids_and_names(
        &self,
    ) -> (Vec<MonsterIdOrMontag>, Vec<MonsterName>, bool, bool) {
        // TODO: these should be sets instead
        let mut ids = vec![];
        let mut names = vec![];

        // okay this is pretty hacky
        let mut next = false;
        let mut previous = false;
        for line in &self.inner_lines {
            // FIXME: very hacky!!
            match line {
                Right(MonsterLine::GrowHp) => previous = true,
                Right(MonsterLine::XpShape) => next = true,
                Right(MonsterLine::ShrinkHp) => next = true,
                _ => {}
            }
        }

        for id in [self.copy_stats, self.copy_spr].iter() {
            reference_id(&mut ids, id);
        }
        for id_or_name in [
            self.mon_present_rec,
            self.owns_mon_rec,
            self.shapechange,
            self.raise_shape,
            self.prophet_shape,
            self.first_shape,
            self.second_shape,
            self.second_tmp_shape,
            self.forest_shape,
            self.plain_shape,
            self.foreign_shape,
            self.home_shape,
            self.dom_shape,
            self.not_dom_shape,
            self.spring_shape,
            self.summer_shape,
            self.autumn_shape,
            self.winter_shape,
            self.land_shape,
            self.water_shape,
            self.twiceborn,
            self.dom_summon,
            self.dom_summon_2,
            self.dom_summon_20,
            self.rare_dom_summon,
            self.temple_trainer,
            self.make_monsters_1,
            self.make_monsters_2,
            self.make_monsters_3,
            self.make_monsters_4,
            self.make_monsters_5,
            self.summon_1,
            self.summon_2,
            self.summon_3,
            self.summon_4,
            self.summon_5,
            self.battle_sum_1,
            self.battle_sum_2,
            self.battle_sum_3,
            self.battle_sum_4,
            self.battle_sum_5,
            self.bat_start_sum_1,
            self.bat_start_sum_2,
            self.bat_start_sum_3,
            self.bat_start_sum_4,
            self.bat_start_sum_5,
            self.bat_start_sum1d3,
            self.bat_start_sum1d6,
            self.bat_start_sum2d6,
            self.bat_start_sum3d6,
            self.bat_start_sum4d6,
            self.bat_start_sum5d6,
            self.bat_start_sum6d6,
            self.bat_start_sum7d6,
            self.bat_start_sum8d6,
            self.bat_start_sum9d6,
            self.slaver,
        ]
        .iter()
        {
            reference_id_or_name(&mut ids, &mut names, id_or_name.as_ref().map(|x| &x.0));
        }
        (ids, names, next, previous)
    }
}

fn reference_id(ids: &mut Vec<MonsterIdOrMontag>, opt_id: &Option<MonsterId>) {
    if let Some(id) = opt_id.as_ref() {
        ids.push(MonsterIdOrMontag(id.0 as i32));
    }
}

fn reference_id_or_name<A: Copy, B: Copy>(
    ids: &mut Vec<A>,
    names: &mut Vec<B>,
    opt_id_or_name: Option<&Either<A, B>>,
) {
    if let Some(id_or_name) = opt_id_or_name {
        match id_or_name {
            Left(id) => ids.push(*id),
            Right(name) => names.push(*name),
        }
    }
}

fn parse_select_monster<'a>(
    input: &'a str,
) -> IResult<&'a str, MonsterDeclaration<'a>, VerboseError<&'a str>> {
    let (input, _) = tag("#selectmonster")(input)?;
    let (input, _) = space0(input)?;
    let (input, either_id_name) = alt((
        map(parse_id, |id| MonsterDeclaration::SelectId(MonsterId(id))),
        map(parse_name, |name| {
            MonsterDeclaration::SelectName(MonsterName(name))
        }),
    ))(input)?;

    Ok((input, either_id_name))
}

fn parse_new_monster<'a>(
    input: &'a str,
) -> IResult<&'a str, MonsterDeclaration<'a>, VerboseError<&'a str>> {
    let (input, _) = tag("#newmonster")(input)?;
    let (input, _) = space0(input)?;
    let (input, opt_id_str) = opt(digit1)(input)?;
    let opt_id = opt_id_str.map(|id_str| {
        u32::from_str(id_str)
            .unwrap_or_else(|_| panic!("could not parse valid integer id from '{}'", id_str))
    }); // FIXME

    let declaration = match opt_id {
        Some(id) => MonsterDeclaration::NewId(MonsterId(id)),
        None => MonsterDeclaration::NewImplicit,
    };

    Ok((input, declaration))
}

fn parse_monster_declaration<'a>(
    input: &'a str,
) -> IResult<&'a str, MonsterDeclaration<'a>, VerboseError<&'a str>> {
    let (input, _) = space0(input)?;

    let (input, declaration) = alt((parse_new_monster, parse_select_monster))(input)?;
    let (input, _) = parse_comment_line_end(input)?;

    Ok((input, declaration))
}

pub fn parse_monster<'a>(input: &'a str) -> IResult<&'a str, Monster<'a>, VerboseError<&'a str>> {
    let (input, declaration) = parse_monster_declaration(input)?;
    let mut input = input;
    let mut inner_lines = vec![];
    let mut name = None;
    let mut copy_stats = None;
    let mut copy_spr = None;
    let mut mon_present_rec = None;
    let mut owns_mon_rec = None;
    let mut raise_shape = None;
    let mut shapechange = None;
    let mut prophet_shape = None;
    let mut first_shape = None;
    let mut second_shape = None;
    let mut second_tmp_shape = None;
    let mut forest_shape = None;
    let mut plain_shape = None;
    let mut foreign_shape = None;
    let mut home_shape = None;
    let mut dom_shape = None;
    let mut not_dom_shape = None;
    let mut spring_shape = None;
    let mut summer_shape = None;
    let mut autumn_shape = None;
    let mut winter_shape = None;
    let mut land_shape = None;
    let mut water_shape = None;
    let mut twiceborn = None;
    let mut dom_summon = None;
    let mut dom_summon_2 = None;
    let mut dom_summon_20 = None;
    let mut rare_dom_summon = None;
    let mut temple_trainer = None;
    let mut make_monsters_1 = None;
    let mut make_monsters_2 = None;
    let mut make_monsters_3 = None;
    let mut make_monsters_4 = None;
    let mut make_monsters_5 = None;
    let mut summon_1 = None;
    let mut summon_2 = None;
    let mut summon_3 = None;
    let mut summon_4 = None;
    let mut summon_5 = None;
    let mut battle_sum_1 = None;
    let mut battle_sum_2 = None;
    let mut battle_sum_3 = None;
    let mut battle_sum_4 = None;
    let mut battle_sum_5 = None;
    let mut bat_start_sum_1 = None;
    let mut bat_start_sum_2 = None;
    let mut bat_start_sum_3 = None;
    let mut bat_start_sum_4 = None;
    let mut bat_start_sum_5 = None;
    let mut bat_start_sum1d3 = None;
    let mut bat_start_sum1d6 = None;
    let mut bat_start_sum2d6 = None;
    let mut bat_start_sum3d6 = None;
    let mut bat_start_sum4d6 = None;
    let mut bat_start_sum5d6 = None;
    let mut bat_start_sum6d6 = None;
    let mut bat_start_sum7d6 = None;
    let mut bat_start_sum8d6 = None;
    let mut bat_start_sum9d6 = None;
    let mut slaver = None;

    loop {
        if let Ok((remaining_input, found_name)) = parse_string_property("#name")(input) {
            if let Some(old_name) = name.replace(MonsterName(found_name)) {
                panic!("Monster had duplicate #name: {:?}", old_name); // fixme
            }
            input = remaining_input;
            inner_lines.push(Right(MonsterLine::Name));
        // this can't be the best way to do it
        } else if parse_id_name_montag_line(
            &mut input,
            &mut inner_lines,
            MonsterLine::OwnsMonRec,
            &mut owns_mon_rec,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line(
            &mut input,
            &mut inner_lines,
            MonsterLine::MonPresentRec,
            &mut mon_present_rec,
        )
        .is_ok()
        {
        } else if let Ok((remaining_input, found_id)) =
            parse_id_property(MonsterLine::CopyStats.line_tag())(input)
        {
            if let Some(old_id) = copy_stats.replace(MonsterId(found_id)) {
                panic!("Monster had duplicate #copystats: {:?}", old_id); // fixme
            }
            input = remaining_input;
            inner_lines.push(Right(MonsterLine::CopyStats));
        } else if let Ok((remaining_input, found_id)) =
            parse_id_property(MonsterLine::CopySpr.line_tag())(input)
        {
            if let Some(old_id) = copy_spr.replace(MonsterId(found_id)) {
                panic!("Monster had duplicate #copyspr: {:?}", old_id); // fixme: handle errors properly
            }
            input = remaining_input;
            inner_lines.push(Right(MonsterLine::CopySpr));
        } else if parse_id_name_montag_line(
            &mut input,
            &mut inner_lines,
            MonsterLine::RaiseShape,
            &mut raise_shape,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line(
            &mut input,
            &mut inner_lines,
            MonsterLine::Shapechange,
            &mut shapechange,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line(
            &mut input,
            &mut inner_lines,
            MonsterLine::ProphetShape,
            &mut prophet_shape,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line(
            &mut input,
            &mut inner_lines,
            MonsterLine::FirstShape,
            &mut first_shape,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line(
            &mut input,
            &mut inner_lines,
            MonsterLine::SecondShape,
            &mut second_shape,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line(
            &mut input,
            &mut inner_lines,
            MonsterLine::SecondTmpShape,
            &mut second_tmp_shape,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line(
            &mut input,
            &mut inner_lines,
            MonsterLine::ForestShape,
            &mut forest_shape,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line(
            &mut input,
            &mut inner_lines,
            MonsterLine::PlainShape,
            &mut plain_shape,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line(
            &mut input,
            &mut inner_lines,
            MonsterLine::ForeignShape,
            &mut foreign_shape,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line(
            &mut input,
            &mut inner_lines,
            MonsterLine::HomeShape,
            &mut home_shape,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line(
            &mut input,
            &mut inner_lines,
            MonsterLine::DomShape,
            &mut dom_shape,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line(
            &mut input,
            &mut inner_lines,
            MonsterLine::NotDomShape,
            &mut not_dom_shape,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line(
            &mut input,
            &mut inner_lines,
            MonsterLine::SpringShape,
            &mut spring_shape,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line(
            &mut input,
            &mut inner_lines,
            MonsterLine::SummerShape,
            &mut summer_shape,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line(
            &mut input,
            &mut inner_lines,
            MonsterLine::AutumnShape,
            &mut autumn_shape,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line(
            &mut input,
            &mut inner_lines,
            MonsterLine::WinterShape,
            &mut winter_shape,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line(
            &mut input,
            &mut inner_lines,
            MonsterLine::LandShape,
            &mut land_shape,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line(
            &mut input,
            &mut inner_lines,
            MonsterLine::WaterShape,
            &mut water_shape,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line(
            &mut input,
            &mut inner_lines,
            MonsterLine::Twiceborn,
            &mut twiceborn,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line(
            &mut input,
            &mut inner_lines,
            MonsterLine::DomSummon,
            &mut dom_summon,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line(
            &mut input,
            &mut inner_lines,
            MonsterLine::DomSummon2,
            &mut dom_summon_2,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line(
            &mut input,
            &mut inner_lines,
            MonsterLine::DomSummon20,
            &mut dom_summon_20,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line(
            &mut input,
            &mut inner_lines,
            MonsterLine::RareDomSummon,
            &mut rare_dom_summon,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line(
            &mut input,
            &mut inner_lines,
            MonsterLine::TempleTrainer,
            &mut temple_trainer,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line(
            &mut input,
            &mut inner_lines,
            MonsterLine::MakeMonsters1,
            &mut make_monsters_1,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line(
            &mut input,
            &mut inner_lines,
            MonsterLine::MakeMonsters2,
            &mut make_monsters_2,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line(
            &mut input,
            &mut inner_lines,
            MonsterLine::MakeMonsters3,
            &mut make_monsters_3,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line(
            &mut input,
            &mut inner_lines,
            MonsterLine::MakeMonsters4,
            &mut make_monsters_4,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line(
            &mut input,
            &mut inner_lines,
            MonsterLine::MakeMonsters5,
            &mut make_monsters_5,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line(
            &mut input,
            &mut inner_lines,
            MonsterLine::Summon1,
            &mut summon_1,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line(
            &mut input,
            &mut inner_lines,
            MonsterLine::Summon2,
            &mut summon_2,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line(
            &mut input,
            &mut inner_lines,
            MonsterLine::Summon3,
            &mut summon_3,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line(
            &mut input,
            &mut inner_lines,
            MonsterLine::Summon4,
            &mut summon_4,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line(
            &mut input,
            &mut inner_lines,
            MonsterLine::Summon5,
            &mut summon_5,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line(
            &mut input,
            &mut inner_lines,
            MonsterLine::BattleSum1,
            &mut battle_sum_1,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line(
            &mut input,
            &mut inner_lines,
            MonsterLine::BattleSum2,
            &mut battle_sum_2,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line(
            &mut input,
            &mut inner_lines,
            MonsterLine::BattleSum3,
            &mut battle_sum_3,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line(
            &mut input,
            &mut inner_lines,
            MonsterLine::BattleSum4,
            &mut battle_sum_4,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line(
            &mut input,
            &mut inner_lines,
            MonsterLine::BattleSum5,
            &mut battle_sum_5,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line(
            &mut input,
            &mut inner_lines,
            MonsterLine::BatStartSum1,
            &mut bat_start_sum_1,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line(
            &mut input,
            &mut inner_lines,
            MonsterLine::BatStartSum2,
            &mut bat_start_sum_2,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line(
            &mut input,
            &mut inner_lines,
            MonsterLine::BatStartSum3,
            &mut bat_start_sum_3,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line(
            &mut input,
            &mut inner_lines,
            MonsterLine::BatStartSum4,
            &mut bat_start_sum_4,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line(
            &mut input,
            &mut inner_lines,
            MonsterLine::BatStartSum5,
            &mut bat_start_sum_5,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line(
            &mut input,
            &mut inner_lines,
            MonsterLine::BatStartSum1d3,
            &mut bat_start_sum1d3,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line(
            &mut input,
            &mut inner_lines,
            MonsterLine::BatStartSum1d6,
            &mut bat_start_sum1d6,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line(
            &mut input,
            &mut inner_lines,
            MonsterLine::BatStartSum2d6,
            &mut bat_start_sum2d6,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line(
            &mut input,
            &mut inner_lines,
            MonsterLine::BatStartSum3d6,
            &mut bat_start_sum3d6,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line(
            &mut input,
            &mut inner_lines,
            MonsterLine::BatStartSum4d6,
            &mut bat_start_sum4d6,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line(
            &mut input,
            &mut inner_lines,
            MonsterLine::BatStartSum5d6,
            &mut bat_start_sum5d6,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line(
            &mut input,
            &mut inner_lines,
            MonsterLine::BatStartSum6d6,
            &mut bat_start_sum6d6,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line(
            &mut input,
            &mut inner_lines,
            MonsterLine::BatStartSum7d6,
            &mut bat_start_sum7d6,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line(
            &mut input,
            &mut inner_lines,
            MonsterLine::BatStartSum8d6,
            &mut bat_start_sum8d6,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line(
            &mut input,
            &mut inner_lines,
            MonsterLine::BatStartSum9d6,
            &mut bat_start_sum9d6,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line(
            &mut input,
            &mut inner_lines,
            MonsterLine::Slaver,
            &mut slaver,
        )
        .is_ok()
        {
        } else if let Ok((remaining_input, _)) =
            tag::<_, _, VerboseError<&'a str>>("#growhp")(input)
        {
            input = remaining_input;
            inner_lines.push(Right(MonsterLine::GrowHp));
        } else if let Ok((remaining_input, _)) =
            tag::<_, _, VerboseError<&'a str>>("#shrinkhp")(input)
        {
            input = remaining_input;
            inner_lines.push(Right(MonsterLine::ShrinkHp));
        } else if let Ok((remaining_input, _)) =
            tag::<_, _, VerboseError<&'a str>>("#xpshape")(input)
        {
            input = remaining_input;
            inner_lines.push(Right(MonsterLine::XpShape));

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

    let monster = Monster {
        declaration,
        name,
        copy_stats,
        copy_spr,
        mon_present_rec,
        owns_mon_rec,
        raise_shape,
        shapechange,
        prophet_shape,
        first_shape,
        second_shape,
        second_tmp_shape,
        forest_shape,
        plain_shape,
        foreign_shape,
        home_shape,
        dom_shape,
        not_dom_shape,
        spring_shape,
        summer_shape,
        autumn_shape,
        winter_shape,
        land_shape,
        water_shape,
        twiceborn,
        dom_summon,
        dom_summon_2,
        dom_summon_20,
        rare_dom_summon,
        temple_trainer,
        make_monsters_1,
        make_monsters_2,
        make_monsters_3,
        make_monsters_4,
        make_monsters_5,
        summon_1,
        summon_2,
        summon_3,
        summon_4,
        summon_5,
        battle_sum_1,
        battle_sum_2,
        battle_sum_3,
        battle_sum_4,
        battle_sum_5,
        bat_start_sum_1,
        bat_start_sum_2,
        bat_start_sum_3,
        bat_start_sum_4,
        bat_start_sum_5,
        bat_start_sum1d3,
        bat_start_sum1d6,
        bat_start_sum2d6,
        bat_start_sum3d6,
        bat_start_sum4d6,
        bat_start_sum5d6,
        bat_start_sum6d6,
        bat_start_sum7d6,
        bat_start_sum8d6,
        bat_start_sum9d6,
        slaver,
        inner_lines,
    };

    Ok((input, monster))
}

fn parse_id_name_montag_line<'a>(
    input: &mut &'a str,
    inner_lines: &mut Vec<Either<&'a str, MonsterLine<'a>>>,
    desired_line: MonsterLine<'a>,
    value: &mut Option<MonsterIdOrMontagOrName<'a>>,
) -> Result<(), ()> {
    let tag = desired_line.line_tag();
    match parse_id_name_montag_property(tag)(*input) {
        Err(_e) => Err(()), // TODO
        Ok((remaining_input, found_id)) => {
            if let Some(old_id) = value.replace(found_id) {
                panic!("Monster had duplicate {}: {:?}", tag, old_id); // fixme
            }
            *input = remaining_input;
            inner_lines.push(Right(desired_line));
            Ok(())
        }
    }
}

/*


TODO:
#growhp <hit points>
The monster grows into the previous monster once it has this many hit points or more. Hydras use this mechanic.

#shrinkhp <hit points>
The monster shrinks to the following monster once it has this many hit points or less. Hydras use this mechanic.

#xpshape <xp value>
The monster will change into the next monster type (next monster number) after reaching this amount of xp.

*/
