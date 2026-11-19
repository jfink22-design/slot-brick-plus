//! Per-game missions for Pip. A mission is a RetroAchievements memory condition
//! (`MemAddr`), either written for a known game or taken from a cached set.
use std::path::Path;

/// One concrete goal. `key` is matched against the cart's file name.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GoalOffer {
    pub key: String,
    pub title: String,
    pub detail: String,
    pub cond: String,
    pub need: u32,
    pub streak: String,
    pub streak_cond: String,
    /// True when this came from a cached RetroAchievements set rather than the hand pack.
    pub auto: bool,
    /// Where the address was confirmed. Empty for the older unverified pack and for auto goals.
    pub verified_from: String,
}

impl GoalOffer {
    fn matches(&self, stem: &str) -> bool {
        let stem = fold(stem);
        let key = fold(&self.key);
        key.len() >= 4 && stem.contains(&key)
    }
}

fn fold(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .flat_map(|c| c.to_lowercase())
        .collect()
}

/// The hand-written pack. Auto-generated goals from a cached set are preferred
/// when both match the same cart, because those conditions are the set's own.
pub fn hand_goals() -> Vec<GoalOffer> {
    const PACK: &[(&str, &str, &str, &str, u32, &str, &str)] = &[
        (
            "balloonfight",
            "Pop 5 balloons",
            "Balloon Fight",
            "0xH0030>=5",
            5,
            "Pop a balloon",
            "0xH0030>=1",
        ),
        (
            "duckhunt",
            "Hit 10 ducks",
            "Duck Hunt",
            "0xH0082>=10",
            10,
            "Hit a duck",
            "0xH0082>=1",
        ),
        (
            "excitebike",
            "Finish a lap",
            "Excitebike",
            "0xH0050>=1",
            1,
            "Finish a lap",
            "0xH0050>=1",
        ),
        (
            "marioworld",
            "Clear a level",
            "Super Mario World",
            "0xH0DD5>=1",
            1,
            "Clear a level",
            "0xH0DD5>=1",
        ),
        (
            "marioland",
            "Collect 50 coins",
            "Super Mario Land",
            "0xH9829>=50",
            50,
            "Clear a level",
            "0xH982A>=1",
        ),
        (
            "drmario",
            "Clear 10 viruses",
            "Dr. Mario",
            "0xH0310>=10",
            10,
            "Clear a virus",
            "0xH0310>=1",
        ),
        (
            "mariobros",
            "Collect 20 coins",
            "Super Mario Bros.",
            "0xH075E>=20",
            20,
            "Clear a level",
            "0xH075F>=1",
        ),
        (
            "mario",
            "Collect 20 coins",
            "Mario",
            "0xH075E>=20",
            20,
            "Clear a level",
            "0xH075F>=1",
        ),
        (
            "tetris",
            "Clear 10 lines",
            "Tetris",
            "0xH00A0>=10",
            10,
            "Clear 10 lines",
            "0xH00A0>=10",
        ),
        (
            "linktothepast",
            "Collect 50 rupees",
            "A Link to the Past",
            "0xH0DB5>=50",
            50,
            "Clear a room",
            "0xH0DB4>=1",
        ),
        (
            "awakening",
            "Clear a dungeon",
            "Link's Awakening",
            "0xHDB4C>=1",
            1,
            "Clear a dungeon",
            "0xHDB4C>=1",
        ),
        (
            "zelda",
            "Collect 50 rupees",
            "Zelda",
            "0xH066D>=50",
            50,
            "Clear a room",
            "0xH066E>=1",
        ),
        (
            "pokemonred",
            "Catch a new Pokemon",
            "Pokemon Red",
            "0xHD16B>=1",
            1,
            "Catch a new Pokemon",
            "0xHD16B>=1",
        ),
        (
            "pokemonblue",
            "Catch a new Pokemon",
            "Pokemon Blue",
            "0xHD16B>=1",
            1,
            "Catch a new Pokemon",
            "0xHD16B>=1",
        ),
        (
            "firered",
            "Beat the first gym",
            "Pokemon FireRed",
            "0xH03F0>=1",
            1,
            "Beat a gym",
            "0xH03F0>=1",
        ),
        (
            "leafgreen",
            "Beat the first gym",
            "Pokemon LeafGreen",
            "0xH03F0>=1",
            1,
            "Beat a gym",
            "0xH03F0>=1",
        ),
        (
            "emerald",
            "Beat the first gym",
            "Pokemon Emerald",
            "0xH03F0>=1",
            1,
            "Beat a gym",
            "0xH03F0>=1",
        ),
        (
            "ruby",
            "Beat the first gym",
            "Pokemon Ruby",
            "0xH03F0>=1",
            1,
            "Beat a gym",
            "0xH03F0>=1",
        ),
        (
            "sapphire",
            "Beat the first gym",
            "Pokemon Sapphire",
            "0xH03F0>=1",
            1,
            "Beat a gym",
            "0xH03F0>=1",
        ),
        (
            "pokemon",
            "Catch a new Pokemon",
            "Pokemon",
            "0xHD16B>=1",
            1,
            "Catch a new Pokemon",
            "0xHD16B>=1",
        ),
        (
            "supermetroid",
            "Collect an energy tank",
            "Super Metroid",
            "0xH09C4>=1",
            1,
            "Collect an item",
            "0xH09C4>=1",
        ),
        (
            "metroid",
            "Collect a missile",
            "Metroid",
            "0xH6878>=1",
            1,
            "Collect an item",
            "0xH6878>=1",
        ),
        (
            "kirby",
            "Clear a stage",
            "Kirby",
            "0xH00D0>=1",
            1,
            "Clear a stage",
            "0xH00D0>=1",
        ),
        (
            "castlevania",
            "Defeat an enemy",
            "Castlevania",
            "0xH0040>=1",
            1,
            "Defeat an enemy",
            "0xH0040>=1",
        ),
        (
            "megaman",
            "Beat a stage",
            "Mega Man",
            "0xH00BF>=1",
            1,
            "Beat a stage",
            "0xH00BF>=1",
        ),
        (
            "chrono",
            "Win a battle",
            "Chrono Trigger",
            "0xH0B00>=1",
            1,
            "Win a battle",
            "0xH0B00>=1",
        ),
        (
            "earthbound",
            "Win a battle",
            "EarthBound",
            "0xH0C00>=1",
            1,
            "Win a battle",
            "0xH0C00>=1",
        ),
        (
            "finalfantasyvii",
            "Win a battle",
            "Final Fantasy VII",
            "0xH0E00>=1",
            1,
            "Win a battle",
            "0xH0E00>=1",
        ),
        (
            "ffvii",
            "Win a battle",
            "Final Fantasy VII",
            "0xH0E00>=1",
            1,
            "Win a battle",
            "0xH0E00>=1",
        ),
        (
            "spyro",
            "Collect 20 gems",
            "Spyro",
            "0xH0F20>=20",
            20,
            "Collect a gem",
            "0xH0F20>=1",
        ),
        (
            "donkeykong",
            "Climb a stage",
            "Donkey Kong",
            "0xH0060>=1",
            1,
            "Climb a stage",
            "0xH0060>=1",
        ),
        (
            "warioland",
            "Collect 50 coins",
            "Wario Land",
            "0xH00C8>=50",
            50,
            "Clear a level",
            "0xH00C9>=1",
        ),
    ];
    let mut goals: Vec<GoalOffer> = PACK
        .iter()
        .map(
            |(key, title, detail, cond, need, streak, streak_cond)| GoalOffer {
                key: (*key).into(),
                title: (*title).into(),
                detail: (*detail).into(),
                cond: (*cond).into(),
                need: *need,
                streak: (*streak).into(),
                streak_cond: (*streak_cond).into(),
                auto: false,
                verified_from: String::new(),
            },
        )
        .collect();
    goals.extend(verified_goals());
    goals
}

