use either::{Either, Left, Right};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space0};
use nom::combinator::{map, opt};
use nom::error::ParseError;
use nom::IResult;
use std::str::FromStr;

use crate::parser::{
    parse_comment_line_end, parse_id, parse_id_name_montag_property, parse_id_property, parse_name,
    parse_string_property, parse_unparsed_line,
};

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
    GrowHp, // TODO: <hit points>
    ShrinkHp, // TODO: <hit points>
    XpShape, // TODO: <xp value>
    Dummy(&'a ()), // never used

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
            MonsterLine::Dummy(_) => unimplemented!(),
        }
    }
}


#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Monster<'a> {
    pub declaration: MonsterDeclaration<'a>,
    pub name: Option<&'a str>,
    pub copy_stats: Option<u32>,
    pub copy_spr: Option<u32>,

    pub mon_present_rec: Option<Either<i32, &'a str>>,
    pub owns_mon_rec: Option<Either<i32, &'a str>>,
    pub raise_shape: Option<Either<i32, &'a str>>,
    pub shapechange: Option<Either<i32, &'a str>>,
    pub prophet_shape: Option<Either<i32, &'a str>>,
    pub first_shape: Option<Either<i32, &'a str>>,
    pub second_shape: Option<Either<i32, &'a str>>,
    pub second_tmp_shape: Option<Either<i32, &'a str>>,
    pub forest_shape: Option<Either<i32, &'a str>>,
    pub plain_shape: Option<Either<i32, &'a str>>,
    pub foreign_shape: Option<Either<i32, &'a str>>,
    pub home_shape: Option<Either<i32, &'a str>>,
    pub dom_shape: Option<Either<i32, &'a str>>,
    pub not_dom_shape: Option<Either<i32, &'a str>>,
    pub spring_shape: Option<Either<i32, &'a str>>,
    pub summer_shape: Option<Either<i32, &'a str>>,
    pub autumn_shape: Option<Either<i32, &'a str>>,
    pub winter_shape: Option<Either<i32, &'a str>>,
    pub land_shape: Option<Either<i32, &'a str>>,
    pub water_shape: Option<Either<i32, &'a str>>,
    pub twiceborn: Option<Either<i32, &'a str>>,
    pub dom_summon: Option<Either<i32, &'a str>>,
    pub dom_summon_2: Option<Either<i32, &'a str>>,
    pub dom_summon_20: Option<Either<i32, &'a str>>,
    pub rare_dom_summon: Option<Either<i32, &'a str>>,
    pub temple_trainer: Option<Either<i32, &'a str>>,
    pub make_monsters_1: Option<Either<i32, &'a str>>,
    pub make_monsters_2: Option<Either<i32, &'a str>>,
    pub make_monsters_3: Option<Either<i32, &'a str>>,
    pub make_monsters_4: Option<Either<i32, &'a str>>,
    pub make_monsters_5: Option<Either<i32, &'a str>>,
    pub summon_1: Option<Either<i32, &'a str>>,
    pub summon_2: Option<Either<i32, &'a str>>,
    pub summon_3: Option<Either<i32, &'a str>>,
    pub summon_4: Option<Either<i32, &'a str>>,
    pub summon_5: Option<Either<i32, &'a str>>,
    pub battle_sum_1: Option<Either<i32, &'a str>>,
    pub battle_sum_2: Option<Either<i32, &'a str>>,
    pub battle_sum_3: Option<Either<i32, &'a str>>,
    pub battle_sum_4: Option<Either<i32, &'a str>>,
    pub battle_sum_5: Option<Either<i32, &'a str>>,
    pub bat_start_sum_1: Option<Either<i32, &'a str>>,
    pub bat_start_sum_2: Option<Either<i32, &'a str>>,
    pub bat_start_sum_3: Option<Either<i32, &'a str>>,
    pub bat_start_sum_4: Option<Either<i32, &'a str>>,
    pub bat_start_sum_5: Option<Either<i32, &'a str>>,
    pub bat_start_sum1d3: Option<Either<i32, &'a str>>,
    pub bat_start_sum1d6: Option<Either<i32, &'a str>>,
    pub bat_start_sum2d6: Option<Either<i32, &'a str>>,
    pub bat_start_sum3d6: Option<Either<i32, &'a str>>,
    pub bat_start_sum4d6: Option<Either<i32, &'a str>>,
    pub bat_start_sum5d6: Option<Either<i32, &'a str>>,
    pub bat_start_sum6d6: Option<Either<i32, &'a str>>,
    pub bat_start_sum7d6: Option<Either<i32, &'a str>>,
    pub bat_start_sum8d6: Option<Either<i32, &'a str>>,
    pub bat_start_sum9d6: Option<Either<i32, &'a str>>,
    pub slaver: Option<Either<i32, &'a str>>,

    /// This field does not contain the declaration or the end
    pub inner_lines: Vec<Either<&'a str, MonsterLine<'a>>>,
}
impl<'a> Monster<'a> {
    pub fn default_from_declaration(declaration: MonsterDeclaration<'a>) -> Self {
        Monster {
            declaration,
            name: None,
            copy_stats: None,
            copy_spr: None,
            mon_present_rec: None,
            owns_mon_rec: None,
            raise_shape: None,
            shapechange: None,
            prophet_shape: None,
            first_shape: None,
            second_shape: None,
            second_tmp_shape: None,
            forest_shape: None,
            plain_shape: None,
            foreign_shape: None,
            home_shape: None,
            dom_shape: None,
            not_dom_shape: None,
            spring_shape: None,
            summer_shape: None,
            autumn_shape: None,
            winter_shape: None,
            land_shape: None,
            water_shape: None,
            twiceborn: None,
            dom_summon: None,
            dom_summon_2: None,
            dom_summon_20: None,
            rare_dom_summon: None,
            temple_trainer: None,
            make_monsters_1: None,
            make_monsters_2: None,
            make_monsters_3: None,
            make_monsters_4: None,
            make_monsters_5: None,
            summon_1: None,
            summon_2: None,
            summon_3: None,
            summon_4: None,
            summon_5: None,
            battle_sum_1: None,
            battle_sum_2: None,
            battle_sum_3: None,
            battle_sum_4: None,
            battle_sum_5: None,
            bat_start_sum_1: None,
            bat_start_sum_2: None,
            bat_start_sum_3: None,
            bat_start_sum_4: None,
            bat_start_sum_5: None,
            bat_start_sum1d3: None,
            bat_start_sum1d6: None,
            bat_start_sum2d6: None,
            bat_start_sum3d6: None,
            bat_start_sum4d6: None,
            bat_start_sum5d6: None,
            bat_start_sum6d6: None,
            bat_start_sum7d6: None,
            bat_start_sum8d6: None,
            bat_start_sum9d6: None,
            slaver: None,
            inner_lines: vec![],
        }
    }
    pub fn referenced_monster_ids_and_names(&self) -> (Vec<i32>, Vec<&'a str>, bool, bool) {
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
            reference_id_or_name(&mut ids, &mut names, id_or_name);
        }
        (ids, names, next, previous)
    }
}

