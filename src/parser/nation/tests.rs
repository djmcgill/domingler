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
    let (_, nation) = parse_nation(input).unwrap();

    match nation.declaration {
        NationDeclaration::SelectId(71) => (), // pass
        other => panic!("Unexpected decl: {:?}", other),
    }

    assert_eq!(
        nation.inner_lines,
        vec![
            Either::Left("#hatesterr 32 -- Non swamp start if at all possible."),
            Either::Left("#likespop 103 -- cynocephaleans"),
            Either::Left("#uwrec 5760"),
            Either::Left("#uwrec 5761"),
            Either::Left("#uwrec 5762"),
            Either::Left("#uwcom 5767"),
            Either::Left("#uwcom 5768"),
            Either::Left("#swamprec 5769"),
            Either::Left("#swamprec 5770"),
            Either::Left("#swamprec 5771"),
            Either::Left("#swampcom 5772"),
        ]
    );
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
    let (_, nation) = parse_nation(input).unwrap();

    match nation.declaration {
        NationDeclaration::SelectId(243) => (), // pass
        other => panic!("Unexpected decl: {:?}", other),
    }

    assert_eq!(nation.inner_lines, vec![
        Either::Left("#name \"Karanaac\""),
        Either::Left("#epithet \"Geodetic Awakening\""),
        Either::Left("#era 1"),
        Either::Left("#idealcold 0"),
        Either::Left("#descr \"In the ages past, the Pantokrator imprisoned many rivals and foes within the Earth along with the Dead, enclosing their souls in stone for all eternity thanks to a Seal so ancient that it predated even the birth of the world. And now, with the Pantokrator gone, there is a great and mysterious Awakening. The ancient Seal itself, that is to say, the geodetic World-Soul, is very slowly emerging from its aeon-long sleep, progressively revealing to mankind a consciousness of cosmic scale, and allowing the ancient spirits it absorbed so long ago to come back in the shape of sentient stones. Thus, a strange phenomenon that started near the Primeval Alignments of Karanaac in the forbidden highlands revered by the elder druids of Marverni is now expanding as an invisible, unexplainable force. Everywhere in the world, megaliths of venerable stature are spontaneously emerging without a hand to erect them, forming incredibly vast arrays of unknown origins and purpose. Reflecting in their precise ordering the framework of the Spheres, they seem to form a great microcosm-macrocosm concordance between the Earth and the Stars, a Harmonic Tapestry of Sacred Geometry - 'As Above, So Below'. Soon, under the unseen influence of those telluric resonances, even the simplest of pebbles shall retrieve their souls, join the Awakening and enter the Polyphony of Stones; and, irreversibly drawing to itself the fascination and worship of both men and animals by the eerie beauty of its unmovable song, a new God entirely estranged from Life shall ascend at the Centre of the World and transfigure all flesh by converting it to perfect stone.\""),
        Either::Left("#brief \"In the silent fields of Karanaac, ancient stone alignments have waited patiently for millenia for the return of their master. Now, with the stars right and the Pantokrator gone, their vast consciousness is slowly awakening, and sentient megaliths are mysteriously rising everywhere in the world, ready to engulf mankind in the mystical vibrations of their gigantic communions.\""),
        Either::Left("#summary \"Race: Sentient stones. Very durable, completely immobile, do not eat, heal, or cost upkeep or resources."),
        Either::Left("Military: Mass communion arrays of menhirs and singing stones that cannot move or attack. Doubled mercenary costs. Rely on teleportation spells to expand and attack."),
        Either::Left("Magic: High Astral, Nature and Earth, some Death. Innate spellcasters. Items hard to use. Exponential research and astral pearl production."),
        Either::Left("Priests: Powerful. Dominion kills 0.1 percent of population per candle and per month.\""),
        Either::Left("#flag \"./Hellenika/Karanaac_flag.tga\""),
        Either::Left("#color 0.4 0.6 0.6"),
        Either::Left("#secondarycolor 0.6 0.7 0.8"),
        Either::Left("#likesterr 4194432"),
        Either::Left("#coastnation"),
        Either::Left("#domkill 1"),
        Either::Left("#bloodblessbonus -1"),
        Either::Left("----- RECRUITS"),
        Either::Left("#addrecunit \"Korrigan\""),
        Either::Left("#addforeignunit \"Korrigan\""),
        Either::Left("#addrecunit 4820"),
        Either::Left("#addforeignunit 4820"),
        Either::Left("#uwrec 4820"),
        Either::Left("#addrecunit 4817"),
        Either::Left("#addforeignunit 4817"),
        Either::Left("#uwrec 4817"),
        Either::Left("----- LEADERS"),
        Either::Left("#addreccom \"Korrigan Elder\""),
        Either::Left("#addforeigncom \"Korrigan Elder\""),
        Either::Left("#addreccom \"Cairn\""),
        Either::Left("#addforeigncom \"Cairn\""),
        Either::Left("#uwcom \"Cairn\""),
        Either::Left("#addreccom \"Dolmen\""),
        Either::Left("#addforeigncom \"Dolmen\""),
        Either::Left("#uwcom \"Dolmen\""),
        Either::Left("#addreccom \"Wise Figure\""),
        Either::Left("#uwcom \"Wise Figure\""),
        Either::Left("#addreccom \"Arcane Menhir\""),
        Either::Left("#uwcom \"Arcane Menhir\""),
        Either::Left("#addreccom \"Speaking Stone\""),
        Either::Left("#uwcom \"Speaking Stone\""),
        Either::Left("-----HEROES"),
        Either::Left("#multihero1 4818"),
        Either::Left("----- GODS"),
        Either::Left("#homerealm 2"),
        Either::Left("#addgod 156"),
        Either::Left("#addgod 245"),
        Either::Left("#addgod 249"),
        Either::Left("#addgod 250"),
        Either::Left("#addgod 251"),
        Either::Left("#cheapgod40 251"),
        Either::Left("#addgod 270"),
        Either::Left("#addgod 401"),
        Either::Left("#addgod 485"),
        Either::Left("#addgod 500"),
        Either::Left("#addgod 501"),
        Either::Left("#addgod 606"),
        Either::Left("#addgod 657"),
        Either::Left("#cheapgod40 657"),
        Either::Left("#addgod 779"),
        Either::Left("#addgod 812"),
        Either::Left("#addgod 958"),
        Either::Left("#addgod 1098"),
        Either::Left("#addgod 1229"),
        Either::Left("#addgod 1230"),
        Either::Left("#addgod 1340"),
        Either::Left("#addgod 1370"),
        Either::Left("#addgod 1378"),
        Either::Left("#addgod 1379"),
        Either::Left("#cheapgod20 1379"),
        Either::Left("#addgod 1561"),
        Either::Left("#addgod 1898"),
        Either::Left("#addgod 2206"),
        Either::Left("#addgod 2234"),
        Either::Left("#addgod 2239"),
        Either::Left("#addgod 2450"),
        Either::Left("#cheapgod20 2450"),
        Either::Left("#addgod 2784"),
        Either::Left("#addgod 2789"),
        Either::Left("#addgod 2795"),
        Either::Left("#addgod 2801"),
        Either::Left("#addgod 2802"),
        Either::Left("#cheapgod20 2802"),
        Either::Left("#addgod 2930"),
        Either::Left("#addgod 2980"),
        Either::Left("#addgod 3076"),
        Either::Left("#addgod 3079"),
        Either::Left("#addgod 3080"),
        Either::Left("#addgod 3081"),
        Either::Left("#addgod 3082"),
        Either::Left("#addgod \"Ubiquitous Crystal\""),
        Either::Left("#addgod 4808"),
        Either::Left("#addgod \"Druid Heresiarch\""),
        Either::Left("#addgod \"Brother of Stones\""),
        Either::Left("#addgod \"Eternal Gardener\""),
        Either::Left("#addgod \"Symbol of Creation\""),
        Either::Left("#addgod \"First Dolmen\""),
        Either::Left("#addgod \"Karanaac Pretender Information\""),
        Either::Left("#addgod \"Duiu of Travel\""),
        Either::Left("#addgod \"Rock of Ages\""),
        Either::Left("#cheapgod40 \"Rock of Ages\""),
        Either::Left("#cheapgod40 1370"),
        Either::Left("#cheapgod40 1561"),
        Either::Left("#cheapgod40 2930"),
        Either::Left("#cheapgod40 3082"),
        Either::Left("#cheapgod40 2239"),
        Either::Left("#cheapgod20 501"),
        Either::Left("------ START SITES"),
        Either::Left("#startsite \"The Primeval Alignment\""),
        Either::Left("----- PROVINCE DEFENSE"),
        Either::Left("#defcom1 \"Dolmen\""),
        Either::Left("#defcom2 \"Arcane Menhir\""),
        Either::Left("#defunit1 -4130"),
        Either::Left("#defunit1b \"Korrigan\""),
        Either::Left("#defunit2 -4130"),
        Either::Left("#defunit2b -4132"),
        Either::Left("#defmult1 8"),
        Either::Left("#defmult1b 4"),
        Either::Left("#defmult2 8"),
        Either::Left("#defmult2b 4"),
        Either::Left("#wallcom \"Arcane Menhir\""),
        Either::Left("#wallunit \"Menhir\""),
        Either::Left("#wallmult 6"),
        Either::Left("----- STARTING UNITS"),
        Either::Left("#startcom \"Wise Figure\""),
        Either::Left("#startscout \"Cairn\""),
        Either::Left("#startunittype1 \"Singing Stone\""),
        Either::Left("#startunittype2 \"Menhir\""),
        Either::Left("#startunitnbrs1 15"),
        Either::Left("#startunitnbrs2 5"),
        Either::Left("----- BUILDINGS"),
        Either::Left("#fortera 0"),
        Either::Left("#templepic 10"),
        Either::Left("#merccost 100"),
        Either::Left("#labcost 1000"),
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
    let (_, nation) = parse_nation(input).unwrap();

    match nation.declaration {
        NationDeclaration::NewImplicit => (), // pass
        other => panic!("Unexpected decl: {:?}", other),
    }

    assert_eq!(nation.inner_lines, vec![
        Either::Left("#name \"Karakor\""),
        Either::Left("#epithet \"Empire Over the Steppe\""),
        Either::Left("#era 2"),
        Either::Left("#descr \"The Draconian Empire of Karakor has formed around a clan of wel-organized warrior-mages. Rejecting tradition, they hired skilled smiths and trained their clansmen relentlessly to fight as units instead of a horde. This new style of battle proved extremely effective at subjugating the nearby human nomads as well as the other draconian tribes. However, old beliefs are slow to change, and it is the ancient rites which are held sacred by the people.\""),
        Either::Left("#summary \"Race: Steppe humans, draconians"),
        Either::Left("Military: Good cavalry, flying monsters, cheap light infantry"),
        Either::Left("Magic: Death, Fire, Air, Nature, Earth."),
        Either::Left("Priests: Shaman-mages. Plentiful but of average strength.\""),
        Either::Left("#brief \"Karakor is a nation of steppe nomads, ruled by a powerful tribe of highly organized Draconians.\""),
        Either::Left("#color 1.0 0.0 0.3"),
        Either::Left("#secondarycolor 0.4 0.6 0.2"),
        Either::Left("#startsite \"The Funeral Crags\""),
        Either::Left("#startsite \"Crag of Hidden Draconians\""),
        Either::Left("#futuresite \"The Imperial Crag\""),
        Either::Left("#idealcold -1"),
        Either::Left("#cavenation 0 --never caves"),
        Either::Left("#fortera 2"),
        Either::Left("#templepic 11"),
        Either::Left("#homefort 17"),
        Either::Left("#hero1 9193"),
        Either::Left("#hero2 9193   -- yes it's the same guy twice, but you can't get a bunch of him from Luck 3 like if it was a multihero"),
        Either::Left("#startcom \"Imperial Sky General\""),
        Either::Left("#startscout \"Steppe Scout\""),
        Either::Left("#startunittype1 \"Spirit Slayer\""),
        Either::Left("#startunitnbrs1 9"),
        Either::Left("#startunittype2 \"Imperial Warrior\""),
        Either::Left("#startunitnbrs2 20"),
        Either::Left("#addreccom \"Steppe Scout\""),
        Either::Left("#addreccom \"Steppe Commander\""),
        Either::Left("#addreccom \"Draconian Chief\""),
        Either::Left("#addreccom \"Steppe Shaman\""),
        Either::Left("#addreccom \"Imperial Smith\""),
        Either::Left("#addreccom \"Imperial Foot Master\""),
        Either::Left("#addforeigncom \"Steppe Scout\""),
        Either::Left("#addforeigncom \"Steppe Commander\""),
        Either::Left("#addforeigncom \"Steppe Shaman\""),
        Either::Left("#addforeigncom \"Draconian Stylite\""),
        Either::Left("#addrecunit \"Pike Footman\""),
        Either::Left("#addrecunit \"Glaive Footman\""),
        Either::Left("#addrecunit \"Shield Footman\""),
        Either::Left("#addrecunit \"Armored Footman\""),
        Either::Left("#addrecunit \"Imperial Warrior\""),
        Either::Left("#addrecunit \"Steppe Light Horseman\""),
        Either::Left("#addrecunit \"Steppe Heavy Horseman\""),
        Either::Left("#addforeignunit \"Steppe Light Horseman\""),
        Either::Left("#addforeignunit \"Steppe Heavy Horseman\""),
        Either::Left("#defcom1 \"Steppe Commander\""),
        Either::Left("#defcom2 \"Imperial Foot Master\""),
        Either::Left("#defunit1 \"Shield Footman\""),
        Either::Left("#defunit1b \"Armored Footman\""),
        Either::Left("#defmult1 30"),
        Either::Left("#defmult1b 15"),
        Either::Left("#defunit2 \"Imperial Warrior\""),
        Either::Left("#defmult2 10"),
        Either::Left("#wallcom \"Imperial Foot Master\""),
        Either::Left("#wallunit 797"),
        Either::Left("#homerealm 4"),
        Either::Left("#homerealm 10"),
        Either::Left("#delgod \"Demilich\"  --since it could too easily be killed by the holy fire before it moved off the cap"),
        Either::Left("#addgod \"Bitch Queen\""),
        Either::Left("#addgod \"Wyrm\""),
        Either::Left("#addgod \"Idol of Sorcery\""),
        Either::Left("#addgod \"Idol of Beasts\""),
        Either::Left("#addgod \"Idol of Men\""),
        Either::Left("#addgod \"Monolith\""),
        Either::Left("#addgod \"Serpent of the Underworld\""),
        Either::Left("#addgod \"Great Siddha\"  -- Buddhism popular in Mongolia"),
        Either::Left("#addgod \"Devi of Darkness\""),
        Either::Left("#addgod \"Devi of Good Fortunes\""),
        Either::Left("#addgod 3060  -- TC Master"),
        Either::Left("#addgod 1025  -- Divine glyph, Islam popular in Mongolia"),
        Either::Left("#addgod \"Draconian Emperor\""),
        Either::Left("#addgod \"Draconian Ancestor\""),
        Either::Left("#cheapgod40 2465 --statue of war"),
    ]);
}
