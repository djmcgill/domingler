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
        ArmourDeclaration::NewId(ArmourId(799)) => (), // pass
        other => panic!("Unexpected declaration: {:?}", other),
    }

    assert_eq!(armour.name, Some(ArmourName("Bronze Hauberk of Heroes")));

    assert_eq!(
        armour.inner_lines,
        vec![
            Either::Right(ArmourLine::Name),
            Either::Left("#magicarmor"),
            Either::Left("#prot 18"),
            Either::Left("#rcost 10"),
            Either::Left("#type 5"),
            Either::Left("#def -1"),
            Either::Left("#enc 2"),
        ]
    );
}