fn reference_id(ids: &mut Vec<i32>, opt_id: &Option<u32>) {
    if let Some(id) = opt_id.as_ref() {
        ids.push(*id as i32);
    }
}

fn reference_id_or_name<A: Copy, B: Copy>(
    ids: &mut Vec<A>,
    names: &mut Vec<B>,
    opt_id_or_name: &Option<Either<A, B>>,
) {
    if let Some(id_or_name) = opt_id_or_name.as_ref() {
        match id_or_name {
            Left(id) => ids.push(*id),
            Right(name) => names.push(*name),
        }
    }
}

fn parse_select_monster<'a, E: ParseError<&'a str>>(
    input: &'a str,
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
    input: &'a str,
) -> IResult<&'a str, MonsterDeclaration<'a>, E> {
    let (input, _) = tag("#newmonster")(input)?;
    let (input, _) = space0(input)?;
    let (input, opt_id_str) = opt(digit1)(input)?;
    let opt_id = opt_id_str.map(|id_str| {
        u32::from_str(id_str)
            .unwrap_or_else(|_| panic!("could not parse valid integer id from '{}'", id_str))
    }); // FIXME

    let declaration = match opt_id {
        Some(id) => MonsterDeclaration::NewId(id),
        None => MonsterDeclaration::NewImplicit,
    };

    Ok((input, declaration))
}

