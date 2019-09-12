use std::collections::{BTreeMap, BTreeSet};
use std::marker::PhantomData;

// A particular thing: weapons, armours, etc
// TODO: could be stricter about these fields, since not all things have all 4
pub struct Definition<'a> {
    // From #newfoo <id> and
    // #selectfoo <id> where id >= FIRST_ASSUMED_ID
    pub explicit_definitions: BTreeMap<u32, Vec<String>>,

    // From #newfoo with no id
    pub implicit_definitions: BTreeSet<Vec<String>>,

    // From #selectfoo <id> where id < FIRST_ASSUMED_ID
    pub vanilla_edited_ids: BTreeSet<u32>,

    // PLACEHOLDER: because I know I'm going to have to put strings in here again
    whatever: PhantomData<&'a ()>,
}
impl<'a> Default for Definition<'a> {
    fn default() -> Self {
        Self {
            explicit_definitions: BTreeMap::new(),
            implicit_definitions: BTreeSet::new(),
            vanilla_edited_ids: BTreeSet::new(),
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
    pub enchantments: BTreeSet<u32>, // no implicit or named declares
}

pub struct MappedModDefinition {
    pub weapons: BTreeMap<u32, u32>,
    pub armours: BTreeMap<u32, u32>,
    pub monsters: BTreeMap<u32, u32>,
    pub name_types: BTreeMap<u32, u32>,
    pub spells: BTreeMap<u32, u32>,
    pub items: BTreeMap<u32, u32>,
    pub sites: BTreeMap<u32, u32>,
    pub nations: BTreeMap<u32, u32>,
    // pub events // can't clash by id
    pub montags: BTreeMap<u32, u32>,
    pub event_codes: BTreeMap<u32, u32>,
    pub restricted_items: BTreeMap<u32, u32>,
    pub enchantments: BTreeMap<u32, u32>,

}
