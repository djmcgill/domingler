//use super::*;
//
//#[test]
//fn parse_a_name() {
//    let input = "\"foo\"";
//    let (remaining, name) = parse_name::<()>(input).unwrap();
//    assert_eq!(remaining.len(), 0);
//    assert_eq!(name.right().unwrap(), "foo");
//}
//
//#[test]
//fn parse_an_id() {
//    let input = "39";
//    let (remaining, name) = parse_id::<()>(input).unwrap();
//    assert_eq!(remaining.len(), 0);
//    assert_eq!(name.left().unwrap(), 39);
//}
//
//#[test]
//fn parse_weapon_1() {
//    let input = r#"#newweapon 852
//#name "Sunlight Blade"
//#dmg 1
//#shock
//#magic
//#armornegating
//#nostr
//#end
//"#;
//    let (remaining, weapon) = Weapon::parse_weapon::<()>(input).unwrap();
//    assert_eq!(weapon.opt_either_id_name.unwrap().left().unwrap(), 852);
//    assert_eq!(weapon.lines, r#"
//#name "Sunlight Blade"
//#dmg 1
//#shock
//#magic
//#armornegating
//#nostr
//"#);
//}
//
//#[test]
//fn parse_weapon_2() {
//    let input = r#"#newweapon
//#copyweapon 20 -- Regular Ass Bite
//#name "Magic Bite"
//#magic
//#end"#;
//
//    let (remaining, weapon) = Weapon::parse_weapon::<()>(input).unwrap();
//    assert!(weapon.opt_either_id_name.is_none());
//    assert_eq!(weapon.lines, r#"
//#copyweapon 20 -- Regular Ass Bite
//#name "Magic Bite"
//#magic
//"#);
//}
//
//#[test]
//fn parse_weapon_3() {
//    let input = r#"#selectweapon "Swallow"
//#explspr 10259 -- Slurpy Signal
//#end"#;
//
//}
