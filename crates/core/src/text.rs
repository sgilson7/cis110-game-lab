//! Reader-visible text: filling `{placeholders}` with values the core
//! decides, and splitting the two inline marks the strings file uses into
//! segments the page draws without reading them. `**Share**` marks an
//! interface name (drawn bold), and `` `GameData.xlsx` `` marks a file or
//! folder name (drawn as code), as Google's developer documentation style
//! asks.

use serde::Serialize;

/// The values a string may name, as (name, value) pairs.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Vars(pub Vec<(String, String)>);

impl Vars {
    pub fn set(&mut self, name: impl Into<String>, value: impl Into<String>) {
        self.0.push((name.into(), value.into()));
    }

    fn get(&self, name: &str) -> Option<&str> {
        self.0
            .iter()
            .rev()
            .find(|(n, _)| n == name)
            .map(|(_, v)| v.as_str())
    }
}

/// Replaces each `{name}` in `text` with its value. A name the core does not
/// fill is an error, so a typo in the strings file cannot reach a student as
/// a literal brace.
pub fn fill(text: &str, vars: &Vars) -> Result<String, String> {
    let mut out = String::new();
    let mut rest = text;
    while let Some(open) = rest.find('{') {
        out.push_str(&rest[..open]);
        let after = &rest[open + 1..];
        let close = after
            .find('}')
            .ok_or_else(|| format!("The text \"{text}\" opens a {{placeholder}} it never closes."))?;
        let name = &after[..close];
        let value = vars
            .get(name)
            .ok_or_else(|| format!("The text \"{text}\" names {{{name}}}, which the core does not fill."))?;
        out.push_str(value);
        rest = &after[close + 1..];
    }
    out.push_str(rest);
    Ok(out)
}

/// One run of text and how to draw it: plain, `b` for an interface name,
/// `c` for a file or folder name.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Seg {
    pub t: String,
    #[serde(skip_serializing_if = "str::is_empty")]
    pub s: &'static str,
}

fn seg(t: &str, s: &'static str) -> Seg {
    Seg { t: t.to_string(), s }
}

/// Splits text into segments at `**…**` and `` `…` ``. An unclosed mark is an
/// error.
pub fn inline(text: &str) -> Result<Vec<Seg>, String> {
    let mut out = Vec::new();
    let mut rest = text;
    loop {
        let bold = rest.find("**");
        let code = rest.find('`');
        let (at, mark, len, style) = match (bold, code) {
            (None, None) => break,
            (Some(b), Some(c)) if c < b => (c, "`", 1, "c"),
            (Some(b), _) => (b, "**", 2, "b"),
            (None, Some(c)) => (c, "`", 1, "c"),
        };
        if at > 0 {
            out.push(seg(&rest[..at], ""));
        }
        let inner = &rest[at + len..];
        let end = inner
            .find(mark)
            .ok_or_else(|| format!("The text \"{text}\" opens {mark} and never closes it."))?;
        out.push(seg(&inner[..end], style));
        rest = &inner[end + len..];
    }
    if !rest.is_empty() {
        out.push(seg(rest, ""));
    }
    Ok(out)
}

/// Fills and then splits: the usual path for a string from the data files.
pub fn render(text: &str, vars: &Vars) -> Result<Vec<Seg>, String> {
    inline(&fill(text, vars)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vars() -> Vars {
        let mut v = Vars::default();
        v.set("total", "100");
        v.set("points:collab", "15");
        v
    }

    #[test]
    fn placeholders_are_filled_from_the_core() {
        assert_eq!(
            fill("Worth {total} points; {points:collab} for teamwork.", &vars()).unwrap(),
            "Worth 100 points; 15 for teamwork."
        );
    }

    #[test]
    fn an_unknown_placeholder_is_refused_by_name() {
        assert_eq!(
            fill("Due {when}.", &vars()).unwrap_err(),
            "The text \"Due {when}.\" names {when}, which the core does not fill."
        );
    }

    #[test]
    fn an_unclosed_placeholder_is_refused() {
        assert!(fill("Due {total", &vars()).is_err());
    }

    #[test]
    fn marks_become_segments() {
        assert_eq!(
            inline("Select **Share**, then open `GameData.xlsx`.").unwrap(),
            vec![
                seg("Select ", ""),
                seg("Share", "b"),
                seg(", then open ", ""),
                seg("GameData.xlsx", "c"),
                seg(".", "")
            ]
        );
    }

    #[test]
    fn plain_text_is_one_segment() {
        assert_eq!(inline("No marks here.").unwrap(), vec![seg("No marks here.", "")]);
    }

    #[test]
    fn an_unclosed_mark_is_refused() {
        assert!(inline("Select **Share.").is_err());
        assert!(inline("Open `GameData.xlsx.").is_err());
    }
}
