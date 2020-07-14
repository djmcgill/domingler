use super::*;

#[test]
fn parse_nation_edit() {
    let input = r#"#selectnation 71 -- Ind
#hatesterr 32 -- Non swamp start if at all possible.
#likespop 103 -- cynocephaleans
#uwrec 5760
#uwrec 5761
#uwrec 5762
#uwcom 5767
#uwcom 5768
#swamprec 5769
#swamprec 5770
#swamprec 5771
#swampcom 5772
#end
"#;
    let (_, nation) = parse_nation::<()>(input).unwrap();

    match nation.declaration {
        NationDeclaration::SelectId(71) => (), // pass
        other => panic!("Unexpected decl: {:?}", other),
    }

    assert_eq!(nation.lines, vec![
        NationLine::Declaration,
        NationLine::Unparsed("#hatesterr 32 -- Non swamp start if at all possible."),
        NationLine::Unparsed("#likespop 103 -- cynocephaleans"),
        NationLine::Unparsed("#uwrec 5760"),
        NationLine::Unparsed("#uwrec 5761"),
        NationLine::Unparsed("#uwrec 5762"),
        NationLine::Unparsed("#uwcom 5767"),
        NationLine::Unparsed("#uwcom 5768"),
        NationLine::Unparsed("#swamprec 5769"),
        NationLine::Unparsed("#swamprec 5770"),
        NationLine::Unparsed("#swamprec 5771"),
        NationLine::Unparsed("#swampcom 5772"),
        NationLine::End,
    ]);
}

