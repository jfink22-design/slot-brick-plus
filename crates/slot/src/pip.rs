//! HOME → Companion. Pip is an original walking cartridge: orange shell, cream label, a face.
//! Not PakUI's companion: different name, different drawing, Slot's own lines.
use slot_gfx::OUT_W;
use slot_input::{Action, Btn};
use slot_store::{Companion, GameSeen, MissionKind, Platform};
use slot_ui::{TexId, UndoFace};

use crate::sheet::Sheet;

/// The HOME menu's type size and row, on Slot's 640×480 screen (the Brick draws that at 1024×768).
const ROW_PX: f32 = 30.0;
const ROW_H: u32 = 40;
const ROW_PITCH: u32 = 46;
const LEGEND_TOP: u32 = 432;

pub struct PipLaunch {
    pub platform: Platform,
    pub stem: String,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Page {
    Main,
    History,
    Streaks,
    Settings,
}

pub struct PipMenu {
    page: Page,
    row: usize,
    reset_armed: bool,
    pub dirty: bool,
    pub face: Option<TexId>,
    launch: Option<PipLaunch>,
    /// Last animation tick already drawn, so a still Pip is not redrawn every frame.
    shown: u64,
}

impl PipMenu {
    pub fn open() -> Self {
        PipMenu {
            page: Page::Main,
            row: 0,
            reset_armed: false,
            dirty: true,
            face: None,
            launch: None,
            shown: u64::MAX,
        }
    }

    pub fn needs_frame(&self, tick: u64, animate: bool) -> bool {
        self.dirty || (animate && self.shown != tick)
    }

    pub fn mark_drawn(&mut self, tick: u64) {
        self.shown = tick;
        self.dirty = false;
    }

    pub fn take_launch(&mut self) -> Option<PipLaunch> {
        self.launch.take()
    }

    pub fn game(&self, pip: &Companion) -> Option<String> {
        pip.suggestion().map(|(_, stem)| stem).or_else(|| {
            pip.active()
                .find(|m| !m.stem.is_empty())
                .map(|m| m.stem.clone())
        })
    }

    /// True when the screen should close. `pip` is updated in place.
    pub fn input(
        &mut self,
        action: Action,
        pip: &mut Companion,
        games: &[GameSeen],
        day: i64,
    ) -> bool {
        if !matches!(
            action,
            Action::GbaDown(_) | Action::ShelfLeft | Action::ShelfRight
        ) {
            return false;
        }
        self.dirty = true;
        if self.page != Page::Main && matches!(action, Action::GbaDown(Btn::B)) {
            self.page = Page::Main;
            self.row = 0;
            self.reset_armed = false;
            return false;
        }
        let rows = self.rows(pip);
        match action {
            Action::GbaDown(Btn::B) => return true,
            Action::GbaDown(Btn::Up) => self.row = (self.row + rows - 1) % rows,
            Action::GbaDown(Btn::Down) => self.row = (self.row + 1) % rows,
            Action::GbaDown(Btn::X) if self.page == Page::Main => {
                pip.refresh_suggestion(games, day)
            }
            Action::GbaDown(Btn::Y) if self.page == Page::Main => {
                let active: Vec<usize> = pip
                    .missions
                    .iter()
                    .enumerate()
                    .filter(|(_, m)| !m.done)
                    .map(|(i, _)| i)
                    .collect();
                if let Some(at) = active.get(self.row) {
                    let _ = pip.complete_by_hand(*at, day);
                }
            }
            Action::GbaDown(Btn::L1) | Action::ShelfLeft if self.page == Page::Main => {
                if let Some((platform, _)) = pip.suggestion() {
                    pip.toggle_excluded(platform);
                    pip.refresh_suggestion(games, day);
                }
            }
            Action::GbaDown(Btn::R1) | Action::ShelfRight if self.page == Page::Main => {
                self.page = Page::Streaks;
                self.row = 0;
            }
            Action::GbaDown(Btn::A) => self.confirm(pip, games, day),
            _ => {}
        }
        false
    }

