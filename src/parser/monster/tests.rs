use super::*;

#[test]
fn parse_monster_example() {
    let input = r#"#newmonster 4837
#spr1 "Hellenika/PsientistMenhirGold.tga"
#name "Elder Stone"
#descr "Elder Stones are huge megaliths of polished black stone in which an incredibly ancient spirit of immense power has been sealed for all eternity by the last Pantokrator for having threatened his authority. Standing in mute circles in Karanaac like dark, immutable guardians and pillars of the sky, they may very well be the greatest megaliths of the world both in sheer mass and in mystical might, and their immeasurable age and wisdom naturally caused them to become the true architects and masterminds of the Geodetic Awakening, second only to the God himself. Each Elder Stone is engraved with a single symbol of either benevolence of malevolence called a triskele, and the Elder Stones' innate attunement to creation and destruction is so great that their triskeles perpetually glow with a dim, supernatural light whose color depends on the personality and magical alignment of the spirit that inhabits it. There are therefore four main types of Elder Stone, three of which receive hugely increased skills in either Earth, Astral or Nature magic, and the last of which receives moderately increased skills in all of these magics as well as in the path of Death. A given Elder Stone's type isn't predictable in any way before it has awakened, and once it has been declared it can only change through the great ritual of Reawakening."
#noheal
#hp 175
#size 6
#prot 23
#mr 18
#mor 30
#str 15
#att 5
#def 0
#prec 5
#ap 2
#mapmove 0
#enc 0
#maxage 2000
#poisonres 25
#eyes 0
#blind
#immobile
#slashres
#pierceres
#inanimate
#amphibian
#neednoteat
#spiritsight
#noleader
#stonebeing
#miscshape
#itemslots 4096
#spellsinger
#gcost 0
#holy
#magicskill 3 1
#magicskill 4 1
#magicskill 6 1
#magicskill 8 2
#magicboost 3 3
#custommagic 11264 100
#custommagic 4096 10
#montag 4131
#bonusspells 1
#researchbonus 8
#taxcollector
#polyimmune
#nametype 142
#end"#;

    let (_, monster) = parse_monster::<()>(input).unwrap();

    match monster.declaration {
        MonsterDeclaration::NewId(4837) => (), // pass
        other => panic!("Unexpected declaration: {:?}", other),
    }

    assert_eq!(monster.lines, vec![
        MonsterLine::Declaration,
        MonsterLine::Unparsed("#spr1 \"Hellenika/PsientistMenhirGold.tga\""),
        MonsterLine::Unparsed("#name \"Elder Stone\""),
        MonsterLine::Unparsed("#descr \"Elder Stones are huge megaliths of polished black stone in which an incredibly ancient spirit of immense power has been sealed for all eternity by the last Pantokrator for having threatened his authority. Standing in mute circles in Karanaac like dark, immutable guardians and pillars of the sky, they may very well be the greatest megaliths of the world both in sheer mass and in mystical might, and their immeasurable age and wisdom naturally caused them to become the true architects and masterminds of the Geodetic Awakening, second only to the God himself. Each Elder Stone is engraved with a single symbol of either benevolence of malevolence called a triskele, and the Elder Stones' innate attunement to creation and destruction is so great that their triskeles perpetually glow with a dim, supernatural light whose color depends on the personality and magical alignment of the spirit that inhabits it. There are therefore four main types of Elder Stone, three of which receive hugely increased skills in either Earth, Astral or Nature magic, and the last of which receives moderately increased skills in all of these magics as well as in the path of Death. A given Elder Stone's type isn't predictable in any way before it has awakened, and once it has been declared it can only change through the great ritual of Reawakening.\""),
        MonsterLine::Unparsed("#noheal"),
        MonsterLine::Unparsed("#hp 175"),
        MonsterLine::Unparsed("#size 6"),
        MonsterLine::Unparsed("#prot 23"),
        MonsterLine::Unparsed("#mr 18"),
        MonsterLine::Unparsed("#mor 30"),
        MonsterLine::Unparsed("#str 15"),
        MonsterLine::Unparsed("#att 5"),
        MonsterLine::Unparsed("#def 0"),
        MonsterLine::Unparsed("#prec 5"),
        MonsterLine::Unparsed("#ap 2"),
        MonsterLine::Unparsed("#mapmove 0"),
        MonsterLine::Unparsed("#enc 0"),
        MonsterLine::Unparsed("#maxage 2000"),
        MonsterLine::Unparsed("#poisonres 25"),
        MonsterLine::Unparsed("#eyes 0"),
        MonsterLine::Unparsed("#blind"),
        MonsterLine::Unparsed("#immobile"),
        MonsterLine::Unparsed("#slashres"),
        MonsterLine::Unparsed("#pierceres"),
        MonsterLine::Unparsed("#inanimate"),
        MonsterLine::Unparsed("#amphibian"),
        MonsterLine::Unparsed("#neednoteat"),
        MonsterLine::Unparsed("#spiritsight"),
        MonsterLine::Unparsed("#noleader"),
        MonsterLine::Unparsed("#stonebeing"),
        MonsterLine::Unparsed("#miscshape"),
        MonsterLine::Unparsed("#itemslots 4096"),
        MonsterLine::Unparsed("#spellsinger"),
        MonsterLine::Unparsed("#gcost 0"),
        MonsterLine::Unparsed("#holy"),
        MonsterLine::Unparsed("#magicskill 3 1"),
        MonsterLine::Unparsed("#magicskill 4 1"),
        MonsterLine::Unparsed("#magicskill 6 1"),
        MonsterLine::Unparsed("#magicskill 8 2"),
        MonsterLine::Unparsed("#magicboost 3 3"),
        MonsterLine::Unparsed("#custommagic 11264 100"),
        MonsterLine::Unparsed("#custommagic 4096 10"),
        MonsterLine::Unparsed("#montag 4131"),
        MonsterLine::Unparsed("#bonusspells 1"),
        MonsterLine::Unparsed("#researchbonus 8"),
        MonsterLine::Unparsed("#taxcollector"),
        MonsterLine::Unparsed("#polyimmune"),
        MonsterLine::Unparsed("#nametype 142"),
        MonsterLine::End,
    ]);

}