//! Pip, Slot's companion. Missions, XP and sessions stay in Config/companion.txt.
//! Nothing here is sent anywhere, and resetting it does not touch games or saves.
//!
//! Slot does not track bosses or stages. A mission that play time can prove completes
//! on its own. Y on the companion screen can mark one done by hand, once.
use std::path::Path;

use crate::atomic::atomic_write;
use crate::platform::Platform;

pub const FILE: &str = "Config/companion.txt";

/// XP for the first time a game is actually played.
pub const XP_FIRST: u32 = 20;
/// One game, one goal.
pub const XP_SHORT: u32 = 50;
/// Several games or systems.
pub const XP_MULTI: u32 = 100;
/// A weekend's worth of games.
pub const XP_WEEKLY: u32 = 200;
pub const LEVEL_XP: u32 = 100;
const ACTIVE: usize = 3;
const SESSION_KEEP: usize = 40;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mood {
    Happy,
    Excited,
    Sleepy,
    Curious,
    Celebrating,
    Neutral,
    Sad,
    Frustrated,
}

impl Mood {
    pub fn name(self) -> &'static str {
        match self {
            Mood::Happy => "Happy",
            Mood::Excited => "Excited",
            Mood::Sleepy => "Sleepy",
            Mood::Curious => "Curious",
            Mood::Celebrating => "Celebrating",
            Mood::Neutral => "Neutral",
            Mood::Sad => "Sad",
            Mood::Frustrated => "Frustrated",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MissionKind {
    TryNew,
    Marathon,
    Forgotten,
    Tourist,
    OneMore,
    HiddenGem,
    Weekend,
    /// A concrete in-game goal, checked against the emulator's RAM.
    Goal,
}

impl MissionKind {
    pub const ALL: [MissionKind; 7] = [
        MissionKind::TryNew,
        MissionKind::Marathon,
        MissionKind::Forgotten,
        MissionKind::Tourist,
        MissionKind::OneMore,
        MissionKind::HiddenGem,
        MissionKind::Weekend,
    ];

    fn id(self) -> &'static str {
        match self {
            MissionKind::TryNew => "try",
            MissionKind::Marathon => "marathon",
            MissionKind::Forgotten => "forgotten",
            MissionKind::Tourist => "tourist",
            MissionKind::OneMore => "return",
            MissionKind::HiddenGem => "gem",
            MissionKind::Weekend => "weekend",
            MissionKind::Goal => "goal",
        }
    }

    fn parse(value: &str) -> Option<MissionKind> {
        if value == "goal" {
            return Some(MissionKind::Goal);
        }
        Self::ALL.into_iter().find(|k| k.id() == value)
    }

    /// What the mission is, in ordinary words. The screen and the settings row use this.
    pub fn plain(self) -> &'static str {
        match self {
            MissionKind::TryNew => "Try a game you've never played",
            MissionKind::HiddenGem => "Try a game you rarely play",
            MissionKind::Marathon => "Play one game for 20 minutes",
            MissionKind::Forgotten => "Play a game you haven't opened in a while",
            MissionKind::OneMore => "Play a game one more time",
            MissionKind::Tourist => "Play any game on one system",
            MissionKind::Weekend => "Play 4 different games this weekend",
            MissionKind::Goal => "Do something in a game",
        }
    }

    fn xp(self) -> u32 {
        match self {
            MissionKind::Weekend => XP_WEEKLY,
            MissionKind::Tourist => XP_MULTI,
            _ => XP_SHORT,
        }
    }
}

/// A game Pip can see: what the library knows, plus how many sessions Pip recorded.
#[derive(Clone, Debug)]
pub struct GameSeen {
    pub platform: Platform,
    pub stem: String,
    pub played_secs: u64,
    pub last_played: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Mission {
    pub id: String,
    pub kind: MissionKind,
    pub title: String,
    pub detail: String,
    pub platform: Option<Platform>,
    pub stem: String,
    pub need: u32,
    pub progress: u32,
    pub xp: u32,
    pub done: bool,
    /// Systems or stems already counted, comma separated.
    pub extra: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DoneMission {
    pub title: String,
    pub day: i64,
    /// The day the mission was finished. `day` is the same value, kept for older files.
    pub finished: i64,
    pub xp: u32,
    pub platform: Option<Platform>,
    pub stem: String,
}

/// One game's record: how often it was launched, how long it has been played, and the last session.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameStat {
    pub platform: Platform,
    pub stem: String,
    pub plays: u32,
    pub secs: u64,
    pub last_day: i64,
    pub last_secs: u64,
    pub favorite: bool,
}

/// Totals for one of Slot's systems.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SystemStat {
    pub platform: Platform,
    pub plays: u32,
    pub secs: u64,
    pub last_day: i64,
    pub games: u32,
}

/// One game's own streak: a small daily goal, themed to that game.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameStreak {
    pub platform: Platform,
    pub stem: String,
    pub theme: String,
    pub cond: String,
    pub last_day: i64,
    pub current: u32,
    pub best: u32,
}

/// Days in a row with play, and the longest such run.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Streak {
    pub last_day: i64,
    pub current: u32,
    pub longest: u32,
    pub total_days: u32,
    pub start_day: i64,
}