    fn confirm(&mut self, pip: &mut Companion, games: &[GameSeen], day: i64) {
        match self.page {
            Page::Main => {
                let n = pip.active().count();
                if self.row < n {
                    let mission = pip.active().nth(self.row).cloned();
                    if let Some(mission) = mission {
                        if let Some(platform) = mission.platform {
                            self.launch = Some(PipLaunch {
                                platform,
                                stem: mission.stem,
                            });
                        }
                    }
                } else if self.row == n {
                    if let Some((platform, stem)) = pip.suggestion() {
                        self.launch = Some(PipLaunch { platform, stem });
                    }
                } else if self.row == n + 1 {
                    self.page = Page::History;
                    self.row = 0;
                } else if self.row == n + 2 {
                    self.page = Page::Streaks;
                    self.row = 0;
                } else if self.row == n + 3 {
                    let _ = pip.refresh_missions(games, day);
                } else {
                    self.page = Page::Settings;
                    self.row = 0;
                }
            }
            Page::Settings => self.settings(pip, games, day),
            Page::History | Page::Streaks => {}
        }
    }

    fn settings(&mut self, pip: &mut Companion, games: &[GameSeen], day: i64) {
        let cats = MissionKind::ALL.len();
        match self.row {
            0 => pip.set_enabled(!pip.enabled),
            1 => pip.toasts = !pip.toasts,
            2 => pip.sounds = !pip.sounds,
            3 => pip.animate = !pip.animate,
            n if n < 4 + cats => {
                pip.toggle_category(MissionKind::ALL[n - 4]);
                pip.ensure_missions(games, day);
            }
            _ => {
                if self.reset_armed {
                    pip.reset();
                    self.reset_armed = false;
                } else {
                    self.reset_armed = true;
                }
            }
        }
    }

    fn rows(&self, pip: &Companion) -> usize {
        match self.page {
            Page::Main => pip.active().count() + 5,
            Page::History | Page::Streaks => 1,
            Page::Settings => 4 + MissionKind::ALL.len() + 1,
        }
        .max(1)
    }

    pub fn render(
        &self,
        pip: &Companion,
        day: i64,
        hour: u32,
        games: &[GameSeen],
        frame: u64,
    ) -> UndoFace {
        let mut sheet = Sheet::new();
        let w = OUT_W.saturating_sub(48);
        match self.page {
            Page::Main => self.render_main(&mut sheet, pip, day, hour, games, frame, w),
            Page::History => self.render_history(&mut sheet, pip, w),
            Page::Streaks => self.render_streaks(&mut sheet, pip, w),
            Page::Settings => self.render_settings(&mut sheet, pip, w),
        }
        sheet.finish()
    }

    fn render_main(
        &self,
        sheet: &mut Sheet,
        pip: &Companion,
        day: i64,
        hour: u32,
        games: &[GameSeen],
        frame: u64,
        w: u32,
    ) {
        let mood = pip.mood_at(day, hour, games);
        let sprite = crate::pip_art::frame(mood, frame, pip.animate);
        let size = crate::pip_art::CELL * crate::pip_art::SCALE;
        sheet.image(8, 4, size, size, &sprite);
        let text_x = 16 + size;
        sheet.text(
            &format!(
                "{}   L{} {}/{}{}",
                pip.say(day, hour, games),
                pip.level(),
                pip.toward_next(),
                pip.next_cost(),
                if pip.ribbon > 0 {
                    format!("  Ribbon {}", pip.ribbon)
                } else {
                    String::new()
                }
            ),
            text_x,
            28,
            OUT_W.saturating_sub(text_x + 12),
            ROW_H,
            ROW_PX,
            false,
        );
        let mut row = 0usize;
        let mut y = 12 + size;
        let pitch = 44u32;
        for mission in pip.active() {
            if row == self.row {
                sheet.rect(0, y, OUT_W, ROW_H, slot_ui::edge());
            }
            let name = if mission.kind == MissionKind::Goal {
                format!("{} · {}", mission.title, mission.stem)
            } else if !mission.stem.is_empty() {
                format!("{} · {}", mission.kind.plain(), mission.stem)
            } else if let Some(platform) = mission.platform {
                format!("{} · {}", mission.kind.plain(), platform.name())
            } else {
                mission.kind.plain().to_string()
            };
            let (done, goal) = if mission.kind != MissionKind::Goal && mission.need >= 60 {
                (mission.progress.min(mission.need) / 60, mission.need / 60)
            } else {
                (mission.progress.min(mission.need), mission.need)
            };
            sheet.text(
                &format!("{name}  {done}/{goal}"),
                16,
                y,
                w,
                ROW_H,
                ROW_PX,
                false,
            );
            row += 1;
            y += pitch;
            if y + pitch > LEGEND_TOP {
                break;
            }
        }
        let suggest = pip
            .suggestion()
            .map(|(_, stem)| format!("Next: {stem}"))
            .unwrap_or_else(|| "Next: nothing new".into());
        if row == self.row {
            sheet.rect(0, y, OUT_W, ROW_H, slot_ui::edge());
        }
        sheet.text(&suggest, 16, y, w, ROW_H, ROW_PX, false);
        y += pitch;
        row += 1;
        let refresh = format!("Refresh missions ({} left)", pip.rerolls_left(day));
        let labels = [
            ("History".to_string(), row),
            ("Streaks".to_string(), row + 1),
            (refresh, row + 2),
            ("Settings".to_string(), row + 3),
        ];
        for (label, at) in labels {
            if y + ROW_H > LEGEND_TOP {
                break;
            }
            if self.row == at {
                sheet.rect(0, y, OUT_W, ROW_H, slot_ui::edge());
            }
            sheet.text(&label, 16, y, w, ROW_H, ROW_PX, false);
            y += pitch;
        }
        sheet.text(
            "A PLAY    X ANOTHER    Y DONE    R1 STREAKS    B BACK",
            16,
            LEGEND_TOP,
            OUT_W.saturating_sub(32),
            ROW_H,
            ROW_PX,
            false,
        );
    }