/// How many distinct games the hand pack covers (one game can have a long and a short key).
pub fn hand_game_count() -> usize {
    let mut titles = Vec::new();
    for goal in hand_goals() {
        if !titles.contains(&goal.detail) {
            titles.push(goal.detail);
        }
    }
    titles.len()
}

fn offer(
    key: &str,
    title: &str,
    detail: &str,
    cond: &str,
    need: u32,
    streak: &str,
    streak_cond: &str,
    verified_from: &str,
) -> GoalOffer {
    GoalOffer {
        key: key.into(),
        title: title.into(),
        detail: detail.into(),
        cond: cond.into(),
        need,
        streak: streak.into(),
        streak_cond: streak_cond.into(),
        auto: false,
        verified_from: verified_from.into(),
    }
}

/// Missions whose addresses were checked against a published RAM map.
/// GBA IWRAM is RetroAchievements address 0. GBA EWRAM starts at 0x8000.
/// A counted goal finishes after that many real changes, not because a save is already there.
pub fn verified_goals() -> Vec<GoalOffer> {
    const FUSION: &str = "Data Crystal Metroid Fusion/RAM map (RA game 785)";
    const ARIA: &str = "Data Crystal Castlevania: Aria of Sorrow/RAM map (RA game 510)";
    const ZERO: &str = "Data Crystal Metroid Zero Mission/RAM map (RA game 534)";
    const MOON: &str = "Data Crystal Castlevania: Circle of the Moon/RAM map (RA game 745)";
    const HARMONY: &str =
        "Data Crystal Castlevania: Harmony of Dissonance/RAM map (RA game 2353)";
    const WARIO: &str = "Data Crystal Wario Land 4/RAM map, USA IWRAM (RA game 800)";
    const KIRBY: &str = "Data Crystal Kirby & the Amazing Mirror/RAM map (RA game 769)";
    const KART: &str = "Data Crystal Mario Kart: Super Circuit/RAM map, USA (RA game 533)";
    const SOTN: &str =
        "Data Crystal Symphony of the Night (PlayStation)/RAM map (RA game 11240)";
    let mut out = Vec::new();
    let mut add = |key: &str,
                   detail: &str,
                   title: &str,
                   cond: &str,
                   need: u32,
                   streak: &str,
                   streak_cond: &str,
                   source: &str| {
        out.push(offer(
            key,
            title,
            detail,
            cond,
            need,
            streak,
            streak_cond,
            source,
        ));
    };
    add("metroidfusion", "Metroid Fusion", "Find an Energy Tank", "0xH000bde=1_0x001312>d0x001312", 1, "Travel between sectors", "0xH000bde=1_0xH00002c!=d0xH00002c", FUSION);
    add("metroidfusion", "Metroid Fusion", "Find a Missile Tank", "0xH000bde=1_0x001316>d0x001316", 1, "Travel between sectors", "0xH000bde=1_0xH00002c!=d0xH00002c", FUSION);
    add("metroidfusion", "Metroid Fusion", "Fire 5 missiles", "0xH000bde=1_d0x001314>0x001314", 5, "Travel between sectors", "0xH000bde=1_0xH00002c!=d0xH00002c", FUSION);
    add("metroidfusion", "Metroid Fusion", "Perform a wall jump", "0xH000bde=1_0xH001245=22_d0xH001245!=22", 1, "Travel between sectors", "0xH000bde=1_0xH00002c!=d0xH00002c", FUSION);
    add("metroidfusion", "Metroid Fusion", "Find a Power Bomb Tank", "0xH000bde=1_0xH001319>d0xH001319", 1, "Travel between sectors", "0xH000bde=1_0xH00002c!=d0xH00002c", FUSION);
    add("metroidfusion", "Metroid Fusion", "Enter Sector 4 (AQA)", "0xH000bde=1_0xH00002c=4_d0xH00002c!=4", 1, "Travel between sectors", "0xH000bde=1_0xH00002c!=d0xH00002c", FUSION);
    add("metroidfusion", "Metroid Fusion", "Perform a Shinespark", "0xH000bde=1_0xH001245=36_d0xH001245!=36", 1, "Travel between sectors", "0xH000bde=1_0xH00002c!=d0xH00002c", FUSION);
    add("metroidfusion", "Metroid Fusion", "Enter Sector 6 (NOC)", "0xH000bde=1_0xH00002c=6_d0xH00002c!=6", 1, "Travel between sectors", "0xH000bde=1_0xH00002c!=d0xH00002c", FUSION);
    add("ariaofsorrow", "Aria of Sorrow", "Absorb a monster soul", "0xH008010=4_0x01b264>d0x01b264", 1, "Absorb a monster soul", "0xH008010=4_0x01b264>d0x01b264", ARIA);
    add("ariaofsorrow", "Aria of Sorrow", "Perform a slide kick", "0xH008010=4_0xH0084ee=3_d0xH0084ee!=3", 1, "Absorb a monster soul", "0xH008010=4_0x01b264>d0x01b264", ARIA);
    add("ariaofsorrow", "Aria of Sorrow", "Level up Soma", "0xH008010=4_0xH01b266=0_0xH01b279>d0xH01b279", 1, "Absorb a monster soul", "0xH008010=4_0x01b264>d0x01b264", ARIA);
    add("ariaofsorrow", "Aria of Sorrow", "Absorb 3 monster souls", "0xH008010=4_0x01b264>d0x01b264", 3, "Absorb a monster soul", "0xH008010=4_0x01b264>d0x01b264", ARIA);
    add("ariaofsorrow", "Aria of Sorrow", "Perform a dive kick", "0xH008010=4_0xH0084ee=7_d0xH0084ee!=7", 1, "Absorb a monster soul", "0xH008010=4_0x01b264>d0x01b264", ARIA);
    add("ariaofsorrow", "Aria of Sorrow", "Transform into a bat", "0xH008010=4_0xH0084ee=8_d0xH0084ee!=8", 1, "Absorb a monster soul", "0xH008010=4_0x01b264>d0x01b264", ARIA);
    add("ariaofsorrow", "Aria of Sorrow", "Defeat a boss", "0xH008010=4_0xH00837f>d0xH00837f", 1, "Absorb a monster soul", "0xH008010=4_0x01b264>d0x01b264", ARIA);
    add("ariaofsorrow", "Aria of Sorrow", "Absorb 5 monster souls", "0xH008010=4_0x01b264>d0x01b264", 5, "Absorb a monster soul", "0xH008010=4_0x01b264>d0x01b264", ARIA);
    add("zeromission", "Metroid Zero Mission", "Find an Energy Tank", "0x000c70=4_0x001530>d0x001530", 1, "Travel between regions", "0x000c70=4_0xH000054!=d0xH000054", ZERO);
    add("zeromission", "Metroid Zero Mission", "Find a Missile Tank", "0x000c70=4_0x001532>d0x001532", 1, "Travel between regions", "0x000c70=4_0xH000054!=d0xH000054", ZERO);
    add("zeromission", "Metroid Zero Mission", "Perform a wall jump", "0x000c70=4_0xH0013d4=13_d0xH0013d4!=13", 1, "Travel between regions", "0x000c70=4_0xH000054!=d0xH000054", ZERO);
    add("zeromission", "Metroid Zero Mission", "Fire a Super Missile", "0x000c70=4_0xH001415=3_d0xH001415!=3", 1, "Travel between regions", "0x000c70=4_0xH000054!=d0xH000054", ZERO);
    add("zeromission", "Metroid Zero Mission", "Find a Power Bomb Tank", "0x000c70=4_0xH001535>d0xH001535", 1, "Travel between regions", "0x000c70=4_0xH000054!=d0xH000054", ZERO);
    add("zeromission", "Metroid Zero Mission", "Enter Norfair", "0x000c70=4_0xH000054=2_d0xH000054!=2", 1, "Travel between regions", "0x000c70=4_0xH000054!=d0xH000054", ZERO);
    add("zeromission", "Metroid Zero Mission", "Perform a Shinespark", "0x000c70=4_0xH0013d4=34_d0xH0013d4!=34", 1, "Travel between regions", "0x000c70=4_0xH000054!=d0xH000054", ZERO);
    add("zeromission", "Metroid Zero Mission", "Enter Chozodia", "0x000c70=4_0xH000054=6_d0xH000054!=6", 1, "Travel between regions", "0x000c70=4_0xH000054!=d0xH000054", ZERO);
    add("circleofthemoon", "Circle of the Moon", "Discover a new room", "0x02d414>d0x02d414", 1, "Discover a room", "0x02d414>d0x02d414", MOON);
    add("circleofthemoon", "Circle of the Moon", "Gain a level", "0xH02d664>d0xH02d664", 1, "Discover a room", "0x02d414>d0x02d414", MOON);
    add("circleofthemoon", "Circle of the Moon", "Collect 5 hearts", "0x02d63c>d0x02d63c", 5, "Discover a room", "0x02d414>d0x02d414", MOON);
    add("circleofthemoon", "Circle of the Moon", "Discover 5 new rooms", "0x02d414>d0x02d414", 5, "Discover a room", "0x02d414>d0x02d414", MOON);
    add("circleofthemoon", "Circle of the Moon", "Increase max HP", "0x02d632>d0x02d632", 1, "Discover a room", "0x02d414>d0x02d414", MOON);
    add("circleofthemoon", "Circle of the Moon", "Find the Salamander card", "0xH02d674=1_d0xH02d674=0", 1, "Discover a room", "0x02d414>d0x02d414", MOON);
    add("circleofthemoon", "Circle of the Moon", "Find the Unicorn card", "0xH02d67c=1_d0xH02d67c=0", 1, "Discover a room", "0x02d414>d0x02d414", MOON);
    add("circleofthemoon", "Circle of the Moon", "Find the Black Dog card", "0xH02d67d=1_d0xH02d67d=0", 1, "Discover a room", "0x02d414>d0x02d414", MOON);
    add("harmonyofdissonance", "Harmony of Dissonance", "Gain a level", "0xH02077c>d0xH02077c", 1, "Pick up hearts", "0x020794>d0x020794", HARMONY);
    add("harmonyofdissonance", "Harmony of Dissonance", "Collect hearts", "0x020794>d0x020794", 1, "Pick up hearts", "0x020794>d0x020794", HARMONY);
    add("harmonyofdissonance", "Harmony of Dissonance", "Increase max HP", "0x020786>d0x020786", 1, "Pick up hearts", "0x020794>d0x020794", HARMONY);
    add("harmonyofdissonance", "Harmony of Dissonance", "Increase max MP", "0x020788>d0x020788", 1, "Pick up hearts", "0x020794>d0x020794", HARMONY);
    add("harmonyofdissonance", "Harmony of Dissonance", "Find the Fire Book", "0xM02083e=1_d0xM02083e=0", 1, "Pick up hearts", "0x020794>d0x020794", HARMONY);
    add("harmonyofdissonance", "Harmony of Dissonance", "Find the Ice Book", "0xN02083e=1_d0xN02083e=0", 1, "Pick up hearts", "0x020794>d0x020794", HARMONY);
    add("harmonyofdissonance", "Harmony of Dissonance", "Find Griffin's Wing", "0xO02083f=1_d0xO02083f=0", 1, "Pick up hearts", "0x020794>d0x020794", HARMONY);
    add("harmonyofdissonance", "Harmony of Dissonance", "Find the Summoning Tome", "0xQ02083e=1_d0xQ02083e=0", 1, "Pick up hearts", "0x020794>d0x020794", HARMONY);
    add("warioland4", "Wario Land 4", "Defeat an enemy", "0xH000c06>d0xH000c06", 1, "Collect money in a level", "0x000bf4>d0x000bf4", WARIO);
    add("warioland4", "Wario Land 4", "Perform a dash jump", "0xH001899=24_d0xH001899!=24", 1, "Collect money in a level", "0x000bf4>d0x000bf4", WARIO);
    add("warioland4", "Wario Land 4", "Defeat 5 enemies", "0xH000c06>d0xH000c06", 5, "Collect money in a level", "0x000bf4>d0x000bf4", WARIO);
    add("warioland4", "Wario Land 4", "Perform a Super Smash", "0xH001899=28_d0xH001899!=28", 1, "Collect money in a level", "0x000bf4>d0x000bf4", WARIO);
    add("warioland4", "Wario Land 4", "Become Zombie Wario", "0xH001898=5_d0xH001898!=5", 1, "Collect money in a level", "0x000bf4>d0x000bf4", WARIO);
    add("warioland4", "Wario Land 4", "Become Vampire Wario", "0xH001898=9_d0xH001898!=9", 1, "Collect money in a level", "0x000bf4>d0x000bf4", WARIO);
    add("amazingmirror", "Kirby and the Amazing Mirror", "Copy Fire", "0xH028fe3=1_d0xH028fe3!=1", 1, "Copy a power", "0xH028fe3!=0_0xH028fe3!=d0xH028fe3", KIRBY);
    add("amazingmirror", "Kirby and the Amazing Mirror", "Copy Wheel", "0xH028fe3=4_d0xH028fe3!=4", 1, "Copy a power", "0xH028fe3!=0_0xH028fe3!=d0xH028fe3", KIRBY);
    add("amazingmirror", "Kirby and the Amazing Mirror", "Copy Sword", "0xH028fe3=18_d0xH028fe3!=18", 1, "Copy a power", "0xH028fe3!=0_0xH028fe3!=d0xH028fe3", KIRBY);
    add("amazingmirror", "Kirby and the Amazing Mirror", "Copy Hammer", "0xH028fe3=17_d0xH028fe3!=17", 1, "Copy a power", "0xH028fe3!=0_0xH028fe3!=d0xH028fe3", KIRBY);
    add("amazingmirror", "Kirby and the Amazing Mirror", "Become Mini Kirby", "0xH028fe3=23_d0xH028fe3!=23", 1, "Copy a power", "0xH028fe3!=0_0xH028fe3!=d0xH028fe3", KIRBY);
    add("amazingmirror", "Kirby and the Amazing Mirror", "Copy UFO", "0xH028fe3=14_d0xH028fe3!=14", 1, "Copy a power", "0xH028fe3!=0_0xH028fe3!=d0xH028fe3", KIRBY);
    add("amazingmirror", "Kirby and the Amazing Mirror", "Copy Smash", "0xH028fe3=22_d0xH028fe3!=22", 1, "Copy a power", "0xH028fe3!=0_0xH028fe3!=d0xH028fe3", KIRBY);
    add("amazingmirror", "Kirby and the Amazing Mirror", "Copy Master", "0xH028fe3=26_d0xH028fe3!=26", 1, "Copy a power", "0xH028fe3!=0_0xH028fe3!=d0xH028fe3", KIRBY);
    add("mariokartsupercircuit", "Mario Kart Super Circuit", "Pick up a coin", "0xH003d10>d0xH003d10", 1, "Collect a coin during a race", "0xH003d10>d0xH003d10", KART);
    add("mariokartsupercircuit", "Mario Kart Super Circuit", "Pick up 5 coins", "0xH003d10>d0xH003d10", 5, "Collect a coin during a race", "0xH003d10>d0xH003d10", KART);
    add("mariokartsupercircuit", "Mario Kart Super Circuit", "Pick up a Mushroom", "0xH003d12=7_0xH003d13=16_d0xH003d12!=7", 1, "Collect a coin during a race", "0xH003d10>d0xH003d10", KART);
    add("mariokartsupercircuit", "Mario Kart Super Circuit", "Pick up 10 coins", "0xH003d10>d0xH003d10", 10, "Collect a coin during a race", "0xH003d10>d0xH003d10", KART);
    add("mariokartsupercircuit", "Mario Kart Super Circuit", "Pick up a Red Shell", "0xH003d12=2_0xH003d13=16_d0xH003d12!=2", 1, "Collect a coin during a race", "0xH003d10>d0xH003d10", KART);
    add("mariokartsupercircuit", "Mario Kart Super Circuit", "Use a Star", "0x003c02>d0x003c02", 1, "Collect a coin during a race", "0xH003d10>d0xH003d10", KART);
    add("mariokartsupercircuit", "Mario Kart Super Circuit", "Drive over a speed panel", "0xH003c1d=22_d0xH003c1d!=22", 1, "Collect a coin during a race", "0xH003d10>d0xH003d10", KART);
    add("mariokartsupercircuit", "Mario Kart Super Circuit", "Pick up Lightning", "0xH003d12=12_0xH003d13=16_d0xH003d12!=12", 1, "Collect a coin during a race", "0xH003d10>d0xH003d10", KART);
    add("symphonyofthenight", "Symphony of the Night", "Defeat 5 enemies", "0xX097ba0>0_0xX097bf4>d0xX097bf4", 5, "Defeat an enemy", "0xX097ba0>0_0xX097bf4>d0xX097bf4", SOTN);
    add("symphonyofthenight", "Symphony of the Night", "Defeat 15 enemies", "0xX097ba0>0_0xX097bf4>d0xX097bf4", 15, "Defeat an enemy", "0xX097ba0>0_0xX097bf4>d0xX097bf4", SOTN);
    add("symphonyofthenight", "Symphony of the Night", "Defeat 40 enemies", "0xX097ba0>0_0xX097bf4>d0xX097bf4", 40, "Defeat an enemy", "0xX097ba0>0_0xX097bf4>d0xX097bf4", SOTN);
    add("symphonyofthenight", "Symphony of the Night", "Enter 3 new rooms", "0xX097ba0>0_0xX03c760>d0xX03c760", 3, "Defeat an enemy", "0xX097ba0>0_0xX097bf4>d0xX097bf4", SOTN);
    add("symphonyofthenight", "Symphony of the Night", "Enter 10 new rooms", "0xX097ba0>0_0xX03c760>d0xX03c760", 10, "Defeat an enemy", "0xX097ba0>0_0xX097bf4>d0xX097bf4", SOTN);
    add("symphonyofthenight", "Symphony of the Night", "Gain hearts 5 times", "0xX097ba0>0_0xX097ba8>d0xX097ba8", 5, "Defeat an enemy", "0xX097ba0>0_0xX097bf4>d0xX097bf4", SOTN);
    add("symphonyofthenight", "Symphony of the Night", "Gain hearts 20 times", "0xX097ba0>0_0xX097ba8>d0xX097ba8", 20, "Defeat an enemy", "0xX097ba0>0_0xX097bf4>d0xX097bf4", SOTN);
    add("symphonyofthenight", "Symphony of the Night", "Gain gold 5 times", "0xX097ba0>0_0xX097bf0>d0xX097bf0", 5, "Defeat an enemy", "0xX097ba0>0_0xX097bf4>d0xX097bf4", SOTN);
    const GOLD: &str = "Data Crystal Pokemon Gold and Silver/RAM map (RA games 576 and 722)";
    const CRYSTAL: &str = "Data Crystal Pokemon Crystal/RAM map (RA game 810)";
    const WARIO3: &str = "Data Crystal Wario Land 3/RAM map (RA game 741)";
    const DELUXE: &str = "Data Crystal Super Mario Bros. Deluxe/RAM map, USA and Europe";
    const SEASONS: &str = "Data Crystal Oracle of Seasons/RAM map (RA game 676)";
    for key in ["pokemongold", "pokemonsilver"] {
        add(key, "Pokemon Gold and Silver", "Have 2 Pokemon in the party", "d0xHDA22<2_0xHDA22>=2", 1, "Level up the lead", "0xHDA49>d0xHDA49_0xHDA2A=d0xHDA2A", GOLD);
        add(key, "Pokemon Gold and Silver", "Have 4 Pokemon in the party", "d0xHDA22<4_0xHDA22>=4", 1, "Level up the lead", "0xHDA49>d0xHDA49_0xHDA2A=d0xHDA2A", GOLD);
        add(key, "Pokemon Gold and Silver", "Have a full party of 6", "d0xHDA22<6_0xHDA22>=6", 1, "Level up the lead", "0xHDA49>d0xHDA49_0xHDA2A=d0xHDA2A", GOLD);
        add(key, "Pokemon Gold and Silver", "Level up the lead", "0xHDA49>d0xHDA49_0xHDA2A=d0xHDA2A", 1, "Level up the lead", "0xHDA49>d0xHDA49_0xHDA2A=d0xHDA2A", GOLD);
        add(key, "Pokemon Gold and Silver", "Raise the lead to level 20", "d0xHDA49<20_0xHDA49>=20_0xHDA2A=d0xHDA2A", 1, "Level up the lead", "0xHDA49>d0xHDA49_0xHDA2A=d0xHDA2A", GOLD);
        add(key, "Pokemon Gold and Silver", "Earn the Zephyr Badge", "d0xMD57C=0_0xMD57C=1", 1, "Level up the lead", "0xHDA49>d0xHDA49_0xHDA2A=d0xHDA2A", GOLD);
        add(key, "Pokemon Gold and Silver", "Earn 3 Johto badges", "d0xKD57C<3_0xKD57C>=3", 1, "Level up the lead", "0xHDA49>d0xHDA49_0xHDA2A=d0xHDA2A", GOLD);
    }
    add("pokemoncrystal", "Pokemon Crystal", "Have 2 Pokemon in the party", "d0xHDCD7<2_0xHDCD7>=2", 1, "Level up the lead", "0xHDCFE>d0xHDCFE_0xHDCDF=d0xHDCDF", CRYSTAL);
    add("pokemoncrystal", "Pokemon Crystal", "Have 4 Pokemon in the party", "d0xHDCD7<4_0xHDCD7>=4", 1, "Level up the lead", "0xHDCFE>d0xHDCFE_0xHDCDF=d0xHDCDF", CRYSTAL);
    add("pokemoncrystal", "Pokemon Crystal", "Have a full party of 6", "d0xHDCD7<6_0xHDCD7>=6", 1, "Level up the lead", "0xHDCFE>d0xHDCFE_0xHDCDF=d0xHDCDF", CRYSTAL);
    add("pokemoncrystal", "Pokemon Crystal", "Level up the lead", "0xHDCFE>d0xHDCFE_0xHDCDF=d0xHDCDF", 1, "Level up the lead", "0xHDCFE>d0xHDCFE_0xHDCDF=d0xHDCDF", CRYSTAL);
    add("pokemoncrystal", "Pokemon Crystal", "Raise the lead to level 20", "d0xHDCFE<20_0xHDCFE>=20_0xHDCDF=d0xHDCDF", 1, "Level up the lead", "0xHDCFE>d0xHDCFE_0xHDCDF=d0xHDCDF", CRYSTAL);
    add("pokemoncrystal", "Pokemon Crystal", "Earn the Zephyr Badge", "d0xMD857=0_0xMD857=1", 1, "Level up the lead", "0xHDCFE>d0xHDCFE_0xHDCDF=d0xHDCDF", CRYSTAL);
    add("pokemoncrystal", "Pokemon Crystal", "Earn the Hive Badge", "d0xND857=0_0xND857=1", 1, "Level up the lead", "0xHDCFE>d0xHDCFE_0xHDCDF=d0xHDCDF", CRYSTAL);
    add("pokemoncrystal", "Pokemon Crystal", "Earn 3 Johto badges", "d0xKD857<3_0xKD857>=3", 1, "Level up the lead", "0xHDCFE>d0xHDCFE_0xHDCDF=d0xHDCDF", CRYSTAL);
    add("warioland3", "Wario Land 3", "Find the Gray Key", "0xHC09B=2_d0xMCA5B=0_0xMCA5B=1", 1, "Finish a stage", "d0xHC09B=2_0xHC09B=3", WARIO3);
    add("warioland3", "Wario Land 3", "Find the Red Key", "0xHC09B=2_d0xNCA5B=0_0xNCA5B=1", 1, "Finish a stage", "d0xHC09B=2_0xHC09B=3", WARIO3);
    add("warioland3", "Wario Land 3", "Find the Green Key", "0xHC09B=2_d0xOCA5B=0_0xOCA5B=1", 1, "Finish a stage", "d0xHC09B=2_0xHC09B=3", WARIO3);
    add("warioland3", "Wario Land 3", "Hold 3 keys", "0xHC09B=2_d0xKCA5B<3_0xKCA5B>=3", 1, "Finish a stage", "d0xHC09B=2_0xHC09B=3", WARIO3);
    add("warioland3", "Wario Land 3", "Become Zombie Wario", "0xHC09B=2_d0xHCA8E!=72_0xHCA8E=72", 1, "Finish a stage", "d0xHC09B=2_0xHC09B=3", WARIO3);
    add("warioland3", "Wario Land 3", "Become Fat Wario", "0xHC09B=2_d0xHCA8E!=132_0xHCA8E=132", 1, "Finish a stage", "d0xHC09B=2_0xHC09B=3", WARIO3);
    add("brosdeluxe", "Super Mario Bros. Deluxe", "Collect a coin", "0xHC160=0_0xHC1F2>d0xHC1F2", 1, "Collect a coin", "0xHC1F2>d0xHC1F2", DELUXE);
    add("brosdeluxe", "Super Mario Bros. Deluxe", "Earn an extra life", "0xHC17F>d0xHC17F", 1, "Collect a coin", "0xHC1F2>d0xHC1F2", DELUXE);
    add("brosdeluxe", "Super Mario Bros. Deluxe", "Reach 5 lives", "d0xHC17F<5_0xHC17F>=5", 1, "Collect a coin", "0xHC1F2>d0xHC1F2", DELUXE);
    add("brosdeluxe", "Super Mario Bros. Deluxe", "Reach 8 lives", "d0xHC17F<8_0xHC17F>=8", 1, "Collect a coin", "0xHC1F2>d0xHC1F2", DELUXE);
    add("brosdeluxe", "Super Mario Bros. Deluxe", "Become Super Mario", "d0xHC1C5=0_0xHC1C5=1_0xHC383=0", 1, "Collect a coin", "0xHC1F2>d0xHC1F2", DELUXE);
    add("brosdeluxe", "Super Mario Bros. Deluxe", "Switch to Luigi", "d0xHC383=0_0xHC383=1", 1, "Collect a coin", "0xHC1F2>d0xHC1F2", DELUXE);
    add("brosdeluxe", "Super Mario Bros. Deluxe", "Jump as Luigi", "0xHC383=1_d0xHC1C7=0_0xHC1C7=20", 1, "Collect a coin", "0xHC1F2>d0xHC1F2", DELUXE);
    add("brosdeluxe", "Super Mario Bros. Deluxe", "Jump while Super", "0xHC1C5=1_d0xHC1C7=0_0xHC1C7=20", 1, "Collect a coin", "0xHC1F2>d0xHC1F2", DELUXE);
    add("oracleofseasons", "Oracle of Seasons", "Defeat an enemy", "0xC620>d0xC620", 1, "Defeat an enemy", "0xC620>d0xC620", SEASONS);
    add("oracleofseasons", "Oracle of Seasons", "Defeat 10 enemies", "0xC620>d0xC620", 10, "Defeat an enemy", "0xC620>d0xC620", SEASONS);
    add("oracleofseasons", "Oracle of Seasons", "Defeat 30 enemies", "0xC620>d0xC620", 30, "Defeat an enemy", "0xC620>d0xC620", SEASONS);
    add("oracleofseasons", "Oracle of Seasons", "Destroy a sign", "0xHC626>d0xHC626", 1, "Defeat an enemy", "0xC620>d0xC620", SEASONS);
    add("oracleofseasons", "Oracle of Seasons", "Find a Piece of Heart", "0xHC6A4>d0xHC6A4", 1, "Defeat an enemy", "0xC620>d0xC620", SEASONS);
    add("oracleofseasons", "Oracle of Seasons", "Raise your max hearts", "0xHC6A3>d0xHC6A3", 1, "Defeat an enemy", "0xC620>d0xC620", SEASONS);
    add("oracleofseasons", "Oracle of Seasons", "Upgrade the sword", "0xHC6AC>d0xHC6AC", 1, "Defeat an enemy", "0xC620>d0xC620", SEASONS);
    add("oracleofseasons", "Oracle of Seasons", "Upgrade the shield", "0xHC6A9>d0xHC6A9", 1, "Defeat an enemy", "0xC620>d0xC620", SEASONS);
    out
}

