#![recursion_limit = "128"]

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

mod mod_line_scanner;
use mod_line_scanner::*;

mod mod_definition;
use mod_definition::*;

lazy_static! {
    static ref MOD_NAME: Regex = Regex::new(
        "^\
         (?P<prefix>#modname \")\
         (?P<name>[^\"]+)\
         (?P<suffix>\".*)$\
         "
    )
    .unwrap();

    static ref END: Regex = Regex::new("^#end").unwrap();

    static ref SPELL_BLOCK_START: Regex = Regex::new("^#(newspell|selectspell)").unwrap();

    static ref SPELL_EFFECT: Regex = Regex::new("^#effect (?P<id>[[:digit:]]+)").unwrap();
    static ref SPELL_DAMAGE: Regex = Regex::new("^#damage (?P<id>[[:digit:]]+)").unwrap();
}

fn scan_all_mods(mods: &Vec<Vec<String>>) -> HashMap<String, ModDefinition> {
    let mut hash_map = HashMap::new();
    for mod_lines in mods {
        let mut option_mod_name: Option<String> = None;
        let mut mod_definition: ModDefinition = ModDefinition::default();

        // Okay here's the deal: parsing global enchantment IDs is a bit weird.
        // So if we parse a #newspell or #selectspell then we need to keep track of
        // all lines until the next #end. Then we can inspect it to see if it declares
        // a new global enchantment ID.
        let mut option_current_spell_block: Option<Vec<&str>> = None;

        for line in mod_lines {
            // Capture name
            if let Some(name_capture) = MOD_NAME.captures(&line) {
                let found_name = name_capture.name("name").unwrap().as_str();
                if option_mod_name.is_none() {
                    option_mod_name = Some(found_name.to_owned());
                } else {
                    panic!("Somehow found two #modname commands in a mod???");
                }
            }
            // If we're inside a block and find a #end, close it
            // Note that not every #end will have a matching block
            // (partly since we're currently ignoring e.g. #selectweapon "name")
            if let Some(current_block) = &mut option_current_spell_block {
                current_block.push(line);
                if END.is_match(line) {
                    parse_spell_block(current_block, &mut mod_definition.enchantments);
                    option_current_spell_block = None;
                    continue;
                }
            } else if SPELL_BLOCK_START.is_match(line) {
                // If we find a #newspell or a #selectspell, start recording lines
                option_current_spell_block = Some(Vec::new());
            }

            // Capture declarations:
            // As soon as any match, move on
            let _ = WEAPON_LINE_SCANNER.scan_line(line, &mut mod_definition.weapons) ||
                ARMOUR_LINE_SCANNER.scan_line(line, &mut mod_definition.armours) ||
                SPELL_LINE_SCANNER.scan_line(line, &mut mod_definition.spells) ||
                MONSTER_LINE_SCANNER.scan_line(line, &mut mod_definition.monsters) ||
                ITEM_LINE_SCANNER.scan_line(line, &mut mod_definition.items) ||
                SITE_LINE_SCANNER.scan_line(line, &mut mod_definition.sites) ||
                NATION_LINE_SCANNER.scan_line(line, &mut mod_definition.nations) ||
                NAMETYPE_LINE_SCANNER.scan_line(line, &mut mod_definition.name_types) ||
                MONTAG_LINE_SCANNER.scan_line(line, &mut mod_definition.montags) ||
                EVENTCODE_LINE_SCANNER.scan_line(line, &mut mod_definition.event_codes) ||
                RESTRICTED_ITEM_LINE_SCANNER.scan_line(line, &mut mod_definition.restricted_items);
        }

        let mod_name = option_mod_name.unwrap_or_else(|| panic!("Could not find the mod's name"));
        hash_map.insert(mod_name, mod_definition);
    }
    hash_map
}

