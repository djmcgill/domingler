use std::collections::{HashSet, HashMap};
use std::marker::PhantomData;

// A particular thing: weapons, armours, etc
// TODO: could be stricter about these fields, since not all things have all 4
pub struct Definition<'a> {
    // From #newfoo <id> and
    // #selectfoo <id> where id >= FIRST_ASSUMED_ID
    pub defined_ids: HashSet<u32>,

    // From #newfoo with no id
    pub implicit_definitions: u32,

    // From #selectfoo <id> where id < FIRST_ASSUMED_ID
    pub vanilla_edited_ids: HashSet<u32>,

    // PLACEHOLDER: because I know I'm going to have to put strings in here again
    whatever: PhantomData<&'a ()>,
}
impl<'a> Default for Definition<'a> {
    fn default() -> Self {
        Self {
            defined_ids: HashSet::new(),
            implicit_definitions: 0,
            vanilla_edited_ids: HashSet::new(),
            whatever: PhantomData,
        }
    }
}
#[derive(Default)]
pub struct ModDefinition<'a> {
    pub name: String,
    pub weapons: Definition<'a>,
    pub armours: Definition<'a>,
    pub monsters: Definition<'a>,
    pub name_types: Definition<'a>,
    pub spells: Definition<'a>,
    pub items: Definition<'a>,
    pub sites: Definition<'a>,
    pub nations: Definition<'a>,
    pub events: Definition<'a>,
    pub poptype: Definition<'a>,
    pub montags: Definition<'a>,
    pub event_codes: Definition<'a>,
    pub restricted_items: Definition<'a>,
    pub enchantments: HashSet<u32>,
}

pub struct MappedModDefinition {
    pub weapons: HashMap<u32, u32>,
    pub armours: HashMap<u32, u32>,
    pub monsters: HashMap<u32, u32>,
    pub name_types: HashMap<u32, u32>,
    pub spells: HashMap<u32, u32>,
    pub nations: HashMap<u32, u32>,
    pub montags: HashMap<u32, u32>,
    pub event_codes: HashMap<u32, u32>,
    pub restricted_items: HashMap<u32, u32>,
}