/// Read every cached RetroAchievements set and turn a few of its conditions into missions.
/// Returns the goals and how many set files produced at least one.
pub fn load_goal_book(root: &Path) -> (Vec<GoalOffer>, usize) {
    let mut goals = Vec::new();
    let mut games = 0usize;
    let dir = root.join("Config/Achievements/sets");
    if let Ok(rd) = std::fs::read_dir(&dir) {
        let mut files: Vec<_> = rd.flatten().collect();
        files.sort_by_key(|e| e.file_name());
        for ent in files {
            let Ok(text) = std::fs::read_to_string(ent.path()) else {
                continue;
            };
            let found = goals_from_set(&text);
            if !found.is_empty() {
                games += 1;
                goals.extend(found);
            }
        }
    }
    goals.extend(hand_goals());
    (goals, games)
}

/// Up to three missions from one set's achievement and leaderboard conditions.
pub fn goals_from_set(body: &str) -> Vec<GoalOffer> {
    let game = first_string(body, "Title").unwrap_or_default();
    if game.len() < 2 {
        return Vec::new();
    }
    let mut found = Vec::new();
    for key in ["MemAddr", "Mem"] {
        let mut from = 0;
        while let Some(rel) = body[from..].find(&format!("\"{key}\"")) {
            let at = from + rel;
            from = at + key.len() + 2;
            let Some(mem) = quoted_after(body, from) else {
                continue;
            };
            if !mem.value.contains("0x") || mem.value.len() > 240 {
                continue;
            }
            let title = preceding(body, at, "Title").unwrap_or_else(|| "In-game goal".into());
            if title == game {
                continue;
            }
            let detail = preceding(body, at, "Description").unwrap_or_default();
            let need = need_of(&mem.value);
            found.push(GoalOffer {
                key: game.clone(),
                title,
                detail,
                cond: mem.value,
                need,
                streak: format!("Make progress in {game}"),
                streak_cond: String::new(),
                auto: true,
                verified_from: String::new(),
            });
        }
    }
    found.sort_by_key(|g| g.cond.len());
    found.truncate(3);
    found
}