fn parse_spell_block(block: &Vec<&str>, mod_enchantments: &mut HashSet<u32>) {
    // parse as u64 because some mods have non-u32 values in here???
    let mut option_damage: Option<u64> = None;
    let mut option_effect: Option<u64> = None;

    for line in block {
        if let Some(capture) = SPELL_EFFECT.captures(line) {
            let found_id = u64::from_str(capture.name("id").unwrap().as_str()).unwrap();
            option_effect = Some(found_id);
        } else if let Some(capture) = SPELL_DAMAGE.captures(line) {
            let found_id = u64::from_str(capture.name("id").unwrap().as_str()).unwrap();
            option_damage = Some(found_id);
        }
    }

    match (option_effect, option_damage) {
        (Some(effect), Some(damage)) if effect == 10042 || (effect >= 10081 && effect <= 10087) => {
            if damage == 4700 {
                println!("damage: 4700, effect: {}", effect);
            }
            mod_enchantments.insert(damage as u32);
        }
        _ => {}
    }
}

fn print_mod_id_usages(hash_map: &HashMap<String, ModDefinition>) {
    for (name, definition) in hash_map {
        println!("Mod: {}", name);
        print_min_max("Weapons", &definition.weapons.defined_ids);
        print_min_max("Armour", &definition.armours.defined_ids);
        print_min_max("Spells", &definition.spells.defined_ids);
        print_min_max("Monsters", &definition.monsters.defined_ids);
        print_min_max("Items", &definition.items.defined_ids);
        print_min_max("Sites", &definition.sites.defined_ids);
        print_list("Nations", &definition.nations.defined_ids);
        print_list("Nametypes", &definition.name_types.defined_ids);
        print_list("Montags", &definition.montags.defined_ids);
        print_list("Event codes", &definition.event_codes.defined_ids);
        print_list("Restricted items", &definition.restricted_items.defined_ids);
        print_list("Enchantments", &definition.enchantments);
        println!();
    }
}

fn print_list(name: &str, items: &HashSet<u32>) {
    let mut items: Vec<u32> = items.iter().cloned().collect();
    items.sort_unstable();
    match items.len() {
        0 => {}
        1 => println!(" - {}: {}", name, items.iter().next().unwrap()),
        _ => {
            print!(" - {}: ", name);
            for item in items {
                print!("{}, ", item);
            }
            println!();
        }
    }
}

fn print_min_max(name: &str, items: &HashSet<u32>) {
    match min_max(items.iter()) {
        None => {}
        Some((min, None)) => {
            println!(" - {}: {}", name, min);
        }
        Some((min, Some(max))) => {
            println!(" - {}: {}-{}", name, min, max);
        }
    }
}

// This is pretty gross honestly
fn min_max<'a>(mut items: impl Iterator<Item = &'a u32>) -> Option<(u32, Option<u32>)> {
    if let Some(&first_item) = items.next() {
        Some({
            if let Some(&second_item) = items.next() {
                let mut min = first_item;
                let mut max = first_item;

                if second_item < min {
                    min = second_item
                } else if second_item > max {
                    max = second_item
                }

                for &item in items {
                    if item < min {
                        min = item;
                    } else if item > max {
                        max = item;
                    }
                }
                (min, Some(max))
            } else {
                (first_item, None)
            }
        })
    } else {
        None
    }
}

//fn map_ids(debug_str: &'static str,
//           lines: &mut Vec<String>,
//           use_regex: &Regex,
//           mapped_ids: &HashMap<u32, u32>) {

//    for line in lines {
//        let option_replacement = match use_regex.captures(line) {
//            None => None,
//            Some(line_capture) => {
////                println!("capture: {:?}", line_capture);
//                let found_id = u32::from_str(line_capture.name("id").unwrap().as_str()).unwrap();
//                match mapped_ids.get(&found_id) {
//                    None => None,
//                    Some(new_id) => {
//                        print!("found use of modded {} {}, mapping to {} ", debug_str, found_id, new_id);
//                        let new_line: String = use_regex.replace(&line, |ref captures: &Captures| -> String {
//                            format!("{}{}{}", &captures["prefix"], new_id, &captures["suffix"])
//                        }).to_string();
//                        println!("new line is: '{}'", new_line);
//                        Some(new_line)
//                    }
//                }
//            }
//        };
//        for replacement in option_replacement {
//            *line = replacement;
//        }
//    }
//}
// FIXME: remove mod name, icon, description, version

