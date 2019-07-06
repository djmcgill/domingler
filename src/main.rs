use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::io::BufRead;
use regex::{Regex, Captures};
use std::str::FromStr;
use lazy_static::lazy_static;
use std::io::Write;
use std::iter::Extend;
use std::ptr::hash;

const ASSUMED_FIRST_WEAPON_ID: u32 = 800;
const ASSUMED_FIRST_ARMOUR_ID: u32 = 300;
const ASSUMED_FIRST_MONSTER_ID: u32 = 4000;
const ASSUMED_FIRST_NAMETYPE_ID: u32 = 165;
const ASSUMED_FIRST_SPELL_ID: u32 = 1300;
const ASSUMED_FIRST_SITE_ID: u32 = 1500;
const ASSUMED_FIRST_NATION_ID: u32 = 120;
const ASSUMED_FIRST_ITEM_ID: u32 = 500;
const ASSUMED_FIRST_MONTAG_ID: u32 = 1000;

// 1: go all over the mods, count the unnumbered definitions and mark the ids used
// 2: remap IDs
// 3: make new mod file

//Weapons: 0-1999, 800+ for modding
//Armor: 0-999, 300+ for modding
//Monsters: 0-8999, 3500+ for modding
//Spells: 0-3999, 1300+ for modding
//Items: 0-999, 500+ for modding
//Magic Sites: 0-1999, 1500+ for modding
//Nations: 0-249, 120+ for modding
//Nametypes: 100-299, 165+ for modding
//Montags: 1000+ for modding
// Events
// ???
//--item restrictions 8056 8066-8068 8081
//--sites --
//--nations 246
//--enchantments 567 569 607-614???



struct Definition {
    defined_ids: HashSet<u32>,
    implicit_definitions: usize,
}
impl Default for Definition {
    fn default() -> Self {
        Self {
            defined_ids: HashSet::new(),
            implicit_definitions: 0,
        }
    }
}
#[derive(Default)]
struct ModDefinition {
    weapons: Definition,
    armours: Definition,
    monsters: Definition,
    name_types: Definition,
    spells: Definition,
    items: Definition,
    sites: Definition,
    nations: Definition,
    events: Definition,
    poptype: Definition,
    montags: Definition,
}