    fn render_streaks(&self, sheet: &mut Sheet, pip: &Companion, w: u32) {
        sheet.text("STREAKS", 16, 8, w, ROW_H, ROW_PX, false);
        sheet.text(
            &format!(
                "All games  {} days  best {}",
                pip.streak.current, pip.streak.longest
            ),
            16,
            56,
            w,
            ROW_H,
            ROW_PX,
            false,
        );
        if pip.game_streaks.is_empty() {
            sheet.text(
                "A game starts its own streak once it has a mission.",
                16,
                108,
                w,
                ROW_H,
                ROW_PX,
                true,
            );
            return;
        }
        let mut y = 108u32;
        for streak in &pip.game_streaks {
            if y + ROW_H > LEGEND_TOP {
                break;
            }
            sheet.text(
                &format!(
                    "{}  {}  {} days  best {}",
                    streak.stem, streak.theme, streak.current, streak.best
                ),
                16,
                y,
                w,
                ROW_H,
                ROW_PX,
                false,
            );
            y += ROW_PITCH;
        }
    }

    fn render_history(&self, sheet: &mut Sheet, pip: &Companion, w: u32) {
        sheet.text("RECENTLY", 16, 8, w, ROW_H, ROW_PX, false);
        if pip.history.is_empty() && pip.sessions.is_empty() {
            sheet.text(
                "No outings yet. Pip is happy to wait.",
                16,
                64,
                w,
                ROW_H,
                ROW_PX,
                true,
            );
        }
        let mut y = 56u32;
        for session in pip.sessions.iter().rev().take(5) {
            if y + ROW_H > LEGEND_TOP {
                break;
            }
            sheet.text(
                &format!(
                    "{}  {}  {}s",
                    session.platform.dir_name(),
                    session.stem,
                    session.secs
                ),
                16,
                y,
                w,
                ROW_H,
                ROW_PX,
                false,
            );
            y += ROW_PITCH;
        }
        for done in pip.history.iter().rev().take(3) {
            if y + ROW_H > LEGEND_TOP {
                break;
            }
            sheet.text(
                &format!("Done  {}", done.title),
                16,
                y,
                w,
                ROW_H,
                ROW_PX,
                true,
            );
            y += ROW_PITCH;
        }
        sheet.text(
            "B BACK",
            16,
            LEGEND_TOP,
            OUT_W.saturating_sub(32),
            ROW_H,
            ROW_PX,
            false,
        );
    }