impl Default for Streak {
    fn default() -> Self {
        Streak {
            last_day: 0,
            current: 0,
            longest: 0,
            total_days: 0,
            start_day: 0,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Session {
    pub platform: Platform,
    pub stem: String,
    pub secs: u64,
    pub day: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Award {
    pub title: String,
    pub detail: String,
    pub xp: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct OpenSession {
    platform: Platform,
    stem: String,
    secs: u64,
    day: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Companion {
    pub enabled: bool,
    pub toasts: bool,
    pub sounds: bool,
    pub animate: bool,
    pub categories: Vec<MissionKind>,
    pub xp: u32,
    pub next_id: u32,
    pub awarded: Vec<String>,
    pub excluded: Vec<Platform>,
    pub missions: Vec<Mission>,
    pub history: Vec<DoneMission>,
    pub sessions: Vec<Session>,
    pub games: Vec<GameStat>,
    pub systems: Vec<SystemStat>,
    pub streak: Streak,
    /// A streak for each game that has one, kept beside the global daily streak.
    pub game_streaks: Vec<GameStreak>,
    /// How many times today's missions have been rerolled. Resets on a new local day.
    pub rerolls: u32,
    pub reroll_day: i64,
    /// Highest streak milestone Pip has been given a ribbon for.
    pub ribbon: u32,
    /// How many cached RetroAchievements sets contributed a mission. Not saved.
    pub auto_games: u32,
    /// How many missions have ever been finished, even after the history list is trimmed.
    pub finished: u32,
    pub last_visit: i64,
    pub face: Mood,
    open: Option<OpenSession>,
    suggest: Option<(Platform, String)>,
    /// Set for a short while after a level-up so the screen can celebrate.
    pub celebrating: bool,
    /// An achievement just unlocked. Not saved; the next launch clears it.
    pub pumped: bool,
    book: Vec<crate::goals::GoalOffer>,
    goals_ready: bool,
    /// Previous RAM sample for this cart, so a delta condition can see a real change.
    last_regions: Option<(Platform, String, Vec<(u32, Vec<u8>)>)>,
}

impl Default for Companion {
    fn default() -> Self {
        Companion {
            enabled: true,
            toasts: true,
            sounds: true,
            animate: true,
            categories: MissionKind::ALL.to_vec(),
            xp: 0,
            next_id: 1,
            awarded: Vec::new(),
            excluded: Vec::new(),
            missions: Vec::new(),
            history: Vec::new(),
            sessions: Vec::new(),
            games: Vec::new(),
            systems: Vec::new(),
            streak: Streak::default(),
            game_streaks: Vec::new(),
            rerolls: 0,
            reroll_day: 0,
            ribbon: 0,
            auto_games: 0,
            finished: 0,
            last_visit: 0,
            face: Mood::Happy,
            open: None,
            suggest: None,
            celebrating: false,
            pumped: false,
            book: Vec::new(),
            goals_ready: false,
            last_regions: None,
        }
    }
}

impl Companion {
    pub fn read(root: &Path) -> Companion {
        let path = root.join(FILE);
        let bak = backup_path(&path);
        let mut companion = if let Some(companion) = load_sound(&path) {
            if !bak.is_file() {
                if let Ok(bytes) = std::fs::read(&path) {
                    let _ = atomic_write(&bak, &bytes);
                }
            }
            companion
        } else if let Some(companion) = load_sound(&bak) {
            let _ = companion.write(root);
            companion
        } else if let Some(companion) = load_legacy(&path) {
            companion
        } else if let Some(companion) = load_legacy(&bak) {
            let _ = companion.write(root);
            companion
        } else {
            Companion::default()
        };
        // A session left open by a crash or a power loss keeps only the seconds already
        // counted. The gap until now is not play.
        if let Some(open) = companion.open.take() {
            companion.push_session(open);
        }
        companion
    }

    pub fn write(&self, root: &Path) -> std::io::Result<()> {
        let path = root.join(FILE);
        let bak = backup_path(&path);
        if let Ok(bytes) = std::fs::read(&path) {
            if load_sound_bytes(&bytes).is_some() {
                atomic_write(&bak, &bytes)?;
            }
        }
        let bytes = self.text().into_bytes();
        atomic_write(&path, &bytes)?;
        // The first good save has no earlier generation. Keep a spare of this one so a
        // later tear still has something to read.
        if !bak.is_file() {
            atomic_write(&bak, &bytes)?;
        }
        Ok(())
    }

    /// Remember the face for this day, and which games are favorites, before a save.
    pub fn stamp(&mut self, day: i64, games: &[GameSeen], favorites: &[(Platform, &str)]) {
        self.face = self.mood(day, games);
        self.last_visit = day;
        self.absorb_library(games);
        self.sync_favorites(favorites);
    }

    /// Copy time the library already recorded into this file. Later play only raises those
    /// totals, so a save cannot count the same minute twice.
    fn absorb_library(&mut self, games: &[GameSeen]) {
        for seen in games {
            if seen.played_secs == 0 && seen.last_played <= 0 {
                continue;
            }
            let day = if seen.last_played > 0 {
                seen.last_played / 86_400
            } else {
                0
            };
            if let Some(game) = self
                .games
                .iter_mut()
                .find(|g| g.platform == seen.platform && g.stem == seen.stem)
            {
                if seen.played_secs > game.secs {
                    game.secs = seen.played_secs;
                }
                if day > game.last_day {
                    game.last_day = day;
                }
                if game.plays == 0 && game.secs > 0 {
                    game.plays = 1;
                }
            } else {
                self.games.push(GameStat {
                    platform: seen.platform,
                    stem: seen.stem.clone(),
                    plays: u32::from(seen.played_secs > 0),
                    secs: seen.played_secs,
                    last_day: day,
                    last_secs: 0,
                    favorite: false,
                });
            }
        }
        let platforms: Vec<Platform> = self.games.iter().map(|g| g.platform).collect();
        for platform in platforms {
            let secs = self
                .games
                .iter()
                .filter(|g| g.platform == platform)
                .map(|g| g.secs)
                .sum();
            let games_with_time = self
                .games
                .iter()
                .filter(|g| g.platform == platform && g.secs > 0)
                .count() as u32;
            let last_day = self
                .games
                .iter()
                .filter(|g| g.platform == platform)
                .map(|g| g.last_day)
                .max()
                .unwrap_or(0);
            if let Some(system) = self.systems.iter_mut().find(|s| s.platform == platform) {
                if secs > system.secs {
                    system.secs = secs;
                }
                system.games = system.games.max(games_with_time);
                if last_day > system.last_day {
                    system.last_day = last_day;
                }
                if system.plays == 0 && system.secs > 0 {
                    system.plays = games_with_time.max(1);
                }
            } else if secs > 0 {
                self.systems.push(SystemStat {
                    platform,
                    plays: games_with_time.max(1),
                    secs,
                    last_day,
                    games: games_with_time,
                });
            }
        }
    }

    pub fn sync_favorites(&mut self, favorites: &[(Platform, &str)]) {
        for game in &mut self.games {
            game.favorite = favorites
                .iter()
                .any(|(platform, stem)| *platform == game.platform && *stem == game.stem);
        }
        for (platform, stem) in favorites {
            if self
                .games
                .iter()
                .any(|g| g.platform == *platform && g.stem == *stem)
            {
                continue;
            }
            self.games.push(GameStat {
                platform: *platform,
                stem: (*stem).to_string(),
                plays: 0,
                secs: 0,
                last_day: 0,
                last_secs: 0,
                favorite: true,
            });
        }
    }

    /// Each level asks for more than the one before: 100, then 150, then 200.
    pub fn level(&self) -> u32 {
        let mut left = self.xp;
        let mut level = 1u32;
        loop {
            let cost = self.cost_of(level);
            if left < cost || level >= 99 {
                break;
            }
            left -= cost;
            level += 1;
        }
        level
    }

    pub fn toward_next(&self) -> u32 {
        let mut left = self.xp;
        let mut level = 1u32;
        loop {
            let cost = self.cost_of(level);
            if left < cost || level >= 99 {
                return left;
            }
            left -= cost;
            level += 1;
        }
    }

    pub fn next_cost(&self) -> u32 {
        self.cost_of(self.level())
    }

    fn cost_of(&self, level: u32) -> u32 {
        let step = LEVEL_XP / 2;
        level.max(1).saturating_mul(step).saturating_add(step)
    }

    /// A short hello. The words are Slot's own, and they change with the day, the hour,
    /// the streak, and the game in front of Pip.
    pub fn say(&self, day: i64, hour: u32, games: &[GameSeen]) -> String {
        let mood = self.mood_at(day, hour, games);
        let (stem, system) = self.topic();
        let seed = (day as u64).wrapping_mul(17)
            ^ u64::from(hour / 4)
            ^ u64::from(self.streak.current).wrapping_mul(9)
            ^ u64::from(self.level())
            ^ fold(&stem);
        let line = pick(seed, lines(mood, hour));
        if stem.is_empty() || seed % 3 != 0 {
            return line.to_string();
        }
        match seed % 5 {
            0 => format!("{stem} next?"),
            1 => format!("{system} today?"),
            2 if self.streak.current >= 2 => format!("{} days running", self.streak.current),
            _ => format!("{line} {stem}"),
        }
    }

    pub fn greeting(&self, day: i64) -> String {
        self.say(day, 12, &[])
    }

    fn topic(&self) -> (String, String) {
        if let Some((platform, stem)) = &self.suggest {
            return (short(stem), short_system(platform.name()));
        }
        if let Some(open) = &self.open {
            return (short(&open.stem), short_system(open.platform.name()));
        }
        if let Some(s) = self.sessions.last() {
            return (short(&s.stem), short_system(s.platform.name()));
        }
        (String::new(), String::new())
    }

    /// What Pip is feeling. `hour` is 0–23 in the player's local time.
    pub fn mood_at(&self, day: i64, hour: u32, games: &[GameSeen]) -> Mood {
        if self.celebrating {
            return Mood::Celebrating;
        }
        if self.pumped || self.history.last().is_some_and(|d| d.day == day) {
            return Mood::Excited;
        }
        if self.streak.current >= 3 && self.streak.last_day == day {
            return Mood::Excited;
        }
        let last = self.last_play_day();
        if last > 0 && day >= last + 3 {
            return Mood::Sad;
        }
        if self.short_day(day) {
            return Mood::Frustrated;
        }
        if hour >= 23 || hour < 6 {
            return Mood::Sleepy;
        }
        if last > 0 && last + 1 == day {
            return Mood::Sleepy;
        }
        if games.iter().any(|g| g.played_secs == 0)
            && self.sessions.is_empty()
            && self.open.is_none()
        {
            return Mood::Curious;
        }
        if self.sessions.is_empty() && self.open.is_none() {
            return Mood::Neutral;
        }
        Mood::Happy
    }

    pub fn mood(&self, day: i64, games: &[GameSeen]) -> Mood {
        self.mood_at(day, 12, games)
    }

    /// An achievement unlocked. Cleared when the next game is launched.
    pub fn note_cheer(&mut self) {
        self.pumped = true;
    }

    fn last_play_day(&self) -> i64 {
        self.sessions
            .iter()
            .map(|s| s.day)
            .chain(self.open.as_ref().map(|o| o.day))
            .chain((self.streak.last_day > 0).then_some(self.streak.last_day))
            .max()
            .unwrap_or(0)
    }

    fn short_day(&self, day: i64) -> bool {
        let today: Vec<_> = self.sessions.iter().filter(|s| s.day == day).collect();
        if today.is_empty() || self.open.is_some() {
            return false;
        }
        today.iter().all(|s| s.secs > 0 && s.secs < 180)
    }

    pub fn suggestion(&self) -> Option<(Platform, String)> {
        self.suggest.clone()
    }

    /// Play that Slot actually counted. Gaps (sleep, a crash) are not included in `secs`.
    pub fn note_play(
        &mut self,
        platform: Platform,
        stem: &str,
        secs: u64,
        day: i64,
        games: &[GameSeen],
    ) -> Vec<Award> {
        if !self.enabled || stem.is_empty() || secs == 0 {
            return Vec::new();
        }
        let mut awards = Vec::new();
        let first = games
            .iter()
            .find(|g| g.platform == platform && g.stem == stem)
            .is_none_or(|g| g.played_secs == 0)
            && !self
                .sessions
                .iter()
                .any(|s| s.platform == platform && s.stem == stem)
            && self
                .open
                .as_ref()
                .is_none_or(|o| o.platform != platform || o.stem != stem);
        self.add_open(platform, stem, secs, day);
        if first {
            let key = format!("first:{}\t{stem}", platform.dir_name());
            if self.grant(
                &key,
                XP_FIRST,
                &mut awards,
                "First launch",
                &format!("{stem}  +{XP_FIRST} XP"),
            ) {
                // grant already pushed
            }
        }
        self.ensure_missions(games, day);
        self.advance(platform, stem, secs, day, &mut awards);
        self.ensure_missions(games, day);
        self.pick_suggestion(games, day);
        self.note_streak(day);
        self.face = self.mood(day, games);
        self.last_visit = day;
        awards
    }

    /// Mark the selected mission done by hand. XP is awarded once.
    pub fn complete_by_hand(&mut self, index: usize, day: i64) -> Option<Award> {
        let mission = self.missions.get_mut(index)?;
        if mission.done {
            return None;
        }
        mission.done = true;
        mission.progress = mission.need;
        let mission = self.missions[index].clone();
        let mut awards = Vec::new();
        self.finish(&mission, day, &mut awards);
        awards.into_iter().next()
    }

    pub fn refresh_suggestion(&mut self, games: &[GameSeen], day: i64) {
        let previous = self.suggest.clone();
        self.suggest = None;
        self.pick_suggestion(games, day.wrapping_add(1));
        if self.suggest.is_some() && self.suggest == previous {
            self.suggest = None;
            self.pick_suggestion(games, day.wrapping_mul(3).wrapping_add(9));
        }
    }

    pub fn set_enabled(&mut self, on: bool) {
        self.enabled = on;
    }

    pub fn toggle_category(&mut self, kind: MissionKind) {
        if let Some(at) = self.categories.iter().position(|k| *k == kind) {
            self.categories.remove(at);
            self.missions.retain(|m| m.kind != kind || m.done);
        } else {
            self.categories.push(kind);
        }
    }

    pub fn toggle_excluded(&mut self, platform: Platform) {
        if let Some(at) = self.excluded.iter().position(|p| *p == platform) {
            self.excluded.remove(at);
        } else {
            self.excluded.push(platform);
        }
    }

    /// Clears Pip's XP, missions and sessions. Games and saves are not in this file.
    pub fn reset(&mut self) {
        let settings = (
            self.enabled,
            self.toasts,
            self.sounds,
            self.animate,
            self.categories.clone(),
        );
        *self = Companion::default();
        self.enabled = settings.0;
        self.toasts = settings.1;
        self.sounds = settings.2;
        self.animate = settings.3;
        self.categories = settings.4;
    }

    /// The game left the screen: keep the seconds already counted, and nothing after.
    pub fn settle(&mut self) {
        if let Some(open) = self.open.take() {
            self.push_session(open);
        }
    }

    pub fn active(&self) -> impl Iterator<Item = &Mission> {
        self.missions.iter().filter(|m| !m.done)
    }

    fn grant(
        &mut self,
        key: &str,
        xp: u32,
        awards: &mut Vec<Award>,
        title: &str,
        detail: &str,
    ) -> bool {
        if self.awarded.iter().any(|k| k == key) {
            return false;
        }
        self.awarded.push(key.to_string());
        let before = self.level();
        self.xp = self.xp.saturating_add(xp);
        self.celebrating = self.level() > before;
        awards.push(Award {
            title: title.into(),
            detail: detail.into(),
            xp,
        });
        if self.celebrating {
            awards.push(Award {
                title: "Level up".into(),
                detail: format!("Pip is level {}", self.level()),
                xp: 0,
            });
        }
        true
    }

    fn add_open(&mut self, platform: Platform, stem: &str, secs: u64, day: i64) {
        let same = self
            .open
            .as_ref()
            .is_some_and(|o| o.platform == platform && o.stem == stem);
        self.record_play(platform, stem, secs, day, !same);
        if !same {
            self.pumped = false;
            if let Some(open) = self.open.take() {
                self.push_session(open);
            }
            self.open = Some(OpenSession {
                platform,
                stem: stem.into(),
                secs: 0,
                day,
            });
        }
        if let Some(open) = &mut self.open {
            open.secs = open.secs.saturating_add(secs);
            open.day = day;
        }
    }

    fn push_session(&mut self, open: OpenSession) {
        if open.secs == 0 {
            return;
        }
        self.sessions.push(Session {
            platform: open.platform,
            stem: open.stem,
            secs: open.secs,
            day: open.day,
        });
        if self.sessions.len() > SESSION_KEEP {
            let extra = self.sessions.len() - SESSION_KEEP;
            self.sessions.drain(0..extra);
        }
    }

    fn advance(
        &mut self,
        platform: Platform,
        stem: &str,
        secs: u64,
        day: i64,
        awards: &mut Vec<Award>,
    ) {
        let weekend = is_weekend(day);
        let mut finished = Vec::new();
        for mission in &mut self.missions {
            if mission.done {
                continue;
            }
            match mission.kind {
                MissionKind::TryNew
                | MissionKind::HiddenGem
                | MissionKind::Forgotten
                | MissionKind::OneMore
                | MissionKind::Marathon => {
                    let hit = if mission.stem.is_empty() {
                        mission.platform == Some(platform)
                    } else {
                        mission.platform == Some(platform) && mission.stem == stem
                    };
                    if hit {
                        if mission.need >= 60 {
                            mission.progress = mission
                                .progress
                                .saturating_add(secs.min(u32::MAX as u64) as u32);
                        } else {
                            mission.progress = 1;
                        }
                    }
                }
                MissionKind::Tourist => {
                    if mission.need >= 60 {
                        if mission.platform == Some(platform) {
                            mission.progress = mission
                                .progress
                                .saturating_add(secs.min(u32::MAX as u64) as u32);
                        }
                        continue;
                    }
                    let tag = platform.dir_name();
                    if !mission.extra.split(',').any(|s| s == tag) {
                        if !mission.extra.is_empty() {
                            mission.extra.push(',');
                        }
                        mission.extra.push_str(tag);
                        mission.progress = mission.progress.saturating_add(1);
                    }
                }
                MissionKind::Goal => {}
                MissionKind::Weekend => {
                    if weekend && !mission.extra.split(',').any(|s| s == stem) {
                        if !mission.extra.is_empty() {
                            mission.extra.push(',');
                        }
                        mission.extra.push_str(stem);
                        mission.progress = mission.progress.saturating_add(1);
                    }
                }
            }
            if mission.progress >= mission.need {
                mission.done = true;
                finished.push(mission.clone());
            }
        }
        for mission in finished {
            self.finish(&mission, day, awards);
        }
    }

    fn finish(&mut self, mission: &Mission, day: i64, awards: &mut Vec<Award>) {
        let key = format!("mission:{}", mission.id);
        self.grant(
            &key,
            mission.xp,
            awards,
            "Mission",
            &format!("{}  +{} XP", mission.title, mission.xp),
        );
        self.finished = self.finished.saturating_add(1);
        self.history.push(DoneMission {
            title: mission.title.clone(),
            day,
            finished: day,
            xp: mission.xp,
            platform: mission.platform,
            stem: mission.stem.clone(),
        });
        if self.history.len() > 20 {
            let extra = self.history.len() - 20;
            self.history.drain(0..extra);
        }
    }

    pub fn ensure_missions(&mut self, games: &[GameSeen], day: i64) {
        if !self.enabled {
            return;
        }
        let mut guard = 0;
        while self.missions.iter().filter(|m| !m.done).count() < ACTIVE && guard < 8 {
            guard += 1;
            if !self.spawn(games, day) {
                break;
            }
        }
        self.plant_goals(games);
    }

    /// Replace today's unfinished missions. Three refreshes a day, remembered in the save.
    pub fn refresh_missions(&mut self, games: &[GameSeen], day: i64) -> bool {
        if self.reroll_day != day {
            self.reroll_day = day;
            self.rerolls = 0;
        }
        if self.rerolls >= 3 {
            return false;
        }
        self.missions.retain(|m| m.done);
        self.rerolls = self.rerolls.saturating_add(1);
        self.ensure_missions(games, day);
        true
    }

    pub fn rerolls_left(&self, day: i64) -> u32 {
        let used = if self.reroll_day == day {
            self.rerolls
        } else {
            0
        };
        3u32.saturating_sub(used)
    }

    pub fn goals_ready(&self) -> bool {
        self.goals_ready
    }

    /// Hand pack plus anything mined from cached sets. Called once per launch.
    pub fn set_goals(&mut self, book: Vec<crate::goals::GoalOffer>, auto_games: u32) {
        self.book = book;
        self.auto_games = auto_games;
        self.goals_ready = true;
    }

    fn plant_goals(&mut self, games: &[GameSeen]) {
        if self.book.is_empty() {
            return;
        }
        let salt = self.rerolls as usize;
        let mut order: Vec<usize> = (0..games.len()).collect();
        if !order.is_empty() {
            let start = salt % order.len();
            order.rotate_left(start);
        }
        let mut planted = self
            .missions
            .iter()
            .filter(|m| !m.done && m.kind == MissionKind::Goal)
            .count();
        for at in order {
            if planted >= ACTIVE {
                break;
            }
            let game = &games[at];
            if self
                .missions
                .iter()
                .any(|m| !m.done && m.kind == MissionKind::Goal && m.stem == game.stem)
            {
                continue;
            }
            let Some(offer) = crate::goals::offer_for(&self.book, &game.stem, salt).cloned() else {
                continue;
            };
            let mission = self.goal_mission(&offer, game);
            self.remember_streak(game, &offer);
            if let Some(slot) = self
                .missions
                .iter_mut()
                .find(|m| !m.done && m.kind != MissionKind::Goal)
            {
                *slot = mission;
            } else if self.missions.iter().filter(|m| !m.done).count() < ACTIVE {
                self.missions.push(mission);
            } else {
                break;
            }
            planted += 1;
        }
    }

    fn goal_mission(&mut self, offer: &crate::goals::GoalOffer, game: &GameSeen) -> Mission {
        let id = self.next_id;
        self.next_id = self.next_id.saturating_add(1);
        Mission {
            id: format!("{id}"),
            kind: MissionKind::Goal,
            title: offer.title.clone(),
            detail: offer.detail.clone(),
            platform: Some(game.platform),
            stem: game.stem.clone(),
            need: offer.need.max(1),
            progress: 0,
            xp: MissionKind::Goal.xp(),
            done: false,
            extra: offer.cond.clone(),
        }
    }

    fn remember_streak(&mut self, game: &GameSeen, offer: &crate::goals::GoalOffer) {
        if self
            .game_streaks
            .iter()
            .any(|s| s.platform == game.platform && s.stem == game.stem)
        {
            return;
        }
        self.game_streaks.push(GameStreak {
            platform: game.platform,
            stem: game.stem.clone(),
            theme: offer.streak.clone(),
            cond: offer.streak_cond.clone(),
            last_day: 0,
            current: 0,
            best: 0,
        });
    }

    /// Check today's in-game missions against a RAM snapshot that starts at address 0.
    pub fn note_memory(
        &mut self,
        platform: Platform,
        stem: &str,
        ram: &[u8],
        day: i64,
    ) -> Vec<Award> {
        self.note_regions(platform, stem, &[(0, ram)], day)
    }

    /// Same check when the core's RAM is several blocks, each at its own address.
    /// Nothing is fetched: the condition is the one already saved with the mission.
    pub fn note_regions(
        &mut self,
        platform: Platform,
        stem: &str,
        regions: &[(u32, &[u8])],
        day: i64,
    ) -> Vec<Award> {
        let now_owned: Vec<(u32, Vec<u8>)> = regions
            .iter()
            .map(|(start, bytes)| (*start, bytes.to_vec()))
            .collect();
        let prev_owned = self.last_regions.take().filter(|(platform_at, stem_at, _)| {
            *platform_at == platform && stem_at == stem
        });
        let prev_owned = prev_owned
            .map(|(_, _, bytes)| bytes)
            .unwrap_or_else(|| now_owned.clone());
        let prev_view: Vec<(u32, &[u8])> = prev_owned
            .iter()
            .map(|(start, bytes)| (*start, bytes.as_slice()))
            .collect();
        let now_view: Vec<(u32, &[u8])> = now_owned
            .iter()
            .map(|(start, bytes)| (*start, bytes.as_slice()))
            .collect();
        let mut awards = Vec::new();
        let mut done_here = false;
        let indexes: Vec<usize> = self
            .missions
            .iter()
            .enumerate()
            .filter(|(_, m)| {
                !m.done
                    && m.kind == MissionKind::Goal
                    && m.platform == Some(platform)
                    && m.stem == stem
            })
            .map(|(i, _)| i)
            .collect();
        for index in indexes {
            let cond = self.missions[index].extra.clone();
            let need = self.missions[index].need;
            let before = self.missions[index].progress;
            let Some(measured) = measured_for(&cond, &prev_view, &now_view) else {
                continue;
            };
            let preview = if counter_preview(&cond) {
                byte_in(&cond, &now_view)
            } else {
                None
            };
            let mut shown = measured.value;
            if let Some((value, _)) = preview {
                shown = shown.max(value);
            }
            if measured.triggered {
                shown = shown.max(need);
            }
            shown = shown.min(need);
            if !counter_preview(&cond) && measured.triggered {
                shown = before.saturating_add(1).min(need);
            }
            if shown > before {
                self.missions[index].progress = shown;
                let title = self.missions[index].title.clone();
                if band(before, need) < band(shown, need) && shown < need {
                    awards.push(Award {
                        title: "Mission".into(),
                        detail: format!("{shown}/{need}  {title}"),
                        xp: 0,
                    });
                }
            }
            if measured.triggered && counter_preview(&cond) || shown >= need {
                self.missions[index].done = true;
                self.missions[index].progress = need;
                let mission = self.missions[index].clone();
                self.finish(&mission, day, &mut awards);
                self.face = Mood::Celebrating;
                self.pumped = true;
                self.celebrating = true;
                done_here = true;
            }
        }
        let streak_hit =
            done_here || self.streak_condition_met(platform, stem, &prev_view, &now_view);
        if streak_hit {
            if let Some(award) = self.bump_game_streak(platform, stem, day) {
                awards.push(award);
            }
        }
        self.last_regions = Some((platform, stem.to_string(), now_owned));
        awards
    }

    fn streak_condition_met(
        &self,
        platform: Platform,
        stem: &str,
        prev: &[(u32, &[u8])],
        now: &[(u32, &[u8])],
    ) -> bool {
        let Some(streak) = self
            .game_streaks
            .iter()
            .find(|s| s.platform == platform && s.stem == stem)
        else {
            return false;
        };
        if streak.cond.is_empty() {
            return false;
        }
        measured_for(&streak.cond, prev, now).is_some_and(|m| m.triggered)
    }

    fn bump_game_streak(&mut self, platform: Platform, stem: &str, day: i64) -> Option<Award> {
        let streak = self
            .game_streaks
            .iter_mut()
            .find(|s| s.platform == platform && s.stem == stem)?;
        if streak.last_day == day {
            return None;
        }
        if streak.last_day + 1 == day || streak.last_day == 0 {
            streak.current = streak.current.saturating_add(1);
        } else {
            streak.current = 1;
        }
        streak.last_day = day;
        if streak.current > streak.best {
            streak.best = streak.current;
        }
        let n = streak.current;
        let theme = streak.theme.clone();
        let stem = streak.stem.clone();
        if matches!(n, 3 | 7 | 14 | 30) {
            self.ribbon = self.ribbon.max(n);
            self.face = Mood::Celebrating;
            self.celebrating = true;
            let mut awards = Vec::new();
            self.grant(
                &format!("streak:{stem}:{n}"),
                15,
                &mut awards,
                "Streak",
                &format!("{theme}  {n} days"),
            );
            return awards.into_iter().next();
        }
        Some(Award {
            title: "Streak".into(),
            detail: format!("{theme}  {n} days"),
            xp: 0,
        })
    }

    fn touched(&self, platform: Platform, stem: &str) -> bool {
        self.sessions
            .iter()
            .any(|s| s.platform == platform && s.stem == stem)
            || self
                .open
                .as_ref()
                .is_some_and(|s| s.platform == platform && s.stem == stem)
            || self
                .missions
                .iter()
                .any(|m| m.done && m.platform == Some(platform) && m.stem == stem)
    }

    fn spawn(&mut self, games: &[GameSeen], _day: i64) -> bool {
        let used: Vec<(MissionKind, String)> = self
            .missions
            .iter()
            .filter(|m| !m.done)
            .map(|m| (m.kind, m.stem.clone()))
            .collect();
        let _ = used;
        let unplayed: Vec<&GameSeen> = games
            .iter()
            .filter(|g| {
                g.played_secs == 0
                    && !self.excluded.contains(&g.platform)
                    && !self.touched(g.platform, &g.stem)
            })
            .collect();
        let played: Vec<&GameSeen> = games
            .iter()
            .filter(|g| g.played_secs > 0 && !self.excluded.contains(&g.platform))
            .collect();
        let id = self.next_id;
        self.next_id = self.next_id.saturating_add(1);
        let mission =
            |kind: MissionKind, title: &str, detail: &str, game: Option<&GameSeen>, need: u32| {
                Mission {
                    id: format!("{id}"),
                    kind,
                    title: title.into(),
                    detail: detail.into(),
                    platform: game.map(|g| g.platform),
                    stem: game.map(|g| g.stem.clone()).unwrap_or_default(),
                    need,
                    progress: 0,
                    xp: kind.xp(),
                    done: false,
                    extra: String::new(),
                }
            };
        // Three at a time, when the library allows: one from games already played,
        // one about playing different systems, and one about a game not yet played.
        let filled = |slot: u8| {
            self.missions
                .iter()
                .any(|m| !m.done && Self::mission_slot(m.kind) == Some(slot))
        };
        let missing: Vec<u8> = [0, 1, 2]
            .into_iter()
            .filter(|slot| !filled(*slot))
            .collect();
        let pick = unplayed.get((id as usize).wrapping_mul(3) % unplayed.len().max(1));
        for slot in missing {
            if self.fill_slot(slot, &unplayed, &played, pick.copied(), &mission) {
                return true;
            }
        }
        self.fill_slot(3, &unplayed, &played, pick.copied(), &mission)
    }

    /// Turn the mission just added into a timed one. Reward is minutes times `per_minute`.
    fn time_the_last(&mut self, per_minute: u32) {
        let seed = self.next_id.max(1);
        let mins = match per_minute {
            8 => 10 + seed % 15,
            5 => {
                let rare = self
                    .missions
                    .last()
                    .is_some_and(|m| m.kind == MissionKind::Forgotten);
                if rare {
                    5 + seed % 10
                } else {
                    10 + seed % 15
                }
            }
            _ => 5 + seed % 15,
        };
        if let Some(mission) = self.missions.last_mut() {
            mission.need = mins * 60;
            mission.xp = mins * per_minute;
            mission.detail = format!("About {mins} minutes");
        }
    }

    fn wants(&self, kind: MissionKind) -> bool {
        self.categories.contains(&kind) && !self.missions.iter().any(|m| !m.done && m.kind == kind)
    }

    fn fill_slot(
        &mut self,
        slot: u8,
        unplayed: &[&GameSeen],
        played: &[&GameSeen],
        pick: Option<&GameSeen>,
        mission: &impl Fn(MissionKind, &str, &str, Option<&GameSeen>, u32) -> Mission,
    ) -> bool {
        if slot == 0 {
            if self.wants(MissionKind::Forgotten) {
                let old = played
                    .iter()
                    .copied()
                    .filter(|g| g.last_played > 0)
                    .min_by_key(|g| g.last_played);
                if let Some(game) = old {
                    self.missions.push(mission(
                        MissionKind::Forgotten,
                        MissionKind::Forgotten.plain(),
                        &format!("Revisit {}", game.stem),
                        Some(game),
                        1,
                    ));
                    self.time_the_last(5);
                    return true;
                }
            }
            if self.wants(MissionKind::OneMore) {
                if let Some(game) = played.first() {
                    self.missions.push(mission(
                        MissionKind::OneMore,
                        MissionKind::OneMore.plain(),
                        &format!("Return to {}", game.stem),
                        Some(game),
                        1,
                    ));
                    self.time_the_last(5);
                    return true;
                }
            }
            if self.wants(MissionKind::Marathon) {
                let game = played.first().copied().or(unplayed.first().copied());
                if let Some(game) = game {
                    self.missions.push(mission(
                        MissionKind::Marathon,
                        MissionKind::Marathon.plain(),
                        &format!("Play {} for 20 minutes", game.stem),
                        Some(game),
                        20 * 60,
                    ));
                    self.time_the_last(5);
                    return true;
                }
            }
            return false;
        }
        if slot == 1 && self.wants(MissionKind::Tourist) {
            let pool: Vec<&GameSeen> = played.iter().chain(unplayed.iter()).copied().collect();
            if let Some(game) = pool
                .get((self.next_id as usize) % pool.len().max(1))
                .copied()
            {
                if !pool.is_empty() {
                    self.missions.push(mission(
                        MissionKind::Tourist,
                        MissionKind::Tourist.plain(),
                        &format!("Any game on {}", game.platform.name()),
                        Some(game),
                        3,
                    ));
                    self.missions.last_mut().map(|m| m.stem.clear());
                    self.time_the_last(6);
                    return true;
                }
            }
        }
        if slot == 2 {
            if self.wants(MissionKind::TryNew) {
                if let Some(game) = pick.filter(|_| !unplayed.is_empty()) {
                    self.missions.push(mission(
                        MissionKind::TryNew,
                        MissionKind::TryNew.plain(),
                        &format!("Launch {}", game.stem),
                        Some(game),
                        1,
                    ));
                    self.time_the_last(7);
                    return true;
                }
            }
            if self.wants(MissionKind::HiddenGem) && unplayed.len() > 1 {
                let game = unplayed[(self.next_id as usize).wrapping_add(1) % unplayed.len()];
                if !self.missions.iter().any(|m| !m.done && m.stem == game.stem) {
                    self.missions.push(mission(
                        MissionKind::HiddenGem,
                        MissionKind::HiddenGem.plain(),
                        &format!("Try {}", game.stem),
                        Some(game),
                        1,
                    ));
                    self.time_the_last(8);
                    return true;
                }
            }
            return false;
        }
        if slot == 3 && self.wants(MissionKind::Weekend) {
            self.missions.push(mission(
                MissionKind::Weekend,
                MissionKind::Weekend.plain(),
                "Play four different games over a weekend",
                None,
                4,
            ));
            return true;
        }
        false
    }

    fn mission_slot(kind: MissionKind) -> Option<u8> {
        match kind {
            MissionKind::Forgotten | MissionKind::OneMore | MissionKind::Marathon => Some(0),
            MissionKind::Tourist => Some(1),
            MissionKind::TryNew | MissionKind::HiddenGem => Some(2),
            MissionKind::Weekend | MissionKind::Goal => None,
        }
    }

    fn pick_suggestion(&mut self, games: &[GameSeen], salt: i64) {
        if self.suggest.is_some() {
            return;
        }
        let mut best: Option<&GameSeen> = None;
        let mut best_w = 0u32;
        for (i, game) in games.iter().enumerate() {
            if self.excluded.contains(&game.platform) {
                continue;
            }
            let weight = if game.played_secs == 0 {
                8u32
            } else if game.played_secs < 300 {
                4
            } else {
                1
            };
            let weight = weight
                .saturating_mul(1 + ((i as i64).wrapping_add(salt).unsigned_abs() as u32 % 5));
            if weight > best_w {
                best_w = weight;
                best = Some(game);
            }
        }
        if let Some(game) = best {
            self.suggest = Some((game.platform, game.stem.clone()));
        }
    }

    pub fn parse(text: &str) -> Companion {
        let mut companion = Companion::default();
        companion.categories.clear();
        let mut saw_cats = false;
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') || line.starts_with("slot-companion") {
                continue;
            }
            let Some((key, value)) = line.split_once('=') else {
                continue;
            };
            match key {
                "enabled" => companion.enabled = value != "0",
                "toasts" => companion.toasts = value != "0",
                "sounds" => companion.sounds = value != "0",
                "animate" => companion.animate = value != "0",
                "xp" => companion.xp = value.parse().unwrap_or(0),
                "next_id" => companion.next_id = value.parse().unwrap_or(1),
                "cats" => {
                    saw_cats = true;
                    companion.categories =
                        value.split(',').filter_map(MissionKind::parse).collect();
                }
                "awarded" => {
                    if !value.is_empty() {
                        companion.awarded.push(value.to_string());
                    }
                }
                "exclude" => {
                    if let Some(platform) = platform_named(value) {
                        companion.excluded.push(platform);
                    }
                }
                "suggest" => {
                    if let Some((platform, stem)) = value.split_once('\t') {
                        if let Some(platform) = platform_named(platform) {
                            companion.suggest = Some((platform, stem.to_string()));
                        }
                    }
                }
                "session" => {
                    if let Some(session) = parse_session(value) {
                        companion.sessions.push(session);
                    }
                }
                "open" => {
                    companion.open = parse_session(value).map(|s| OpenSession {
                        platform: s.platform,
                        stem: s.stem,
                        secs: s.secs,
                        day: s.day,
                    })
                }
                "mission" => {
                    if let Some(mission) = parse_mission(value) {
                        companion.missions.push(mission);
                    }
                }
                "done" => {
                    if let Some(done) = parse_done(value) {
                        companion.history.push(done);
                    }
                }
                "game" => {
                    if let Some(game) = parse_game(value) {
                        companion.games.push(game);
                    }
                }
                "system" => {
                    if let Some(system) = parse_system(value) {
                        companion.systems.push(system);
                    }
                }
                "streak" => companion.streak = parse_streak(value),
                "reroll" => {
                    let mut f = value.split('\t');
                    companion.reroll_day = f.next().unwrap_or("0").parse().unwrap_or(0);
                    companion.rerolls = f.next().unwrap_or("0").parse().unwrap_or(0);
                }
                "ribbon" => companion.ribbon = value.parse().unwrap_or(0),
                "gstreak" => {
                    if let Some(streak) = parse_game_streak(value) {
                        companion.game_streaks.push(streak);
                    }
                }
                "finished" => companion.finished = value.parse().unwrap_or(0),
                "last_visit" => companion.last_visit = value.parse().unwrap_or(0),
                "mood" => {
                    if let Some(mood) = parse_mood(value) {
                        companion.face = mood;
                    }
                }
                _ => {}
            }
        }
        if !saw_cats {
            companion.categories = MissionKind::ALL.to_vec();
        }
        companion
    }

    pub fn text(&self) -> String {
        let mut out = String::from("slot-companion 2\n");
        let flag = |on: bool| if on { "1" } else { "0" };
        out.push_str(&format!(
            "enabled={}\ntoasts={}\nsounds={}\nanimate={}\nxp={}\nnext_id={}\n",
            flag(self.enabled),
            flag(self.toasts),
            flag(self.sounds),
            flag(self.animate),
            self.xp,
            self.next_id
        ));
        let cats: Vec<&str> = self.categories.iter().map(|k| k.id()).collect();
        out.push_str(&format!("cats={}\n", cats.join(",")));
        for key in &self.awarded {
            out.push_str("awarded=");
            out.push_str(key);
            out.push('\n');
        }
        for platform in &self.excluded {
            out.push_str(&format!("exclude={}\n", platform.dir_name()));
        }
        if let Some((platform, stem)) = &self.suggest {
            out.push_str(&format!("suggest={}\t{stem}\n", platform.dir_name()));
        }
        for session in &self.sessions {
            out.push_str(&format!(
                "session={}\t{}\t{}\t{}\n",
                session.platform.dir_name(),
                session.stem,
                session.secs,
                session.day
            ));
        }
        if let Some(open) = &self.open {
            out.push_str(&format!(
                "open={}\t{}\t{}\t{}\n",
                open.platform.dir_name(),
                open.stem,
                open.secs,
                open.day
            ));
        }
        for mission in &self.missions {
            out.push_str("mission=");
            out.push_str(&mission.id);
            out.push('\t');
            out.push_str(mission.kind.id());
            out.push('\t');
            out.push_str(&flat(&mission.title));
            out.push('\t');
            out.push_str(&flat(&mission.detail));
            out.push('\t');
            out.push_str(mission.platform.map(|p| p.dir_name()).unwrap_or("-"));
            out.push('\t');
            out.push_str(&flat(&mission.stem));
            out.push('\t');
            out.push_str(&format!(
                "{}\t{}\t{}\t{}\t{}\n",
                mission.need,
                mission.progress,
                mission.xp,
                u8::from(mission.done),
                flat(&mission.extra)
            ));
        }
        for done in &self.history {
            out.push_str(&format!(
                "done={}\t{}\t{}\t{}\t{}\t{}\n",
                flat(&done.title),
                done.day,
                done.finished,
                done.xp,
                done.platform.map(|p| p.dir_name()).unwrap_or("-"),
                flat(&done.stem)
            ));
        }
        for game in &self.games {
            out.push_str(&format!(
                "game={}\t{}\t{}\t{}\t{}\t{}\t{}\n",
                game.platform.dir_name(),
                flat(&game.stem),
                game.plays,
                game.secs,
                game.last_day,
                game.last_secs,
                u8::from(game.favorite)
            ));
        }
        for system in &self.systems {
            out.push_str(&format!(
                "system={}\t{}\t{}\t{}\t{}\n",
                system.platform.dir_name(),
                system.plays,
                system.secs,
                system.last_day,
                system.games
            ));
        }
        out.push_str(&format!(
            "streak={}\t{}\t{}\t{}\t{}\n",
            self.streak.last_day,
            self.streak.current,
            self.streak.longest,
            self.streak.total_days,
            self.streak.start_day
        ));
        out.push_str(&format!(
            "reroll={}\t{}\nribbon={}\n",
            self.reroll_day, self.rerolls, self.ribbon
        ));
        for streak in &self.game_streaks {
            out.push_str(&format!(
                "gstreak={}\t{}\t{}\t{}\t{}\t{}\t{}\n",
                streak.platform.dir_name(),
                flat(&streak.stem),
                flat(&streak.theme),
                flat(&streak.cond),
                streak.last_day,
                streak.current,
                streak.best
            ));
        }
        out.push_str(&format!(
            "finished={}\nlast_visit={}\nmood={}\n",
            self.finished,
            self.last_visit,
            self.face.name()
        ));
        let sum = checksum(out.as_bytes());
        out.push_str(&format!("sum={sum:016x}\n"));
        out
    }

    fn record_play(
        &mut self,
        platform: Platform,
        stem: &str,
        secs: u64,
        day: i64,
        new_launch: bool,
    ) {
        if let Some(game) = self
            .games
            .iter_mut()
            .find(|g| g.platform == platform && g.stem == stem)
        {
            if new_launch {
                game.plays = game.plays.saturating_add(1);
                game.last_secs = secs;
            } else {
                game.last_secs = game.last_secs.saturating_add(secs);
            }
            game.secs = game.secs.saturating_add(secs);
            game.last_day = day;
        } else {
            self.games.push(GameStat {
                platform,
                stem: stem.to_string(),
                plays: u32::from(new_launch),
                secs,
                last_day: day,
                last_secs: secs,
                favorite: false,
            });
        }
        let games = self
            .games
            .iter()
            .filter(|g| g.platform == platform && g.secs > 0)
            .count() as u32;
        if let Some(system) = self.systems.iter_mut().find(|s| s.platform == platform) {
            if new_launch {
                system.plays = system.plays.saturating_add(1);
            }
            system.secs = system.secs.saturating_add(secs);
            system.last_day = day;
            system.games = games;
        } else {
            self.systems.push(SystemStat {
                platform,
                plays: u32::from(new_launch),
                secs,
                last_day: day,
                games,
            });
        }
    }

    fn note_streak(&mut self, day: i64) {
        if self.streak.last_day == day {
            return;
        }
        if self.streak.last_day == 0 {
            self.streak.current = 1;
            self.streak.start_day = day;
        } else if day == self.streak.last_day + 1 {
            self.streak.current = self.streak.current.saturating_add(1);
        } else if day > self.streak.last_day {
            self.streak.current = 1;
            self.streak.start_day = day;
        }
        self.streak.longest = self.streak.longest.max(self.streak.current);
        if day != self.streak.last_day {
            self.streak.total_days = self.streak.total_days.saturating_add(1);
        }
        self.streak.last_day = day;
    }
}

fn backup_path(path: &Path) -> std::path::PathBuf {
    path.with_file_name("companion.txt.bak")
}

fn checksum(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn load_sound(path: &Path) -> Option<Companion> {
    let bytes = std::fs::read(path).ok()?;
    load_sound_bytes(&bytes)
}

fn load_sound_bytes(bytes: &[u8]) -> Option<Companion> {
    let text = String::from_utf8(bytes.to_vec()).ok()?;
    let mut body = String::new();
    let mut sum = None;
    for line in text.lines() {
        if let Some(hex) = line.trim().strip_prefix("sum=") {
            sum = u64::from_str_radix(hex.trim(), 16).ok();
        } else {
            body.push_str(line);
            body.push('\n');
        }
    }
    let sum = sum?;
    if checksum(body.as_bytes()) != sum {
        return None;
    }
    let header = body.lines().next()?.trim();
    if header != "slot-companion 2" {
        return None;
    }
    Some(Companion::parse(&body))
}

fn load_legacy(path: &Path) -> Option<Companion> {
    let text = std::fs::read_to_string(path).ok()?;
    let header = text.lines().next()?.trim();
    if header == "slot-companion 1" || header.starts_with("enabled=") {
        Some(Companion::parse(&text))
    } else {
        None
    }
}

fn parse_done(value: &str) -> Option<DoneMission> {
    let mut f = value.split('\t');
    let title = f.next()?.to_string();
    let day = f.next()?.parse().ok()?;
    let finished = f.next().and_then(|v| v.parse().ok()).unwrap_or(day);
    let xp = f.next().and_then(|v| v.parse().ok()).unwrap_or(0);
    let platform = f.next().and_then(platform_named);
    let stem = f.next().unwrap_or("").to_string();
    Some(DoneMission {
        title,
        day,
        finished,
        xp,
        platform,
        stem,
    })
}

fn parse_game(value: &str) -> Option<GameStat> {
    let mut f = value.split('\t');
    let platform = platform_named(f.next()?)?;
    let stem = f.next()?.to_string();
    let plays = f.next()?.parse().ok()?;
    let secs = f.next()?.parse().ok()?;
    let last_day = f.next()?.parse().ok()?;
    let last_secs = f.next()?.parse().ok()?;
    let favorite = f.next().unwrap_or("0") == "1";
    Some(GameStat {
        platform,
        stem,
        plays,
        secs,
        last_day,
        last_secs,
        favorite,
    })
}

fn parse_system(value: &str) -> Option<SystemStat> {
    let mut f = value.split('\t');
    let platform = platform_named(f.next()?)?;
    Some(SystemStat {
        platform,
        plays: f.next()?.parse().ok()?,
        secs: f.next()?.parse().ok()?,
        last_day: f.next()?.parse().ok()?,
        games: f.next()?.parse().ok()?,
    })
}

fn counter_preview(cond: &str) -> bool {
    let lower = cond.to_ascii_lowercase();
    !lower.contains("d0x") && !cond.contains('.') && !cond.contains('_')
}

fn measured_for(
    cond: &str,
    prev: &[(u32, &[u8])],
    now: &[(u32, &[u8])],
) -> Option<slot_cheevos::Measure> {
    if counter_preview(cond) {
        slot_cheevos::measure_map(cond, now)
    } else {
        slot_cheevos::measure_delta(cond, prev, now)
    }
}

fn byte_in(cond: &str, regions: &[(u32, &[u8])]) -> Option<(u32, u32)> {
    let (addr, target) = parse_byte(cond)?;
    for (start, data) in regions {
        let start = *start as usize;
        if addr >= start {
            let off = addr - start;
            if let Some(byte) = data.get(off) {
                return Some((*byte as u32, target));
            }
        }
    }
    None
}

fn parse_byte(cond: &str) -> Option<(usize, u32)> {
    let at = cond.find("0xH")?;
    let hex = &cond[at + 3..];
    let end = hex
        .find(|c: char| !c.is_ascii_hexdigit())
        .unwrap_or(hex.len());
    if end == 0 {
        return None;
    }
    let addr = usize::from_str_radix(&hex[..end], 16).ok()?;
    let tail = cond.rsplit(['=', '<', '>']).next().unwrap_or("");
    let digits: String = tail
        .chars()
        .skip_while(|c| !c.is_ascii_digit())
        .take_while(|c| c.is_ascii_digit())
        .collect();
    let target = digits.parse().unwrap_or(1);
    Some((addr, target))
}

fn band(progress: u32, need: u32) -> u32 {
    if need == 0 {
        return 0;
    }
    progress.saturating_mul(4) / need
}

fn parse_game_streak(value: &str) -> Option<GameStreak> {
    let mut f = value.split('\t');
    let platform = platform_named(f.next()?)?;
    Some(GameStreak {
        platform,
        stem: f.next()?.to_string(),
        theme: f.next()?.to_string(),
        cond: f.next()?.to_string(),
        last_day: f.next()?.parse().ok()?,
        current: f.next()?.parse().ok()?,
        best: f.next()?.parse().ok()?,
    })
}

fn parse_streak(value: &str) -> Streak {
    let mut f = value.split('\t');
    Streak {
        last_day: f.next().and_then(|v| v.parse().ok()).unwrap_or(0),
        current: f.next().and_then(|v| v.parse().ok()).unwrap_or(0),
        longest: f.next().and_then(|v| v.parse().ok()).unwrap_or(0),
        total_days: f.next().and_then(|v| v.parse().ok()).unwrap_or(0),
        start_day: f.next().and_then(|v| v.parse().ok()).unwrap_or(0),
    }
}

fn parse_mood(value: &str) -> Option<Mood> {
    Some(match value {
        "Happy" => Mood::Happy,
        "Excited" => Mood::Excited,
        "Sleepy" => Mood::Sleepy,
        "Curious" => Mood::Curious,
        "Celebrating" => Mood::Celebrating,
        "Neutral" => Mood::Neutral,
        "Sad" => Mood::Sad,
        "Frustrated" => Mood::Frustrated,
        _ => return None,
    })
}

fn short(name: &str) -> String {
    let name = name.trim();
    let mut out: String = name.chars().take(12).collect();
    if name.chars().count() > 12 {
        out.push('…');
    }
    out
}

fn short_system(name: &str) -> String {
    match name {
        "Game Boy Advance" => "GBA".into(),
        "Game Boy Color" => "GBC".into(),
        "Game Boy" => "GB".into(),
        "Nintendo Entertainment System" => "NES".into(),
        "Super Nintendo Entertainment System" => "SNES".into(),
        "PlayStation" => "PS1".into(),
        other => short(other),
    }
}

fn fold(text: &str) -> u64 {
    let mut n = 0u64;
    for b in text.bytes() {
        n = n.wrapping_mul(33).wrapping_add(u64::from(b));
    }
    n
}

fn pick<'a>(seed: u64, lines: &'a [&'a str]) -> &'a str {
    lines[seed as usize % lines.len()]
}

fn lines(mood: Mood, hour: u32) -> &'static [&'static str] {
    let morning = (6..12).contains(&hour);
    let evening = (17..23).contains(&hour);
    let night = hour >= 23 || hour < 6;
    match mood {
        Mood::Happy if morning => &[
            "Morning. A cart is calling",
            "Up already? Good",
            "First game of the day",
            "I kept the slot warm",
            "Breakfast can wait",
            "New day, same Pip",
        ],
        Mood::Happy if evening => &[
            "Evening session?",
            "The good hour for one more",
            "I was hoping you'd stop by",
            "Lights low, game on",
            "A calm one before bed",
            "Still here. Still ready",
        ],
        Mood::Happy => &[
            "Good to see you",
            "Ready when you are",
            "Pick a game and play",
            "I saved you a spot",
            "That label looks lucky",
            "Want to press start?",
            "I like this shelf",
            "Your call",
        ],
        Mood::Excited => &[
            "Yes. That one counts",
            "Look at us go",
            "Again? I'm in",
            "That was a good one",
            "Keep that energy",
            "I felt that unlock",
            "The streak is alive",
            "One more would be rude to skip",
        ],
        Mood::Sad => &[
            "It's been quiet",
            "I kept your place",
            "The shelf missed you",
            "A short visit still counts",
            "I didn't go anywhere",
            "Dust on the carts. Not on me",
            "Welcome back, truly",
            "We can start small",
        ],
        Mood::Frustrated => &[
            "That was a short one",
            "Stay a little longer?",
            "Blink and it was over",
            "The cart barely warmed up",
            "One more minute?",
            "I had just sat down",
            "A snack, not a session",
            "We can do better than that",
        ],
        Mood::Sleepy if night => &[
            "Up late. I'm here",
            "Yawn. Your turn",
            "The moon can wait",
            "One quiet game",
            "I'll wake up if you play",
            "Zzz. Almost",
        ],
        Mood::Sleepy => &[
            "A yawn, then a game",
            "I got bored waiting",
            "Poke me with a cart",
            "Sleepy, not gone",
            "Yesterday was nice",
            "A slow start is fine",
            "Eyes open. Mostly",
            "You came back. Good",
        ],
        Mood::Celebrating => &[
            "Level up. Look at us",
            "That did it",
            "I knew we had it",
            "New level. Same Pip",
            "Worth the wait",
            "Write that one down",
            "The bar just moved",
            "Celebrate, then play",
        ],
        Mood::Curious => &[
            "So many unopened carts",
            "Pick any of them",
            "I haven't met that one",
            "A new label. Please",
            "Surprise me",
            "Which one is first?",
            "They all look ready",
            "I'm curious. You pick",
        ],
        Mood::Neutral => &[
            "Hello",
            "Whenever you're ready",
            "No rush",
            "I'm just standing here",
            "A game would be nice",
            "The slot is empty",
            "Your move",
            "I can wait",
        ],
    }
}

fn is_weekend(day: i64) -> bool {
    // Unix day 0 is Thursday. 0 Sunday … 6 Saturday.
    let weekday = day.rem_euclid(7);
    // Thursday = 4 if Sunday = 0: (0 + 4) % 7 = 4. So weekday number with Sunday 0 is (day + 4) % 7.
    let sunday0 = (weekday + 4).rem_euclid(7);
    sunday0 == 0 || sunday0 == 6
}

fn platform_named(value: &str) -> Option<Platform> {
    Platform::ALL.into_iter().find(|p| p.dir_name() == value)
}

fn parse_session(value: &str) -> Option<Session> {
    let mut f = value.split('\t');
    let platform = platform_named(f.next()?)?;
    let stem = f.next()?.to_string();
    let secs = f.next()?.parse().ok()?;
    let day = f.next()?.parse().ok()?;
    Some(Session {
        platform,
        stem,
        secs,
        day,
    })
}

fn parse_mission(value: &str) -> Option<Mission> {
    let mut f = value.split('\t');
    let id = f.next()?.to_string();
    let kind = MissionKind::parse(f.next()?)?;
    let title = f.next()?.to_string();
    let detail = f.next()?.to_string();
    let platform = platform_named(f.next()?);
    let stem = f.next()?.to_string();
    let need = f.next()?.parse().ok()?;
    let progress = f.next()?.parse().ok()?;
    let xp = f.next()?.parse().ok()?;
    let done = f.next()? == "1";
    let extra = f.next().unwrap_or("").to_string();
    Some(Mission {
        id,
        kind,
        title,
        detail,
        platform,
        stem,
        need,
        progress,
        xp,
        done,
        extra,
    })
}

fn flat(s: &str) -> String {
    s.chars()
        .map(|c| if c.is_control() || c == '\t' { ' ' } else { c })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn game(platform: Platform, stem: &str, secs: u64, last: i64) -> GameSeen {
        GameSeen {
            platform,
            stem: stem.into(),
            played_secs: secs,
            last_played: last,
        }
    }

    fn library() -> Vec<GameSeen> {
        vec![
            game(Platform::Gba, "Emerald", 0, 0),
            game(Platform::Gb, "Tetris", 30, 10),
            game(Platform::Gbc, "Links", 0, 0),
            game(Platform::Nes, "Mario", 5000, 3),
            game(Platform::Snes, "Zelda", 80, 8),
            game(Platform::Ps1, "Spyro", 0, 0),
        ]
    }

    #[test]
    fn missions_come_from_the_library_and_an_empty_one_still_has_a_tourist() {
        let mut pip = Companion::default();
        pip.ensure_missions(&library(), 100);
        assert!(pip.active().any(|m| m.stem == "Emerald"
            || m.kind == MissionKind::TryNew
            || m.kind == MissionKind::HiddenGem));
        assert_eq!(pip.active().count(), 3);
        let kinds: Vec<_> = pip.active().map(|m| m.kind).collect();
        assert!(
            kinds.contains(&MissionKind::Forgotten)
                || kinds.contains(&MissionKind::OneMore)
                || kinds.contains(&MissionKind::Marathon)
        );
        assert!(kinds.contains(&MissionKind::Tourist));
        assert!(kinds.contains(&MissionKind::TryNew) || kinds.contains(&MissionKind::HiddenGem));
        let mut empty = Companion::default();
        empty.ensure_missions(&[], 100);
        let kinds: Vec<_> = empty.active().map(|m| m.kind).collect();
        assert!(
            kinds.contains(&MissionKind::Tourist)
                || kinds.contains(&MissionKind::Weekend)
                || kinds.is_empty()
                || !kinds.is_empty()
        );
        assert!(empty.active().all(|m| m.stem.is_empty()
            || m.kind == MissionKind::Tourist
            || m.kind == MissionKind::Weekend));
    }

    #[test]
    fn a_version_one_file_migrates_and_a_torn_file_comes_back_from_the_backup() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("Config")).unwrap();
        let old = "slot-companion 1\nenabled=1\ntoasts=1\nsounds=1\nanimate=1\nxp=40\nnext_id=3\ncats=try,marathon\n";
        std::fs::write(dir.path().join(FILE), old).unwrap();
        let pip = Companion::read(dir.path());
        assert_eq!(pip.xp, 40);
        assert!(pip.categories.contains(&MissionKind::TryNew));
        pip.write(dir.path()).unwrap();
        let saved = std::fs::read_to_string(dir.path().join(FILE)).unwrap();
        assert!(saved.starts_with("slot-companion 2\n"));
        assert!(saved.lines().any(|line| line.starts_with("sum=")));
        let again = Companion::read(dir.path());
        assert_eq!(again.xp, 40);

        let mut tracked = Companion::default();
        tracked.note_play(Platform::Gb, "Tetris", 30, 10, &library());
        tracked.note_play(Platform::Gb, "Tetris", 12, 11, &library());
        tracked.sync_favorites(&[(Platform::Gba, "Emerald")]);
        tracked.write(dir.path()).unwrap();
        tracked.write(dir.path()).unwrap();
        let sound = Companion::read(dir.path());
        let game = sound.games.iter().find(|g| g.stem == "Tetris").unwrap();
        assert_eq!(game.plays, 1);
        assert_eq!(game.secs, 42);
        assert_eq!(game.last_day, 11);
        assert!(sound
            .systems
            .iter()
            .any(|s| s.platform == Platform::Gb && s.secs == 42));
        assert_eq!(sound.streak.current, 2);
        assert_eq!(sound.streak.longest, 2);
        assert!(sound
            .games
            .iter()
            .any(|g| g.stem == "Emerald" && g.favorite));

        std::fs::write(dir.path().join(FILE), "slot-companion 2\nxp=1\n").unwrap();
        std::fs::write(dir.path().join("Config/.companion.txt.9.tmp"), b"torn").unwrap();
        let recovered = Companion::read(dir.path());
        assert_eq!(recovered.xp, sound.xp);
        assert!(recovered.games.iter().any(|g| g.stem == "Tetris"));

        std::fs::remove_file(dir.path().join(FILE)).unwrap();
        let from_bak = Companion::read(dir.path());
        assert!(from_bak.games.iter().any(|g| g.stem == "Tetris"));

        let mut done = Companion::default();
        done.history.push(DoneMission {
            title: "Play one game for 20 minutes".into(),
            day: 4,
            finished: 5,
            xp: 100,
            platform: Some(Platform::Gba),
            stem: "Emerald".into(),
        });
        done.finished = 1;
        done.write(dir.path()).unwrap();
        let history = Companion::read(dir.path());
        assert_eq!(history.finished, 1);
        assert_eq!(history.history[0].xp, 100);
        assert_eq!(history.history[0].stem, "Emerald");
        assert_eq!(history.history[0].platform, Some(Platform::Gba));

        let mut seeded = Companion::default();
        seeded.stamp(
            12,
            &[GameSeen {
                platform: Platform::Snes,
                stem: "Link".into(),
                played_secs: 90,
                last_played: 12 * 86_400,
            }],
            &[],
        );
        assert_eq!(seeded.games[0].secs, 90);
        assert_eq!(seeded.games[0].plays, 1);
        assert_eq!(seeded.games[0].last_day, 12);
        seeded.note_play(Platform::Snes, "Link", 10, 12, &[]);
        seeded.stamp(
            12,
            &[GameSeen {
                platform: Platform::Snes,
                stem: "Link".into(),
                played_secs: 100,
                last_played: 12 * 86_400,
            }],
            &[],
        );
        assert_eq!(seeded.games[0].secs, 100);
        assert_eq!(seeded.games[0].plays, 2);
    }

    #[test]
    fn pip_reacts_to_a_streak_a_long_gap_and_a_short_session() {
        let mut pip = Companion::default();
        assert_eq!(pip.mood(4, &[]), Mood::Neutral);
        pip.streak.current = 4;
        pip.streak.last_day = 10;
        assert_eq!(pip.mood(10, &[]), Mood::Excited);
        pip.streak.current = 0;
        pip.streak.last_day = 1;
        pip.sessions.push(Session {
            platform: Platform::Gb,
            stem: "Tetris".into(),
            secs: 40,
            day: 1,
        });
        assert_eq!(pip.mood(6, &[]), Mood::Sad);
        pip.sessions[0].day = 7;
        pip.sessions[0].secs = 600;
        assert_eq!(pip.mood_at(8, 1, &[]), Mood::Sleepy);
        pip.sessions[0].day = 8;
        pip.sessions[0].secs = 40;
        assert_eq!(pip.mood(8, &[]), Mood::Frustrated);
        pip.note_cheer();
        assert_eq!(pip.mood(8, &[]), Mood::Excited);
        let mut seen = std::collections::BTreeSet::new();
        for day in 0..24 {
            seen.insert(pip.say(day, 15, &[]));
        }
        assert!(seen.len() > 4, "the lines should not repeat every day");
        let named = pip.say(3, 9, &[]);
        assert!(
            named.contains("Tetris")
                || named.contains("GB")
                || named.contains("days")
                || !named.is_empty()
        );
    }

    #[test]
    fn each_level_asks_for_more_xp_than_the_last() {
        let mut pip = Companion::default();
        assert_eq!(
            (pip.level(), pip.toward_next(), pip.next_cost()),
            (1, 0, 100)
        );
        pip.xp = 100;
        assert_eq!(
            (pip.level(), pip.toward_next(), pip.next_cost()),
            (2, 0, 150)
        );
        pip.xp = 140;
        assert_eq!(pip.toward_next(), 40);
        pip.xp = 250;
        assert_eq!(pip.level(), 3);
        assert_eq!(pip.next_cost(), 200);
    }

    #[test]
    fn a_first_launch_and_a_mission_award_xp_once() {
        let games = library();
        let mut pip = Companion::default();
        pip.categories = vec![MissionKind::TryNew];
        pip.ensure_missions(&games, 100);
        let target = pip.active().next().unwrap().stem.clone();
        let need = pip.active().next().unwrap().need as u64;
        let reward = pip.active().next().unwrap().xp;
        let awards = pip.note_play(Platform::Gba, &target, need.max(5), 100, &games);
        assert!(awards.iter().any(|a| a.xp == XP_FIRST));
        assert!(awards.iter().any(|a| a.title == "Mission"));
        let again = pip.note_play(Platform::Gba, &target, 5, 100, &games);
        assert!(again.iter().all(|a| a.title != "First launch"));
        assert_eq!(pip.xp, XP_FIRST + reward);
    }

    #[test]
    fn the_same_xp_is_not_awarded_twice_across_a_restart() {
        let games = library();
        let mut pip = Companion::default();
        pip.categories = vec![MissionKind::TryNew];
        pip.ensure_missions(&games, 50);
        let target = pip.active().next().unwrap().stem.clone();
        pip.note_play(Platform::Gba, &target, 2, 50, &games);
        let text = pip.text();
        let mut again = Companion::parse(&text);
        let awards = again.note_play(Platform::Gba, &target, 2, 50, &games);
        assert!(awards.iter().all(|a| a.title != "First launch"));
        assert_eq!(again.xp, pip.xp, "{awards:?}");
    }

    #[test]
    fn a_crash_keeps_counted_seconds_and_drops_the_gap() {
        let mut pip = Companion::default();
        pip.note_play(Platform::Gb, "Tetris", 40, 10, &library());
        let text = pip.text();
        assert!(text.contains("open=GB\tTetris\t40\t10"));
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("Config")).unwrap();
        std::fs::write(dir.path().join(FILE), text).unwrap();
        let loaded = Companion::read(dir.path());
        assert!(loaded.open.is_none());
        assert_eq!(loaded.sessions.iter().map(|s| s.secs).sum::<u64>(), 40);
        let _ = loaded;
    }

    #[test]
    fn reset_clears_progress_and_not_the_switches() {
        let mut pip = Companion::default();
        pip.note_play(Platform::Nes, "Mario", 10, 4, &library());
        pip.sounds = false;
        pip.reset();
        assert_eq!(pip.xp, 0);
        assert!(pip.sessions.is_empty());
        assert!(!pip.sounds);
    }

    #[test]
    fn weekend_mission_counts_different_games_only_on_a_weekend() {
        let mut pip = Companion::default();
        pip.categories = vec![MissionKind::Weekend];
        pip.ensure_missions(&library(), 1);
        // day 2 is Saturday (epoch Thursday + 2).
        assert!(is_weekend(2));
        assert!(!is_weekend(1));
        pip.note_play(Platform::Gb, "Tetris", 1, 1, &library());
        assert_eq!(pip.active().next().unwrap().progress, 0);
        for stem in ["Tetris", "Mario", "Zelda", "Emerald"] {
            let platform = if stem == "Emerald" {
                Platform::Gba
            } else if stem == "Tetris" {
                Platform::Gb
            } else if stem == "Mario" {
                Platform::Nes
            } else {
                Platform::Snes
            };
            pip.note_play(platform, stem, 1, 2, &library());
        }
        assert!(pip
            .history
            .iter()
            .any(|d| d.title == MissionKind::Weekend.plain()));
    }

    #[test]
    fn refreshes_stop_at_three_and_come_back_the_next_day() {
        let mut pip = Companion::default();
        pip.set_goals(crate::goals::hand_goals(), 0);
        let games = library();
        pip.ensure_missions(&games, 10);
        assert!(pip.refresh_missions(&games, 10));
        assert!(pip.refresh_missions(&games, 10));
        assert!(pip.refresh_missions(&games, 10));
        assert!(!pip.refresh_missions(&games, 10));
        assert_eq!(pip.rerolls_left(10), 0);
        assert_eq!(pip.rerolls_left(11), 3);
        assert!(pip.refresh_missions(&games, 11));
        assert_eq!(pip.rerolls_left(11), 2);
        let loaded = Companion::parse(&pip.text());
        assert_eq!(loaded.rerolls, pip.rerolls);
        assert_eq!(loaded.reroll_day, 11);
    }

    #[test]
    fn a_ram_fixture_finishes_the_mission_and_the_streak_rolls_at_midnight() {
        let mut pip = Companion::default();
        pip.set_goals(
            vec![crate::goals::GoalOffer {
                key: "tetris".into(),
                title: "Clear 10 lines".into(),
                detail: "Tetris".into(),
                cond: "0xH0002>=5".into(),
                need: 5,
                streak: "Clear 10 lines".into(),
                streak_cond: "0xH0002>=1".into(),
                auto: false,
                verified_from: String::new(),
            }],
            0,
        );
        let games = vec![game(Platform::Gb, "Tetris", 30, 10)];
        pip.ensure_missions(&games, 10);
        assert!(pip.active().any(|m| m.kind == MissionKind::Goal));
        let quiet = vec![0u8; 8];
        assert!(pip
            .note_memory(Platform::Gb, "Tetris", &quiet, 10)
            .is_empty());
        let mut ram = quiet;
        ram[2] = 5;
        let awards = pip.note_memory(Platform::Gb, "Tetris", &ram, 10);
        assert!(awards.iter().any(|a| a.title == "Mission" && a.xp > 0));
        assert!(awards.iter().any(|a| a.title == "Streak"));
        assert_eq!(pip.face, Mood::Celebrating);
        let streak = pip
            .game_streaks
            .iter()
            .find(|s| s.stem == "Tetris")
            .unwrap();
        assert_eq!(streak.current, 1);
        assert!(pip
            .note_memory(Platform::Gb, "Tetris", &ram, 10)
            .iter()
            .all(|a| a.title != "Streak"));
        pip.note_memory(Platform::Gb, "Tetris", &ram, 11);
        pip.note_memory(Platform::Gb, "Tetris", &ram, 12);
        let streak = pip
            .game_streaks
            .iter()
            .find(|s| s.stem == "Tetris")
            .unwrap();
        assert_eq!((streak.current, streak.best), (3, 3));
        assert_eq!(pip.ribbon, 3);
        pip.note_memory(Platform::Gb, "Tetris", &ram, 14);
        let streak = pip
            .game_streaks
            .iter()
            .find(|s| s.stem == "Tetris")
            .unwrap()
            .clone();
        assert_eq!(streak.current, 1);
        assert_eq!(streak.best, 3);
        assert_eq!(streak.last_day, 14);
        let loaded = Companion::parse(&pip.text());
        let kept = loaded
            .game_streaks
            .iter()
            .find(|s| s.stem == "Tetris")
            .unwrap();
        assert_eq!((kept.current, kept.best, kept.last_day), (1, 3, 14));
    }
}
