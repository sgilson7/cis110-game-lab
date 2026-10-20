//! The project's calendar and rubric, read from `data/schedule.json`, which
//! copies the syllabus (GL-1). This module decides which checkpoint is next
//! for a given moment, what state each checkpoint is in, and the totals the
//! page shows; the page does no date arithmetic (GL-2).

use serde::{Deserialize, Serialize};

use crate::dates::{parse_day, DAY_MINUTES};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CheckpointIn {
    pub id: String,
    pub date: String,
    pub phase: String,
    pub syllabus: String,
    #[serde(default)]
    pub graded: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BreakIn {
    pub from: String,
    pub to: String,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RubricIn {
    pub id: String,
    pub part: String,
    pub points: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FileIn {
    pub id: String,
    pub due: String,
    pub rubric: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PrototypeIn {
    pub built_from: String,
    pub links_by: String,
    pub requests_by: String,
}

/// `data/schedule.json` as written.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Schedule {
    #[serde(default, rename = "_source")]
    pub source: String,
    pub zone: String,
    pub due_hour: i64,
    pub due_minute: i64,
    pub team_min: i64,
    pub team_max: i64,
    pub late_percent: i64,
    pub word_pages_min: i64,
    pub word_pages_max: i64,
    pub slides_min: i64,
    pub slides_max: i64,
    pub min_rows: i64,
    pub checkpoints: Vec<CheckpointIn>,
    pub breaks: Vec<BreakIn>,
    pub rubric: Vec<RubricIn>,
    pub files: Vec<FileIn>,
    pub prototype: PrototypeIn,
}

/// Where a checkpoint stands at a given moment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Done,
    Next,
    Upcoming,
}

impl Schedule {
    /// Reads and checks the schedule. A schedule the page could not explain
    /// consistently is refused with the reason.
    pub fn parse(json: &str) -> Result<Schedule, String> {
        let s: Schedule =
            serde_json::from_str(json).map_err(|e| format!("The schedule file could not be read: {e}"))?;
        s.check()?;
        Ok(s)
    }

    fn check(&self) -> Result<(), String> {
        if self.checkpoints.is_empty() {
            return Err("The schedule has no checkpoints.".into());
        }
        if !(0..24).contains(&self.due_hour) || !(0..60).contains(&self.due_minute) {
            return Err("The schedule's due time is not a time of day.".into());
        }
        if self.team_min < 1 || self.team_min > self.team_max {
            return Err("The schedule's team size runs backwards.".into());
        }
        let mut last = i64::MIN;
        for c in &self.checkpoints {
            let day = parse_day(&c.date)?;
            if day <= last {
                return Err(format!("Checkpoint {} is not after the one before it.", c.id));
            }
            last = day;
        }
        let graded: Vec<&CheckpointIn> = self.checkpoints.iter().filter(|c| c.graded).collect();
        if graded.len() != 1 || !self.checkpoints.last().is_some_and(|c| c.graded) {
            return Err("The schedule needs exactly one graded checkpoint, and it must be the last.".into());
        }
        for b in &self.breaks {
            if parse_day(&b.from)? > parse_day(&b.to)? {
                return Err(format!("The break \"{}\" ends before it starts.", b.label));
            }
        }
        for (i, r) in self.rubric.iter().enumerate() {
            if r.points <= 0 {
                return Err(format!("The rubric part \"{}\" has no points.", r.part));
            }
            if self.rubric[..i].iter().any(|o| o.id == r.id) {
                return Err(format!("The rubric id \"{}\" is used twice.", r.id));
            }
        }
        for f in &self.files {
            self.checkpoint(&f.due)?;
            self.points(&f.rubric)?;
        }
        self.checkpoint(&self.prototype.built_from)?;
        self.checkpoint(&self.prototype.requests_by)?;
        parse_day(&self.prototype.links_by)?;
        Ok(())
    }

    /// The checkpoint with this id.
    pub fn checkpoint(&self, id: &str) -> Result<&CheckpointIn, String> {
        self.checkpoints
            .iter()
            .find(|c| c.id == id)
            .ok_or_else(|| format!("There is no checkpoint \"{id}\"."))
    }

    /// The points of the rubric part with this id.
    pub fn points(&self, id: &str) -> Result<i64, String> {
        self.rubric
            .iter()
            .find(|r| r.id == id)
            .map(|r| r.points)
            .ok_or_else(|| format!("There is no rubric part \"{id}\"."))
    }

    /// All the rubric's points.
    pub fn total(&self) -> i64 {
        self.rubric.iter().map(|r| r.points).sum()
    }

    /// The points the rubric gives to parts that are not one of the files.
    pub fn rest(&self) -> i64 {
        self.rubric
            .iter()
            .filter(|r| !self.files.iter().any(|f| f.rubric == r.id))
            .map(|r| r.points)
            .sum()
    }

    /// The largest single rubric part, the full width of a rubric bar.
    pub fn largest(&self) -> i64 {
        self.rubric.iter().map(|r| r.points).max().unwrap_or(0)
    }

    /// The minute a checkpoint closes: its date at the due time.
    pub fn closes(&self, c: &CheckpointIn) -> i64 {
        parse_day(&c.date).unwrap_or(0) * DAY_MINUTES + self.due_hour * 60 + self.due_minute
    }

    /// The index of the first checkpoint still open at `now`, or None once the
    /// last has closed. A checkpoint is open until its due minute; at that
    /// minute it has passed.
    pub fn next(&self, now: i64) -> Option<usize> {
        self.checkpoints.iter().position(|c| now < self.closes(c))
    }

    /// Each checkpoint's status at `now`.
    pub fn statuses(&self, now: i64) -> Vec<Status> {
        let next = self.next(now);
        (0..self.checkpoints.len())
            .map(|i| match next {
                Some(n) if i == n => Status::Next,
                Some(n) if i > n => Status::Upcoming,
                _ => Status::Done,
            })
            .collect()
    }

    /// Whole days from `now`'s date to the checkpoint's date: 0 on the day.
    pub fn days_until(&self, c: &CheckpointIn, now: i64) -> i64 {
        parse_day(&c.date).unwrap_or(0) - now.div_euclid(DAY_MINUTES)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dates::parse_moment;

    const SAMPLE: &str = r#"{
      "zone": "ET", "due_hour": 17, "due_minute": 0, "team_min": 3, "team_max": 5,
      "late_percent": 20, "word_pages_min": 2, "word_pages_max": 4, "slides_min": 5, "slides_max": 8, "min_rows": 5,
      "checkpoints": [
        {"id": "A", "date": "2026-10-26", "phase": "a", "syllabus": "a"},
        {"id": "B", "date": "2026-11-02", "phase": "b", "syllabus": "b"},
        {"id": "Z", "date": "2026-12-10", "phase": "z", "syllabus": "z", "graded": true}
      ],
      "breaks": [{"from": "2026-11-25", "to": "2026-11-27", "label": "break"}],
      "rubric": [{"id": "w", "part": "w", "points": 25}, {"id": "c", "part": "c", "points": 15}],
      "files": [{"id": "word", "due": "B", "rubric": "w"}],
      "prototype": {"built_from": "B", "links_by": "2026-12-03", "requests_by": "Z"}
    }"#;

    fn sample() -> Schedule {
        Schedule::parse(SAMPLE).unwrap()
    }

    fn at(text: &str) -> i64 {
        parse_moment(text).unwrap()
    }

    #[test]
    fn the_next_checkpoint_is_the_first_still_open() {
        let s = sample();
        assert_eq!(s.next(at("2026-10-19 10:00")), Some(0));
        assert_eq!(s.next(at("2026-10-27 09:00")), Some(1));
    }

    #[test]
    fn a_checkpoint_stays_open_until_five_pm_on_its_day() {
        let s = sample();
        assert_eq!(s.next(at("2026-10-26 16:59")), Some(0));
        assert_eq!(s.next(at("2026-10-26 17:00")), Some(1));
    }

    #[test]
    fn after_the_last_deadline_nothing_is_next() {
        let s = sample();
        assert_eq!(s.next(at("2026-12-10 17:00")), None);
        assert_eq!(s.statuses(at("2026-12-11 08:00")), vec![Status::Done; 3]);
    }

    #[test]
    fn statuses_split_into_done_next_and_upcoming() {
        let s = sample();
        assert_eq!(
            s.statuses(at("2026-10-30 12:00")),
            vec![Status::Done, Status::Next, Status::Upcoming]
        );
    }

    #[test]
    fn days_until_counts_calendar_days_not_hours() {
        let s = sample();
        let a = s.checkpoint("A").unwrap();
        assert_eq!(s.days_until(a, at("2026-10-19 23:59")), 7);
        assert_eq!(s.days_until(a, at("2026-10-26 08:00")), 0);
    }

    #[test]
    fn totals_come_from_the_rubric() {
        let s = sample();
        assert_eq!(s.total(), 40);
        assert_eq!(s.rest(), 15);
        assert_eq!(s.largest(), 25);
    }

    #[test]
    fn checkpoints_out_of_order_are_refused() {
        let bad = SAMPLE.replace("\"date\": \"2026-11-02\"", "\"date\": \"2026-10-26\"");
        assert_eq!(
            Schedule::parse(&bad).unwrap_err(),
            "Checkpoint B is not after the one before it."
        );
    }

    #[test]
    fn the_graded_checkpoint_must_be_the_last_and_only_one() {
        let bad = SAMPLE.replace("\"syllabus\": \"z\", \"graded\": true", "\"syllabus\": \"z\"");
        assert_eq!(
            Schedule::parse(&bad).unwrap_err(),
            "The schedule needs exactly one graded checkpoint, and it must be the last."
        );
    }

    #[test]
    fn a_file_pointing_at_a_missing_checkpoint_is_refused() {
        let bad = SAMPLE.replace("\"due\": \"B\"", "\"due\": \"Q\"");
        assert_eq!(
            Schedule::parse(&bad).unwrap_err(),
            "There is no checkpoint \"Q\"."
        );
    }

    #[test]
    fn a_break_that_ends_before_it_starts_is_refused() {
        let bad = SAMPLE.replace("\"to\": \"2026-11-27\"", "\"to\": \"2026-11-20\"");
        assert_eq!(
            Schedule::parse(&bad).unwrap_err(),
            "The break \"break\" ends before it starts."
        );
    }
}
