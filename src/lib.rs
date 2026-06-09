/// Check if the `have` string contains all lines of the `want` string.
///
/// # Examples
///
/// Let's say we _have_ terminal output,
/// and we _want_ to check if all documented lines were printed.
/// The terminal output can contain more lines than we want to check for,
/// we ignore those.
/// But if and we want to know which lines are missing.
///
/// ```
/// use contains_lines::contains_lines;
///
/// let have = "one\ntwo\nthree";
/// let want = "one\nthree\nfour";
/// let result = contains_lines(have, want);
/// assert_eq!(result, vec!["four"]);
/// ```
pub fn contains_lines<'a>(have: &str, want: &'a str) -> Vec<&'a str> {
    let have_lines: Vec<&str> = have.lines().collect();
    let want_lines: Vec<&str> = want.lines().collect();

    let mut missing = Vec::new();
    let mut have_idx = 0;

    for want_line in want_lines {
        let start_idx = have_idx;
        let mut found = false;
        while have_idx < have_lines.len() {
            if have_lines[have_idx] == want_line {
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

    missing
}

#[cfg(test)]
mod tests {
    use super::contains_lines;

    #[test]
    fn equal() {
        let left = "one\ntwo\nthree";
        let right = "one\ntwo\nthree";
        let have = contains_lines(left, right);
        let want: Vec<&str> = vec![];
        assert_eq!(have, want);
    }

    #[test]
    fn left_contains_more() {
        let left = "one\ntwo\nthree\nfour\nfive";
        let right = "one\nthree\nfour";
        let have = contains_lines(left, right);
        let want: Vec<&str> = vec![];
        assert_eq!(have, want);
    }

    #[test]
    fn right_contains_more() {
        let left = "one\nthree";
        let right = "one\ntwo\nthree\nfour";
        let have = contains_lines(left, right);
        let want: Vec<&str> = vec!["two", "four"];
        assert_eq!(have, want);
    }

    #[test]
    fn no_overlap() {
        let left = "one\ntwo";
        let right = "three\nfour";
        let have = contains_lines(left, right);
        let want: Vec<&str> = vec!["three", "four"];
        assert_eq!(have, want);
    }

    #[test]
    fn out_of_order() {
        let left = "one\ntwo\nthree";
        let right = "three\ntwo\none";
        let have = contains_lines(left, right);
        let want: Vec<&str> = vec!["two", "one"];
        assert_eq!(have, want);
    }
}
