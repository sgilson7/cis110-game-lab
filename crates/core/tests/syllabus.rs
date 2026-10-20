//! GL-1: every date, time and point value on the page comes from the
//! syllabus. Each expected value below was typed by hand from
//! CIS110_LN182_Fall2026_Second8Weeks_Final_SimpleSyllabus.pdf (page 9, the
//! project; page 23, its calendar and rubric; page 8, the late penalty), and
//! never copied from data/schedule.json. When the syllabus changes, change
//! this file and the schedule together.

use std::fs;
use std::path::Path;

use tool_core::schedule::Schedule;

fn schedule() -> Schedule {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/schedule.json");
    Schedule::parse(&fs::read_to_string(path).expect("read data/schedule.json")).expect("schedule parses")
}

/// Page 23's dates, except C3 to C5, which T11 moved a week earlier (from 16,
/// 23 and 30 November) so that every file is in before Thanksgiving.
#[test]
fn the_seven_project_deadlines_match_page_23_with_t11() {
    let dates: Vec<String> = schedule().checkpoints.iter().map(|c| c.date.clone()).collect();
    assert_eq!(
        dates,
        vec![
            "2026-10-26",
            "2026-11-02",
            "2026-11-09",
            "2026-11-16",
            "2026-11-23",
            "2026-12-07",
            "2026-12-10"
        ]
    );
}

#[test]
fn deadlines_are_at_five_pm_eastern() {
    let s = schedule();
    assert_eq!((s.due_hour, s.due_minute, s.zone.as_str()), (17, 0, "ET"));
}

#[test]
fn only_the_december_10_submission_is_graded() {
    let graded: Vec<String> = schedule()
        .checkpoints
        .iter()
        .filter(|c| c.graded)
        .map(|c| c.date.clone())
        .collect();
    assert_eq!(graded, vec!["2026-12-10"]);
}

#[test]
fn the_rubric_matches_page_23() {
    let parts: Vec<(String, i64)> = schedule()
        .rubric
        .iter()
        .map(|r| (r.part.clone(), r.points))
        .collect();
    assert_eq!(
        parts,
        vec![
            ("Game concept and playable scope".to_string(), 20),
            ("Word game design document".to_string(), 25),
            ("PowerPoint presentation".to_string(), 20),
            ("Excel structured game information".to_string(), 20),
            ("Shared-folder organization and collaboration".to_string(), 15),
        ]
    );
}

#[test]
fn the_limits_match_page_9_and_the_late_penalty_page_8() {
    let s = schedule();
    assert_eq!((s.team_min, s.team_max), (3, 5));
    assert_eq!((s.word_pages_min, s.word_pages_max), (2, 4));
    assert_eq!((s.slides_min, s.slides_max), (5, 8));
    assert_eq!(s.min_rows, 5);
    assert_eq!(s.late_percent, 20);
}

#[test]
fn thanksgiving_break_matches_page_23() {
    let b = &schedule().breaks[0];
    assert_eq!((b.from.as_str(), b.to.as_str()), ("2026-11-25", "2026-11-27"));
}
