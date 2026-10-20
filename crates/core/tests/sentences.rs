//! TONE.md rule 2, held by a test (T5): every sentence a student reads is
//! under 25 words. The copy lint checks listed phrases; this checks length,
//! which a reviewer otherwise has to count. A placeholder counts as one word.

use std::fs;
use std::path::Path;

const LIMIT: usize = 25;

fn strings_in(value: &serde_json::Value, at: &str, out: &mut Vec<(String, String)>) {
    match value {
        serde_json::Value::String(s) => out.push((at.to_string(), s.clone())),
        serde_json::Value::Array(items) => {
            for (k, v) in items.iter().enumerate() {
                strings_in(v, &format!("{at}[{k}]"), out);
            }
        }
        serde_json::Value::Object(map) => {
            for (k, v) in map {
                if !k.starts_with('_') {
                    strings_in(v, &format!("{at}.{k}"), out);
                }
            }
        }
        _ => {}
    }
}

/// Sentences in `text` longer than the limit. A sentence ends at a word
/// ending in a full stop or a question mark.
fn long_sentences(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut sentence: Vec<&str> = Vec::new();
    for word in text.split_whitespace() {
        sentence.push(word);
        if word.ends_with('.') || word.ends_with('?') {
            if sentence.len() > LIMIT {
                out.push(sentence.join(" "));
            }
            sentence.clear();
        }
    }
    if sentence.len() > LIMIT {
        out.push(sentence.join(" "));
    }
    out
}

#[test]
fn every_student_facing_sentence_is_under_25_words() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let mut problems = Vec::new();
    for name in ["data/strings.json", "data/schedule.json"] {
        let text = fs::read_to_string(root.join(name)).expect("read data file");
        let value: serde_json::Value = serde_json::from_str(&text).expect("data file is JSON");
        let mut found = Vec::new();
        strings_in(&value, "", &mut found);
        for (at, s) in found {
            for long in long_sentences(&s) {
                problems.push(format!(
                    "{name}{at}: {} words: {long}",
                    long.split_whitespace().count()
                ));
            }
        }
    }
    assert!(
        problems.is_empty(),
        "sentences over {LIMIT} words:\n{}",
        problems.join("\n")
    );
}

#[test]
fn the_length_check_counts_sentences_separately() {
    assert!(long_sentences("One two three. Four five six.").is_empty());
    let long = "word ".repeat(LIMIT) + "end.";
    assert_eq!(long_sentences(&long).len(), 1);
}
