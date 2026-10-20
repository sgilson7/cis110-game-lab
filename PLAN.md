# Plan

Stage 2, written by the agent from `BRIEF.md` after the person approved the defaults on 2026-10-19 (T1). The plan is approved in advance from that date, so it was written and built in one pass; the notebook (round 2) records what changed while building. `B2 Require step-by-step justification`.

| # | Decision | Reason | Test that shows it holds |
| --- | --- | --- | --- |
| 1 | Syllabus values go in `data/schedule.json`; reader text in `data/strings.json`; placeholders join them. | One place for each date and point value (GL-1). | `tests/syllabus.rs` (values typed from the syllabus by hand); `text.rs` refuses unknown placeholders. |
| 2 | Dates are whole days and minutes from 1970, in integer arithmetic. | Builder bans clocks and floats in the core (D2). | `dates.rs` tests: the syllabus Mondays are Mondays; impossible dates refused. |
| 3 | The page passes its local time in the course's zone, and `?on=` overrides it; the core picks the next checkpoint. | The page does no date arithmetic (GL-2); tests and the journal need a fixed date. | `api.rs` tests at 4:59 PM and 5:00 PM on a due day; browser interactions with `?on=`. |
| 4 | The view model is a list of typed sections; the page has one drawing function per kind. | The page draws, the core decides (R6). | boundary scan of `web/`; `act` is the one door for the setup tabs. |
| 5 | Every count, total and date label on the page is computed by the core. | Numbers are rules, and the page holds none (D3). | `stats_are_counted_by_the_core`, `file_cards_carry_their_draft_date_and_points`, `the_rubric_totals_one_hundred`. |
| 6 | System fonts; no web fonts. | Nothing from another origin (GL-6); the look the person asked for (T2). | CSP test; browser check watches requests. |
| 7 | Templates and screenshots are served from `data/`. | The boundary scan reads `web/` as text (R2.2). | build copies `data/`; the browser interaction finds the template link. |
| 8 | Publish to `sgilson7/cis110-game-lab` with `./scripts/publish.sh`, then run the browser check on the live address. | The grant in `BRIEF.md` (T4). | `BUILDER_ORIGIN=… tests/browser/check.py`. |

## Open questions for the person

The rows of `SECOND-ORDER.md` with status *person*, by number: R1.3 (answered by GL-Q6), R1.4 (answered by GL-Q1), R2.7, R2.8, R2.9, R2.10, R2.12.

## Status

- [x] Approved in advance on: 2026-10-19 (T1)
