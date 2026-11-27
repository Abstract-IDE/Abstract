//! fzf-style fuzzy matching.
//!
//! A query is split on whitespace into terms that must all match (AND):
//!
//! | term      | meaning                         |
//! | --------- | ------------------------------- |
//! | `abc`     | fuzzy: `a`, `b`, `c` in order   |
//! | `'abc`    | exact substring                 |
//! | `^abc`    | starts with                     |
//! | `abc$`    | ends with                       |
//! | `^abc$`   | equals                          |
//! | `!abc`    | does not contain (also `!^`, `!$`) |
//!
//! Each term is smart-case: case-insensitive unless it has an uppercase letter.
//! Scoring follows fzf's v1 algorithm: points per matched char, bonuses for
//! matches at word boundaries (after `/`, `_`, `-`, `.`, a space, a camelCase
//! hump) and for consecutive runs, penalties for gaps.

const SCORE_MATCH: i64 = 16;
const GAP_START: i64 = -3;
const GAP_EXTENSION: i64 = -1;
const BONUS_BOUNDARY: i64 = SCORE_MATCH / 2;
const BONUS_BOUNDARY_WHITE: i64 = BONUS_BOUNDARY + 2;
const BONUS_BOUNDARY_DELIMITER: i64 = BONUS_BOUNDARY + 1;
const BONUS_NON_WORD: i64 = SCORE_MATCH / 2;
const BONUS_CAMEL: i64 = BONUS_BOUNDARY + GAP_EXTENSION;
const BONUS_CONSECUTIVE: i64 = -(GAP_START + GAP_EXTENSION);
const BONUS_FIRST_CHAR_MULTIPLIER: i64 = 2;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Class {
    White,
    Delimiter,
    NonWord,
    Lower,
    Upper,
    Letter,
    Number,
}

fn class_of(c: char) -> Class {
    if c.is_ascii_lowercase() {
        Class::Lower
    } else if c.is_ascii_uppercase() {
        Class::Upper
    } else if c.is_ascii_digit() {
        Class::Number
    } else if c.is_whitespace() {
        Class::White
    } else if matches!(c, '/' | '\\' | ',' | ':' | ';' | '|') {
        Class::Delimiter
    } else if c.is_alphabetic() {
        if c.is_lowercase() {
            Class::Lower
        } else if c.is_uppercase() {
            Class::Upper
        } else {
            Class::Letter
        }
    } else if c.is_numeric() {
        Class::Number
    } else {
        Class::NonWord
    }
}

fn is_word(c: Class) -> bool {
    !matches!(c, Class::White | Class::Delimiter | Class::NonWord)
}

