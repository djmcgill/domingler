use crate::Definition;
use regex::Regex;
use std::str::FromStr;
use lazy_static::lazy_static;

pub const ASSUMED_FIRST_WEAPON_ID: u32 = 800;
pub const ASSUMED_FIRST_ARMOUR_ID: u32 = 300;
pub const ASSUMED_FIRST_MONSTER_ID: u32 = 4000;
pub const ASSUMED_FIRST_NAMETYPE_ID: u32 = 165;
pub const ASSUMED_FIRST_SPELL_ID: u32 = 1300;
pub const ASSUMED_FIRST_SITE_ID: u32 = 1500;
pub const ASSUMED_FIRST_NATION_ID: u32 = 120;
pub const ASSUMED_FIRST_ITEM_ID: u32 = 500;
pub const ASSUMED_FIRST_MONTAG_ID: u32 = 1000;
pub const ASSUMED_FIRST_EVENTCODE_ID: u32 = 300; // technically it's negative but whatever
pub const ASSUMED_FIRST_RESTRICTED_ITEM_ID: u32 = 1;

pub struct ModLineScanner {
    pub option_new_numbered_regex: Option<&'static Regex>,
    pub option_new_unnumbered_regex: Option<&'static Regex>,
    pub option_select_numbered_regex: Option<&'static Regex>,
    pub assumed_minimum: u32,
}

impl ModLineScanner {
    /// Captures:
    /// - #newthing <id>
    /// - #newthing
    /// - #selectthing <id>
    /// - #selectthing "name"
    /// Note that a line can be only one of those things so this function returns
    /// as soon as one of the regex matches
    /// Returns true if it matched anything
    pub fn scan_line<'a>(&self,
                         line: &'a str,
                         thing_definition: &mut Definition<'a>) -> bool {
        if let Some(new_numbered_regex) = self.option_new_numbered_regex {
            if let Some(capture) = new_numbered_regex.captures(line) {
                let found_id = u32::from_str(capture.name("id").unwrap().as_str()).unwrap();
                let not_already_there = thing_definition.defined_ids.insert(found_id);
                assert!(not_already_there);
                return true;
            }
        } else if let Some(new_unnumbered_regex) = self.option_new_unnumbered_regex {
            if new_unnumbered_regex.is_match(line) {
                thing_definition.implicit_definitions += 1;
                return true;
            }
        } else if let Some(select_numbered_regex) = self.option_select_numbered_regex {
            if let Some(capture) = select_numbered_regex.captures(line) {
                let found_id = u32::from_str(capture.name("id").unwrap().as_str()).unwrap();
                if found_id >= self.assumed_minimum {
                    thing_definition.defined_ids.insert(found_id);
                } else {
                    thing_definition.vanilla_edited_ids.insert(found_id);
                }
                return true;
            }
        }
        false
    }
}