#[test]
fn parse_new_selectnation() {
    let input = r#"#selectnation 243
#name "Karanaac"
#epithet "Geodetic Awakening"
#era 1
#idealcold 0
#descr "In the ages past, the Pantokrator imprisoned many rivals and foes within the Earth along with the Dead, enclosing their souls in stone for all eternity thanks to a Seal so ancient that it predated even the birth of the world. And now, with the Pantokrator gone, there is a great and mysterious Awakening. The ancient Seal itself, that is to say, the geodetic World-Soul, is very slowly emerging from its aeon-long sleep, progressively revealing to mankind a consciousness of cosmic scale, and allowing the ancient spirits it absorbed so long ago to come back in the shape of sentient stones. Thus, a strange phenomenon that started near the Primeval Alignments of Karanaac in the forbidden highlands revered by the elder druids of Marverni is now expanding as an invisible, unexplainable force. Everywhere in the world, megaliths of venerable stature are spontaneously emerging without a hand to erect them, forming incredibly vast arrays of unknown origins and purpose. Reflecting in their precise ordering the framework of the Spheres, they seem to form a great microcosm-macrocosm concordance between the Earth and the Stars, a Harmonic Tapestry of Sacred Geometry - 'As Above, So Below'. Soon, under the unseen influence of those telluric resonances, even the simplest of pebbles shall retrieve their souls, join the Awakening and enter the Polyphony of Stones; and, irreversibly drawing to itself the fascination and worship of both men and animals by the eerie beauty of its unmovable song, a new God entirely estranged from Life shall ascend at the Centre of the World and transfigure all flesh by converting it to perfect stone."
#brief "In the silent fields of Karanaac, ancient stone alignments have waited patiently for millenia for the return of their master. Now, with the stars right and the Pantokrator gone, their vast consciousness is slowly awakening, and sentient megaliths are mysteriously rising everywhere in the world, ready to engulf mankind in the mystical vibrations of their gigantic communions."
#summary "Race: Sentient stones. Very durable, completely immobile, do not eat, heal, or cost upkeep or resources.
Military: Mass communion arrays of menhirs and singing stones that cannot move or attack. Doubled mercenary costs. Rely on teleportation spells to expand and attack.
Magic: High Astral, Nature and Earth, some Death. Innate spellcasters. Items hard to use. Exponential research and astral pearl production.
Priests: Powerful. Dominion kills 0.1 percent of population per candle and per month."
#flag "./Hellenika/Karanaac_flag.tga"
#color 0.4 0.6 0.6
#secondarycolor 0.6 0.7 0.8
#likesterr 4194432
#coastnation
#domkill 1
#bloodblessbonus -1

----- RECRUITS

#addrecunit "Korrigan"
#addforeignunit "Korrigan"
#addrecunit 4820
#addforeignunit 4820
#uwrec 4820
#addrecunit 4817
#addforeignunit 4817
#uwrec 4817


----- LEADERS

#addreccom "Korrigan Elder"
#addforeigncom "Korrigan Elder"
#addreccom "Cairn"
#addforeigncom "Cairn"
#uwcom "Cairn"
#addreccom "Dolmen"
#addforeigncom "Dolmen"
#uwcom "Dolmen"
#addreccom "Wise Figure"
#uwcom "Wise Figure"
#addreccom "Arcane Menhir"
#uwcom "Arcane Menhir"
#addreccom "Speaking Stone"
#uwcom "Speaking Stone"


-----HEROES

#multihero1 4818


----- GODS

#homerealm 2
#addgod 156
#addgod 245
#addgod 249
#addgod 250
#addgod 251
#cheapgod40 251
#addgod 270
#addgod 401
#addgod 485
#addgod 500
#addgod 501
#addgod 606
#addgod 657
#cheapgod40 657
#addgod 779
#addgod 812
#addgod 958
#addgod 1098
#addgod 1229
#addgod 1230
#addgod 1340
#addgod 1370
#addgod 1378
#addgod 1379
#cheapgod20 1379
#addgod 1561
#addgod 1898
#addgod 2206
#addgod 2234
#addgod 2239
#addgod 2450
#cheapgod20 2450
#addgod 2784
#addgod 2789
#addgod 2795
#addgod 2801
#addgod 2802
#cheapgod20 2802
#addgod 2930
#addgod 2980
#addgod 3076
#addgod 3079
#addgod 3080
#addgod 3081
#addgod 3082
#addgod "Ubiquitous Crystal"
#addgod 4808
#addgod "Druid Heresiarch"
#addgod "Brother of Stones"
#addgod "Eternal Gardener"
#addgod "Symbol of Creation"
#addgod "First Dolmen"
#addgod "Karanaac Pretender Information"
#addgod "Duiu of Travel"
#addgod "Rock of Ages"
#cheapgod40 "Rock of Ages"
#cheapgod40 1370
#cheapgod40 1561
#cheapgod40 2930
#cheapgod40 3082
#cheapgod40 2239
#cheapgod20 501

------ START SITES

#startsite "The Primeval Alignment"


----- PROVINCE DEFENSE

#defcom1 "Dolmen"
#defcom2 "Arcane Menhir"
#defunit1 -4130
#defunit1b "Korrigan"
#defunit2 -4130
#defunit2b -4132
#defmult1 8
#defmult1b 4
#defmult2 8
#defmult2b 4

#wallcom "Arcane Menhir"
#wallunit "Menhir"
#wallmult 6

----- STARTING UNITS

#startcom "Wise Figure"
#startscout "Cairn"
#startunittype1 "Singing Stone"
#startunittype2 "Menhir"
#startunitnbrs1 15
#startunitnbrs2 5

----- BUILDINGS

#fortera 0
#templepic 10
#merccost 100
#labcost 1000

#end
"#;
    let (_, nation) = parse_nation::<()>(input).unwrap();

    match nation.declaration {
        NationDeclaration::SelectId(243) => (), // pass
        other => panic!("Unexpected decl: {:?}", other),
    }

    assert_eq!(nation.lines, vec![
        NationLine::Declaration,
        NationLine::Unparsed("#name \"Karanaac\""),
        NationLine::Unparsed("#epithet \"Geodetic Awakening\""),
        NationLine::Unparsed("#era 1"),
        NationLine::Unparsed("#idealcold 0"),
        NationLine::Unparsed("#descr \"In the ages past, the Pantokrator imprisoned many rivals and foes within the Earth along with the Dead, enclosing their souls in stone for all eternity thanks to a Seal so ancient that it predated even the birth of the world. And now, with the Pantokrator gone, there is a great and mysterious Awakening. The ancient Seal itself, that is to say, the geodetic World-Soul, is very slowly emerging from its aeon-long sleep, progressively revealing to mankind a consciousness of cosmic scale, and allowing the ancient spirits it absorbed so long ago to come back in the shape of sentient stones. Thus, a strange phenomenon that started near the Primeval Alignments of Karanaac in the forbidden highlands revered by the elder druids of Marverni is now expanding as an invisible, unexplainable force. Everywhere in the world, megaliths of venerable stature are spontaneously emerging without a hand to erect them, forming incredibly vast arrays of unknown origins and purpose. Reflecting in their precise ordering the framework of the Spheres, they seem to form a great microcosm-macrocosm concordance between the Earth and the Stars, a Harmonic Tapestry of Sacred Geometry - 'As Above, So Below'. Soon, under the unseen influence of those telluric resonances, even the simplest of pebbles shall retrieve their souls, join the Awakening and enter the Polyphony of Stones; and, irreversibly drawing to itself the fascination and worship of both men and animals by the eerie beauty of its unmovable song, a new God entirely estranged from Life shall ascend at the Centre of the World and transfigure all flesh by converting it to perfect stone.\""),
        NationLine::Unparsed("#brief \"In the silent fields of Karanaac, ancient stone alignments have waited patiently for millenia for the return of their master. Now, with the stars right and the Pantokrator gone, their vast consciousness is slowly awakening, and sentient megaliths are mysteriously rising everywhere in the world, ready to engulf mankind in the mystical vibrations of their gigantic communions.\""),
        NationLine::Unparsed("#summary \"Race: Sentient stones. Very durable, completely immobile, do not eat, heal, or cost upkeep or resources."),
        NationLine::Unparsed("Military: Mass communion arrays of menhirs and singing stones that cannot move or attack. Doubled mercenary costs. Rely on teleportation spells to expand and attack."),
        NationLine::Unparsed("Magic: High Astral, Nature and Earth, some Death. Innate spellcasters. Items hard to use. Exponential research and astral pearl production."),
        NationLine::Unparsed("Priests: Powerful. Dominion kills 0.1 percent of population per candle and per month.\""),
        NationLine::Unparsed("#flag \"./Hellenika/Karanaac_flag.tga\""),
        NationLine::Unparsed("#color 0.4 0.6 0.6"),
        NationLine::Unparsed("#secondarycolor 0.6 0.7 0.8"),
        NationLine::Unparsed("#likesterr 4194432"),
        NationLine::Unparsed("#coastnation"),
        NationLine::Unparsed("#domkill 1"),
        NationLine::Unparsed("#bloodblessbonus -1"),
        NationLine::Unparsed("----- RECRUITS"),
        NationLine::Unparsed("#addrecunit \"Korrigan\""),
        NationLine::Unparsed("#addforeignunit \"Korrigan\""),
        NationLine::Unparsed("#addrecunit 4820"),
        NationLine::Unparsed("#addforeignunit 4820"),
        NationLine::Unparsed("#uwrec 4820"),
        NationLine::Unparsed("#addrecunit 4817"),
        NationLine::Unparsed("#addforeignunit 4817"),
        NationLine::Unparsed("#uwrec 4817"),
        NationLine::Unparsed("----- LEADERS"),
        NationLine::Unparsed("#addreccom \"Korrigan Elder\""),
        NationLine::Unparsed("#addforeigncom \"Korrigan Elder\""),
        NationLine::Unparsed("#addreccom \"Cairn\""),
        NationLine::Unparsed("#addforeigncom \"Cairn\""),
        NationLine::Unparsed("#uwcom \"Cairn\""),
        NationLine::Unparsed("#addreccom \"Dolmen\""),
        NationLine::Unparsed("#addforeigncom \"Dolmen\""),
        NationLine::Unparsed("#uwcom \"Dolmen\""),
        NationLine::Unparsed("#addreccom \"Wise Figure\""),
        NationLine::Unparsed("#uwcom \"Wise Figure\""),
        NationLine::Unparsed("#addreccom \"Arcane Menhir\""),
        NationLine::Unparsed("#uwcom \"Arcane Menhir\""),
        NationLine::Unparsed("#addreccom \"Speaking Stone\""),
        NationLine::Unparsed("#uwcom \"Speaking Stone\""),
        NationLine::Unparsed("-----HEROES"),
        NationLine::Unparsed("#multihero1 4818"),
        NationLine::Unparsed("----- GODS"),
        NationLine::Unparsed("#homerealm 2"),
        NationLine::Unparsed("#addgod 156"),
        NationLine::Unparsed("#addgod 245"),
        NationLine::Unparsed("#addgod 249"),
        NationLine::Unparsed("#addgod 250"),
        NationLine::Unparsed("#addgod 251"),
        NationLine::Unparsed("#cheapgod40 251"),
        NationLine::Unparsed("#addgod 270"),
        NationLine::Unparsed("#addgod 401"),
        NationLine::Unparsed("#addgod 485"),
        NationLine::Unparsed("#addgod 500"),
        NationLine::Unparsed("#addgod 501"),
        NationLine::Unparsed("#addgod 606"),
        NationLine::Unparsed("#addgod 657"),
        NationLine::Unparsed("#cheapgod40 657"),
        NationLine::Unparsed("#addgod 779"),
        NationLine::Unparsed("#addgod 812"),
        NationLine::Unparsed("#addgod 958"),
        NationLine::Unparsed("#addgod 1098"),
        NationLine::Unparsed("#addgod 1229"),
        NationLine::Unparsed("#addgod 1230"),
        NationLine::Unparsed("#addgod 1340"),
        NationLine::Unparsed("#addgod 1370"),
        NationLine::Unparsed("#addgod 1378"),
        NationLine::Unparsed("#addgod 1379"),
        NationLine::Unparsed("#cheapgod20 1379"),
        NationLine::Unparsed("#addgod 1561"),
        NationLine::Unparsed("#addgod 1898"),
        NationLine::Unparsed("#addgod 2206"),
        NationLine::Unparsed("#addgod 2234"),
        NationLine::Unparsed("#addgod 2239"),
        NationLine::Unparsed("#addgod 2450"),
        NationLine::Unparsed("#cheapgod20 2450"),
        NationLine::Unparsed("#addgod 2784"),
        NationLine::Unparsed("#addgod 2789"),
        NationLine::Unparsed("#addgod 2795"),
        NationLine::Unparsed("#addgod 2801"),
        NationLine::Unparsed("#addgod 2802"),
        NationLine::Unparsed("#cheapgod20 2802"),
        NationLine::Unparsed("#addgod 2930"),
        NationLine::Unparsed("#addgod 2980"),
        NationLine::Unparsed("#addgod 3076"),
        NationLine::Unparsed("#addgod 3079"),
        NationLine::Unparsed("#addgod 3080"),
        NationLine::Unparsed("#addgod 3081"),
        NationLine::Unparsed("#addgod 3082"),
        NationLine::Unparsed("#addgod \"Ubiquitous Crystal\""),
        NationLine::Unparsed("#addgod 4808"),
        NationLine::Unparsed("#addgod \"Druid Heresiarch\""),
        NationLine::Unparsed("#addgod \"Brother of Stones\""),
        NationLine::Unparsed("#addgod \"Eternal Gardener\""),
        NationLine::Unparsed("#addgod \"Symbol of Creation\""),
        NationLine::Unparsed("#addgod \"First Dolmen\""),
        NationLine::Unparsed("#addgod \"Karanaac Pretender Information\""),
        NationLine::Unparsed("#addgod \"Duiu of Travel\""),
        NationLine::Unparsed("#addgod \"Rock of Ages\""),
        NationLine::Unparsed("#cheapgod40 \"Rock of Ages\""),
        NationLine::Unparsed("#cheapgod40 1370"),
        NationLine::Unparsed("#cheapgod40 1561"),
        NationLine::Unparsed("#cheapgod40 2930"),
        NationLine::Unparsed("#cheapgod40 3082"),
        NationLine::Unparsed("#cheapgod40 2239"),
        NationLine::Unparsed("#cheapgod20 501"),
        NationLine::Unparsed("------ START SITES"),
        NationLine::Unparsed("#startsite \"The Primeval Alignment\""),
        NationLine::Unparsed("----- PROVINCE DEFENSE"),
        NationLine::Unparsed("#defcom1 \"Dolmen\""),
        NationLine::Unparsed("#defcom2 \"Arcane Menhir\""),
        NationLine::Unparsed("#defunit1 -4130"),
        NationLine::Unparsed("#defunit1b \"Korrigan\""),
        NationLine::Unparsed("#defunit2 -4130"),
        NationLine::Unparsed("#defunit2b -4132"),
        NationLine::Unparsed("#defmult1 8"),
        NationLine::Unparsed("#defmult1b 4"),
        NationLine::Unparsed("#defmult2 8"),
        NationLine::Unparsed("#defmult2b 4"),
        NationLine::Unparsed("#wallcom \"Arcane Menhir\""),
        NationLine::Unparsed("#wallunit \"Menhir\""),
        NationLine::Unparsed("#wallmult 6"),
        NationLine::Unparsed("----- STARTING UNITS"),
        NationLine::Unparsed("#startcom \"Wise Figure\""),
        NationLine::Unparsed("#startscout \"Cairn\""),
        NationLine::Unparsed("#startunittype1 \"Singing Stone\""),
        NationLine::Unparsed("#startunittype2 \"Menhir\""),
        NationLine::Unparsed("#startunitnbrs1 15"),
        NationLine::Unparsed("#startunitnbrs2 5"),
        NationLine::Unparsed("----- BUILDINGS"),
        NationLine::Unparsed("#fortera 0"),
        NationLine::Unparsed("#templepic 10"),
        NationLine::Unparsed("#merccost 100"),
        NationLine::Unparsed("#labcost 1000"),
        NationLine::End,
    ]);
}