    fn render_settings(&self, sheet: &mut Sheet, pip: &Companion, w: u32) {
        sheet.text("SETTINGS", 16, 8, w, ROW_H, ROW_PX, false);
        let mut rows = vec![
            format!("Pip  {}", on(pip.enabled)),
            format!("Toasts  {}", on(pip.toasts)),
            format!("Sounds  {}", on(pip.sounds)),
            format!("Animations  {}", on(pip.animate)),
        ];
        for kind in MissionKind::ALL {
            rows.push(format!(
                "{}  {}",
                kind.plain(),
                on(pip.categories.contains(&kind))
            ));
        }
        rows.push(if self.reset_armed {
            "Reset progress?  A again".into()
        } else {
            "Reset progress".into()
        });
        let visible = ((LEGEND_TOP - 56) / ROW_PITCH) as usize;
        let first = self.row.saturating_sub(visible.saturating_sub(1));
        for (line, label) in rows.iter().enumerate().skip(first).take(visible) {
            let y = 56 + (line - first) as u32 * ROW_PITCH;
            if line == self.row {
                sheet.rect(0, y, OUT_W, ROW_H, slot_ui::edge());
            }
            sheet.text(label, 16, y, w, ROW_H, ROW_PX, false);
        }
        sheet.text(
            "A CHANGE    B BACK",
            16,
            LEGEND_TOP,
            OUT_W.saturating_sub(32),
            ROW_H,
            ROW_PX,
            false,
        );
    }
}

fn on(flag: bool) -> &'static str {
    if flag {
        "On"
    } else {
        "Off"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use slot_store::MissionKind;

    fn games() -> Vec<GameSeen> {
        vec![GameSeen {
            platform: Platform::Gba,
            stem: "Emerald".into(),
            played_secs: 0,
            last_played: 0,
        }]
    }

    #[test]
    fn the_controls_launch_refresh_and_come_back() {
        let mut menu = PipMenu::open();
        let mut pip = Companion::default();
        pip.ensure_missions(&games(), 10);
        assert!(!menu.input(Action::GbaDown(Btn::X), &mut pip, &games(), 10));
        assert!(pip.suggestion().is_some() || pip.active().next().is_some());
        menu.row = pip.active().count();
        let closed = menu.input(Action::GbaDown(Btn::A), &mut pip, &games(), 10);
        assert!(!closed);
        assert!(menu.take_launch().is_some() || pip.suggestion().is_none());
        assert!(menu.input(Action::GbaDown(Btn::B), &mut pip, &games(), 10));
    }

    #[test]
    fn settings_can_turn_a_mission_kind_off_and_reset_asks_twice() {
        let mut menu = PipMenu::open();
        let mut pip = Companion::default();
        menu.page = Page::Settings;
        menu.row = 4;
        menu.input(Action::GbaDown(Btn::A), &mut pip, &games(), 1);
        assert!(!pip.categories.contains(&MissionKind::TryNew));
        menu.row = 4 + MissionKind::ALL.len();
        menu.input(Action::GbaDown(Btn::A), &mut pip, &games(), 1);
        assert!(menu.reset_armed);
        pip.xp = 40;
        menu.input(Action::GbaDown(Btn::A), &mut pip, &games(), 1);
        assert_eq!(pip.xp, 0);
        let face = menu.render(&pip, 1, 12, &games(), 0);
        assert_eq!(face.w, OUT_W);
        assert!(face.rgba.iter().any(|b| *b != 0));
        if let Ok(dir) = std::env::var("SLOT_FEATURE_PREVIEW") {
            let dir = std::path::PathBuf::from(dir);
            let _ = std::fs::create_dir_all(&dir);
            let mut shown = PipMenu::open();
            let mut pip = Companion::default();
            pip.xp = 140;
            pip.ensure_missions(&games(), 3);
            write_png(
                &dir.join("pip.png"),
                &shown.render(&pip, 3, 12, &games(), 0),
            );
            shown.page = Page::Settings;
            write_png(
                &dir.join("pip-settings.png"),
                &shown.render(&pip, 3, 12, &games(), 2),
            );
        }
    }

    fn write_png(path: &std::path::Path, face: &UndoFace) {
        let file = std::fs::File::create(path).unwrap();
        let mut encoder = png::Encoder::new(file, face.w, face.h);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        encoder
            .write_header()
            .unwrap()
            .write_image_data(&face.rgba)
            .unwrap();
    }
}