lazy_static! {
    // Weapons
    static ref NEW_NUMBERED_WEAPON: Regex = Regex::new("^\
        (?P<prefix>#newweapon )\
        (?P<id>[[:digit:]]+)\
        (?P<suffix>.*)$\
    ").unwrap();
    static ref USE_NUMBERED_WEAPON: Regex = Regex::new("^\
        (?P<prefix>(#(?:\
            newweapon|\
            weapon|\
            copyweapon|\
            secondaryeffect|\
            secondaryeffectalways|\
            selectweapon) ))\
        (?P<id>[[:digit:]]+)\
        (?P<suffix>.*)$\
    ").unwrap();

    // Armours
    static ref NEW_NUMBERED_ARMOUR: Regex = Regex::new("^\
        (?P<prefix>#newarmor )\
        (?P<id>[[:digit:]]+)\
        (?P<suffix>.*)$\
    ").unwrap();
    static ref USE_NUMBERED_ARMOUR: Regex = Regex::new("^\
        (?P<prefix>(#(?:\
            newarmor|\
            armor|\
            copyarmor) ))\
        (?P<id>[[:digit:]]+)\
        (?P<suffix>.*)$\
    ").unwrap();

    // Spells
    static ref NEW_UNNUMBERED_SPELL: Regex = Regex::new("^\
        (?P<prefix>#newspell)\
        (?P<suffix>.*)$\
    ").unwrap();

    static ref SELECT_NUMBERED_SPELL: Regex = Regex::new("^\
        (?P<prefix>#selectspell )\
        (?P<id>[[:digit:]]+)\
        (?P<suffix>.*)$\
    ").unwrap();

    static ref USE_NUMBERED_SPELL: Regex = Regex::new("^\
        (?P<prefix>(#(?:\
            selectspell|\
            copyspell|\
            nextspell|\
        ) ))\
        (?P<id>[[:digit:]]+)\
        (?P<suffix>.*)$\
    ").unwrap();

    // Items
   static ref NEW_UNNUMBERED_ITEM: Regex = Regex::new("^\
        (?P<prefix>#newitem)\
        (?P<suffix>.*)$\
    ").unwrap();

    static ref SELECT_NUMBERED_ITEM: Regex = Regex::new("^\
        (?P<prefix>#selectitem )\
        (?P<id>[[:digit:]]+)\
        (?P<suffix>.*)$\
    ").unwrap();

    // Sites
    static ref NEW_NUMBERED_SITE: Regex = Regex::new("^\
        (?P<prefix>#newsite )\
        (?P<id>[[:digit:]]+)\
        (?P<suffix>.*)$\
    ").unwrap();

    static ref SELECT_NUMBERED_SITE: Regex = Regex::new("^\
        (?P<prefix>#selectsite )\
        (?P<id>[[:digit:]]+)\
        (?P<suffix>.*)$\
    ").unwrap();

    // remember to check for numbered sites first
    static ref NEW_UNNUMBERED_SITE: Regex = Regex::new("^\
        (?P<prefix>#newsite)\
        (?P<suffix>.*)$\
    ").unwrap();

    // Monsters
    static ref NEW_NUMBERED_MONSTER: Regex = Regex::new("^\
        (?P<prefix>#newmonster )\
        (?P<id>[[:digit:]]+)\
        (?P<suffix>.*)$\
    ").unwrap();

    static ref SELECT_NUMBERED_MONSTER: Regex = Regex::new("^\
        (?P<prefix>#selectmonster )\
        (?P<id>[[:digit:]]+)\
        (?P<suffix>.*)$\
    ").unwrap();

    // "#newmonster", or "#newmonster -- whatever"
    // n.b. make sure to check it doesn't match the numbered (or named) monster first!
    static ref NEW_UNNUMBERED_MONSTER: Regex = Regex::new("^\
        (?P<prefix>#newmonster)\
        (?P<suffix>.*)$\
    ").unwrap();

    // FIXME: negative number for montags
    static ref USE_MONSTER: Regex = Regex::new("^\
        (?P<prefix>(#(?:\
            newmonster|\
            copyspr|\
            monpresentrec|\
            ownsmonrec|\
            raiseshape|\
            shapechange|\
            prophetshape|\
            firstshape|\
            secondshape|\
            secondtmpshape|\
            forestshape|\
            plainshape|\
            foreignshape|\
            homeshape|\
            springshape|\
            summershape|\
            autumnshape|\
            wintershape|\
            landshape|\
            watershape|\
            domsummon|\
            domsummon2|\
            raredomsummon|\
            templetrainer|\
            makemonsters1|\
            makemonsters2|\
            makemonsters3|\
            makemonsters4|\
            makemonsters5|\
            summon1|\
            summon2|\
            summon3|\
            summon4|\
            summon5|\
            battlesum1|\
            battlesum2|\
            battlesum3|\
            battlesum4|\
            battlesum5|\
            batstartsum1|\
            batstartsum2|\
            batstartsum3|\
            batstartsum4|\
            batstartsum5|\
            batstartsum1d6|\
            batstartsum2d6|\
            batstartsum3d6|\
            batstartsum4d6|\
            batstartsum5d6|\
            batstartsum6d6|\
            batstartsum7d6|\
            batstartsum8d6|\
            batstartsum9d6|\
            farsumcom|\
            onlymnr|\
            homemon|\
            homecom|\
            mon|\
            com|\
            summon|\
            summonlvl2|\
            summonlvl3|\
            summonlvl4|\
            wallcom|\
            wallunit|\
            uwwallunit|\
            uwwallcom|\
            startcom|\
            coastcom1|\
            coastcom2|\
            addforeignunit|\
            addforeigncom|\
            forestrec|\
            mountainrec|\
            swamprec|\
            wasterec|\
            caverec|\
            startscout|\
            forestcom|\
            mountaincom|\
            swampcom|\
            wastecom|\
            cavecom|\
            startunittype1|\
            startunittype2|\
            addrecunit|\
            addreccom|\
            uwrec|\
            uwcom|\
            coastunit1|\
            coastunit2|\
            coastunit3|\
            landrec|\
            landcom|\
            hero1|\
            hero2|\
            hero3|\
            hero4|\
            hero5|\
            hero6|\
            hero7|\
            hero8|\
            hero9|\
            hero10|\
            multihero1|\
            multihero2|\
            multihero3|\
            multihero4|\
            multihero5|\
            multihero6|\
            multihero7|\
            defcom1|\
            defcom2|\
            defunit1|\
            defunit1b|\
            defunit1c|\
            defunit1d|\
            defunit2|\
            defunit2b|\
            wallcom|\
            wallunit|\
            uwwallunit|\
            uwwallcom|\
            addgod|\
            delgod|\
            cheapgod20|\
            cheapgod40|\
            addrecunit|\
            addreccom|\
            copystats) ))\
        (?P<id>[[:digit:]]+)\
        (?P<suffix>.*)$\
    ").unwrap();

    // Nations
    static ref SELECT_NUMBERED_NATION: Regex = Regex::new("^\
        (?P<prefix>#selectnation )\
        (?P<id>[[:digit:]]+)\
        (?P<suffix>.*)$\
    ").unwrap();

    static ref NEW_UNNUMBERED_NATION: Regex = Regex::new("^\
        (?P<prefix>#newnation)\
        (?P<suffix>.*)$\
    ").unwrap();

    // Name types
    static ref SELECT_NUMBERED_NAME_TYPE: Regex = Regex::new("^\
        (?P<prefix>#selectnametype )\
        (?P<id>[[:digit:]]+)\
        (?P<suffix>.*)$\
    ").unwrap();

    // Montags
    static ref NEW_NUMBERED_MONTAG: Regex = Regex::new("^\
        (?P<prefix>#montag )\
        (?P<id>[[:digit:]]+)\
        (?P<suffix>.*)$\
    ").unwrap();


    // Other
    static ref MOD_NAME: Regex = Regex::new("^\
        (?P<prefix>#modname \")\
        (?P<name>[^\"]+)\
        (?P<suffix>\".*)$\
    ").unwrap();
}

