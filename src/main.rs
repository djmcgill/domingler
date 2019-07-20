#![recursion_limit = "128"]

use lazy_static::lazy_static;
use regex::{Regex, Captures};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::str::FromStr;

mod scanner;
mod mapper;

mod mod_line_scanner;
use mod_line_scanner::*;

mod mod_definition;
use mod_definition::*;

lazy_static! {
    static ref MOD_NAME: Regex = Regex::new(
        "^\
         (?P<prefix>#modname[ ]+\")\
         (?P<name>[^\"]+)\
         (?P<suffix>\".*)$\
         "
    )
    .unwrap();

    static ref END: Regex = Regex::new("^#end").unwrap();

    static ref SPELL_BLOCK_START: Regex = Regex::new("^#(newspell|selectspell)").unwrap();

    static ref SPELL_EFFECT: Regex = Regex::new("^\
        (?P<prefix>[ \t]*#effect[ ]+)\
        (?P<id>[-]?[[:digit:]]+)\
        (?P<suffix>.*)$\
        ").unwrap();
    static ref SPELL_DAMAGE: Regex = Regex::new("^\
        (?P<prefix>[ \t]*#damage[ ]+)\
        (?P<id>[-]?[[:digit:]]+)\
        (?P<suffix>.*)$\
        ").unwrap();
    static ref SPELL_COPY_ID: Regex = Regex::new("^\
        (?P<prefix>[ \t]*#copyspell[ ]+)\
        (?P<id>[-]?[[:digit:]]+)\
        (?P<suffix>.*)$\
        ").unwrap();
    static ref SPELL_COPY_NAME: Regex = Regex::new("^\
        (?P<prefix>[ \t]*#copyspell[ ]+\")\
        (?P<id>[^\"]+)\
        (?P<suffix>\".*)$\
        ").unwrap();

    static ref MOD_NAME_LINE: Regex = Regex::new("#modname").unwrap();
    static ref MOD_ICON_LINE: Regex = Regex::new("#icon").unwrap();
    static ref MOD_VERSION_LINE: Regex = Regex::new("#version").unwrap();
    static ref MOD_DOMVERSION_LINE: Regex = Regex::new("#domversion").unwrap();
    static ref MOD_DESCRIPTION_LINE: Regex = Regex::new("#description[ ]+\"[^\"]*\"").unwrap();
    // n.b. check for `MOD_DESCRIPTION_LINE` first
    static ref MOD_DESCRIPTION_START: Regex = Regex::new("#description").unwrap();

    // This must be the worst line of code I've ever written lol
    static ref MOD_DESCRIPTION_STOP: Regex = Regex::new("\"").unwrap();

    static ref SUMMONING_EFFECTS: HashSet<u64> = {
        let v = vec![
            1,     21,    31,    37,    38,    43,    50,    54,    62,    89,    93,    119,   126,   130,   137,
            10001, 10021, 10031, 10037, 10038, 10043, 10050, 10054, 10062, 10089, 10093, 10119, 10126, 10130, 10137];
        v.into_iter().collect()
    };

    static ref ENCHANTMENT_EFFECTS: HashSet<u64> = {
        let v = vec![81, 10081, 10082, 10084, 10085, 10086];
        v.into_iter().collect()
    };
}

// Unfortunately when we encounter a #damage it may refer to a monster, a montag, or an enchantment
// and we can't know until we've also looked at the spell's #effect
pub enum LazyString {
    S(String),
    Thunk(Box<dyn Fn() -> String>)
}

fn main() {
    // TODO: get this from the user somehow
    let mod_file_paths = vec![
        "/mnt/c/Users/David/AppData/Roaming/Dominions5/mods/EA_Karanaac_v1.26.dm",
        "/mnt/c/Users/David/AppData/Roaming/Dominions5/mods/Firepower.dm",
        "/mnt/c/Users/David/AppData/Roaming/Dominions5/mods/LA_Hollowmoor.dm",
        "/mnt/c/Users/David/AppData/Roaming/Dominions5/mods/Juhera_Iram_0.2.dm",
        "/mnt/c/Users/David/AppData/Roaming/Dominions5/mods/Warhammer-Complete.dm",
        "/mnt/c/Users/David/AppData/Roaming/Dominions5/mods/Myrmecos.dm",
        "/mnt/c/Users/David/AppData/Roaming/Dominions5/mods/D5_MA_Drangleic_1.02.dm",
        "/mnt/c/Users/David/AppData/Roaming/Dominions5/mods/Vespika.dm",
        "/mnt/c/Users/David/AppData/Roaming/Dominions5/mods/EA_Azarien.dm",
        "/mnt/c/Users/David/AppData/Roaming/Dominions5/mods/ExtraPretenders1_8.dm",
        "/mnt/c/Users/David/AppData/Roaming/Dominions5/mods/SILT_v7.dm",
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
    let parsed_mods = scanner::scan_all_mods(&mod_files);
    scanner::print_mod_id_usages(&parsed_mods);

    let remapped_ids = mapper::remap_ids(&parsed_mods);

    // TODO: add the mod names to the description
    let mut lines: Vec<LazyString> = vec![
        LazyString::S("#modname \"domingler mod test\"".to_owned()),
        LazyString::S(format!("#description \"a combination of: some shit or whatever\"")),
    ];

    // TODO: pick an era and then also map all nations to same era?
    mapper::apply_remapped_ids(&mut lines, &remapped_ids);


    let new_file = File::create("/mnt/c/Users/David/AppData/Roaming/Dominions5/mods/domingler-test.dm").unwrap();
    let mut writer = BufWriter::new(new_file);
    for line in lines {
        match line {
            LazyString::S(line_string) => write!(&mut writer, "{}\n", line_string).unwrap(),
            LazyString::Thunk(line_fn) => write!(&mut writer, "{}\n", line_fn()).unwrap(),
        }
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
