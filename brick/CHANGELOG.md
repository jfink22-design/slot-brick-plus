# Changelog — Slot. for TrimUI Brick

## Unreleased
- HOME → Companion: Metroid Fusion, Zero Mission, Aria of Sorrow, Circle of the Moon, Harmony of Dissonance, Wario Land 4, Kirby & the Amazing Mirror, Mario Kart: Super Circuit, Symphony of the Night, Pokémon Gold, Silver and Crystal, Wario Land 3, Super Mario Bros. Deluxe, and Oracle of Seasons have checked in-game goals (find a tank, copy a power, defeat enemies, and so on). A goal counts a change while you play, so a save that is already past the target does not finish it. Today's missions can also be real goals in the game, such as popping balloons, collecting coins, or clearing a level, for Mario, Zelda, Pokémon, Tetris, Kirby, Metroid, Balloon Fight, and other games on the card. A game that already has a saved achievement set uses goals from that set. Refresh missions, on the Companion screen, rerolls today's three missions, up to three times a day, and that count is still there after a restart. Each game keeps its own streak, with a best and a reward for Pip at 3, 7, 14 and 30 days, beside the streak for playing any game. Progress shows while you play, and Pip celebrates when a mission is done. Games with no goal of their own still get a time mission.
- HOME → Videos: leaving a video clears the picture, so the list underneath is visible. X while it plays switches between filling the screen and fitting it with black bars, and that choice is remembered. A file with no sound says so, is marked in Saved, and Y queues it for download again.
- HOME → Videos is three tabs: Saved, Downloads, and Channels. L1 and R1 move between them, and it opens on Saved when something is already on the card. Downloads shows progress and a count while something is queued. The same tabs are on HOME → Music.
- A download shows its own progress, and cancelling it stops that download so the next one starts from zero. A video is saved only when the whole file has arrived, as H.264 with sound. A connection that sends nothing for 30 seconds is tried again from where it stopped.
- HOME → Music: add a YouTube channel, @handle, or playlist and download the audio. Files are saved under Music with the picture from the video. Play, pause, skip, shuffle, repeat, and a seek bar. It keeps playing on the system menu and stops when a game starts. The volume is the system volume. The Music shelf uses the same space limit as Videos. B goes up one level, and the keyboard opens only from Add a channel or playlist.
- The system menu shows the same battery, Wi-Fi and clock as the shelf. The battery stays the usual colour above 20%, turns yellow below that, and red at 10% or less. The charging mark shows only while a cable is plugged in.
- A video or music download starts even when the card cannot lock a file. If it cannot start, Slot says so instead of filling the log and closing.
- HOME → Videos: while a download is running the screen stays on and the Brick does not sleep by itself. Pressing Power still sleeps, and the download carries on from where it stopped when the Brick wakes. On battery it says to plug in. A dropped connection, including one closed mid-file, is tried again in pieces before it is reported. A Japanese channel name no longer closes the screen.
- RetroAchievements on Game Boy Color can read the console's work RAM again, including the extra banks, so a condition that looks there is no longer skipped.
- HOME → Videos: a download keeps going while a game is open, and after sleep or a restart, picking up the part it already saved. It runs slowly in the background so the game stays smooth. Videos → Pause downloads in games is off until you turn it on. The Videos shelf shows what is still queued. B from a channel's videos returns to the Videos page, and B again leaves for the shelf. The keyboard opens only from Add a channel. Choosing a video says it was queued and stays on that channel's list.
- HOME → Online → Update checks this Slot's own releases and installs a newer one. It checks the download before replacing Slot, keeps the previous one, and puts it back if the new one does not start. The screen shows the release's title and what changed. With no Wi-Fi it says so and leaves Slot as it is.
- HOME → Videos no longer closes when a channel's pictures are shown. A download that fails stays on that screen and says why, and a finished video appears on the Videos shelf. What happened is kept in Logs/videos.log.
- HOME → Companion: Pip, a small pixel friend, suggests a game you have not played much and keeps a few missions (try a new game, revisit one, play three systems, a weekend of four games). Playing earns XP and levels. Toasts show a mission or a level-up. Y on a mission marks it done if Slot could not see it happen. Settings turn Pip, the toasts, and each mission kind on or off, and can reset Pip without touching your games. X on the shelf opens Pip too.
- HOME → Videos: add a channel by its name, @handle, or link. A on the channel lists its latest videos. If the list cannot be fetched, the screen says why. Videos used and free space on the card are on that screen. Video space limit starts at No limit; Left and Right set 1, 2, 4, 8, or 16 GB. Sign-in is gone.
- HOME → Companion: Pip is a chunky walking cartridge, orange with a shaded shell, a big cream face, and cream sneakers. It squishes while it idles, waves a whole arm, sulks in the rain, stomps when a session was tiny, jumps when a streak or an achievement lands, yawns, tilts its head when it is curious, and throws both arms up on a level-up. The line under it changes with the time of day, the streak, and the game. Animations and Sounds in its settings still turn those off.
- HOME → Online → RetroAchievements works away from Wi-Fi once a game's set has been saved. CACHE ACHIEVEMENT SETS, over Wi-Fi, saves every game in the library, including its badges. Unlocks still happen on the device, show a toast, and stay in VIEW ACHIEVEMENTS. They are sent when Wi-Fi returns, and a torn save is restored from its spare copy. This is softcore only: a hardcore unlock is not kept to send later.
- HOME → Companion and HOME → Videos use the same large type as the rest of Slot's menus. Missions read as what to do, such as playing one game for a few minutes or any game on one system. Companion keeps three of them when it can: one from games you have played, one about a system, and one about a game you have not played. The next level costs 100 XP, then 150, then 200. Companion's save keeps a spare copy and comes back if the file is torn. It also keeps each game's time, a streak, and favorites.
- The Videos shelf: L1 and R1 reach it beside GBA, GB, GBC, NES, SNES and PlayStation once a video is saved. Each video is a tape with that picture on the label. A plays it from where it stopped. Y asks, then deletes it. Library → Systems → Videos hides the shelf. It stays off the carousel when nothing is saved.
- Gameplay → Swap A/B: trade A and B's places, in games and in Slot's own menus. Also reachable without leaving the game: HOME's own menu now has a Buttons entry with the same switch.
- Fixed: on Slot OS, the mute switch did nothing, on the Brick and the Brick Pro. Thanks to @jyav-19 for testing Slot OS on a Brick Pro.
- Tidier downloads: each release now holds only `slot-stock-…zip` for stock firmware and `slot-os-brick-…img.xz` / `slot-os-brickpro-…img.xz` for Slot OS. Online → Update gets its files from a separate Updates release. Coming from v0.60 or older, update to this version by hand once: on stock firmware, copy what is inside the `SD-CARD` folder of `slot-stock-…zip` onto your card; on Slot OS, do the same with `slot-os-brick-…-update.zip` (or the Brick Pro's) from the Updates release. Your games, saves and settings stay.

## v0.60
**New since v0.50:**

### Features
- Added a check for updates: on boot, then every 30 minutes and whenever you slot a game out, while Wi-Fi is on. A small arrow shows in the footer when a newer Slot is out.
- Added a pause menu: tap HOME in a game to resume, save or load a state, reset or quit. By @kirby-letsgo
- Added support for zipped games (`.zip` and `.7z`) for every system except PS1. By @kirby-letsgo
- Favorites got a makeover: press Y on a cartridge and a star pops onto it with a nice sound.
- Added various system sounds, and you can turn each one on or off in System → System Sounds.
- Speaking of menus, the settings have been reorganized so related things sit together. They're still rough in places, so expect more changes.

### Fixes
- Fixed PS1 games slowing down startup. By @TT-Hipster1941
- Fixed Wi-Fi not showing as connected after waking from sleep.
- Fixed the GBA cart colour during the label animation.

**Slot OS for the Brick Pro** hasn't been fully tested on a Brick Pro, so if anything acts weird, please open an issue.

## v0.50
**New since v0.44:**
- PlayStation: PS1 games from `Slot/Games/PS1` (`.cue`/`.bin`, `.chd`, `.pbp`, `.iso`, `.m3u`) on a shelf of discs that spin into the slot, games of several discs included.
- Rumble: games that rumble now drive the Brick's motor.
- Favorites (Y) and sorting (SELECT), with play time and a title card for the selected game.
- Brick Pro support: its left stick works as the D-pad, and its stick lights have their own LED row.
- Fonts: HOME → System → Font, with GohuFont included, and room for your own in `Slot/Fonts`.
- A reworked HOME menu: Emulation, Gameplay, Library, Online and System.
- Faster startup, and Wi-Fi fixes.

## v0.44
**New since v0.43:**
- HOME → Update: update Slot over Wi-Fi. It shows what's new (L1 / R1 scroll long notes), asks before installing, and leaves your games, saves, states, labels and settings alone. Updates are signed and checked before anything is replaced; if the new version doesn't start, the previous one is put back, and an update cut short by the Brick switching off is cleared away. Needs 20% battery or a charger.
- HOME → System: LEDs, Date & Time and About are now under System, leaving the menu shorter.
- System → Auto Clock: with Wi-Fi on, the clock now sets itself from an internet time server, on connecting and every six hours. On by default; the time zone is still set under Date & Time.

## v0.43
**New since v0.42:**
- Overlays for every system come with Slot: KrutzOtrem's *Vanilla* set for GB, GBC and GBA and *CRT* set for NES and SNES, made for the Brick. Thanks to @KrutzOtrem!
- Overlays with a grid or tint over the game (such as KrutzOtrem's LCD and DMG grids) now work: the game fills the part of the overlay that isn't fully opaque, at the size the overlay was drawn for.
- An overlay name too long for its row is shortened, and scrolls slowly while the row is selected.
- Gameplay → Auto Save: save the game every 1, 5 or 10 minutes, or never. It was every minute.
- Gameplay → Low Battery Shutdown: save and shut down at 3%, 5% or 10% battery, or never. It was 5%.

## v0.42
**New since v0.41:**
- Screen settings are sorted by system: Game Boy / GBC, Game Boy Advance, NES and SNES each have their own page. NES and SNES shaders can now be set there too.
- Colour correction is set separately for the Game Boys and the GBA. A setting saved before keeps working for both.
- Overlays: put your own 1024×768 PNG overlays in `Slot/Overlays/<system>` and pick one on the system's Screen page. The game fits itself into the overlay's window. In game, HOME + L1 / R1 switches between them.

## v0.41
**New since v0.40:**
- Choosing a label now ends with it peeling onto the cart like a sticker, on every cart. On GBA carts the front closes for it, then opens again.
- Low-battery warning on the F1/F2 lights: steady red at 15% or below, blinking at 10% or below, even with the lights turned off. Thanks to @wargenng!
- Fixed: the About screen showed the wrong version.

## v0.40
**New since v0.23:**
- Wifi status icon on the shelf footer. Icons may change.
- Label Picker. Press Y when in the edit screen on a game cart. It has a label editor built in, so you can resize the label and make it fit the cart cutout
- Finally fixed Wi-Fi
- Customize cartridges: color, finish and shape for all shelves.
- Add SNES support: SNES and PAL cartridges
- Add NES support: NES and FAMICOM Cartridges
- Add RetroAchievements support, softcore only
- Automatic label fetcher in main menu, it fetches labels for all carts at once, but it is really barebones, so refer to the label picker for more control

**FOR DEVELOPERS**
- The Brick simulator - for testing in the browser before real hardware testing. Refer to the Development part of the Readme.

## v0.23
- Settings > About credits the port to LordNordyx.
- Short app names in the stock Apps list: Slot, Logo, Check and Stock, each with its own icon.
  Folder names are unchanged, so existing Apps/BootLogo/Image pictures keep working.
- Card README rewritten with the Brick's actual button layout (HOME = left button, SELECT = middle).
- Removed leftovers: the old Tools/tg5040/Slot.pak launcher and the RetroAchievements research note.

## v0.22.2
- Fixed "works once, then stock menu": the startup hook now waits until the real SD card is
  mounted instead of reading a stray Slot folder on internal storage.
- Check lists files hidden under the card mount on internal storage.

## v0.22.1
- The startup hook waits up to 30 s (was 10 s) for slow-mounting cards and logs each decision
  to Slot/Logs/hook.log.

## v0.22
- A read-only or damaged card no longer drops silently to the stock menu; Slot shows
  "Can't write to SD card".
- Check writes startup-report.txt at the top of the card.

## v0.21
- Fixed autostart failing after a firmware update or reinstall.
- New Check app: read-only startup report.

## v0.17 – v0.20
- SELECT + X switches GBA between 3:2 and 4:3 full screen, remembered per game.
- About screen describes the Brick port.

## v0.16
- Waking no longer needs two Power presses; Wi-Fi reconnects in the background.

## v0.15
- Merged upstream Slot 1.6.0: GB cartridge clocks kept in saves, read-only card alarm, faster
  library loading, shuffled wallpapers, crash logs.
- Boot Logo app (now Logo) replaces the separate install/restore logo apps.

## v0.14
- Lower sleep drain: radios off during suspend, real suspend-to-RAM where available.

## v0.13
- Auto-sleep after 5 min idle in menus or 15 min in game (Slot/DISABLE_AUTO_SLEEP turns it off).

## v0.12
- Wi-Fi works on stock firmware; minimum CPU clock in games fixes crackling audio.

Earlier versions: Brick screen, input, LEDs, power menu and autostart (see SOURCES.md).