fn need_of(cond: &str) -> u32 {
    let tail = cond.rsplit(['=', '<', '>']).next().unwrap_or("");
    let digits: String = tail
        .chars()
        .skip_while(|c| !c.is_ascii_digit())
        .take_while(|c| c.is_ascii_digit())
        .collect();
    digits.parse().unwrap_or(1).max(1)
}

struct Quoted {
    value: String,
}

fn first_string(body: &str, key: &str) -> Option<String> {
    let at = body.find(&format!("\"{key}\""))?;
    quoted_after(body, at + key.len() + 2).map(|q| q.value)
}

fn preceding(body: &str, at: usize, key: &str) -> Option<String> {
    let start = at.saturating_sub(600);
    let window = &body[start..at];
    let needle = format!("\"{key}\"");
    let rel = window.rfind(&needle)?;
    quoted_after(window, rel + needle.len()).map(|q| q.value)
}

fn quoted_after(body: &str, at: usize) -> Option<Quoted> {
    let rest = &body[at..];
    let colon = rest.find(':')?;
    let after = rest[colon + 1..].trim_start();
    if !after.starts_with('"') {
        return None;
    }
    let mut out = String::new();
    let mut chars = after[1..].chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            if let Some(n) = chars.next() {
                out.push(n);
            }
            continue;
        }
        if c == '"' {
            break;
        }
        out.push(c);
    }
    if out.is_empty() {
        return None;
    }
    Some(Quoted { value: out })
}

