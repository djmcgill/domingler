use super::*;

#[test]
fn parse_armour_1() {
    let input = r#"#newarmor 799
#name "Bronze Hauberk of Heroes"
#magicarmor
#prot 18
#rcost 10
#type 5
#def -1
#enc 2
#end"#;
    let (_, armour) = parse_armour::<()>(input).unwrap();

    match armour.declaration {
        ArmourDeclaration::NewId(799) => (), // pass
        other => panic!("Unexpected declaration: {:?}", other),
    }

    assert_eq!(
        armour.inner_lines,
        vec![
            ArmourLine::Unparsed("#name \"Bronze Hauberk of Heroes\""),
            ArmourLine::Unparsed("#magicarmor"),
            ArmourLine::Unparsed("#prot 18"),
            ArmourLine::Unparsed("#rcost 10"),
            ArmourLine::Unparsed("#type 5"),
            ArmourLine::Unparsed("#def -1"),
            ArmourLine::Unparsed("#enc 2"),
        ]
    );
}
