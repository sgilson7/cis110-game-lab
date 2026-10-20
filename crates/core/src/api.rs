//! The door into the core. Every function takes strings and returns a JSON
//! string: `{"ok": ...}` on success or `{"error": "..."}` with a message the
//! page can show as it is. The shim forwards these calls and nothing else.
//!
//! The core describes the page as a view model (`Ui`). Settings come back
//! through one function, `act`, with a dotted name such as `setup.show`, so a
//! new option needs a new `match` arm here and no new shim function. (After
//! Peter's Mandible.)

use serde::{Deserialize, Serialize};

use crate::dates::parse_moment;
use crate::glossary;
use crate::schedule::Schedule;
use crate::text::fill;
use crate::view::{page, vars, Page, Strings};

fn ok<T: Serialize>(value: &T) -> String {
    serde_json::json!({ "ok": value }).to_string()
}

fn error(message: String) -> String {
    serde_json::json!({ "error": message }).to_string()
}

/// What the page hands back on the next call. The page stores it and does
/// not read it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    pub strings: Strings,
    pub schedule: Schedule,
    pub now: i64,
    pub tab: String,
    /// The printable edition: the page without a date (`?edition=print`).
    #[serde(default)]
    pub printable: bool,
}

/// A term and its definition, for the page's hover hints.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Hint {
    pub term: String,
    pub definition: String,
}

/// Everything the page draws.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Ui {
    pub screen: String,
    pub page: Page,
    pub hints: Vec<Hint>,
    pub state: State,
}

fn hints_from(glossary_md: &str) -> Result<Vec<Hint>, String> {
    let entries = glossary::parse(glossary_md).map_err(|e| e.message())?;
    Ok(entries
        .into_iter()
        .map(|e| Hint {
            term: e.term,
            definition: e.definition,
        })
        .collect())
}