fn main() {
    let mod_file_paths = vec![
        "/mnt/c/Users/David/AppData/Roaming/Dominions5/mods/EA_Karanaac_v1.26.dm",
        "/mnt/c/Users/David/AppData/Roaming/Dominions5/mods/EA_Bethel_Sheem_v1.05.dm",
        "/mnt/c/Users/David/AppData/Roaming/Dominions5/mods/EA_Nabatem_v1.24.dm",
        "/mnt/c/Users/David/AppData/Roaming/Dominions5/mods/EA_U_v1.12.dm",
        "/mnt/c/Users/David/AppData/Roaming/Dominions5/mods/MA_Sawaikii.dm",
    ];
    let mod_files: Vec<Vec<String>> = mod_file_paths
        .into_iter()
        .map(|path| {
            let file = File::open(path).unwrap();
            let file_buff = BufReader::new(file);
            let line_iter = file_buff.lines().map(|result| result.unwrap());
            let lines: Vec<String> = line_iter.collect();
            lines
        })
        .collect();
    let parsed_mods = scan_all_mods(&mod_files);
    print_mod_id_usages(&parsed_mods);

    //    let mut first_available_weapon_id = ASSUMED_FIRST_WEAPON_ID;
    //    let mut first_available_armour_id = ASSUMED_FIRST_ARMOUR_ID;
    //    let mut first_available_monster_id = ASSUMED_FIRST_MONSTER_ID;
    //
    //    let mut lines: Vec<String> = vec![
    //        "#modname \"domingler mod test\"".to_owned(),
    //        format!("#description \"a combination of: some shit or whatever\""),
    //    ];
    //
    //    let mod_files: Vec<Vec<String>> = mod_file_paths.into_iter().map(|path| {
    //        let file = File::open(path).unwrap();
    //        let file_buff = BufReader::new(file);
    //        let line_iter = file_buff.lines().map(|result| result.unwrap());
    //        let lines: Vec<String> = line_iter.collect();
    //        lines
    //    }).collect();
    //// todo: add the mod names to the description
    //
    //    for mut mod_file in mod_files {
    //
    //        println!("Looking for #newweapon:");
    //        let mapped_weapon_ids = create_id_map(mod_file.iter(), &NEW_WEAPON, &mut first_available_weapon_id);
    //        println!("Looking for #newarmor:");
    //        let mapped_armor_ids = create_id_map(mod_file.iter(), &NEW_ARMOUR, &mut first_available_armour_id);
    //        println!("Looking for #newmonster:");
    //        let mapped_monster_ids = create_id_map(mod_file.iter(), &NEW_MONSTER, &mut first_available_monster_id);
    //
    //
    //        println!("Looking for weapon use:");
    //        map_ids("weapon", &mut mod_file, &USE_WEAPON, &mapped_weapon_ids);
    //        println!("Looking for armour use:");
    //        map_ids("armour", &mut mod_file, &USE_ARMOUR, &mapped_armor_ids);
    //        println!("Looking monster use:");
    //        map_ids("monster", &mut mod_file, &USE_MONSTER, &mapped_monster_ids);
    //
    //        lines.extend(mod_file);
    //    }
    //
    //    let new_file = File::create("/mnt/c/Users/David/AppData/Roaming/Dominions5/mods/domingler-test.dm").unwrap();
    //    let mut writer = BufWriter::new(new_file);
    //    for line in lines {
    //        write!(&mut writer, "{}\n", line).unwrap();
    //    }
}
