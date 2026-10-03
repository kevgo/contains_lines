use regex::Regex;

/// Check if `have` contains lines that match every pattern in `want`, in order.
///
/// Each line of `want` is a regular expression. A pattern matches a line of `have`
/// when it matches anywhere in that line. Extra lines in `have` are ignored.
/// Patterns that never match a remaining line are returned.
/// Anchor a pattern with `^` and `$` when it must match an entire line.
///
/// # Examples
///
/// ```
/// use contains_lines::contains_lines_matching;
///
/// let have = "one\ntwo\nthree";
/// let want = "o.\nthree\nf.*";
/// let result = contains_lines_matching(have, want).unwrap();
/// assert_eq!(result, vec!["f.*"]);
/// ```
///
/// # Errors
///
/// Returns an error if a line of `want` is not a valid regular expression.
pub fn contains_lines_matching<'a>(
    have: &str,
    want: &'a str,
) -> Result<Vec<&'a str>, regex::Error> {
    let have_lines: Vec<&str> = have.lines().collect();
    let want_lines: Vec<&str> = want.lines().collect();
    let patterns = want_lines
        .iter()
        .map(|pattern| Regex::new(pattern))
        .collect::<Result<Vec<_>, _>>()?;

    let mut missing = Vec::new();
    let mut have_idx = 0;

    for (want_line, pattern) in want_lines.into_iter().zip(patterns) {
        let start_idx = have_idx;
        let mut found = false;
        while have_idx < have_lines.len() {
            if pattern.is_match(have_lines[have_idx]) {
                found = true;
                have_idx += 1;
                break;
            }
            have_idx += 1;
        }
        if !found {
            have_idx = start_idx;
            missing.push(want_line);
        }
    }

    Ok(missing)
}

#[cfg(test)]
mod tests {
    use super::contains_lines_matching;

    #[test]
    fn equal() {
        let left = "one\ntwo\nthree";
        let right = "o.\nt.*\nthree";
        let have = contains_lines_matching(left, right).unwrap();
        let want: Vec<&str> = vec![];
        assert_eq!(have, want);
    }

    #[test]
    fn ignores_extra_lines() {
        let left = "one\ntwo\nthree\nfour\nfive";
        let right = "^one$\n^th.*\nfour";
        let have = contains_lines_matching(left, right).unwrap();
        let want: Vec<&str> = vec![];
        assert_eq!(have, want);
    }

    #[test]
    fn reports_missing_patterns() {
        let left = "one\nthree";
        let right = "one\ntwo\nthree\nfour";
        let have = contains_lines_matching(left, right).unwrap();
        let want: Vec<&str> = vec!["two", "four"];
        assert_eq!(have, want);
    }

    #[test]
    fn no_overlap() {
        let left = "one\ntwo";
        let right = "three\nfour";
        let have = contains_lines_matching(left, right).unwrap();
        let want: Vec<&str> = vec!["three", "four"];
        assert_eq!(have, want);
    }

    #[test]
    fn out_of_order() {
        let left = "one\ntwo\nthree";
        let right = "three\ntwo\none";
        let have = contains_lines_matching(left, right).unwrap();
        let want: Vec<&str> = vec!["two", "one"];
        assert_eq!(have, want);
    }

    #[test]
    fn unanchored_pattern_matches_inside_a_line() {
        let left = "prefix one suffix\ntwo";
        let right = "one\ntwo";
        let have = contains_lines_matching(left, right).unwrap();
        let want: Vec<&str> = vec![];
        assert_eq!(have, want);
    }

    #[test]
    fn anchored_pattern_requires_the_whole_line() {
        let left = "prefix one suffix\ntwo";
        let right = "^one$\ntwo";
        let have = contains_lines_matching(left, right).unwrap();
        let want: Vec<&str> = vec!["^one$"];
        assert_eq!(have, want);
    }

    #[test]
    fn same_pattern_can_match_successive_lines() {
        let left = "one\ntwo\nthree";
        let right = "t.*\nt.*";
        let have = contains_lines_matching(left, right).unwrap();
        let want: Vec<&str> = vec![];
        assert_eq!(have, want);
    }

    #[test]
    fn invalid_pattern() {
        let result = contains_lines_matching("one\ntwo", "one\n(");
        assert!(result.is_err());
    }
}
