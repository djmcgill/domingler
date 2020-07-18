use super::*;
use crate::parser::monster::MonsterDeclaration;
use crate::parser::nation::NationDeclaration;
use crate::parser::weapon::WeaponDeclaration;
use crate::parser::armour::ArmourDeclaration;
use nom::error::VerboseError;

#[test]
fn parse_all_of_hellenika() {
    let mod_contents = std::fs::read_to_string("sample_files/Hellenika_v2_13.dm").unwrap();
    let (_, parsed_mod) = parse_mod::<VerboseError<&str>>(&mod_contents).unwrap();

    assert_eq!(parsed_mod.0[0], ModItem::ModName("Hellenika v2.13f"));

    let mut new_monster_id_count = 0;
    let mut new_monster_implicit_count = 0;
    let mut select_monster_id_count = 0;
    let mut select_monster_name_count = 0;

    let mut new_nation_implicit_count = 0;
    let mut select_nation_id_count = 0;

    let mut new_weapon_id_count = 0;
    let mut new_weapon_implicit_count = 0;
    let mut select_weapon_id_count = 0;
    let mut select_weapon_name_count = 0;

    let mut new_armour_id_count = 0;
    let mut new_armour_implicit_count = 0;
    let mut select_armour_id_count = 0;
    let mut select_armour_name_count = 0;

    for item in &parsed_mod.0 {
        match item {
            ModItem::Monster(monster) => {
                match monster.declaration {
                    MonsterDeclaration::NewImplicit => new_monster_implicit_count += 1,
                    MonsterDeclaration::NewId(_) => new_monster_id_count += 1,
                    MonsterDeclaration::SelectName(_) => select_monster_name_count += 1,
                    MonsterDeclaration::SelectId(_) => select_monster_id_count += 1,
                }
            }
            ModItem::Nation(nation) => {
                match nation.declaration {
                    NationDeclaration::SelectId(_) => select_nation_id_count += 1,
                    NationDeclaration::NewImplicit => new_nation_implicit_count += 1,
                }
            }
            ModItem::Weapon(weapon) => {
                match weapon.declaration {
                    WeaponDeclaration::NewImplicit => new_weapon_implicit_count += 1,
                    WeaponDeclaration::NewId(_) => new_weapon_id_count += 1,
                    WeaponDeclaration::SelectId(_) => select_weapon_id_count += 1,
                    WeaponDeclaration::SelectName(_) => select_weapon_name_count += 1,
                }
            }
            ModItem::Armour(armour) => {
                match armour.declaration {
                    ArmourDeclaration::NewImplicit => new_armour_implicit_count += 1,
                    ArmourDeclaration::NewId(_) => new_armour_id_count += 1,
                    ArmourDeclaration::SelectName(_) => select_armour_name_count += 1,
                    ArmourDeclaration::SelectId(_) => select_armour_id_count += 1,
                }
            }
            _ => {}
        }
    }

    // These counts are independently verified with regex
    assert_eq!(new_monster_implicit_count, 788+30);
    assert_eq!(new_monster_id_count, 395);
    assert_eq!(select_monster_id_count, 1110);
    assert_eq!(select_monster_name_count, 0);

    assert_eq!(new_nation_implicit_count, 0);
    assert_eq!(select_nation_id_count, 106);

    assert_eq!(new_weapon_implicit_count, 0);
    assert_eq!(new_weapon_id_count, 107);
    assert_eq!(select_weapon_id_count, 13);
    assert_eq!(select_weapon_name_count, 0);

    assert_eq!(new_armour_implicit_count, 0);
    assert_eq!(new_armour_id_count, 16);
    assert_eq!(select_armour_id_count, 1);
    assert_eq!(select_armour_name_count, 0);
}
