#![recursion_limit = "128"]

use lazy_static::lazy_static;
use regex::{Regex, Captures};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
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

    static ref MOD_NAME_LINE: Regex = Regex::new("#modname").unwrap();
    static ref MOD_ICON_LINE: Regex = Regex::new("#icon").unwrap();
    static ref MOD_VERSION_LINE: Regex = Regex::new("#version").unwrap();
    static ref MOD_DOMVERSION_LINE: Regex = Regex::new("#domversion").unwrap();
    static ref MOD_DESCRIPTION_LINE: Regex = Regex::new("#description \"[^\"]*\"").unwrap();
    // n.b. check for `MOD_DESCRIPTION_LINE` first
    static ref MOD_DESCRIPTION_START: Regex = Regex::new("#description").unwrap();

    // This must be the worst line of code I've ever written lol
    static ref MOD_DESCRIPTION_STOP: Regex = Regex::new("\"").unwrap();
}

fn remap_ids(mod_definitions: &HashMap<String, ModDefinition>) -> HashMap<String, MappedModDefinition> {
    let mut weapons_implicit_definition_count = 0;
    let mut armours_implicit_definition_count = 0;
    let mut monsters_implicit_definition_count = 0;
    let mut name_types_implicit_definition_count = 0;
    let mut spells_implicit_definition_count = 0;
    let mut items_implicit_definition_count = 0;
    let mut sites_implicit_definition_count = 0;
    let mut nations_implicit_definition_count = 0;
    let mut events_implicit_definition_count = 0;
    let mut poptype_implicit_definition_count = 0;
    let mut montags_implicit_definition_count = 0;
    let mut event_codes_implicit_definition_count = 0;
    let mut restricted_items_implicit_definition_count = 0;
    let mut enchantments_implicit_definition_count = 0;
    let mut items_implicit_definition_count = 0;

    for mod_definition in mod_definitions.values() {
        weapons_implicit_definition_count += mod_definition.weapons.implicit_definitions;
        armours_implicit_definition_count += mod_definition.armours.implicit_definitions;
        monsters_implicit_definition_count += mod_definition.monsters.implicit_definitions;
        name_types_implicit_definition_count += mod_definition.name_types.implicit_definitions;
        spells_implicit_definition_count += mod_definition.spells.implicit_definitions;
        nations_implicit_definition_count += mod_definition.nations.implicit_definitions;
        events_implicit_definition_count += mod_definition.events.implicit_definitions;
        poptype_implicit_definition_count += mod_definition.poptype.implicit_definitions;
        montags_implicit_definition_count += mod_definition.montags.implicit_definitions;
        event_codes_implicit_definition_count += mod_definition.event_codes.implicit_definitions;
        restricted_items_implicit_definition_count += mod_definition.restricted_items.implicit_definitions;
        items_implicit_definition_count += mod_definition.items.implicit_definitions;
    }

    let mut first_available_weapon_id = ASSUMED_FIRST_WEAPON_ID + weapons_implicit_definition_count;
    let mut first_available_armour_id = ASSUMED_FIRST_ARMOUR_ID + armours_implicit_definition_count;
    let mut first_available_monster_id = ASSUMED_FIRST_MONSTER_ID + monsters_implicit_definition_count;
    let mut first_available_name_type_id = ASSUMED_FIRST_NAMETYPE_ID + name_types_implicit_definition_count;
    let mut first_available_spell_id = ASSUMED_FIRST_SPELL_ID + spells_implicit_definition_count;
    let mut first_available_nations_id = ASSUMED_FIRST_NATION_ID + nations_implicit_definition_count;
    let mut first_available_montags_id = ASSUMED_FIRST_MONTAG_ID + montags_implicit_definition_count;
    let mut first_available_event_codes_id = ASSUMED_FIRST_EVENTCODE_ID + event_codes_implicit_definition_count;
    let mut first_available_restricted_items_id = ASSUMED_FIRST_RESTRICTED_ITEM_ID + restricted_items_implicit_definition_count;
    let mut first_available_enchantment_id = 0; // FIXME: should this be zero?
    let mut first_available_item_id = ASSUMED_FIRST_ITEM_ID + items_implicit_definition_count;
    let mut first_available_site_id = ASSUMED_FIRST_SITE_ID + sites_implicit_definition_count;

    let mut mapped_mods = HashMap::new();
    for (mod_name, mod_definition) in mod_definitions.into_iter() {
        let mapped_mod = MappedModDefinition {
            weapons: remap_particular_ids(&mut first_available_weapon_id, &mod_definition.weapons.defined_ids),
            armours: remap_particular_ids(&mut first_available_armour_id, &mod_definition.armours.defined_ids),
            monsters: remap_particular_ids(&mut first_available_monster_id, &mod_definition.monsters.defined_ids),
            name_types: remap_particular_ids(&mut first_available_name_type_id, &mod_definition.name_types.defined_ids),
            spells: remap_particular_ids(&mut first_available_spell_id, &mod_definition.spells.defined_ids),
            nations: remap_particular_ids(&mut first_available_nations_id, &mod_definition.nations.defined_ids),
            montags: remap_particular_ids(&mut first_available_montags_id, &mod_definition.montags.defined_ids),
            event_codes: remap_particular_ids(&mut first_available_event_codes_id, &mod_definition.event_codes.defined_ids),
            restricted_items: remap_particular_ids(&mut first_available_restricted_items_id, &mod_definition.restricted_items.defined_ids),
            enchantments: remap_particular_ids(&mut first_available_enchantment_id, &mod_definition.enchantments),
            items: remap_particular_ids(&mut first_available_item_id, &mod_definition.items.defined_ids),
            sites: remap_particular_ids(&mut first_available_site_id, &mod_definition.sites.defined_ids),
//            poptype: unimplemented!(), FIXME: is this an issue or not?
        };

        // Clone doesn't seem to be needed if we consume self
        mapped_mods.insert(mod_name.clone(), mapped_mod);
    }

    mapped_mods
}

