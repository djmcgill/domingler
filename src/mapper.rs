use crate::{LazyString, replace_use};
use std::collections::{HashMap, BTreeSet};
use crate::mod_definition::{MappedModDefinition, ModDefinition};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cell::RefCell;
use std::rc::Rc;
use regex::{Captures, Regex};
use std::str::FromStr;

pub fn remap_ids(mod_definitions: &HashMap<String, ModDefinition>) -> HashMap<String, MappedModDefinition> {
    let mut weapons_implicit_definition_count = 0;
    let mut armours_implicit_definition_count = 0;
    let mut monsters_implicit_definition_count = 0;
    let mut name_types_implicit_definition_count = 0;
    let mut spells_implicit_definition_count = 0;
    let mut items_implicit_definition_count = 0;
    let mut sites_implicit_definition_count = 0;
    let mut nations_implicit_definition_count = 0;
//    let mut events_implicit_definition_count = 0; // I think we don't remap events
    let mut poptype_implicit_definition_count = 0;
    let mut montags_implicit_definition_count = 0;
    let mut event_codes_implicit_definition_count = 0;
    let mut restricted_items_implicit_definition_count = 0;
    let mut enchantments_implicit_definition_count = 0;

    for mod_definition in mod_definitions.values() {
        weapons_implicit_definition_count += mod_definition.weapons.implicit_definitions;
        armours_implicit_definition_count += mod_definition.armours.implicit_definitions;
        monsters_implicit_definition_count += mod_definition.monsters.implicit_definitions;
        name_types_implicit_definition_count += mod_definition.name_types.implicit_definitions;
        spells_implicit_definition_count += mod_definition.spells.implicit_definitions;
        nations_implicit_definition_count += mod_definition.nations.implicit_definitions;
//        events_implicit_definition_count += mod_definition.events.implicit_definitions;
        poptype_implicit_definition_count += mod_definition.poptype.implicit_definitions;
        montags_implicit_definition_count += mod_definition.montags.implicit_definitions;
        event_codes_implicit_definition_count += mod_definition.event_codes.implicit_definitions;
        restricted_items_implicit_definition_count += mod_definition.restricted_items.implicit_definitions;
        items_implicit_definition_count += mod_definition.items.implicit_definitions;
        sites_implicit_definition_count += mod_definition.sites.implicit_definitions;
    }

    let mut first_available_weapon_id = crate::ASSUMED_FIRST_WEAPON_ID + weapons_implicit_definition_count;
    let mut first_available_armour_id = crate::ASSUMED_FIRST_ARMOUR_ID + armours_implicit_definition_count;
    let mut first_available_monster_id = crate::ASSUMED_FIRST_MONSTER_ID + monsters_implicit_definition_count;
    let mut first_available_name_type_id = crate::ASSUMED_FIRST_NAMETYPE_ID + name_types_implicit_definition_count;
    let mut first_available_spell_id = crate::ASSUMED_FIRST_SPELL_ID + spells_implicit_definition_count;

    // This has been really annoying let's just add a safety net
    let mut first_available_nations_id = 20 + crate::ASSUMED_FIRST_NATION_ID + nations_implicit_definition_count;

    let mut first_available_montags_id = crate::ASSUMED_FIRST_MONTAG_ID + montags_implicit_definition_count;
    let mut first_available_event_codes_id = crate::ASSUMED_FIRST_EVENTCODE_ID + event_codes_implicit_definition_count;
    let mut first_available_restricted_items_id = crate::ASSUMED_FIRST_RESTRICTED_ITEM_ID + restricted_items_implicit_definition_count;
    let mut first_available_enchantment_id = 0; // FIXME: should this be zero?
    let mut first_available_item_id = crate::ASSUMED_FIRST_ITEM_ID + items_implicit_definition_count;
    let mut first_available_site_id = crate::ASSUMED_FIRST_SITE_ID + sites_implicit_definition_count;

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

fn remap_particular_ids(first_available_id: &mut u32, mod_definitions: &BTreeSet<u32>) -> HashMap<u32, u32> {
    let mut mapped_ids = HashMap::new();

    for mod_definition_id in mod_definitions {
        mapped_ids.insert(*mod_definition_id, *first_available_id);
        *first_available_id += 1;
    }
    mapped_ids
}

pub fn apply_remapped_ids(lines: &mut Vec<LazyString>, remapped_ids: &HashMap<String, MappedModDefinition>) {
    use LazyString::*;

    for (path, mapped_definition) in remapped_ids {
        println!("Starting to map {}", path);
        let file = File::open(path).unwrap();
        let file_buff = BufReader::new(file);
        let line_iter = file_buff.lines().map(|result| result.unwrap());

        let mut option_current_spell_block_and_damage: Option<(Vec<String>, Rc<RefCell<String>>)> = None;

        let mut is_in_description = false;
        for line in line_iter {

            if is_in_description {
                if crate::MOD_DESCRIPTION_STOP.is_match(&line) {
                    // End of description, ditch this line and then continue as normal
                    is_in_description = false;
                    continue;
                } else {
                    // Throw away a description line
                    continue;
                }
            } else {

                if let Some((current_spell_block, damage_line)) = &mut option_current_spell_block_and_damage {
                    current_spell_block.push(line.clone());
                    if crate::SPELL_DAMAGE.is_match(&line) {
                        {
                            let mut b = damage_line.borrow_mut();
                            *b = line.clone();
                        }

                        let new_rc = Rc::clone(&damage_line);
                        lines.push(Thunk(Box::new(
                            move || {
                                let b = new_rc.borrow();
                                let st: &String = &*b;
                                st.clone()
                            }
                        ))); // help we have a #damage and we don't know how to map it yet
                        continue;
                    } else if crate::END.is_match(&line) {
                        // URGH going to need some lookahead on this
                        let mut option_effect = None;
                        for spell_line in current_spell_block.iter() {
                            if let Some(effect_capture) = crate::SPELL_EFFECT.captures(spell_line) {
                                let found_id = u64::from_str(effect_capture.name("id").unwrap().as_str()).unwrap();
                                option_effect = Some(found_id)
                            }
                        }
                        if let Some(effect) = option_effect {

                            if crate::ENCHANTMENT_EFFECTS.contains(&effect) {
                                // TODO: don't actually need to scan the regex twice
                                let mut b = damage_line.borrow_mut();
                                if let Some(damage_capture) = crate::SPELL_DAMAGE.captures(&b) {
                                    let found_id = u64::from_str(damage_capture.name("id").unwrap().as_str()).unwrap();
                                    if let Some(new_id) = mapped_definition.enchantments.get(&(found_id as u32)) {
                                        let new_string = crate::SPELL_DAMAGE.replace(&b, |ref captures: &Captures| -> String {
                                            format!("{}{}{}", &captures["prefix"], new_id, &captures["suffix"])
                                        }).to_string();
                                        *b = new_string;
                                    }
                                }
                            } else if crate::SUMMONING_EFFECTS.contains(&effect) {
                                // TODO: don't actually need to scan the regex twice
                                let mut b = damage_line.borrow_mut();
                                if let Some(damage_capture) = crate::SPELL_DAMAGE.captures(&b) {
                                    let found_id = i64::from_str(damage_capture.name("id").unwrap().as_str()).unwrap();
                                    if found_id > 0 {
                                        // lookup in monsters
                                        if let Some(new_id) = mapped_definition.monsters.get(&(found_id as u32)) {
                                            let new_string = crate::SPELL_DAMAGE.replace(&b, |ref captures: &Captures| -> String {
                                                format!("{}{}{}", &captures["prefix"], new_id, &captures["suffix"])
                                            }).to_string();
                                            *b = new_string;
                                        }
                                    } else {
                                        // lookup in montags. Found_id is negative
                                        if let Some(new_id) = mapped_definition.montags.get(&(-found_id as u32)) {
                                            let new_montag_id = - (*new_id as i32);
                                            let new_string = crate::SPELL_DAMAGE.replace(&b, |ref captures: &Captures| -> String {
                                                format!("{}{}{}", &captures["prefix"], new_montag_id, &captures["suffix"])
                                            }).to_string();
                                            *b = new_string;
                                        }
                                    }



                                }
                            }
                        }

                        option_current_spell_block_and_damage = None;
                    }
                } else if crate::SPELL_BLOCK_START.is_match(&line) {
                    // If we find a #newspell or a #selectspell, start recording lines
                    // TODO: make the string in refcell optional
                    option_current_spell_block_and_damage = Some((Vec::new(), Rc::new(RefCell::new("#damage whatever".to_owned()))));
                }
            }

            // TODO: also ditch icon and version and domversion
            if crate::MOD_NAME_LINE.is_match(&line) ||
                crate::MOD_DESCRIPTION_LINE.is_match(&line) ||
                crate::MOD_ICON_LINE.is_match(&line) ||
                crate::MOD_VERSION_LINE.is_match(&line) ||
                crate::MOD_DOMVERSION_LINE.is_match(&line) {
                // ditch the mod info
                continue;
            } else if crate::MOD_DESCRIPTION_START.is_match(&line) {
                // Description has started, ditch the line
                is_in_description = true;
                continue;
            } else {
                let new_line = replace_use(
                    &line,
                    &mapped_definition.weapons,
                    &crate::mod_line_scanner::USE_NUMBERED_WEAPON
                ).or_else(||
                    replace_use(
                        &line,
                        &mapped_definition.armours,
                        &crate::mod_line_scanner::USE_NUMBERED_ARMOUR)
                ).or_else(|| {
                    if let Some(capture) = crate::mod_line_scanner::USE_MONSTER.captures(&line) {

                        let found_id = i32::from_str(capture.name("id").unwrap().as_str()).unwrap();
                        if found_id > 0 {

                            if let Some(new_id) = mapped_definition.monsters.get(&(found_id as u32)) {
                                let new_line: String = crate::mod_line_scanner::USE_MONSTER.replace(&line, |ref captures: &Captures| -> String {
                                    format!("{}{}{}", &captures["prefix"], new_id, &captures["suffix"])
                                }).to_string();
                                Some(new_line)
                            } else {
                                Some(line.clone())
                            }
                        } else {
                            // it's a montag!
                            if let Some(new_id) = mapped_definition.montags.get(&(-found_id as u32)) {
                                let new_line: String = crate::mod_line_scanner::USE_MONSTER.replace(&line, |ref captures: &Captures| -> String {
                                    format!("{}-{}{}", &captures["prefix"], new_id, &captures["suffix"])
                                }).to_string();
                                Some(new_line)
                            } else {
                                Some(line.clone())
                            }
                        }
                    } else { None }
                }).or_else(||
                    replace_use(&line, &mapped_definition.name_types, &crate::mod_line_scanner::USE_NAMETYPE)
                ).or_else(||
                    replace_use(&line, &mapped_definition.spells, &crate::mod_line_scanner::USE_NUMBERED_SPELL)
                ).or_else(||
                    replace_use(&line, &mapped_definition.nations, &crate::mod_line_scanner::USE_NUMBERED_NATION)
                ).or_else(||
                    // n.b.: some of the montags have been mapped in the monsters step above
                    replace_use(&line, &mapped_definition.montags, &crate::mod_line_scanner::USE_NUMBERED_MONTAG)
                ).or_else(||
                    replace_use(&line, &mapped_definition.event_codes, &crate::mod_line_scanner::USE_NUMBERED_EVENTCODE)
                ).or_else(||
                    replace_use(&line, &mapped_definition.restricted_items, &crate::mod_line_scanner::USE_NUMBERED_RESTRICTED_ITEM)
                ).or_else(||
                    replace_use(&line, &mapped_definition.items, &crate::mod_line_scanner::USE_NUMBERED_ITEM)
                ).or_else(||
                    replace_use(&line, &mapped_definition.sites, &crate::mod_line_scanner::USE_NUMBERED_SITE)
                )
                .or_else(||
                    replace_use(&line, &mapped_definition.enchantments, &crate::mod_line_scanner::USE_GLOBAL_ENCHANTMENT)
                )
                    .or_else(|| Some(line.clone()));

                if let Some(some_new_line) = new_line {
                    lines.push(S(some_new_line));
                }
            }
        }

    }
}
