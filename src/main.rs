use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::io::BufRead;
use regex::{Regex, Captures};
use std::str::FromStr;
use lazy_static::lazy_static;
use std::io::Write;
use std::iter::Extend;

const ASSUMED_FIRST_WEAPON_ID: u32 = 800;
const ASSUMED_FIRST_ARMOUR_ID: u32 = 300;
const ASSUMED_FIRST_MONSTER_ID: u32 = 4000;
//Weapons: 0-1999, 800+ for modding
//Armor: 0-999, 300+ for modding
//Monsters: 0-8999, 3500+ for modding
//Nametypes: 100-299, 165+ for modding
//Spells: 0-3999, 1300+ for modding
//Items: 0-999, 500+ for modding
//Magic Sites: 0-1999, 1500+ for modding
//Nations: 0-249, 120+ for modding
//Descriptions: 1999 characters


lazy_static! {
    static ref NEW_WEAPON: Regex = Regex::new("^\
        (?P<prefix>#newweapon )\
        (?P<id>[[:digit:]]+)\
        (?P<suffix>.*)\
    ").unwrap();
    static ref USE_WEAPON: Regex = Regex::new("^\
        (?P<prefix>(#(?:\
            newweapon|\
            weapon|\
            copyweapon|\
            secondaryeffect|\
            secondaryeffectalways|\
            selectweapon) ))\
        (?P<id>[[:digit:]]+)\
        (?P<suffix>.*)\
    ").unwrap();

    static ref NEW_ARMOUR: Regex = Regex::new("^\
        (?P<prefix>#newarmor )\
        (?P<id>[[:digit:]]+)\
        (?P<suffix>.*)\
    ").unwrap();
    static ref USE_ARMOUR: Regex = Regex::new("^\
        (?P<prefix>(#(?:\
            newarmor|\
            armor|\
            copyarmor) ))\
        (?P<id>[[:digit:]]+)\
        (?P<suffix>.*)\
    ").unwrap();

    static ref NEW_MONSTER: Regex = Regex::new("^\
        (?P<prefix>#newmonster )\
        (?P<id>[[:digit:]]+)\
        (?P<suffix>.*)\
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
        (?P<suffix>.*)\
    ").unwrap();

}

fn create_id_map(path: &str, declaration_regex: &Regex, first_available_id: &mut u32) -> HashMap<u32, u32> {
    let mut found_ids = vec![];
    let file = File::open(path).unwrap();
    let file_buff = BufReader::new(file);
    for line in file_buff.lines() {
        let line = line.unwrap();
        for line_capture in declaration_regex.captures(&line) {
            let found_id = u32::from_str(line_capture.name("id").unwrap().as_str()).unwrap();
            println!("found id {}", found_id);
            found_ids.push(found_id);
        }
    }
    found_ids.sort_unstable();

    let mut mapped_ids = HashMap::new();

    for found_id in found_ids {
        let mapped_id = *first_available_id;
        *first_available_id += 1;
        mapped_ids.insert(found_id, mapped_id);
    }

    mapped_ids
}

fn map_ids(debug_str: &'static str,
           lines: &mut Vec<String>,
           use_regex: &Regex,
           mapped_ids: &HashMap<u32, u32>) {

    for line in lines {
        let option_replacement = match use_regex.captures(line) {
            None => None,
            Some(line_capture) => {
//                println!("capture: {:?}", line_capture);
                let found_id = u32::from_str(line_capture.name("id").unwrap().as_str()).unwrap();
                match mapped_ids.get(&found_id) {
                    None => None,
                    Some(new_id) => {
                        print!("found use of modded {} {}, mapping to {} ", debug_str, found_id, new_id);
                        let new_line: String = use_regex.replace(&line, |ref captures: &Captures| -> String {
                            format!("{}{}{}", &captures["prefix"], new_id, &captures["suffix"])
                        }).to_string();
                        println!("new line is: '{}'", new_line);
                        Some(new_line)
                    }
                }
            }
        };
        for replacement in option_replacement {
            *line = replacement;
        }
    }
}
// FIXME: remove mod name, icon, description, version

fn main() {
    let mod_files = vec!["/mnt/c/Users/David/AppData/Roaming/Dominions5/mods/jBBM_v_0_5_9c.dm", "/mnt/c/Users/David/AppData/Roaming/Dominions5/mods/EA_Karanaac_v1.26.dm"];
    let mut first_available_weapon_id = ASSUMED_FIRST_WEAPON_ID;
    let mut first_available_armour_id = ASSUMED_FIRST_ARMOUR_ID;
    let mut first_available_monster_id = ASSUMED_FIRST_MONSTER_ID;

    let mut lines: Vec<String> = vec![
        "#modname \"domingler mod test\"".to_owned(),
        format!("#description \"a combination of: some shit or whatever\""),
    ];
// todo: add the mod names to the description
    for mod_file in mod_files {
        let file = File::open(mod_file).unwrap();
        let file_buff = BufReader::new(file);
        let line_iter = file_buff.lines().map(|result| result.unwrap());
        let mut mod_lines: Vec<String> = line_iter.collect(); // whatever it's late I'm tired

        println!("Looking for #newweapon:");
        let mapped_weapon_ids = create_id_map(mod_file, &NEW_WEAPON, &mut first_available_weapon_id);

        println!("Looking for weapon use:");

        map_ids("weapon", &mut mod_lines, &USE_WEAPON, &mapped_weapon_ids);

        println!("Looking for #newarmor:");
        let mapped_armor_ids = create_id_map(mod_file, &NEW_ARMOUR, &mut first_available_armour_id);

        println!("Looking for armour use:");
        map_ids("armour", &mut mod_lines, &USE_ARMOUR, &mapped_armor_ids);

        println!("Looking for #newmonster:");
        let mapped_monster_ids = create_id_map(mod_file, &NEW_MONSTER, &mut first_available_monster_id);

        println!("Looking monster use:");
        map_ids("monster", &mut mod_lines, &USE_MONSTER, &mapped_monster_ids);

        lines.extend(mod_lines);
    }

    let new_file = File::create("/mnt/c/Users/David/AppData/Roaming/Dominions5/mods/domingler-test.dm").unwrap();
    let mut writer = BufWriter::new(new_file);
    for line in lines {
        write!(&mut writer, "{}\n", line).unwrap();
    }
}