fn ui(state: State, hints: Vec<Hint>) -> Result<Ui, String> {
    let page = page(
        &state.strings,
        &state.schedule,
        state.now,
        &state.tab,
        state.printable,
    )?;
    let v = vars(&state.strings, &state.schedule)?;
    let hints = hints
        .into_iter()
        .map(|h| {
            Ok(Hint {
                definition: fill(&h.definition, &v)?,
                term: h.term,
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(Ui {
        screen: "home".into(),
        page,
        hints,
        state,
    })
}

/// The moment the page is drawn for. `on` is the `?on=` value from the
/// address, which lets a reader (or a test) see the page as of another date;
/// when it is empty or cannot be read, the page clock is used.
pub fn moment(clock: &str, on: &str) -> Result<i64, String> {
    match parse_moment(on) {
        Ok(m) if !on.trim().is_empty() => Ok(m),
        _ => parse_moment(clock).map_err(|e| format!("The page clock could not be read: {e}")),
    }
}

/// Which edition the address asks for: `""` for the web page, `"print"` for
/// the edition the PDF and the single-file HTML are made from.
pub fn edition(name: &str) -> Result<bool, String> {
    match name {
        "" => Ok(false),
        "print" => Ok(true),
        other => Err(format!("There is no edition \"{other}\".")),
    }
}

/// Checks the strings, schedule and glossary and returns the page as of
/// `clock` (the page's local time in the course's time zone), or as of `on`
/// when the address names a date, in the edition the address names.
pub fn start(
    strings_json: &str,
    schedule_json: &str,
    glossary_md: &str,
    clock: &str,
    on: &str,
    edition_name: &str,
) -> String {
    let run = || -> Result<Ui, String> {
        let strings = Strings::parse(strings_json)?;
        let schedule = Schedule::parse(schedule_json)?;
        let hints = hints_from(glossary_md)?;
        let now = moment(clock, on)?;
        let printable = edition(edition_name)?;
        ui(
            State {
                strings,
                schedule,
                now,
                tab: String::new(),
                printable,
            },
            hints,
        )
    };
    match run() {
        Ok(u) => ok(&u),
        Err(e) => error(e),
    }
}

/// Applies one named action to the state the page holds. `setup.show` opens
/// one tab of the setup section.
pub fn act(state_json: &str, glossary_md: &str, name: &str, value: &str) -> String {
    let run = || -> Result<Ui, String> {
        let mut state: State =
            serde_json::from_str(state_json).map_err(|e| format!("The state could not be read: {e}"))?;
        let hints = hints_from(glossary_md)?;
        match name {
            "setup.show" => {
                let known = state
                    .strings
                    .sections
                    .iter()
                    .any(|s| s.tabs.iter().any(|t| t.id == value));
                if !known {
                    return Err(format!("There is no setup tab \"{value}\"."));
                }
                state.tab = value.to_string();
            }
            other => return Err(format!("Unknown action \"{other}\".")),
        }
        ui(state, hints)
    };
    match run() {
        Ok(u) => ok(&u),
        Err(e) => error(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const STRINGS: &str = include_str!("../../../data/strings.json");
    const SCHEDULE: &str = include_str!("../../../data/schedule.json");
    const GLOSSARY: &str = "### Checkpoint\n\nA date when part of the project is due.\n";

    fn json(text: &str) -> serde_json::Value {
        serde_json::from_str(text).unwrap()
    }

    fn at(clock: &str) -> serde_json::Value {
        json(&start(STRINGS, SCHEDULE, GLOSSARY, clock, "", ""))
    }

    fn section<'a>(v: &'a serde_json::Value, id: &str) -> &'a serde_json::Value {
        v["ok"]["page"]["sections"]
            .as_array()
            .unwrap()
            .iter()
            .find(|s| s["id"] == id)
            .unwrap()
    }

    fn words(segs: &serde_json::Value) -> String {
        segs.as_array()
            .unwrap()
            .iter()
            .map(|s| s["t"].as_str().unwrap())
            .collect()
    }

    #[test]
    fn on_census_day_the_next_checkpoint_is_team_up_in_seven_days() {
        let v = at("2026-10-19 10:00:00");
        let next = &v["ok"]["page"]["next"];
        assert_eq!(next["heading"], "Monday, October 26");
        assert_eq!(next["when"], "Due in 7 days, at 5:00 PM ET");
        assert_eq!(next["progress"], "Checkpoint 1 of 7");
        assert_eq!(
            words(&next["what"]),
            "Join a team in Blackboard and introduce yourself in your team discussion."
        );
        assert_eq!(next["finished"], false);
    }

    #[test]
    fn the_day_before_and_the_day_of_read_tomorrow_and_today() {
        assert_eq!(
            at("2026-11-15 20:00:00")["ok"]["page"]["next"]["when"],
            "Due tomorrow at 5:00 PM ET"
        );
        assert_eq!(
            at("2026-11-16 09:00:00")["ok"]["page"]["next"]["when"],
            "Due today at 5:00 PM ET"
        );
    }

    #[test]
    fn at_five_pm_the_card_moves_to_the_next_checkpoint() {
        let v = at("2026-11-16 17:00:00");
        assert_eq!(v["ok"]["page"]["next"]["heading"], "Monday, November 23");
    }

    #[test]
    fn after_the_final_deadline_the_card_says_the_project_is_complete() {
        let next = &at("2026-12-10 17:01:00")["ok"]["page"]["next"];
        assert_eq!(next["finished"], true);
        assert_eq!(next["heading"], "The project is complete");
        assert!(words(&next["what"]).contains("Thursday, December 10"));
    }

    #[test]
    fn the_address_override_wins_over_the_clock_and_a_bad_one_is_ignored() {
        let v = json(&start(
            STRINGS,
            SCHEDULE,
            GLOSSARY,
            "2026-10-19 10:00:00",
            "2026-11-20T10:00",
            "",
        ));
        assert_eq!(v["ok"]["page"]["next"]["heading"], "Monday, November 23");
        let v = json(&start(
            STRINGS,
            SCHEDULE,
            GLOSSARY,
            "2026-10-19 10:00:00",
            "next week",
            "",
        ));
        assert_eq!(v["ok"]["page"]["next"]["heading"], "Monday, October 26");
    }

    #[test]
    fn stats_are_counted_by_the_core() {
        let stats = &at("2026-10-19 10:00:00")["ok"]["page"]["stats"];
        let values: Vec<&str> = stats
            .as_array()
            .unwrap()
            .iter()
            .map(|s| s["value"].as_str().unwrap())
            .collect();
        assert_eq!(values, vec!["3–5", "3", "7", "100"]);
    }

    #[test]
    fn the_schedule_has_seven_checkpoints_and_the_break_in_date_order() {
        let v = at("2026-11-20 10:00:00");
        let rows = section(&v, "schedule")["items"].as_array().unwrap().clone();
        let dates: Vec<&str> = rows.iter().map(|r| r["date"].as_str().unwrap()).collect();
        assert_eq!(
            dates,
            vec![
                "Mon, Oct 26",
                "Mon, Nov 2",
                "Mon, Nov 9",
                "Mon, Nov 16",
                "Mon, Nov 23",
                "Nov 25–27",
                "Mon, Dec 7",
                "Thu, Dec 10"
            ]
        );
        let status: Vec<&str> = rows.iter().map(|r| r["status"].as_str().unwrap()).collect();
        assert_eq!(
            status,
            vec!["done", "done", "done", "done", "next", "break", "upcoming", "upcoming"]
        );
        assert_eq!(rows[0]["status_label"], "Closed");
        assert_eq!(rows[7]["status_label"], "Graded · 100 points");
    }

    #[test]
    fn file_cards_carry_their_draft_date_and_points() {
        let v = at("2026-10-19 10:00:00");
        let cards = section(&v, "submit")["items"].as_array().unwrap().clone();
        let got: Vec<(&str, &str, &str)> = cards
            .iter()
            .map(|c| {
                (
                    c["file"].as_str().unwrap(),
                    c["date"].as_str().unwrap(),
                    c["value"].as_str().unwrap(),
                )
            })
            .collect();
        assert_eq!(
            got,
            vec![
                ("GameDesign.docx", "Mon, Nov 9", "25"),
                ("GameData.xlsx", "Mon, Nov 16", "20"),
                ("GamePitch.pptx", "Mon, Nov 23", "20")
            ]
        );
        assert_eq!(
            words(&section(&v, "submit")["note"]),
            "The remaining 35 points cover your game concept and scope (20 points) and how your team organized and shared the folder (15 points)."
        );
    }

    #[test]
    fn the_rubric_totals_one_hundred() {
        let v = at("2026-10-19 10:00:00");
        assert_eq!(section(&v, "grading")["total"], "100");
    }

    #[test]
    fn the_setup_section_opens_on_microsoft_365_and_switches_through_the_one_door() {
        let first = at("2026-10-19 10:00:00");
        let tabs = &section(&first, "setup")["tabs"];
        assert_eq!(tabs[0]["id"], "m365");
        assert_eq!(tabs[0]["selected"], true);
        let state = first["ok"]["state"].to_string();
        let v = json(&act(&state, GLOSSARY, "setup.show", "google"));
        let tabs = &section(&v, "setup")["tabs"];
        assert_eq!(tabs[0]["selected"], false);
        assert_eq!(tabs[1]["selected"], true);
    }

    #[test]
    fn act_refuses_an_unknown_action_and_tab_by_name() {
        let state = at("2026-10-19 10:00:00")["ok"]["state"].to_string();
        assert_eq!(
            json(&act(&state, GLOSSARY, "screen.jump", "x"))["error"],
            "Unknown action \"screen.jump\"."
        );
        assert_eq!(
            json(&act(&state, GLOSSARY, "setup.show", "dropbox"))["error"],
            "There is no setup tab \"dropbox\"."
        );
    }

    #[test]
    fn hover_hints_have_their_numbers_filled_by_the_core() {
        let v = json(&start(
            STRINGS,
            SCHEDULE,
            "### Checkpoint\n\nDue at {due_time}.\n",
            "2026-10-19 10:00:00",
            "",
            "",
        ));
        assert_eq!(v["ok"]["hints"][0]["definition"], "Due at 5:00 PM ET.");
    }

    #[test]
    fn a_bad_clock_is_reported() {
        let v = json(&start(STRINGS, SCHEDULE, GLOSSARY, "soon", "", ""));
        assert!(v["error"]
            .as_str()
            .unwrap()
            .starts_with("The page clock could not be read"));
    }

    fn print_edition() -> serde_json::Value {
        json(&start(
            STRINGS,
            SCHEDULE,
            GLOSSARY,
            "2026-11-20 10:00:00",
            "",
            "print",
        ))
    }

    #[test]
    fn the_print_edition_has_no_next_checkpoint_and_no_dated_status() {
        let v = print_edition();
        assert_eq!(v["ok"]["page"]["edition"], "print");
        assert!(v["ok"]["page"]["next"].is_null());
        let rows = section(&v, "schedule")["items"].as_array().unwrap();
        // On 20 November the web page marks C1 to C3 Closed and C4 Next; the
        // printable edition marks none, and only the graded row keeps a label.
        let labels: Vec<&str> = rows
            .iter()
            .map(|r| r["status_label"].as_str().unwrap_or(""))
            .collect();
        assert_eq!(labels, vec!["", "", "", "", "", "", "", "Graded · 100 points"]);
        assert!(rows
            .iter()
            .all(|r| r["status"] != "done" && r["status"] != "next"));
    }

    #[test]
    fn the_print_edition_opens_every_setup_tab_and_faq_answer() {
        let v = print_edition();
        let tabs = section(&v, "setup")["tabs"].as_array().unwrap();
        assert_eq!(tabs.len(), 2);
        assert!(tabs.iter().all(|t| t["selected"] == true));
        assert_eq!(tabs[0]["heading"], "Microsoft 365");
        assert_eq!(tabs[1]["heading"], "Google Drive (backup option)");
        let faq = section(&v, "faq")["items"].as_array().unwrap();
        assert_eq!(faq.len(), 9);
        assert!(faq.iter().all(|q| q["open"] == true));
    }

    #[test]
    fn the_web_page_links_the_printable_files_and_the_print_edition_names_the_site() {
        let web = at("2026-11-20 10:00:00");
        let links = web["ok"]["page"]["intro"]["links"].as_array().unwrap();
        assert_eq!(links.len(), 2);
        assert_eq!(links[0]["href"], "cis110-game-project.pdf");
        assert_eq!(links[1]["href"], "cis110-game-project.html");
        assert!(web["ok"]["page"]["intro"]["edition_note"]
            .as_array()
            .unwrap()
            .is_empty());
        assert!(section(&web, "faq")["items"][0]["open"].is_null());
        assert_eq!(section(&web, "setup")["tabs"][1]["selected"], false);
        let printable = print_edition();
        assert!(printable["ok"]["page"]["intro"]["links"]
            .as_array()
            .unwrap()
            .is_empty());
        assert!(words(&printable["ok"]["page"]["intro"]["edition_note"])
            .contains("https://sgilson7.github.io/cis110-game-lab/"));
    }

    #[test]
    fn the_print_edition_survives_a_round_trip_through_act() {
        let state = print_edition()["ok"]["state"].to_string();
        let v = json(&act(&state, GLOSSARY, "setup.show", "google"));
        assert!(v["ok"]["page"]["next"].is_null());
        assert!(section(&v, "setup")["tabs"]
            .as_array()
            .unwrap()
            .iter()
            .all(|t| t["selected"] == true));
    }

    #[test]
    fn an_unknown_edition_is_refused() {
        let v = json(&start(
            STRINGS,
            SCHEDULE,
            GLOSSARY,
            "2026-11-20 10:00:00",
            "",
            "poster",
        ));
        assert_eq!(v["error"], "There is no edition \"poster\".");
    }
}
