use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use regex::{Regex, Captures};
use std::str::FromStr;

const ASSUMED_FIRST_WEAPON_ID: u32 = 1000;

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

fn map_ids<'iter, 'b: 'iter, 'c: 'iter> (source: impl Iterator<Item=String> + 'iter, use_regex: &'b Regex, mapped_ids: &'c HashMap<u32, u32>) -> impl Iterator<Item=String> + 'iter {
    source.map(move |line| {
        match use_regex.captures(&line) {
            None => line,
            Some(line_capture) => {
                let found_id = u32::from_str(line_capture.name("id").unwrap().as_str()).unwrap();
                match mapped_ids.get(&found_id) {
                    None => line,
                    Some(new_id) => {
                        print!("found use of modded weapon {}, mapping to {} ", found_id, new_id);
                        let new_line: String = use_regex.replace(&line, |ref captures: &Captures| -> String {
                            format!("{}{}{}", &captures["prefix"], new_id, &captures["suffix"])
                        }).to_string();
                        println!("new line is: '{}'", new_line);
                        new_line
                    }
                }
            }
        }
    })
}

fn main() {
    let mod_files = vec!["/mnt/c/Users/David/AppData/Roaming/Dominions5/mods/EA_Karanaac_v1.26.dm", "/mnt/c/Users/David/AppData/Roaming/Dominions5/mods/EA_Karanaac_v1.26.dm"];
    let mut first_available_weapon_id = ASSUMED_FIRST_WEAPON_ID;

    let new_weapon_regex = Regex::new(r"^(?P<prefix>#newweapon )(?P<id>[[:digit:]]*)(?P<suffix>.*)").unwrap();
    let use_weapon_regex = Regex::new(r"^(?P<prefix>(#(?:newweapon|weapon) ))(?P<id>[[:digit:]]*)(?P<suffix>.*)").unwrap();

    for mod_file in mod_files {
        println!("Looking for #newweapon:");
        let mapped_weapon_ids = create_id_map(mod_file, &new_weapon_regex, &mut first_available_weapon_id);

        println!("Looking for #weapon:");
        let file = File::open(mod_file).unwrap();
        let file_buff = BufReader::new(file);
        let lines = file_buff.lines().map(|result| result.unwrap());
        let new_file: Vec<String> = map_ids(lines, &use_weapon_regex, &mapped_weapon_ids).collect();


    }

}
