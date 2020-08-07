use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space0};
use nom::combinator::{map, opt};
use nom::error::ParseError;
use nom::IResult;
use std::str::FromStr;
use either::*;
use crate::parser::*;

#[cfg(test)]
mod tests;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ArmourId(pub u32);
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ArmourName<'a>(&'a str);
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ArmourIdOrName<'a>(pub Either<ArmourId, ArmourName<'a>>);
impl<'a> From<Either<u32, &'a str>> for ArmourIdOrName<'a> {
    fn from(raw: Either<u32, &'a str>) -> Self {
        ArmourIdOrName(raw.map_left(ArmourId).map_right(ArmourName))
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ArmourDeclaration<'a> {
    SelectId(ArmourId),
    SelectName(ArmourName<'a>),
    NewId(ArmourId),
    NewImplicit,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ArmourLine<'a> {
    Name,
    CopyArmour,
    Dummy(&'a ()),
}
impl<'a> ArmourLine<'a> {
    pub fn line_tag(&self) -> &'static str {
        match self {
            ArmourLine::Name => "#name",
            ArmourLine::CopyArmour => "#copyarmor",
            ArmourLine::Dummy(_) => unimplemented!(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Armour<'a> {
    pub declaration: ArmourDeclaration<'a>,
    pub name: Option<ArmourName<'a>>,
    pub copy_armour: Option<ArmourIdOrName<'a>>,
    /// This field does not contain the declaration or the end
    pub inner_lines: Vec<Either<&'a str, ArmourLine<'a>>>,
}

fn parse_select_armour<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, ArmourDeclaration<'a>, E> {
    let (input, _) = tag("#selectarmor")(input)?;
    let (input, _) = space0(input)?;
    let (input, either_id_name) = alt((
        map(parse_id, |id| ArmourDeclaration::SelectId(ArmourId(id))),
        map(parse_name, |name| ArmourDeclaration::SelectName(ArmourName(name))),
    ))(input)?;

    Ok((input, either_id_name))
}

fn parse_new_armour<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, ArmourDeclaration<'a>, E> {
    let (input, _) = tag("#newarmor")(input)?;
    let (input, _) = space0(input)?;
    let (input, opt_armour_id_str) = opt(digit1)(input)?;
    let opt_armour_id = opt_armour_id_str.map(|armour_id_str| {
        u32::from_str(armour_id_str)
            .unwrap_or_else(|_| panic!("could not parse valid integer id from '{}'", armour_id_str))
    }); // FIXME

    let declaration = match opt_armour_id {
        Some(armour_id) => ArmourDeclaration::NewId(ArmourId(armour_id)),
        None => ArmourDeclaration::NewImplicit,
    };

    Ok((input, declaration))
}

fn parse_armour_declaration<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, ArmourDeclaration<'a>, E> {
    let (input, _) = space0(input)?;
    let (input, armour_declaration) = alt((parse_new_armour, parse_select_armour))(input)?;
    let (input, _) = parse_comment_line_end(input)?;

    Ok((input, armour_declaration))
}

pub fn parse_armour<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Armour<'a>, E> {
    let mut inner_lines = vec![];
    let mut name = None;
    let mut copy_armour = None;

    let (input, declaration) = parse_armour_declaration(input)?;
    let mut input = input;

    loop {
        if let Ok((remaining_input, found_name)) = parse_string_property::<E>("#name")(input) {
            if let Some(old_name) = name.replace(ArmourName(found_name)) {
                panic!("Armour had duplicate #name: {:?}", old_name); // fixme
            }
            input = remaining_input;
            inner_lines.push(Right(ArmourLine::Name));
        } else if parse_id_name_line::<E>(
            &mut input,
            &mut inner_lines,
            ArmourLine::CopyArmour,
            &mut copy_armour,
        ).is_ok() {

        // These two must be last
        } else if let Ok((remaining_input, _)) = tag::<_, _, E>("#end")(input) {
            input = remaining_input;
            break; // we're done
        } else {
            // "cannot" fail
            let (remaining_input, unparsed_line) = parse_unparsed_line(input)?;
            input = remaining_input;
            inner_lines.push(Left(unparsed_line));
        }
    }

    let armour = Armour {
        declaration,
        name,
        copy_armour,
        inner_lines,
    };

    Ok((
        input,
        armour,
    ))
}

fn parse_id_name_line<'a, E: ParseError<&'a str>>(
    input: &mut &'a str,
    inner_lines: &mut Vec<Either<&'a str, ArmourLine<'a>>>,
    desired_line: ArmourLine<'a>,
    value: &mut Option<ArmourIdOrName<'a>>,
) -> Result<(), ()> {
    let tag = desired_line.line_tag();
    match parse_id_name_property::<E>(tag)(*input) {
        Err(_e) => Err(()), // TODO
        Ok((remaining_input, found_id)) => {
            if let Some(old_id) = value.replace(found_id.into()) {
                panic!("Monster had duplicate {}: {:?}", tag, old_id); // fixme
            }
            *input = remaining_input;
            inner_lines.push(Right(desired_line));
            Ok(())
        }
    }
}