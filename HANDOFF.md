# Handoff

Stage 9, written by the agent. `C5 Consolidate in your own words`. A fresh session starts from this file, not from memory.

## What this is

The assignment page for the CIS 110 LN182 Collaborative Game Design Project, live at https://sgilson7.github.io/cis110-game-lab/ from the public repository sgilson7/cis110-game-lab. It shows what teams submit, the seven checkpoints with the next one computed for today, Microsoft 365 setup with a Google Drive backup, roles, grading and what happens to the game. The Builder additions for student submissions are PR sgilson7/builder-setting#1, not merged.

## Rules that hold it up

| Rule | Guard |
| --- | --- |
| Dates, times, limits and points match the syllabus (GL-1) | `crates/core/tests/syllabus.rs` |
| The next checkpoint changes at 5:00 PM ET on its day (GL-2) | `schedule.rs` and `api.rs` tests; browser interactions with `?on=` |
| Every sentence under 25 words (TONE.md rule 2) | `crates/core/tests/sentences.rs` |
| Builder's words stay off the page (GL-4) | `GLOSSARY.md` `Not:` lines, `copy.rs` |
| The page holds no number and loads nothing from elsewhere (D3, GL-6) | `boundary.rs`, browser check |

## Shape of the code

| What | Where |
| --- | --- |
| Syllabus values | `data/schedule.json` |
| Every word on the page | `data/strings.json`, `GLOSSARY.md` |
| Templates and screenshots | `data/templates/`, `data/img/games/` |
| Dates, schedule, placeholders, view model | `crates/core/src/{dates,schedule,text,view,api}.rs` |
| Drawing | `web/app.js`, `web/style.css` |

## Commands

`./scripts/check.sh` (needs Playwright; this machine used `/Users/samgilson/Documents/LevelTwenty/.venv/bin` on `PATH`); `./scripts/publish.sh` from a clean tree; then `BUILDER_ORIGIN=https://sgilson7.github.io/cis110-game-lab python3 tests/browser/check.py`. Preview another date with `?on=2026-11-20T10:00`.

## What will bite within the hour

Binary files under `web/` break the boundary scan; put them in `data/` (R2.2). `cargo fmt` rewraps lines that `tests/breaks.json` rows match; rerun `scripts/break_tests.py` after formatting. `publish.sh` needs `user.name` and `user.email` in this repository's git config.

## The journal

`../game-codesign-lab-journal/index.html`, also published privately at https://claude.ai/artifact/98FJQU85jomchGMSkfrJkb (republish from a copy without the document wrapper; see the scratchpad publish step). Newest entry: the mockups.

## Open for the person

- R1.3: decide where the Builder additions live and whether `builder-setting` gets its git history back (GL-Q6).
- R1.4: decide whether students get a prototype to review before the final deadline, and when (GL-Q1).
- R1.5: confirm the MerryQuery site address the mockups follow (GL-Q3).
- R1.6: confirm how much of the course grade the project carries before the page states a percentage.
- R1.8: approve fixed template files and names for the three deliverables (GL-Q4).
- R2.7: walk through the Microsoft 365 steps once with a CCCC student account.
- R2.8: read the role names, FAQ, to-do lists and example games the agent wrote.
- R2.9: confirm the instructor email may appear on the public page.
- R2.10: confirm the prototype date, Thursday, December 3.
- R2.12: confirm with Peter that Ape Race may appear in the gallery.

## Mistakes so far

Page: three small decisions in the first `app.js` (R2.4), moved to the core. Copy: four sentences over 25 words and two unclear labels reached the first deploy (R2.13, R2.14); a test now holds the length. Dates: one wrong claim about SAM timing in the mockup (R1.11).

## What passed

2026-10-19, commit 1219e20: `./scripts/check.sh` passed; core tests 71 passed; breaks 13 of 13 caught; browser check passed locally and at the live address in Chromium, Firefox, WebKit and an iPhone profile (117 checks each run).

## The next action

Walk through the Microsoft 365 setup steps with a CCCC student account (R2.7), then link the page from Blackboard.
