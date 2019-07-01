use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use regex::{Regex, Captures};
use std::str::FromStr;

const ASSUMED_FIRST_WEAPON_ID: usize = 1000;

fn main() {
    let mut mod_files = vec!["/mnt/c/Users/David/AppData/Roaming/Dominions5/mods/EA_Karanaac_v1.26.dm", "/mnt/c/Users/David/AppData/Roaming/Dominions5/mods/EA_Karanaac_v1.26.dm"];
    let mut first_available_weapon_id = ASSUMED_FIRST_WEAPON_ID;

    let new_weapon_regex = Regex::new(r"^(?P<prefix>#newweapon )(?P<id>[[:digit:]]*)(?P<suffix>.*)").unwrap();
    let use_weapon_regex = Regex::new(r"^(?P<prefix>(#(?:newweapon|weapon) ))(?P<id>[[:digit:]]*)(?P<suffix>.*)").unwrap();

    for mod_file in mod_files {
        let mut found_weapon_ids: Vec<usize> = vec![];
        let mut mapped_weapon_ids: HashMap<usize, usize> = HashMap::new();

        println!("Looking for #newweapon:");
        {
            let file = File::open(mod_file).unwrap();
            let file_buff = BufReader::new(file);
            for line in file_buff.lines() {
                let line = line.unwrap();
                for line_capture in new_weapon_regex.captures(&line) {
                    let found_id = usize::from_str(line_capture.name("id").unwrap().as_str()).unwrap();
                    found_weapon_ids.push(found_id);
                }
            }
            found_weapon_ids.sort_unstable();

            for found_weapon_id in found_weapon_ids {
                let mapped_id = first_available_weapon_id;
                first_available_weapon_id += 1;
                mapped_weapon_ids.insert(found_weapon_id, mapped_id);
            }
        }

        println!("Looking for #weapon:");
        let file = File::open(mod_file).unwrap();
        let file_buff = BufReader::new(file);
        for line in file_buff.lines() {
            let line = line.unwrap();
            for line_capture in use_weapon_regex.captures(&line) {
                let found_id = usize::from_str(line_capture.name("id").unwrap().as_str()).unwrap();
                match mapped_weapon_ids.get(&found_id) {
                    None => {}
                    Some(new_id) => {
                        print!("found use of modded weapon {}, mapping to {} ", found_id, new_id);
                        let new_line = use_weapon_regex.replace(&line, |ref captures: &Captures| -> String {
                            format!("{}{}{}", &captures["prefix"], new_id, &captures["suffix"])
                        });
                        println!("new line is: '{}'", new_line);
                    }
                }
            }
        }

    }

}