fn remap_particular_ids(first_available_id: &mut u32, mod_definitions: &HashSet<u32>) -> HashMap<u32, u32> {
    let mut mapped_ids = HashMap::new();

    for mod_definition_id in mod_definitions {
        mapped_ids.insert(*mod_definition_id, *first_available_id);
        *first_available_id += 1;
    }
    mapped_ids
}


fn scan_all_mods(mods: &Vec<(String, Vec<String>)>) -> HashMap<String, ModDefinition> {
    let mut hash_map = HashMap::new();
    for (path, mod_lines) in mods {
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
                mod_definition.name = found_name.to_owned();
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
            // TODO: combine these into a single regex to speed up
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

        hash_map.insert(path.clone(), mod_definition);
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

fn main() {
    // TODO: get this from the user somehow
    let mod_file_paths = vec![
        "/mnt/c/Users/David/AppData/Roaming/Dominions5/mods/EA_Karanaac_v1.26.dm",
//        "/mnt/c/Users/David/AppData/Roaming/Dominions5/mods/Firepower.dm",
    ];
    // TODO: no real point loading these all into memory
    let mod_files: Vec<(String, Vec<String>)> = mod_file_paths
        .into_iter()
        .map(|path| {
            let file = File::open(path).unwrap();
            let file_buff = BufReader::new(file);
            let line_iter = file_buff.lines().map(|result| result.unwrap());
            let lines: Vec<String> = line_iter.collect();
            (path.to_owned(), lines)
        })
        .collect();
    let parsed_mods = scan_all_mods(&mod_files);
    print_mod_id_usages(&parsed_mods);

    let remapped_ids = remap_ids(&parsed_mods);
    for k in remapped_ids.keys() {
        println!("KEY: {}", k);
    }

    // TODO: add the mod names to the description
    let mut lines: Vec<String> = vec![
        "#modname \"domingler mod test\"".to_owned(),
        format!("#description \"a combination of: some shit or whatever\""),
    ];

    for (path, mapped_definition) in remapped_ids {
        println!("NATION MAP:");
        for (k, v) in &mapped_definition.nations {
            println!("- {}: {}", k, v);
        }

        let file = File::open(path).unwrap();
        let file_buff = BufReader::new(file);
        let line_iter = file_buff.lines().map(|result| result.unwrap());

//        let mut option_current_spell_block: Option<Vec<String>> = None;

        let mut is_in_description = false;
        for line in line_iter {
            if is_in_description {
                if MOD_DESCRIPTION_STOP.is_match(&line) {
                    // End of description, ditch this line and then continue as normal
                    is_in_description = false;
                    continue;
                } else {
                    // Throw away a description line
                    continue;
                }
            } else {
                // FIXME: this should not throw away lines but should put them into the mod!!!
//                if let Some(current_spell_block) = &mut option_current_spell_block {
//                    current_spell_block.push(line.clone());
//                    if END.is_match(&line) {
//                        // FIXME: map ench
//                        // URGH going to need some lookahead on this
//
//                        option_current_spell_block = None;
//                        continue; // an #end isn't something we care about otherwise
//                    }
//                } else if SPELL_BLOCK_START.is_match(&line) {
//                    // If we find a #newspell or a #selectspell, start recording lines
//                    option_current_spell_block = Some(Vec::new());
//                }
            }

            // TODO: also ditch icon and version and domversion
            if MOD_NAME_LINE.is_match(&line) ||
                MOD_DESCRIPTION_LINE.is_match(&line) ||
                MOD_ICON_LINE.is_match(&line) ||
                MOD_VERSION_LINE.is_match(&line) ||
                MOD_DOMVERSION_LINE.is_match(&line) {
                // ditch the mod info
                continue;
            } else if MOD_DESCRIPTION_START.is_match(&line) {
                // Description has started, ditch the line
                is_in_description = true;
                continue;
            } else {
                let new_line = replace_use(
                    &line,
                    &mapped_definition.weapons,
                    &mod_line_scanner::USE_NUMBERED_WEAPON
                ).or_else(||
                    replace_use(
                        &line,
                        &mapped_definition.armours,
                        &mod_line_scanner::USE_NUMBERED_ARMOUR)
                ).or_else(|| {
                    if let Some(capture) = mod_line_scanner::USE_MONSTER.captures(&line) {
                        let found_id = i32::from_str(capture.name("id").unwrap().as_str()).unwrap();
                        if found_id > 0 {
                            if let Some(new_id) = mapped_definition.monsters.get(&(found_id as u32)) {
                                let new_line: String = USE_MONSTER.replace(&line, |ref captures: &Captures| -> String {
                                    format!("{}{}{}", &captures["prefix"], new_id, &captures["suffix"])
                                }).to_string();
                                Some(new_line)
                            } else {
                                Some(line.clone())
                            }
                        } else {
                            // it's a montag!
                            Some(line.clone())
                        }
                    } else { None }
                }).or_else(||
                    replace_use(&line, &mapped_definition.name_types, &mod_line_scanner::USE_NAMETYPE)
                ).or_else(||
                    replace_use(&line, &mapped_definition.spells, &mod_line_scanner::USE_NUMBERED_SPELL)
                ).or_else(||
                    replace_use(&line, &mapped_definition.nations, &mod_line_scanner::USE_NUMBERED_NATION)
                ).or_else(||
                    // n.b.: some of the montags have been mapped in the monsters step above
                    replace_use(&line, &mapped_definition.montags, &mod_line_scanner::USE_NUMBERED_MONTAG)
                ).or_else(||
                    replace_use(&line, &mapped_definition.event_codes, &mod_line_scanner::USE_NUMBERED_EVENTCODE)
                ).or_else(||
                    replace_use(&line, &mapped_definition.restricted_items, &mod_line_scanner::USE_NUMBERED_RESTRICTED_ITEM)
                ).or_else(||
                    replace_use(&line, &mapped_definition.items, &mod_line_scanner::USE_NUMBERED_ITEM)
                ).or_else(||
                    replace_use(&line, &mapped_definition.sites, &mod_line_scanner::USE_NUMBERED_SITE)
                )
                    // FIXME: enchantment mapping is currently broken
//                .or_else(||
//                    replace_use(&line, &mapped_definition.enchantments, &mod_line_scanner::USE_GLOBAL_ENCHANTMENT)
//                )
                .or_else(|| Some(line.clone()));

                if let Some(some_new_line) = new_line {
                    lines.push(some_new_line);
                }
            }
        }

    }

    let new_file = File::create("/mnt/c/Users/David/AppData/Roaming/Dominions5/mods/domingler-test.dm").unwrap();
    let mut writer = BufWriter::new(new_file);
    for line in lines {
        write!(&mut writer, "{}\n", line).unwrap();
    }
}

fn replace_use(line: &str, map: &HashMap<u32, u32>, regex: &Regex) -> Option<String> {
    if let Some(capture) = regex.captures(&line) {
        let found_id = u32::from_str(capture.name("id").unwrap().as_str()).unwrap();
        if let Some(new_id) = map.get(&found_id) {
            let new_line: String = regex.replace(&line, |ref captures: &Captures| -> String {
                format!("{}{}{}", &captures["prefix"], new_id, &captures["suffix"])
            }).to_string();
            Some(new_line)
        } else {
            Some(line.to_owned())
        }
    } else {
        None
    }
}
