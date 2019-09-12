# Domingler

Takes mods for dominions 5, and scans them for the IDs that they use. In the future it will actually
mingle them together and remap IDs as needed.

### TODO:
- ID types still not scanned:
    - events
    - poptypes
- scan names:
    - all existing IDs
    - also mercs
- proper way to give inputs etc
- check limits
- there's a bunch of FIXMEs
- FIXME: `#newmonster100` works wtf.
- FIXME:  visible sites or unlimited bodyguard mod doesn't work
- FIXME: some issue with a touhou pretender
- make a proper mod - description, eras
- take only the nations that we care about (TODO: also look at nation restriction)
    - Weapons - found on weapons, monsters, items
    - Armour - found on armour, monsters, items
    - Spells - found on items, monsters, spells, 
    - Monsters - found on nations, poptype, monsters, spells, mercs, sites, events
    - Items - found on 
    - Sites
    - Nations
    - Nametypes
    - Montags
    - Event
    - Restricted
    - Enchantments

### Usage:
Put executable in the same fold as some .dm files. It could be your dominions mods folder, it could be a different one. Run it.

Now you have a `domingler.dm` mod and can 1) edit to change name, description, icon etc and 2) package it with the sprite folders from its constituent mods.

Order:
- mod info
- weapons
- armours
- units
- names
- sites
- nations
- spells
- items
- general
- poptypes
- mercs
- events