lazy_static! {
    /// Weapons:
    /// - #newweapon <id>
    /// - #newweapon
    /// - #selectweapon <id>
    pub static ref WEAPON_LINE_SCANNER: ModLineScanner =
        ModLineScanner {
            option_new_numbered_regex: Some(&NEW_NUMBERED_WEAPON),
            option_new_unnumbered_regex: Some(&NEW_UNNUMBERED_WEAPON),
            option_select_numbered_regex: Some(&SELECT_NUMBERED_WEAPON),
            assumed_minimum: ASSUMED_FIRST_WEAPON_ID,
        };

    /// Armours:
    /// - #newarmor <id>
    /// - #newarmor
    /// - #selectarmor <id>
    pub static ref ARMOUR_LINE_SCANNER: ModLineScanner =
        ModLineScanner {
            option_new_numbered_regex: Some(&NEW_NUMBERED_ARMOUR),
            option_new_unnumbered_regex: Some(&NEW_UNNUMBERED_ARMOUR),
            option_select_numbered_regex: Some(&SELECT_NUMBERED_ARMOUR),
            assumed_minimum: ASSUMED_FIRST_ARMOUR_ID,
        };

    /// Monsters:
    /// - #newmonster <id>
    /// - #newmonster
    /// - #selectmonster <id>
    pub static ref MONSTER_LINE_SCANNER: ModLineScanner =
        ModLineScanner {
            option_new_numbered_regex: Some(&NEW_NUMBERED_MONSTER),
            option_new_unnumbered_regex: Some(&NEW_UNNUMBERED_MONSTER),
            option_select_numbered_regex: Some(&SELECT_NUMBERED_MONSTER),
            assumed_minimum: ASSUMED_FIRST_MONSTER_ID,
        };

    /// Spells:
    /// - #newspell
    /// - #selectspell <id>
    pub static ref SPELL_LINE_SCANNER: ModLineScanner =
        ModLineScanner {
            option_new_numbered_regex: None,
            option_new_unnumbered_regex: Some(&NEW_UNNUMBERED_SPELL),
            option_select_numbered_regex: Some(&SELECT_NUMBERED_SPELL),
            assumed_minimum: ASSUMED_FIRST_SPELL_ID,
        };

    /// Items:
    /// - #newitem
    /// - #selectitem <id>
    pub static ref ITEM_LINE_SCANNER: ModLineScanner =
        ModLineScanner {
            option_new_numbered_regex: None,
            option_new_unnumbered_regex: Some(&NEW_UNNUMBERED_ITEM),
            option_select_numbered_regex: Some(&SELECT_NUMBERED_ITEM),
            assumed_minimum: ASSUMED_FIRST_ITEM_ID,
        };

    /// Sites:
    /// - #newsite <id>
    /// - #newsite
    /// - #selectsite <id>
    pub static ref SITE_LINE_SCANNER: ModLineScanner =
        ModLineScanner {
            option_new_numbered_regex: Some(&NEW_NUMBERED_SITE),
            option_new_unnumbered_regex: Some(&NEW_UNNUMBERED_SITE),
            option_select_numbered_regex: Some(&SELECT_NUMBERED_SITE),
            assumed_minimum: ASSUMED_FIRST_SITE_ID,
        };

    /// Nations:
    /// - #newnation
    /// - #selectnation <id> (where id >= ASSUMED_FIRST_NATION_ID)
    pub static ref NATION_LINE_SCANNER: ModLineScanner =
        ModLineScanner {
            option_new_numbered_regex: None,
            option_new_unnumbered_regex: Some(&NEW_UNNUMBERED_NATION),
            option_select_numbered_regex: Some(&SELECT_NUMBERED_NATION),
            assumed_minimum: ASSUMED_FIRST_NATION_ID,
        };

    /// Name types:
    /// - #selectnametype <id>
    pub static ref NAMETYPE_LINE_SCANNER: ModLineScanner =
        ModLineScanner {
            option_new_numbered_regex: None,
            option_new_unnumbered_regex: None,
            option_select_numbered_regex: Some(&SELECT_NUMBERED_NAMETYPE),
            assumed_minimum: ASSUMED_FIRST_NAMETYPE_ID,
        };

    /// Montags:
    /// - #montag <id>
    pub static ref MONTAG_LINE_SCANNER: ModLineScanner =
        ModLineScanner {
            option_new_numbered_regex: None,
            option_new_unnumbered_regex: None,
            option_select_numbered_regex: Some(&SELECT_NUMBERED_MONTAG),
            assumed_minimum: ASSUMED_FIRST_MONTAG_ID,
        };

    /// Event codes:
    /// - #code -<id>
    /// - #code2 -<id>
    pub static ref EVENTCODE_LINE_SCANNER: ModLineScanner =
        ModLineScanner {
            option_new_numbered_regex: None,
            option_new_unnumbered_regex: None,
            option_select_numbered_regex: Some(&SELECT_NUMBERED_EVENTCODE),
            assumed_minimum: ASSUMED_FIRST_EVENTCODE_ID,
        };

    /// Restricted items:
    /// - #restricteditem <id>
    pub static ref RESTRICTED_ITEM_LINE_SCANNER: ModLineScanner =
        ModLineScanner {
            option_new_numbered_regex: None,
            option_new_unnumbered_regex: None,
            option_select_numbered_regex: Some(&SELECT_NUMBERED_RESTRICTED_ITEM),
            assumed_minimum: ASSUMED_FIRST_RESTRICTED_ITEM_ID,
        };

    // Weapons
    static ref NEW_NUMBERED_WEAPON: Regex = Regex::new("^\
        (?P<prefix>#newweapon )\
        (?P<id>[[:digit:]]+)\
        (?P<suffix>.*)$\
    ").unwrap();

    static ref NEW_UNNUMBERED_WEAPON: Regex = Regex::new("^\
        (?P<prefix>#newweapon)\
        (?P<suffix>.*)$\
    ").unwrap();

    static ref SELECT_NUMBERED_WEAPON: Regex = Regex::new("^\
        (?P<prefix>#selectweapon )\
        (?P<id>[[:digit:]]+)\
        (?P<suffix>.*)$\
    ").unwrap();

    pub static ref USE_NUMBERED_WEAPON: Regex = Regex::new("^\
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
    static ref NEW_UNNUMBERED_ARMOUR: Regex = Regex::new("^\
        (?P<prefix>#newarmor)\
        (?P<suffix>.*)$\
    ").unwrap();

    pub static ref USE_NUMBERED_ARMOUR: Regex = Regex::new("^\
        (?P<prefix>(#(?:\
            newarmor|\
            armor|\
            copyarmor) ))\
        (?P<id>[[:digit:]]+)\
        (?P<suffix>.*)$\
    ").unwrap();

    static ref NEW_NAMED_ARMOUR: Regex = Regex::new("^\
        (?P<prefix>#newarmor \")\
        (?P<name>[^\"]+)\
        (?P<suffix>\".*)$\
    ").unwrap();

    static ref SELECT_NUMBERED_ARMOUR: Regex = Regex::new("^\
        (?P<prefix>#selectarmor )\
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
    pub static ref USE_MONSTER: Regex = Regex::new("^\
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
        (?P<id>[-]?[[:digit:]]+)\
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
    static ref SELECT_NUMBERED_NAMETYPE: Regex = Regex::new("^\
        (?P<prefix>#selectnametype )\
        (?P<id>[[:digit:]]+)\
        (?P<suffix>.*)$\
    ").unwrap();

    // Montags
    static ref SELECT_NUMBERED_MONTAG: Regex = Regex::new("^\
        (?P<prefix>#montag )\
        (?P<id>[[:digit:]]+)\
        (?P<suffix>.*)$\
    ").unwrap();


    // Other
    static ref SELECT_NUMBERED_EVENTCODE: Regex = Regex::new("^\
        (?P<prefix>#(?:code|code2) -)\
        (?P<id>[[:digit:]]+)\
        (?P<suffix>.*)$\
    ").unwrap();

    static ref SELECT_NUMBERED_RESTRICTED_ITEM: Regex = Regex::new("^\
        (?P<prefix>#restricteditem )\
        (?P<id>[[:digit:]]+)\
        (?P<suffix>.*)$\
    ").unwrap();
}