#[test]
fn parse_new_new_nation() {
    let input = r#"#newnation
#name "Karakor"
#epithet "Empire Over the Steppe"
#era 2
#descr "The Draconian Empire of Karakor has formed around a clan of wel-organized warrior-mages. Rejecting tradition, they hired skilled smiths and trained their clansmen relentlessly to fight as units instead of a horde. This new style of battle proved extremely effective at subjugating the nearby human nomads as well as the other draconian tribes. However, old beliefs are slow to change, and it is the ancient rites which are held sacred by the people."
#summary "Race: Steppe humans, draconians
Military: Good cavalry, flying monsters, cheap light infantry
Magic: Death, Fire, Air, Nature, Earth.
Priests: Shaman-mages. Plentiful but of average strength."
#brief "Karakor is a nation of steppe nomads, ruled by a powerful tribe of highly organized Draconians."
#color 1.0 0.0 0.3
#secondarycolor 0.4 0.6 0.2


#startsite "The Funeral Crags"
#startsite "Crag of Hidden Draconians"
#futuresite "The Imperial Crag"

#idealcold -1
#cavenation 0 --never caves
#fortera 2
#templepic 11
#homefort 17

#hero1 9193
#hero2 9193   -- yes it's the same guy twice, but you can't get a bunch of him from Luck 3 like if it was a multihero

#startcom "Imperial Sky General"
#startscout "Steppe Scout"
#startunittype1 "Spirit Slayer"
#startunitnbrs1 9
#startunittype2 "Imperial Warrior"
#startunitnbrs2 20

#addreccom "Steppe Scout"
#addreccom "Steppe Commander"
#addreccom "Draconian Chief"
#addreccom "Steppe Shaman"
#addreccom "Imperial Smith"
#addreccom "Imperial Foot Master"
#addforeigncom "Steppe Scout"
#addforeigncom "Steppe Commander"
#addforeigncom "Steppe Shaman"
#addforeigncom "Draconian Stylite"

#addrecunit "Pike Footman"
#addrecunit "Glaive Footman"
#addrecunit "Shield Footman"
#addrecunit "Armored Footman"
#addrecunit "Imperial Warrior"
#addrecunit "Steppe Light Horseman"
#addrecunit "Steppe Heavy Horseman"
#addforeignunit "Steppe Light Horseman"
#addforeignunit "Steppe Heavy Horseman"

#defcom1 "Steppe Commander"
#defcom2 "Imperial Foot Master"
#defunit1 "Shield Footman"
#defunit1b "Armored Footman"
#defmult1 30
#defmult1b 15
#defunit2 "Imperial Warrior"
#defmult2 10
#wallcom "Imperial Foot Master"
#wallunit 797

#homerealm 4
#homerealm 10
#delgod "Demilich"  --since it could too easily be killed by the holy fire before it moved off the cap
#addgod "Bitch Queen"
#addgod "Wyrm"
#addgod "Idol of Sorcery"
#addgod "Idol of Beasts"
#addgod "Idol of Men"
#addgod "Monolith"
#addgod "Serpent of the Underworld"
#addgod "Great Siddha"  -- Buddhism popular in Mongolia
#addgod "Devi of Darkness"
#addgod "Devi of Good Fortunes"
#addgod 3060  -- TC Master
#addgod 1025  -- Divine glyph, Islam popular in Mongolia
#addgod "Draconian Emperor"
#addgod "Draconian Ancestor"

#cheapgod40 2465 --statue of war
#end
"#;
    let (_, nation) = parse_nation::<()>(input).unwrap();

    match nation.declaration {
        NationDeclaration::NewNation => (), // pass
        other => panic!("Unexpected decl: {:?}", other),
    }

    assert_eq!(nation.lines, vec![
        NationLine::Declaration,
        NationLine::Unparsed("#name \"Karakor\""),
        NationLine::Unparsed("#epithet \"Empire Over the Steppe\""),
        NationLine::Unparsed("#era 2"),
        NationLine::Unparsed("#descr \"The Draconian Empire of Karakor has formed around a clan of wel-organized warrior-mages. Rejecting tradition, they hired skilled smiths and trained their clansmen relentlessly to fight as units instead of a horde. This new style of battle proved extremely effective at subjugating the nearby human nomads as well as the other draconian tribes. However, old beliefs are slow to change, and it is the ancient rites which are held sacred by the people.\""),
        NationLine::Unparsed("#summary \"Race: Steppe humans, draconians"),
        NationLine::Unparsed("Military: Good cavalry, flying monsters, cheap light infantry"),
        NationLine::Unparsed("Magic: Death, Fire, Air, Nature, Earth."),
        NationLine::Unparsed("Priests: Shaman-mages. Plentiful but of average strength.\""),
        NationLine::Unparsed("#brief \"Karakor is a nation of steppe nomads, ruled by a powerful tribe of highly organized Draconians.\""),
        NationLine::Unparsed("#color 1.0 0.0 0.3"),
        NationLine::Unparsed("#secondarycolor 0.4 0.6 0.2"),
        NationLine::Unparsed("#startsite \"The Funeral Crags\""),
        NationLine::Unparsed("#startsite \"Crag of Hidden Draconians\""),
        NationLine::Unparsed("#futuresite \"The Imperial Crag\""),
        NationLine::Unparsed("#idealcold -1"),
        NationLine::Unparsed("#cavenation 0 --never caves"),
        NationLine::Unparsed("#fortera 2"),
        NationLine::Unparsed("#templepic 11"),
        NationLine::Unparsed("#homefort 17"),
        NationLine::Unparsed("#hero1 9193"),
        NationLine::Unparsed("#hero2 9193   -- yes it's the same guy twice, but you can't get a bunch of him from Luck 3 like if it was a multihero"),
        NationLine::Unparsed("#startcom \"Imperial Sky General\""),
        NationLine::Unparsed("#startscout \"Steppe Scout\""),
        NationLine::Unparsed("#startunittype1 \"Spirit Slayer\""),
        NationLine::Unparsed("#startunitnbrs1 9"),
        NationLine::Unparsed("#startunittype2 \"Imperial Warrior\""),
        NationLine::Unparsed("#startunitnbrs2 20"),
        NationLine::Unparsed("#addreccom \"Steppe Scout\""),
        NationLine::Unparsed("#addreccom \"Steppe Commander\""),
        NationLine::Unparsed("#addreccom \"Draconian Chief\""),
        NationLine::Unparsed("#addreccom \"Steppe Shaman\""),
        NationLine::Unparsed("#addreccom \"Imperial Smith\""),
        NationLine::Unparsed("#addreccom \"Imperial Foot Master\""),
        NationLine::Unparsed("#addforeigncom \"Steppe Scout\""),
        NationLine::Unparsed("#addforeigncom \"Steppe Commander\""),
        NationLine::Unparsed("#addforeigncom \"Steppe Shaman\""),
        NationLine::Unparsed("#addforeigncom \"Draconian Stylite\""),
        NationLine::Unparsed("#addrecunit \"Pike Footman\""),
        NationLine::Unparsed("#addrecunit \"Glaive Footman\""),
        NationLine::Unparsed("#addrecunit \"Shield Footman\""),
        NationLine::Unparsed("#addrecunit \"Armored Footman\""),
        NationLine::Unparsed("#addrecunit \"Imperial Warrior\""),
        NationLine::Unparsed("#addrecunit \"Steppe Light Horseman\""),
        NationLine::Unparsed("#addrecunit \"Steppe Heavy Horseman\""),
        NationLine::Unparsed("#addforeignunit \"Steppe Light Horseman\""),
        NationLine::Unparsed("#addforeignunit \"Steppe Heavy Horseman\""),
        NationLine::Unparsed("#defcom1 \"Steppe Commander\""),
        NationLine::Unparsed("#defcom2 \"Imperial Foot Master\""),
        NationLine::Unparsed("#defunit1 \"Shield Footman\""),
        NationLine::Unparsed("#defunit1b \"Armored Footman\""),
        NationLine::Unparsed("#defmult1 30"),
        NationLine::Unparsed("#defmult1b 15"),
        NationLine::Unparsed("#defunit2 \"Imperial Warrior\""),
        NationLine::Unparsed("#defmult2 10"),
        NationLine::Unparsed("#wallcom \"Imperial Foot Master\""),
        NationLine::Unparsed("#wallunit 797"),
        NationLine::Unparsed("#homerealm 4"),
        NationLine::Unparsed("#homerealm 10"),
        NationLine::Unparsed("#delgod \"Demilich\"  --since it could too easily be killed by the holy fire before it moved off the cap"),
        NationLine::Unparsed("#addgod \"Bitch Queen\""),
        NationLine::Unparsed("#addgod \"Wyrm\""),
        NationLine::Unparsed("#addgod \"Idol of Sorcery\""),
        NationLine::Unparsed("#addgod \"Idol of Beasts\""),
        NationLine::Unparsed("#addgod \"Idol of Men\""),
        NationLine::Unparsed("#addgod \"Monolith\""),
        NationLine::Unparsed("#addgod \"Serpent of the Underworld\""),
        NationLine::Unparsed("#addgod \"Great Siddha\"  -- Buddhism popular in Mongolia"),
        NationLine::Unparsed("#addgod \"Devi of Darkness\""),
        NationLine::Unparsed("#addgod \"Devi of Good Fortunes\""),
        NationLine::Unparsed("#addgod 3060  -- TC Master"),
        NationLine::Unparsed("#addgod 1025  -- Divine glyph, Islam popular in Mongolia"),
        NationLine::Unparsed("#addgod \"Draconian Emperor\""),
        NationLine::Unparsed("#addgod \"Draconian Ancestor\""),
        NationLine::Unparsed("#cheapgod40 2465 --statue of war"),
        NationLine::End,
    ]);
}