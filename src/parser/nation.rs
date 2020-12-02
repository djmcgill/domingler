use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{space0, space1};
use nom::IResult;
use nom::{branch::alt, error::VerboseError};

use crate::parser::{parse_comment_line_end, parse_id};

#[cfg(test)]
mod tests;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum NationDeclaration {
    SelectId(u32), // select number only
    NewImplicit,   // no new with number allowed
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum NationLine<'a> {
    Unparsed(&'a str),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Nation<'a> {
    pub declaration: NationDeclaration,
    /// This field does not contain the declaration or the end
    pub inner_lines: Vec<NationLine<'a>>,
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
    let (input, declaration) = parse_nation_declaration(input)?;

    let (input, raw_lines) = take_until("#end")(input)?;
    for line in raw_lines.lines() {
        if !line.is_empty() {
            inner_lines.push(NationLine::Unparsed(line));
        }
    }

    let (input, _) = tag("#end")(input)?;

    let (input, _) = parse_comment_line_end(input)?;

    Ok((
        input,
        Nation {
            declaration,
            inner_lines,
        },
    ))
}