fn parse_monster_declaration<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, MonsterDeclaration<'a>, E> {
    let (input, _) = space0(input)?;

    let (input, declaration) = alt((parse_new_monster, parse_select_monster))(input)?;
    let (input, _) = parse_comment_line_end(input)?;

    Ok((input, declaration))
}

pub fn parse_monster<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, Monster<'a>, E> {
    let (input, declaration) = parse_monster_declaration(input)?;
    let mut monster = Monster::default_from_declaration(declaration);
    let mut input = input;

    loop {
        if let Ok((remaining_input, found_name)) = parse_string_property::<E>("#name")(input) {
            if let Some(old_name) = monster.name.replace(found_name) {
                panic!("Monster had duplicate #name: {}", old_name); // fixme
            }
            input = remaining_input;
            monster.inner_lines.push(Right(MonsterLine::Name));
        // this can't be the best way to do it
        } else if parse_id_name_montag_line::<E>(
            &mut input,
            &mut monster.inner_lines,
            MonsterLine::OwnsMonRec,
            &mut monster.owns_mon_rec,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line::<E>(
            &mut input,
            &mut monster.inner_lines,
            MonsterLine::MonPresentRec,
            &mut monster.mon_present_rec,
        )
        .is_ok()
        {
        } else if let Ok((remaining_input, found_id)) = parse_id_property::<E>(MonsterLine::CopyStats.line_tag())(input)
        {
            if let Some(old_id) = monster.copy_stats.replace(found_id) {
                panic!("Monster had duplicate #copystats: {}", old_id); // fixme
            }
            input = remaining_input;
            monster.inner_lines.push(Right(MonsterLine::CopyStats));
        } else if let Ok((remaining_input, found_id)) = parse_id_property::<E>(MonsterLine::CopySpr.line_tag())(input) {
            if let Some(old_id) = monster.copy_spr.replace(found_id) {
                panic!("Monster had duplicate #copyspr: {}", old_id); // fixme: handle errors properly
            }
            input = remaining_input;
            monster.inner_lines.push(Right(MonsterLine::CopySpr));
        } else if parse_id_name_montag_line::<E>(
            &mut input,
            &mut monster.inner_lines,
            MonsterLine::RaiseShape,
            &mut monster.raise_shape,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line::<E>(
            &mut input,
            &mut monster.inner_lines,
            MonsterLine::Shapechange,
            &mut monster.shapechange,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line::<E>(
            &mut input,
            &mut monster.inner_lines,
            MonsterLine::ProphetShape,
            &mut monster.prophet_shape,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line::<E>(
            &mut input,
            &mut monster.inner_lines,
            MonsterLine::FirstShape,
            &mut monster.first_shape,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line::<E>(
            &mut input,
            &mut monster.inner_lines,
            MonsterLine::SecondShape,
            &mut monster.second_shape,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line::<E>(
            &mut input,
            &mut monster.inner_lines,
            MonsterLine::SecondTmpShape,
            &mut monster.second_tmp_shape,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line::<E>(
            &mut input,
            &mut monster.inner_lines,
            MonsterLine::ForestShape,
            &mut monster.forest_shape,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line::<E>(
            &mut input,
            &mut monster.inner_lines,
            MonsterLine::PlainShape,
            &mut monster.plain_shape,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line::<E>(
            &mut input,
            &mut monster.inner_lines,
            MonsterLine::ForeignShape,
            &mut monster.foreign_shape,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line::<E>(
            &mut input,
            &mut monster.inner_lines,
            MonsterLine::HomeShape,
            &mut monster.home_shape,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line::<E>(
            &mut input,
            &mut monster.inner_lines,
            MonsterLine::DomShape,
            &mut monster.dom_shape,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line::<E>(
            &mut input,
            &mut monster.inner_lines,
            MonsterLine::NotDomShape,
            &mut monster.not_dom_shape,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line::<E>(
            &mut input,
            &mut monster.inner_lines,
            MonsterLine::SpringShape,
            &mut monster.spring_shape,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line::<E>(
            &mut input,
            &mut monster.inner_lines,
            MonsterLine::SummerShape,
            &mut monster.summer_shape,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line::<E>(
            &mut input,
            &mut monster.inner_lines,
            MonsterLine::AutumnShape,
            &mut monster.autumn_shape,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line::<E>(
            &mut input,
            &mut monster.inner_lines,
            MonsterLine::WinterShape,
            &mut monster.winter_shape,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line::<E>(
            &mut input,
            &mut monster.inner_lines,
            MonsterLine::LandShape,
            &mut monster.land_shape,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line::<E>(
            &mut input,
            &mut monster.inner_lines,
            MonsterLine::WaterShape,
            &mut monster.water_shape,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line::<E>(
            &mut input,
            &mut monster.inner_lines,
            MonsterLine::Twiceborn,
            &mut monster.twiceborn,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line::<E>(
            &mut input,
            &mut monster.inner_lines,
            MonsterLine::DomSummon,
            &mut monster.dom_summon,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line::<E>(
            &mut input,
            &mut monster.inner_lines,
            MonsterLine::DomSummon2,
            &mut monster.dom_summon_2,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line::<E>(
            &mut input,
            &mut monster.inner_lines,
            MonsterLine::DomSummon20,
            &mut monster.dom_summon_20,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line::<E>(
            &mut input,
            &mut monster.inner_lines,
            MonsterLine::RareDomSummon,
            &mut monster.rare_dom_summon,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line::<E>(
            &mut input,
            &mut monster.inner_lines,
            MonsterLine::TempleTrainer,
            &mut monster.temple_trainer,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line::<E>(
            &mut input,
            &mut monster.inner_lines,
            MonsterLine::MakeMonsters1,
            &mut monster.make_monsters_1,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line::<E>(
            &mut input,
            &mut monster.inner_lines,
            MonsterLine::MakeMonsters2,
            &mut monster.make_monsters_2,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line::<E>(
            &mut input,
            &mut monster.inner_lines,
            MonsterLine::MakeMonsters3,
            &mut monster.make_monsters_3,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line::<E>(
            &mut input,
            &mut monster.inner_lines,
            MonsterLine::MakeMonsters4,
            &mut monster.make_monsters_4,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line::<E>(
            &mut input,
            &mut monster.inner_lines,
            MonsterLine::MakeMonsters5,
            &mut monster.make_monsters_5,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line::<E>(
            &mut input,
            &mut monster.inner_lines,
            MonsterLine::Summon1,
            &mut monster.summon_1,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line::<E>(
            &mut input,
            &mut monster.inner_lines,
            MonsterLine::Summon2,
            &mut monster.summon_2,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line::<E>(
            &mut input,
            &mut monster.inner_lines,
            MonsterLine::Summon3,
            &mut monster.summon_3,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line::<E>(
            &mut input,
            &mut monster.inner_lines,
            MonsterLine::Summon4,
            &mut monster.summon_4,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line::<E>(
            &mut input,
            &mut monster.inner_lines,
            MonsterLine::Summon5,
            &mut monster.summon_5,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line::<E>(
            &mut input,
            &mut monster.inner_lines,
            MonsterLine::BattleSum1,
            &mut monster.battle_sum_1,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line::<E>(
            &mut input,
            &mut monster.inner_lines,
            MonsterLine::BattleSum2,
            &mut monster.battle_sum_2,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line::<E>(
            &mut input,
            &mut monster.inner_lines,
            MonsterLine::BattleSum3,
            &mut monster.battle_sum_3,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line::<E>(
            &mut input,
            &mut monster.inner_lines,
            MonsterLine::BattleSum4,
            &mut monster.battle_sum_4,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line::<E>(
            &mut input,
            &mut monster.inner_lines,
            MonsterLine::BattleSum5,
            &mut monster.battle_sum_5,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line::<E>(
            &mut input,
            &mut monster.inner_lines,
            MonsterLine::BatStartSum1,
            &mut monster.bat_start_sum_1,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line::<E>(
            &mut input,
            &mut monster.inner_lines,
            MonsterLine::BatStartSum2,
            &mut monster.bat_start_sum_2,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line::<E>(
            &mut input,
            &mut monster.inner_lines,
            MonsterLine::BatStartSum3,
            &mut monster.bat_start_sum_3,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line::<E>(
            &mut input,
            &mut monster.inner_lines,
            MonsterLine::BatStartSum4,
            &mut monster.bat_start_sum_4,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line::<E>(
            &mut input,
            &mut monster.inner_lines,
            MonsterLine::BatStartSum5,
            &mut monster.bat_start_sum_5,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line::<E>(
            &mut input,
            &mut monster.inner_lines,
            MonsterLine::BatStartSum1d3,
            &mut monster.bat_start_sum1d3,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line::<E>(
            &mut input,
            &mut monster.inner_lines,
            MonsterLine::BatStartSum1d6,
            &mut monster.bat_start_sum1d6,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line::<E>(
            &mut input,
            &mut monster.inner_lines,
            MonsterLine::BatStartSum2d6,
            &mut monster.bat_start_sum2d6,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line::<E>(
            &mut input,
            &mut monster.inner_lines,
            MonsterLine::BatStartSum3d6,
            &mut monster.bat_start_sum3d6,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line::<E>(
            &mut input,
            &mut monster.inner_lines,
            MonsterLine::BatStartSum4d6,
            &mut monster.bat_start_sum4d6,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line::<E>(
            &mut input,
            &mut monster.inner_lines,
            MonsterLine::BatStartSum5d6,
            &mut monster.bat_start_sum5d6,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line::<E>(
            &mut input,
            &mut monster.inner_lines,
            MonsterLine::BatStartSum6d6,
            &mut monster.bat_start_sum6d6,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line::<E>(
            &mut input,
            &mut monster.inner_lines,
            MonsterLine::BatStartSum7d6,
            &mut monster.bat_start_sum7d6,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line::<E>(
            &mut input,
            &mut monster.inner_lines,
            MonsterLine::BatStartSum8d6,
            &mut monster.bat_start_sum8d6,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line::<E>(
            &mut input,
            &mut monster.inner_lines,
            MonsterLine::BatStartSum9d6,
            &mut monster.bat_start_sum9d6,
        )
        .is_ok()
        {
        } else if parse_id_name_montag_line::<E>(
            &mut input,
            &mut monster.inner_lines,
            MonsterLine::Slaver,
            &mut monster.slaver,
        )
        .is_ok()
        {

        } else if let Ok((remaining_input, _)) = tag::<_, _, E>("#growhp")(input){
            input = remaining_input;
            monster.inner_lines.push(Right(MonsterLine::GrowHp));
        } else if let Ok((remaining_input, _)) = tag::<_, _, E>("#shrinkhp")(input){
            input = remaining_input;
            monster.inner_lines.push(Right(MonsterLine::ShrinkHp));
        } else if let Ok((remaining_input, _)) = tag::<_, _, E>("#xpshape")(input){
            input = remaining_input;
            monster.inner_lines.push(Right(MonsterLine::XpShape));

            // These two must be last
        } else if let Ok((remaining_input, _)) = tag::<_, _, E>("#end")(input) {
            input = remaining_input;
            break; // we're done
        } else {
            // "cannot" fail
            let (remaining_input, unparsed_line) = parse_unparsed_line(input)?;
            input = remaining_input;
            monster.inner_lines.push(Left(unparsed_line));
        }
    }

    Ok((
        input,
        monster,
    ))
}

fn parse_id_name_montag_line<'a, E: ParseError<&'a str>>(
    input: &mut &'a str,
    inner_lines: &mut Vec<Either<&'a str, MonsterLine<'a>>>,
    desired_line: MonsterLine<'a>,
    value: &mut Option<Either<i32, &'a str>>,
) -> Result<(), ()> {
    let tag = desired_line.line_tag();
    match parse_id_name_montag_property::<E>(tag)(*input) {
        Err(_e) => Err(()), // TODO
        Ok((remaining_input, found_id)) => {
            if let Some(old_id) = value.replace(found_id) {
                panic!("Monster had duplicate {}: {}", tag, old_id); // fixme
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