fn scan_all_mods(mods: &Vec<Vec<String>>) -> HashMap<String, ModDefinition> {
    let mut hash_map = HashMap::new();
    for mod_lines in mods {
        let mut option_mod_name: Option<String> = None;
        let mut mod_definition: ModDefinition = ModDefinition::default();

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

            // Capture declarations:
            // Weapons:
            // - #newweapon <id>
            if let Some(line_capture) = NEW_NUMBERED_WEAPON.captures(&line) {
                let found_id = u32::from_str(line_capture.name("id").unwrap().as_str()).unwrap();
                let not_already_there = mod_definition.weapons.defined_ids.insert(found_id);
                assert!(not_already_there);
            }
            // Armours:
            // - #newarmour <id>
            if let Some(line_capture) = NEW_NUMBERED_ARMOUR.captures(&line) {
                let found_id = u32::from_str(line_capture.name("id").unwrap().as_str()).unwrap();
                let not_already_there = mod_definition.armours.defined_ids.insert(found_id);
                assert!(not_already_there);
            }
            // Spells:
            // - #selectspell <id> (where id >= ASSUMED_FIRST_SPELL_ID)
            // - #newspell
            if let Some(line_capture) = SELECT_NUMBERED_SPELL.captures(&line) {
                let found_id = u32::from_str(line_capture.name("id").unwrap().as_str()).unwrap();
                if found_id >= ASSUMED_FIRST_SPELL_ID {
                    let not_already_there = mod_definition.spells.defined_ids.insert(found_id);
                    assert!(not_already_there);
                }
            } else if NEW_UNNUMBERED_SPELL.is_match(&line) {
                mod_definition.spells.implicit_definitions += 1;
            }
            // Monsters:
            // - #newmonster <id>
            // - #newmonster
            // - #selectmonster <id> (where id >= ASSUMED_FIRST_MONSTER_ID)
            if let Some(line_capture) = NEW_NUMBERED_MONSTER.captures(&line) {
                let found_id = u32::from_str(line_capture.name("id").unwrap().as_str()).unwrap();
                let not_already_there = mod_definition.monsters.defined_ids.insert(found_id);
                assert!(not_already_there);
            } else if NEW_UNNUMBERED_MONSTER.is_match(&line) {
                mod_definition.monsters.implicit_definitions += 1;
            } else if let Some(line_capture) = SELECT_NUMBERED_MONSTER.captures(&line) {
                let found_id = u32::from_str(line_capture.name("id").unwrap().as_str()).unwrap();
                if found_id >= ASSUMED_FIRST_MONSTER_ID {
                    let not_already_there = mod_definition.monsters.defined_ids.insert(found_id);
                    assert!(not_already_there);
                }
            }

            // Items:
            // - #selectitem <id> (where id >= ASSUMED_FIRST_ITEM_ID)
            // - #newitem
            if let Some(line_capture) = SELECT_NUMBERED_ITEM.captures(&line) {
                let found_id = u32::from_str(line_capture.name("id").unwrap().as_str()).unwrap();
                if found_id >= ASSUMED_FIRST_ITEM_ID {
                    let not_already_there = mod_definition.items.defined_ids.insert(found_id);
                    assert!(not_already_there);
                }
            } else if NEW_UNNUMBERED_ITEM.is_match(&line) {
                mod_definition.items.implicit_definitions += 1;
            }

            // Sites:
            // - #newsite <id>
            // - #newsite
            // - #selectsite <id> (where id >= ASSUMED_FIRST_SITE_ID)
            if let Some(line_capture) = NEW_NUMBERED_SITE.captures(&line) {
                let found_id = u32::from_str(line_capture.name("id").unwrap().as_str()).unwrap();
                let not_already_there = mod_definition.sites.defined_ids.insert(found_id);
                assert!(not_already_there);
            } else if NEW_UNNUMBERED_SITE.is_match(&line) {
                mod_definition.sites.implicit_definitions += 1;
            } else if let Some(line_capture) = SELECT_NUMBERED_SITE.captures(&line) {
                let found_id = u32::from_str(line_capture.name("id").unwrap().as_str()).unwrap();
                if found_id >= ASSUMED_FIRST_SITE_ID {
                    let not_already_there = mod_definition.sites.defined_ids.insert(found_id);
                    assert!(not_already_there);
                }
            }

            // Nations:
            // - #selectnation <id> (where id >= ASSUMED_FIRST_NATION_ID)
            // - #newnation
            if let Some(line_capture) = SELECT_NUMBERED_NATION.captures(&line) {
                let found_id = u32::from_str(line_capture.name("id").unwrap().as_str()).unwrap();
                if found_id >= ASSUMED_FIRST_NATION_ID {
                    let not_already_there = mod_definition.nations.defined_ids.insert(found_id);
                    assert!(not_already_there);
                }
            } else if NEW_UNNUMBERED_NATION.is_match(&line) {
                mod_definition.nations.implicit_definitions += 1;
            }

            // Name types:
            // - #selectnametype <id> (where id >= ASSUMED_FIRST_NAME_TYPE_ID)
            if let Some(line_capture) = SELECT_NUMBERED_NAME_TYPE.captures(&line) {
                let found_id = u32::from_str(line_capture.name("id").unwrap().as_str()).unwrap();
                if found_id >= ASSUMED_FIRST_NAMETYPE_ID {
                    let not_already_there = mod_definition.name_types.defined_ids.insert(found_id);
                    assert!(not_already_there);
                }
            }

            // Monster tags:
            // - #montag <id> (where id >= ASSUMED_FIRST_MONTAG_ID)
            if let Some(line_capture) = NEW_NUMBERED_MONTAG.captures(&line) {
                let found_id = u32::from_str(line_capture.name("id").unwrap().as_str()).unwrap();
                if found_id >= ASSUMED_FIRST_MONTAG_ID {
                    // With montags they're implicitly declared as they're used
                    mod_definition.montags.defined_ids.insert(found_id);
                }
            }

            // FIXME: events
            // FIXME: poptypes
        }

        let mod_name = option_mod_name.unwrap_or_else(|| panic!("Could not find the mod's name"));
        hash_map.insert(mod_name, mod_definition);
    }
    hash_map
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
        print_min_max("Nations", &definition.nations.defined_ids);
        print_min_max("Nametypes", &definition.name_types.defined_ids);
        print_min_max("Montags", &definition.montags.defined_ids);
        println!();
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
fn min_max<'a>(mut items: impl Iterator<Item=&'a u32>) -> Option<(u32, Option<u32>)> {
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
    let mod_files: Vec<Vec<String>> = mod_file_paths.into_iter().map(|path| {
        let file = File::open(path).unwrap();
        let file_buff = BufReader::new(file);
        let line_iter = file_buff.lines().map(|result| result.unwrap());
        let lines: Vec<String> = line_iter.collect();
        lines
    }).collect();
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