/// The bonus for matching a char of class `cur` that follows one of class `prev`.
fn bonus_for(prev: Class, cur: Class) -> i64 {
    if is_word(cur) {
        match prev {
            Class::White => return BONUS_BOUNDARY_WHITE,
            Class::Delimiter => return BONUS_BOUNDARY_DELIMITER,
            Class::NonWord => return BONUS_BOUNDARY,
            _ => {},
        }
    }
    if (prev == Class::Lower && cur == Class::Upper) || (prev != Class::Number && cur == Class::Number) {
        return BONUS_CAMEL;
    }
    match cur {
        Class::NonWord | Class::Delimiter => BONUS_NON_WORD,
        Class::White => BONUS_BOUNDARY_WHITE,
        _ => 0,
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Kind {
    Fuzzy,
    Exact,
    Prefix,
    Suffix,
    Equal,
}

#[derive(Clone, Debug)]
struct Term {
    kind: Kind,
    /// Lowercased unless `case_sensitive`.
    chars: Vec<char>,
    case_sensitive: bool,
    negate: bool,
}

/// A parsed query.
#[derive(Clone, Debug, Default)]
pub struct Pattern {
    terms: Vec<Term>,
}

/// Reusable buffers, so scoring many candidates doesn't allocate per call.
#[derive(Default)]
pub struct Scratch {
    text: Vec<char>,
    lower: Vec<char>,
}

impl Pattern {
    pub fn parse(query: &str) -> Self {
        let terms = query.split_whitespace().filter_map(parse_term).collect();
        Self { terms }
    }

    /// No terms: everything matches (with score 0).
    pub fn is_empty(&self) -> bool {
        self.terms.is_empty()
    }

    /// Score `text` (higher is better), or `None` when it doesn't match.
    pub fn score(&self, text: &str, scratch: &mut Scratch) -> Option<i64> {
        self.run(text, scratch, None)
    }

    /// The char indices of `text` to highlight for this pattern (sorted, no
    /// duplicates). Empty when it doesn't match.
    pub fn positions(&self, text: &str) -> Vec<usize> {
        let mut scratch = Scratch::default();
        let mut positions = Vec::new();
        if self.run(text, &mut scratch, Some(&mut positions)).is_none() {
            return Vec::new();
        }
        positions.sort_unstable();
        positions.dedup();
        positions
    }

    fn run(&self, text: &str, scratch: &mut Scratch, mut positions: Option<&mut Vec<usize>>) -> Option<i64> {
        if self.terms.is_empty() {
            return Some(0);
        }
        scratch.text.clear();
        scratch.text.extend(text.chars());
        let need_lower = self.terms.iter().any(|t| !t.case_sensitive);
        if need_lower {
            scratch.lower.clear();
            scratch.lower.extend(scratch.text.iter().map(|c| lower(*c)));
        }

        let mut total = 0;
        for term in &self.terms {
            let hay: &[char] = if term.case_sensitive { &scratch.text } else { &scratch.lower };
            let found = match_term(term, hay, &scratch.text, positions.as_deref_mut());
            match (found, term.negate) {
                (Some(_), true) | (None, false) => return None,
                (Some(score), false) => total += score,
                (None, true) => {},
            }
        }
        Some(total)
    }
}

fn lower(c: char) -> char {
    if c.is_ascii() { c.to_ascii_lowercase() } else { c.to_lowercase().next().unwrap_or(c) }
}

fn parse_term(raw: &str) -> Option<Term> {
    let mut s = raw;
    let mut negate = false;
    if let Some(rest) = s.strip_prefix('!') {
        negate = true;
        s = rest;
    }
    let mut kind = Kind::Fuzzy;
    if let Some(rest) = s.strip_prefix('\'') {
        kind = Kind::Exact;
        s = rest;
    } else if let Some(rest) = s.strip_prefix('^') {
        kind = Kind::Prefix;
        s = rest;
    }
    if s.len() > 1
        && let Some(rest) = s.strip_suffix('$')
    {
        kind = if kind == Kind::Prefix { Kind::Equal } else { Kind::Suffix };
        s = rest;
    }
    // A negated term is an exact one (fzf): "doesn't contain", not "doesn't fuzzy-match".
    if negate && kind == Kind::Fuzzy {
        kind = Kind::Exact;
    }
    if s.is_empty() {
        return None;
    }
    let case_sensitive = s.chars().any(|c| c.is_uppercase());
    let chars = if case_sensitive { s.chars().collect() } else { s.chars().map(lower).collect() };
    Some(Term { kind, chars, case_sensitive, negate })
}

/// Match one term; returns its score and records matched positions.
fn match_term(term: &Term, hay: &[char], orig: &[char], positions: Option<&mut Vec<usize>>) -> Option<i64> {
    let pat = &term.chars;
    let (start, end) = match term.kind {
        Kind::Fuzzy => fuzzy_window(pat, hay)?,
        Kind::Exact => {
            let i = find(hay, pat)?;
            (i, i + pat.len())
        },
        Kind::Prefix => (hay.starts_with(pat)).then_some((0, pat.len()))?,
        Kind::Suffix => (hay.ends_with(pat)).then(|| (hay.len() - pat.len(), hay.len()))?,
        Kind::Equal => (hay == pat.as_slice()).then_some((0, hay.len()))?,
    };
    Some(score_window(pat, hay, orig, start, end, positions))
}

/// fzf v1: the earliest window ending where `pat` first completes as a
/// subsequence, tightened from the right.
fn fuzzy_window(pat: &[char], hay: &[char]) -> Option<(usize, usize)> {
    let mut pi = 0;
    let mut end = None;
    for (i, c) in hay.iter().enumerate() {
        if *c == pat[pi] {
            pi += 1;
            if pi == pat.len() {
                end = Some(i + 1);
                break;
            }
        }
    }
    let end = end?;
    let mut pi = pat.len() - 1;
    let mut start = 0;
    for i in (0..end).rev() {
        if hay[i] == pat[pi] {
            if pi == 0 {
                start = i;
                break;
            }
            pi -= 1;
        }
    }
    Some((start, end))
}

fn find(hay: &[char], pat: &[char]) -> Option<usize> {
    if pat.len() > hay.len() {
        return None;
    }
    (0..=hay.len() - pat.len()).find(|&i| hay[i..i + pat.len()] == *pat)
}

/// Score the greedy match of `pat` inside `hay[start..end]` (fzf's
/// `calculateScore`); classes come from the original-case text.
fn score_window(
    pat: &[char],
    hay: &[char],
    orig: &[char],
    start: usize,
    end: usize,
    mut positions: Option<&mut Vec<usize>>,
) -> i64 {
    let mut pi = 0;
    let mut score = 0;
    let mut in_gap = false;
    let mut consecutive = 0;
    let mut first_bonus = 0;
    let mut prev_class = if start > 0 { class_of(orig[start - 1]) } else { Class::White };
    for idx in start..end {
        let class = class_of(orig[idx]);
        if pi < pat.len() && hay[idx] == pat[pi] {
            score += SCORE_MATCH;
            let mut bonus = bonus_for(prev_class, class);
            if consecutive == 0 {
                first_bonus = bonus;
            } else {
                if bonus >= BONUS_BOUNDARY && bonus > first_bonus {
                    first_bonus = bonus;
                }
                bonus = bonus.max(first_bonus).max(BONUS_CONSECUTIVE);
            }
            score += if pi == 0 { bonus * BONUS_FIRST_CHAR_MULTIPLIER } else { bonus };
            in_gap = false;
            consecutive += 1;
            pi += 1;
            if let Some(p) = positions.as_deref_mut() {
                p.push(idx);
            }
        } else {
            score += if in_gap { GAP_EXTENSION } else { GAP_START };
            in_gap = true;
            consecutive = 0;
            first_bonus = 0;
        }
        prev_class = class;
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    fn score(q: &str, t: &str) -> Option<i64> {
        Pattern::parse(q).score(t, &mut Scratch::default())
    }

    fn best<'a>(q: &str, items: &[&'a str]) -> &'a str {
        let p = Pattern::parse(q);
        let mut s = Scratch::default();
        items
            .iter()
            .filter_map(|t| p.score(t, &mut s).map(|sc| (sc, *t)))
            .max_by(|a, b| a.0.cmp(&b.0).then(b.1.len().cmp(&a.1.len())))
            .map(|(_, t)| t)
            .unwrap()
    }

    #[test]
    fn subsequence_matches_and_non_subsequence_does_not() {
        assert!(score("abc", "a_b_c").is_some());
        assert!(score("abc", "acb").is_none());
        assert!(score("", "anything").is_some());
    }

    #[test]
    fn smart_case() {
        assert!(score("main", "src/MAIN.rs").is_some());
        assert!(score("Main", "src/main.rs").is_none());
        assert!(score("Main", "src/Main.rs").is_some());
    }

    #[test]
    fn boundaries_and_runs_rank_higher() {
        assert_eq!(best("main", &["src/domain_handler.rs", "src/main.rs"]), "src/main.rs");
        assert_eq!(best("fb", &["foobar", "foo_bar"]), "foo_bar");
        assert_eq!(best("fb", &["foobar", "fooBar"]), "fooBar");
        assert_eq!(best("lib", &["src/plugins/lazy/inner/bits.rs", "src/lib.rs"]), "src/lib.rs");
    }

    #[test]
    fn all_terms_must_match() {
        assert!(score("src main", "src/main.rs").is_some());
        assert!(score("src nope", "src/main.rs").is_none());
    }

    #[test]
    fn extended_syntax() {
        assert!(score("'ain", "src/main.rs").is_some());
        assert!(score("'mn", "src/main.rs").is_none());
        assert!(score("^src", "src/main.rs").is_some());
        assert!(score("^main", "src/main.rs").is_none());
        assert!(score(".rs$", "src/main.rs").is_some());
        assert!(score(".rs$", "src/main.rsx").is_none());
        assert!(score("^ab$", "ab").is_some());
        assert!(score("^ab$", "abc").is_none());
        assert!(score("!test", "src/main.rs").is_some());
        assert!(score("!main", "src/main.rs").is_none());
        assert!(score("rs !test", "tests/test.rs").is_none());
    }

    #[test]
    fn positions_point_at_the_matched_chars() {
        assert_eq!(Pattern::parse("mrs").positions("src/main.rs"), vec![4, 9, 10]);
        assert_eq!(Pattern::parse("'ain").positions("src/main.rs"), vec![5, 6, 7]);
        assert_eq!(Pattern::parse("nope").positions("src/main.rs"), Vec::<usize>::new());
        // Positions are char indices, not bytes.
        assert_eq!(Pattern::parse("é").positions("aé"), vec![1]);
    }

    #[test]
    fn lone_operators_are_ignored() {
        assert!(Pattern::parse("! ' ^").is_empty());
    }
}
