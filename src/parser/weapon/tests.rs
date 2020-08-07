use super::*;

#[test]
fn parse_a_name() {
    let input = "\"foo\"";
    let (remaining, name) = parse_name::<()>(input).unwrap();
    assert_eq!(remaining.len(), 0);
    assert_eq!(name, "foo");
}

#[test]
fn parse_an_id() {
    let input = "39";
    let (remaining, id) = parse_id::<()>(input).unwrap();
    assert_eq!(remaining.len(), 0);
    assert_eq!(id, 39);
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
    let (_, weapon) = parse_weapon::<()>(input).unwrap();

    match weapon.declaration {
        WeaponDeclaration::NewId(WeaponId(852)) => (), // pass
        other => panic!("Unexpected weapon declaration: {:?}", other),
    }

    assert_eq!(
        weapon.inner_lines,
        vec![
            Either::Right(WeaponLine::Name), // ("#name \"Sunlight Blade\"")),
            Either::Left("#dmg 1"),
            Either::Left("#shock"),
            Either::Left("#magic"),
            Either::Left("#armornegating"),
            Either::Left("#nostr"),
        ]
    );
}

#[test]
fn parse_weapon_2() {
    let input = r#"#newweapon
#copyweapon 20 -- Regular Ass Bite
#name "Magic Bite"
#magic
#end"#;

    let (_, weapon) = parse_weapon::<()>(input).unwrap();

    match weapon.declaration {
        WeaponDeclaration::NewImplicit => (), // pass
        other => panic!("Unexpected decl: {:?}", other),
    }

    assert_eq!(
        weapon.inner_lines,
        vec![
            Either::Right(WeaponLine::CopyWeapon), // ("#copyweapon 20 -- Regular Ass Bite"),
            Either::Right(WeaponLine::Name), // ("#name \"Magic Bite\""),
            Either::Left("#magic"),
        ]
    );
}

#[test]
fn parse_weapon_3() {
    let input = r#"#selectweapon "Swallow"
#explspr 10259 -- Slurpy Signal
#end"#;

    let (_, weapon) = parse_weapon::<()>(input).unwrap();
    match weapon.declaration {
        WeaponDeclaration::SelectName(WeaponName("Swallow")) => (), // pass
        other => panic!("Unexpected decl: {:?}", other),
    }

    assert_eq!(
        weapon.inner_lines,
        vec![Either::Left("#explspr 10259 -- Slurpy Signal"),]
    );
}