/// The offer to use for this cart. Cached-set goals win. Otherwise the longest hand key.
pub fn offer_for<'a>(book: &'a [GoalOffer], stem: &str, salt: usize) -> Option<&'a GoalOffer> {
    let hits: Vec<&GoalOffer> = book.iter().filter(|g| g.matches(stem)).collect();
    if hits.is_empty() {
        return None;
    }
    let autos: Vec<&GoalOffer> = hits.iter().copied().filter(|g| g.auto).collect();
    let pool = if !autos.is_empty() {
        autos
    } else {
        let best = hits.iter().map(|g| g.key.len()).max().unwrap_or(0);
        hits.into_iter().filter(|g| g.key.len() == best).collect()
    };
    pool.get(salt % pool.len()).copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_cached_set_becomes_three_missions_and_beats_the_hand_pack() {
        let body = r#"{"Title":"Tetris","Achievements":[
            {"Title":"Clear","Description":"Clear a line","MemAddr":"0xH123=1"},
            {"Title":"Tall","Description":"A stack","MemAddr":"0xH124>=10"},
            {"Title":"Wide","Description":"A row","MemAddr":"0xH125>=3"},
            {"Title":"Essay","Description":"Too long","MemAddr":"0xH126>=1.and.a.very.long.condition.that.we.keep.only.if.it.is.among.the.three.shortest.which.it.is.not.because.this.sentence.goes.on.and.on.until.it.passes.the.ones.we.want"}
        ]}"#;
        let auto = goals_from_set(body);
        assert_eq!(auto.len(), 3);
        assert!(auto.iter().all(|g| g.auto && g.key == "Tetris"));
        assert!(auto
            .iter()
            .any(|g| g.title == "Clear" && g.cond == "0xH123=1"));
        let mut book = auto;
        book.extend(hand_goals());
        let picked = offer_for(&book, "Tetris", 0).unwrap();
        assert!(picked.auto);
        assert_eq!(picked.cond, "0xH123=1");
        let again = offer_for(&book, "Tetris", 1).unwrap();
        assert_ne!(picked.cond, again.cond);
    }

    #[test]
    fn the_longest_hand_key_wins_and_a_short_name_still_matches() {
        let book = hand_goals();
        let world = offer_for(&book, "Super Mario World", 0).unwrap();
        assert_eq!(world.key, "marioworld");
        let mario = offer_for(&book, "Mario", 0).unwrap();
        assert_eq!(mario.key, "mario");
        assert!(offer_for(&book, "Spyro", 0).is_some());
        assert!(offer_for(&book, "Something Else", 0).is_none());
        assert!(hand_game_count() >= 20);
    }

    #[test]
    fn verified_conditions_parse_and_name_their_source() {
        let goals = verified_goals();
        assert!(goals.len() >= 60);
        for goal in &goals {
            assert!(
                slot_cheevos::measure(&goal.cond, &[]).is_some(),
                "{}",
                goal.cond
            );
            assert!(
                slot_cheevos::measure(&goal.streak_cond, &[]).is_some(),
                "{}",
                goal.streak_cond
            );
            assert!(goal.verified_from.contains("Data Crystal"), "{}", goal.detail);
        }
        let fusion = offer_for(&goals, "Metroid Fusion", 0).unwrap();
        assert_eq!(fusion.key, "metroidfusion");
        assert!(fusion.cond.contains("0x001312"));
    }

    #[test]
    fn every_hand_condition_is_a_real_memory_check() {
        for goal in hand_goals() {
            assert!(
                slot_cheevos::measure(&goal.cond, &[]).is_some(),
                "{}",
                goal.cond
            );
            assert!(
                slot_cheevos::measure(&goal.streak_cond, &[]).is_some(),
                "{}",
                goal.streak_cond
            );
        }
    }
}
