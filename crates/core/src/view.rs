//! The assignment page as a view model. The strings file says what each
//! section says; the schedule says when and how much; this module combines
//! them for a given moment and returns everything the page draws. The page
//! knows how to draw each kind of section and nothing about what it means.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::dates::{civil_from_days, parse_day, weekday};
use crate::schedule::{Schedule, Status};
use crate::text::{fill, render, Seg, Vars};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Names {
    pub weekdays: Vec<String>,
    pub weekdays_short: Vec<String>,
    pub months: Vec<String>,
    pub months_short: Vec<String>,
    pub am: String,
    pub pm: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Link {
    pub href: String,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Intro {
    pub eyebrow: String,
    pub paragraphs: Vec<String>,
    pub note: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NextText {
    pub label: String,
    pub progress: String,
    pub due_today: String,
    pub due_tomorrow: String,
    pub due_in: String,
    pub finished_heading: String,
    pub finished_text: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CheckpointText {
    pub detail: String,
    pub todo: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stat {
    pub value: String,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StatusText {
    pub done: String,
    pub next: String,
    pub upcoming: String,
    pub graded: String,
    #[serde(rename = "break")]
    pub pause: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct ItemIn {
    pub id: String,
    pub title: String,
    pub text: String,
    pub file: String,
    pub tag: String,
    pub href: String,
    pub image: String,
    pub alt: String,
    pub owner: String,
    pub bullets: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct TabIn {
    pub id: String,
    pub label: String,
    pub lede: String,
    pub items: Vec<ItemIn>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Share {
    pub who: String,
    pub access: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FolderFile {
    pub name: String,
    pub kind: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Figure {
    pub label: String,
    pub path: String,
    pub folder: String,
    pub files: Vec<FolderFile>,
    pub sharing: Vec<Share>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct SectionIn {
    pub id: String,
    pub kind: String,
    pub eyebrow: String,
    pub heading: String,
    pub lede: Vec<String>,
    pub items: Vec<ItemIn>,
    pub labels: BTreeMap<String, String>,
    pub note: String,
    pub downloads: Vec<ItemIn>,
    pub tabs: Vec<TabIn>,
    pub figure: Option<Figure>,
    pub gallery_label: String,
    pub gallery: Vec<ItemIn>,
}

/// `data/strings.json`: every word on the page.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Strings {
    #[serde(default, rename = "_note")]
    pub note: String,
    pub title: String,
    pub course: String,
    pub names: Names,
    pub nav: Vec<Link>,
    pub intro: Intro,
    pub next: NextText,
    pub checkpoints: BTreeMap<String, CheckpointText>,
    pub stats: Vec<Stat>,
    pub status: StatusText,
    pub sections: Vec<SectionIn>,
    pub footer: Vec<String>,
}

/// The section kinds the page can draw.
pub const KINDS: &[&str] = &[
    "files", "schedule", "flow", "setup", "roles", "rubric", "rules", "faq",
];

impl Strings {
    pub fn parse(json: &str) -> Result<Strings, String> {
        let s: Strings =
            serde_json::from_str(json).map_err(|e| format!("The strings file could not be read: {e}"))?;
        if s.title.trim().is_empty() {
            return Err("The strings file has an empty \"title\".".into());
        }
        if s.names.weekdays.len() != 7 || s.names.weekdays_short.len() != 7 {
            return Err("The strings file needs seven weekday names, Monday first.".into());
        }
        if s.names.months.len() != 12 || s.names.months_short.len() != 12 {
            return Err("The strings file needs twelve month names, January first.".into());
        }
        for sec in &s.sections {
            if !KINDS.contains(&sec.kind.as_str()) {
                return Err(format!(
                    "Section \"{}\" has kind \"{}\", which the page cannot draw.",
                    sec.id, sec.kind
                ));
            }
        }
        Ok(s)
    }
}

// ----- What the page draws -------------------------------------------------

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct Item {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub id: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub title: Vec<Seg>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub text: Vec<Seg>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub detail: Vec<Seg>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub bullets: Vec<Vec<Seg>>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub file: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub tag: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub href: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub image: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub alt: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub owner: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub owner_label: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub date: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub status: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub status_label: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub value: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub max: String,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub graded: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Tab {
    pub id: String,
    pub label: String,
    pub lede: Vec<Seg>,
    pub items: Vec<Item>,
    pub selected: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Section {
    pub id: String,
    pub kind: String,
    pub eyebrow: String,
    pub heading: String,
    pub lede: Vec<Vec<Seg>>,
    pub items: Vec<Item>,
    pub labels: BTreeMap<String, String>,
    pub note: Vec<Seg>,
    pub downloads: Vec<Item>,
    pub tabs: Vec<Tab>,
    pub figure: Option<Figure>,
    pub gallery_label: String,
    pub gallery: Vec<Item>,
    pub total: String,
    /// Alternate sections sit on a tinted band.
    pub band: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Next {
    pub label: String,
    pub finished: bool,
    pub heading: String,
    pub when: String,
    pub what: Vec<Seg>,
    pub todo: Vec<Vec<Seg>>,
    pub progress: String,
    pub rail: Vec<Status>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Intro2 {
    pub eyebrow: Vec<Seg>,
    pub paragraphs: Vec<Vec<Seg>>,
    pub note: Vec<Seg>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Page {
    pub title: String,
    pub course: String,
    pub nav: Vec<Link>,
    pub intro: Intro2,
    pub next: Next,
    pub stats: Vec<Stat>,
    pub sections: Vec<Section>,
    pub footer: Vec<Vec<Seg>>,
}

// ----- Dates in words --------------------------------------------------------

/// "Monday, October 26".
pub fn long_date(names: &Names, day: i64) -> String {
    let c = civil_from_days(day);
    format!(
        "{}, {} {}",
        names.weekdays[weekday(day)],
        names.months[(c.month - 1) as usize],
        c.day
    )
}

/// "Mon, Oct 26".
pub fn short_date(names: &Names, day: i64) -> String {
    let c = civil_from_days(day);
    format!(
        "{}, {} {}",
        names.weekdays_short[weekday(day)],
        names.months_short[(c.month - 1) as usize],
        c.day
    )
}

/// "Nov 25–27", or "Nov 30–Dec 2" across a month.
pub fn short_range(names: &Names, from: i64, to: i64) -> String {
    let (a, b) = (civil_from_days(from), civil_from_days(to));
    let ma = &names.months_short[(a.month - 1) as usize];
    if a.month == b.month {
        format!("{ma} {}–{}", a.day, b.day)
    } else {
        format!(
            "{ma} {}–{} {}",
            a.day,
            names.months_short[(b.month - 1) as usize],
            b.day
        )
    }
}

/// "5:00 PM ET".
pub fn due_time(names: &Names, s: &Schedule) -> String {
    let h12 = (s.due_hour + 11) % 12 + 1;
    let half = if s.due_hour < 12 { &names.am } else { &names.pm };
    format!("{h12}:{:02} {half} {}", s.due_minute, s.zone)
}

/// The values every string may name.
pub fn vars(strings: &Strings, s: &Schedule) -> Result<Vars, String> {
    let n = &strings.names;
    let mut v = Vars::default();
    for (name, value) in [
        ("team_min", s.team_min),
        ("team_max", s.team_max),
        ("late_percent", s.late_percent),
        ("word_pages_min", s.word_pages_min),
        ("word_pages_max", s.word_pages_max),
        ("slides_min", s.slides_min),
        ("slides_max", s.slides_max),
        ("min_rows", s.min_rows),
        ("files", s.files.len() as i64),
        ("checkpoints", s.checkpoints.len() as i64),
        ("total", s.total()),
        ("rest", s.rest()),
    ] {
        v.set(name, value.to_string());
    }
    v.set("due_time", due_time(n, s));
    for r in &s.rubric {
        v.set(format!("points:{}", r.id), r.points.to_string());
    }
    for c in &s.checkpoints {
        let day = parse_day(&c.date)?;
        v.set(format!("long:{}", c.id), long_date(n, day));
        v.set(format!("short:{}", c.id), short_date(n, day));
    }
    v.set("links_by", long_date(n, parse_day(&s.prototype.links_by)?));
    Ok(v)
}

fn all(texts: &[String], v: &Vars) -> Result<Vec<Vec<Seg>>, String> {
    texts.iter().map(|t| render(t, v)).collect()
}

fn item(i: &ItemIn, v: &Vars) -> Result<Item, String> {
    Ok(Item {
        id: i.id.clone(),
        title: render(&i.title, v)?,
        text: render(&i.text, v)?,
        bullets: all(&i.bullets, v)?,
        file: i.file.clone(),
        tag: i.tag.clone(),
        href: i.href.clone(),
        image: i.image.clone(),
        alt: i.alt.clone(),
        owner: i.owner.clone(),
        ..Item::default()
    })
}

fn status_label(t: &StatusText, st: Status) -> &str {
    match st {
        Status::Done => &t.done,
        Status::Next => &t.next,
        Status::Upcoming => &t.upcoming,
    }
}

fn status_key(st: Status) -> &'static str {
    match st {
        Status::Done => "done",
        Status::Next => "next",
        Status::Upcoming => "upcoming",
    }
}

fn schedule_rows(strings: &Strings, s: &Schedule, now: i64, v: &Vars) -> Result<Vec<Item>, String> {
    let n = &strings.names;
    let statuses = s.statuses(now);
    let mut rows = Vec::new();
    let mut breaks: Vec<(i64, i64, &str)> = Vec::new();
    for b in &s.breaks {
        breaks.push((parse_day(&b.from)?, parse_day(&b.to)?, b.label.as_str()));
    }
    for (i, c) in s.checkpoints.iter().enumerate() {
        let day = parse_day(&c.date)?;
        for (from, to, label) in breaks.iter().filter(|(from, _, _)| *from < day) {
            let before = i == 0 || parse_day(&s.checkpoints[i - 1].date)? < *from;
            if before {
                rows.push(Item {
                    title: render(label, v)?,
                    text: render(&strings.status.pause, v)?,
                    date: short_range(n, *from, *to),
                    status: "break".into(),
                    ..Item::default()
                });
            }
        }
        let words = strings
            .checkpoints
            .get(&c.id)
            .ok_or_else(|| format!("The strings file has no text for checkpoint {}.", c.id))?;
        let st = statuses[i];
        rows.push(Item {
            id: c.id.clone(),
            tag: (i + 1).to_string(),
            title: render(&c.phase, v)?,
            text: render(&c.syllabus, v)?,
            detail: render(&words.detail, v)?,
            date: short_date(n, day),
            status: status_key(st).into(),
            graded: c.graded,
            status_label: if c.graded && st != Status::Next {
                fill(&strings.status.graded, v)?
            } else {
                status_label(&strings.status, st).to_string()
            },
            ..Item::default()
        });
    }
    Ok(rows)
}

fn section(
    index: usize,
    sec: &SectionIn,
    strings: &Strings,
    s: &Schedule,
    now: i64,
    tab: &str,
    v: &Vars,
) -> Result<Section, String> {
    let mut items = Vec::new();
    let mut total = String::new();
    match sec.kind.as_str() {
        "files" => {
            for i in &sec.items {
                let f = s
                    .files
                    .iter()
                    .find(|f| f.id == i.id)
                    .ok_or_else(|| format!("The file card \"{}\" has no entry in the schedule.", i.id))?;
                let mut out = item(i, v)?;
                out.date = short_date(&strings.names, parse_day(&s.checkpoint(&f.due)?.date)?);
                out.value = s.points(&f.rubric)?.to_string();
                items.push(out);
            }
        }
        "schedule" => items = schedule_rows(strings, s, now, v)?,
        "rubric" => {
            for r in &s.rubric {
                items.push(Item {
                    id: r.id.clone(),
                    title: render(&r.part, v)?,
                    value: r.points.to_string(),
                    max: s.largest().to_string(),
                    ..Item::default()
                });
            }
            total = s.total().to_string();
        }
        _ => {
            for i in &sec.items {
                let mut out = item(i, v)?;
                if !i.owner.is_empty() {
                    out.owner_label = sec.labels.get(&i.owner).cloned().ok_or_else(|| {
                        format!("Section \"{}\" has no label for owner \"{}\".", sec.id, i.owner)
                    })?;
                }
                items.push(out);
            }
        }
    }
    let selected = if sec.tabs.iter().any(|t| t.id == tab) {
        tab.to_string()
    } else {
        sec.tabs.first().map(|t| t.id.clone()).unwrap_or_default()
    };
    let mut tabs = Vec::new();
    for t in &sec.tabs {
        tabs.push(Tab {
            id: t.id.clone(),
            label: t.label.clone(),
            lede: render(&t.lede, v)?,
            items: t.items.iter().map(|i| item(i, v)).collect::<Result<_, _>>()?,
            selected: t.id == selected,
        });
    }
    Ok(Section {
        id: sec.id.clone(),
        kind: sec.kind.clone(),
        eyebrow: fill(&sec.eyebrow, v)?,
        heading: fill(&sec.heading, v)?,
        lede: all(&sec.lede, v)?,
        items,
        labels: sec.labels.clone(),
        note: render(&sec.note, v)?,
        downloads: sec
            .downloads
            .iter()
            .map(|i| item(i, v))
            .collect::<Result<_, _>>()?,
        tabs,
        figure: sec.figure.clone(),
        gallery_label: sec.gallery_label.clone(),
        gallery: sec.gallery.iter().map(|i| item(i, v)).collect::<Result<_, _>>()?,
        total,
        band: index % 2 == 1,
    })
}

fn next(strings: &Strings, s: &Schedule, now: i64, v: &Vars) -> Result<Next, String> {
    let t = &strings.next;
    let rail = s.statuses(now);
    let Some(i) = s.next(now) else {
        return Ok(Next {
            label: t.label.clone(),
            finished: true,
            heading: fill(&t.finished_heading, v)?,
            when: String::new(),
            what: render(&t.finished_text, v)?,
            todo: Vec::new(),
            progress: String::new(),
            rail,
        });
    };
    let c = &s.checkpoints[i];
    let days = s.days_until(c, now);
    let mut v = v.clone();
    v.set("days", days.to_string());
    v.set("n", (i + 1).to_string());
    v.set("count", s.checkpoints.len().to_string());
    let when = match days {
        0 => &t.due_today,
        1 => &t.due_tomorrow,
        _ => &t.due_in,
    };
    let words = strings
        .checkpoints
        .get(&c.id)
        .ok_or_else(|| format!("The strings file has no text for checkpoint {}.", c.id))?;
    Ok(Next {
        label: t.label.clone(),
        finished: false,
        heading: long_date(&strings.names, parse_day(&c.date)?),
        when: fill(when, &v)?,
        what: render(&format!("{}: {}", c.phase, c.syllabus), &v)?,
        todo: all(&words.todo, &v)?,
        progress: fill(&t.progress, &v)?,
        rail,
    })
}

/// The whole page at minute `now`, with tab `tab` open in the setup section.
pub fn page(strings: &Strings, s: &Schedule, now: i64, tab: &str) -> Result<Page, String> {
    let v = vars(strings, s)?;
    Ok(Page {
        title: strings.title.clone(),
        course: strings.course.clone(),
        nav: strings.nav.clone(),
        intro: Intro2 {
            eyebrow: render(&strings.intro.eyebrow, &v)?,
            paragraphs: all(&strings.intro.paragraphs, &v)?,
            note: render(&strings.intro.note, &v)?,
        },
        next: next(strings, s, now, &v)?,
        stats: strings
            .stats
            .iter()
            .map(|st| {
                Ok(Stat {
                    value: fill(&st.value, &v)?,
                    label: fill(&st.label, &v)?,
                })
            })
            .collect::<Result<_, String>>()?,
        sections: strings
            .sections
            .iter()
            .enumerate()
            .map(|(i, sec)| section(i, sec, strings, s, now, tab, &v))
            .collect::<Result<_, _>>()?,
        footer: all(&strings.footer, &v)?,
    })
}
