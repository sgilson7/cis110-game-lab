# Collaborative Game Design Project, CIS 110

The assignment page for the Collaborative Game Design Project in CIS 110 LN182 (Introduction to Computers), Fall 2026, second 8-week session. Teams of 3 to 5 students design a small video game in three Microsoft Office files kept in a shared OneDrive folder. The instructor then builds a playable version of each design with [Builder](https://github.com/sgilson7/builder-setting).

**Live page:** <https://sgilson7.github.io/cis110-game-lab/>

The page shows what each team submits, the seven checkpoints and their due dates, how to set up and share the folder in Microsoft 365 (with Google Drive as a backup), team roles, and the grading rubric. It runs in the browser, stores nothing, and sends nothing.

## Where things live

| What | Where |
| --- | --- |
| Dates, times, limits and points, copied from the syllabus | `data/schedule.json` |
| Every word on the page (game version, at the site's root) | `data/strings.json` |
| Every word on the tool version, at `tool/` (T14) | `data/tool/strings.json`, with its page shell in `web/tool/index.html` |
| What each term means | `GLOSSARY.md` |
| The Word, Excel and PowerPoint templates | `data/templates/` (game), `data/templates/tool/` (tool) |
| The rules: which checkpoint is next, totals, dates in words | `crates/core/src/` |

To change a date or a point value, edit `data/schedule.json` and the matching test in `crates/core/tests/syllabus.rs` together. To preview the page as of another date, add `?on=2026-11-20T10:00` to the address.

## Build, test and serve

```bash
./scripts/setup.sh              # once: Python venv with Playwright and three browsers
cargo test --workspace          # the core, boundary, copy and syllabus tests, in seconds
./scripts/build.sh              # release WebAssembly build into dist/
python3 scripts/serve.py        # http://127.0.0.1:8000/
./scripts/check.sh              # everything, before publishing
python3 scripts/break_tests.py  # each row of tests/breaks.json must make its test fail
./scripts/publish.sh            # push dist/ to gh-pages, from a clean tree
```

This directory is a copy of the Builder template. Its rules are in `BUILDER-SPEC.md`, a coding agent's instructions are in `CLAUDE.md` and `AGENTS.md`, and the rules for every string are in `TONE.md`.
