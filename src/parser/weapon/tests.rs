use super::*;

#[test]
fn parse_a_name() {
   let input = "\"foo\"";
   let (remaining, name) = parse_name_either::<()>(input).unwrap();
   assert_eq!(remaining.len(), 0);
   assert_eq!(name.right().unwrap(), "foo");
}

#[test]
fn parse_an_id() {
   let input = "39";
   let (remaining, name) = parse_id_either::<()>(input).unwrap();
   assert_eq!(remaining.len(), 0);
   assert_eq!(name.left().unwrap(), 39);
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
      WeaponDeclaration::NewWeapon(Some(852)) => (), // pass
      other => panic!("Unexpected weapon declaration: {:?}", other),
   }

   assert_eq!(weapon.lines, vec![
      WeaponLine::Declaration,
      WeaponLine::Unparsed("#name \"Sunlight Blade\""),
      WeaponLine::Unparsed("#dmg 1"),
      WeaponLine::Unparsed("#shock"),
      WeaponLine::Unparsed("#magic"),
      WeaponLine::Unparsed("#armornegating"),
      WeaponLine::Unparsed("#nostr"),
      WeaponLine::End,
   ]);
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
      WeaponDeclaration::NewWeapon(None) => (), // pass
      other => panic!("Unexpected decl: {:?}", other),
   }

   assert_eq!(weapon.lines, vec![
      WeaponLine::Declaration,
      WeaponLine::Unparsed("#copyweapon 20 -- Regular Ass Bite"),
      WeaponLine::Unparsed("#name \"Magic Bite\""),
      WeaponLine::Unparsed("#magic"),
      WeaponLine::End,
   ]);
}

#[test]
fn parse_weapon_3() {
   let input = r#"#selectweapon "Swallow"
#explspr 10259 -- Slurpy Signal
#end"#;

   let (_, weapon) = parse_weapon::<()>(input).unwrap();
   match weapon.declaration {
      WeaponDeclaration::SelectName("Swallow") => (), // pass
      other => panic!("Unexpected decl: {:?}", other),
   }

   assert_eq!(weapon.lines, vec![
      WeaponLine::Declaration,
      WeaponLine::Unparsed("#explspr 10259 -- Slurpy Signal"),
      WeaponLine::End,
   ]);
}
