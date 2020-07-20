use super::*;

use nom::error::VerboseError;

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
#monpresentrec -30
#ownsmonrec "foo"
#montag 4131
#bonusspells 1
#researchbonus 8
#taxcollector
#polyimmune
#nametype 142
#end"#;

    let (_, monster) = parse_monster::<VerboseError<&str>>(input).unwrap();

    match monster.declaration {
        MonsterDeclaration::NewId(4837) => (), // pass
        other => panic!("Unexpected declaration: {:?}", other),
    }

    assert_eq!(monster.name, Some("Elder Stone"));
    assert_eq!(monster.owns_mon_rec, Some(Right("foo")));
    assert_eq!(monster.mon_present_rec, Some(Left(-30)));

    let (referenced_ids, referenced_names, next, previous) =
        monster.referenced_monster_ids_and_names();
    assert_eq!(referenced_ids, vec![-30]);
    assert_eq!(referenced_names, vec!["foo"]);
    assert!(!next);
    assert!(!previous);

    assert_eq!(monster.inner_lines, vec![
        Left("#spr1 \"Hellenika/PsientistMenhirGold.tga\""),
        Right(MonsterLine::Name),
        Left("#descr \"Elder Stones are huge megaliths of polished black stone in which an incredibly ancient spirit of immense power has been sealed for all eternity by the last Pantokrator for having threatened his authority. Standing in mute circles in Karanaac like dark, immutable guardians and pillars of the sky, they may very well be the greatest megaliths of the world both in sheer mass and in mystical might, and their immeasurable age and wisdom naturally caused them to become the true architects and masterminds of the Geodetic Awakening, second only to the God himself. Each Elder Stone is engraved with a single symbol of either benevolence of malevolence called a triskele, and the Elder Stones' innate attunement to creation and destruction is so great that their triskeles perpetually glow with a dim, supernatural light whose color depends on the personality and magical alignment of the spirit that inhabits it. There are therefore four main types of Elder Stone, three of which receive hugely increased skills in either Earth, Astral or Nature magic, and the last of which receives moderately increased skills in all of these magics as well as in the path of Death. A given Elder Stone's type isn't predictable in any way before it has awakened, and once it has been declared it can only change through the great ritual of Reawakening.\""),
        Left("#noheal"),
        Left("#hp 175"),
        Left("#size 6"),
        Left("#prot 23"),
        Left("#mr 18"),
        Left("#mor 30"),
        Left("#str 15"),
        Left("#att 5"),
        Left("#def 0"),
        Left("#prec 5"),
        Left("#ap 2"),
        Left("#mapmove 0"),
        Left("#enc 0"),
        Left("#maxage 2000"),
        Left("#poisonres 25"),
        Left("#eyes 0"),
        Left("#blind"),
        Left("#immobile"),
        Left("#slashres"),
        Left("#pierceres"),
        Left("#inanimate"),
        Left("#amphibian"),
        Left("#neednoteat"),
        Left("#spiritsight"),
        Left("#noleader"),
        Left("#stonebeing"),
        Left("#miscshape"),
        Left("#itemslots 4096"),
        Left("#spellsinger"),
        Left("#gcost 0"),
        Left("#holy"),
        Left("#magicskill 3 1"),
        Left("#magicskill 4 1"),
        Left("#magicskill 6 1"),
        Left("#magicskill 8 2"),
        Left("#magicboost 3 3"),
        Left("#custommagic 11264 100"),
        Left("#custommagic 4096 10"),
        Right(MonsterLine::MonPresentRec),
        Right(MonsterLine::OwnsMonRec),
        Left("#montag 4131"),
        Left("#bonusspells 1"),
        Left("#researchbonus 8"),
        Left("#taxcollector"),
        Left("#polyimmune"),
        Left("#nametype 142"),
    ]);
}
