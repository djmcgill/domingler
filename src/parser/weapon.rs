use crate::parser::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space0};
use nom::combinator::{map, opt};
use nom::IResult;
use std::str::FromStr;

#[cfg(test)]
mod tests;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct WeaponId(pub u32);
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct WeaponName<'a>(&'a str);
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct WeaponIdOrName<'a>(pub Either<WeaponId, WeaponName<'a>>);
impl<'a> From<Either<u32, &'a str>> for WeaponIdOrName<'a> {
    fn from(raw: Either<u32, &'a str>) -> Self {
        WeaponIdOrName(raw.map_left(WeaponId).map_right(WeaponName))
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum WeaponDeclaration<'a> {
    SelectId(WeaponId),
    SelectName(WeaponName<'a>),
    NewId(WeaponId),
    NewImplicit,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum WeaponLine<'a> {
    Name,
    CopyWeapon,
    SecondaryEffect,
    SecondaryEffectAlways,
    _Dummy(&'a ()),
}
impl<'a> WeaponLine<'a> {
    pub fn line_tag(&self) -> &'static str {
        match self {
            WeaponLine::Name => "#name",
            WeaponLine::CopyWeapon => "#copyweapon",
            WeaponLine::SecondaryEffect => "#secondaryeffect",
            WeaponLine::SecondaryEffectAlways => "#secondaryeffectalways",
            WeaponLine::_Dummy(_) => unimplemented!(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Weapon<'a> {
    pub declaration: WeaponDeclaration<'a>,
    pub name: Option<WeaponName<'a>>,
    pub copy_weapon: Option<WeaponIdOrName<'a>>,
    pub secondary_effect: Option<WeaponIdOrName<'a>>,
    pub secondary_effect_always: Option<WeaponIdOrName<'a>>,

    /// This field does not contain the declaration or the end
    pub inner_lines: Vec<Either<&'a str, WeaponLine<'a>>>,
}

fn parse_select_weapon<'a>(
    input: &'a str,
) -> IResult<&'a str, WeaponDeclaration<'a>, VerboseError<&'a str>> {
    let (input, _) = tag("#selectweapon")(input)?;
    let (input, _) = space0(input)?;
    let (input, either_id_name) = alt((
        map(parse_id, |id| WeaponDeclaration::SelectId(WeaponId(id))),
        map(parse_name, |name| {
            WeaponDeclaration::SelectName(WeaponName(name))
        }),
    ))(input)?;

    Ok((input, either_id_name))
}

fn parse_new_weapon<'a>(
    input: &'a str,
) -> IResult<&'a str, WeaponDeclaration<'a>, VerboseError<&'a str>> {
    let (input, _) = tag("#newweapon")(input)?;
    let (input, _) = space0(input)?;
    let (input, opt_weapon_id_str) = opt(digit1)(input)?;
    let opt_weapon_id = opt_weapon_id_str.map(|weapon_id_str| {
        u32::from_str(weapon_id_str)
            .unwrap_or_else(|_| panic!("could not parse valid integer id from '{}'", weapon_id_str))
    }); // FIXME

    let declaration = match opt_weapon_id {
        Some(weapon_id) => WeaponDeclaration::NewId(WeaponId(weapon_id)),
        None => WeaponDeclaration::NewImplicit,
    };

    Ok((input, declaration))
}

fn parse_weapon_declaration<'a>(
    input: &'a str,
) -> IResult<&'a str, WeaponDeclaration<'a>, VerboseError<&'a str>> {
    let (input, _) = space0(input)?;

    let (input, weapon_declaration) = alt((parse_new_weapon, parse_select_weapon))(input)?;
    let (input, _) = parse_comment_line_end(input)?;

    Ok((input, weapon_declaration))
}

pub fn parse_weapon<'a>(input: &'a str) -> IResult<&'a str, Weapon<'a>, VerboseError<&'a str>> {
    let mut inner_lines = vec![];
    let (input, declaration) = parse_weapon_declaration(input)?;
    let mut input = input;
    let mut name = None;
    let mut copy_weapon = None;
    let mut secondary_effect = None;
    let mut secondary_effect_always = None;

    loop {
        if let Ok((remaining_input, found_name)) = parse_string_property("#name")(input) {
            if let Some(old_name) = name.replace(WeaponName(found_name)) {
                panic!("Weapon had duplicate #name: {:?}", old_name); // fixme
            }
            input = remaining_input;
            inner_lines.push(Right(WeaponLine::Name));
        } else if parse_id_name_line(
            &mut input,
            &mut inner_lines,
            WeaponLine::CopyWeapon,
            &mut copy_weapon,
        )
        .is_ok()
        {
        } else if parse_id_name_line(
            &mut input,
            &mut inner_lines,
            WeaponLine::SecondaryEffect,
            &mut secondary_effect,
        )
        .is_ok()
        {
        } else if parse_id_name_line(
            &mut input,
            &mut inner_lines,
            WeaponLine::SecondaryEffectAlways,
            &mut secondary_effect_always,
        )
        .is_ok()
        {

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
    let weapon = Weapon {
        declaration,
        name,
        copy_weapon,
        secondary_effect,
        secondary_effect_always,
        inner_lines,
    };

    Ok((input, weapon))
}

fn parse_id_name_line<'a>(
    input: &mut &'a str,
    inner_lines: &mut Vec<Either<&'a str, WeaponLine<'a>>>,
    desired_line: WeaponLine<'a>,
    value: &mut Option<WeaponIdOrName<'a>>,
) -> Result<(), ()> {
    let tag = desired_line.line_tag();
    match parse_id_name_property(tag)(*input) {
        Err(_e) => Err(()), // TODO
        Ok((remaining_input, found_id)) => {
            if let Some(old_id) = value.replace(found_id.into()) {
                panic!("Weapon had duplicate {}: {:?}", tag, old_id); // fixme
            }
            *input = remaining_input;
            inner_lines.push(Right(desired_line));
            Ok(())
        }
    }
}
