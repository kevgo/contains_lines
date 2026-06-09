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
/// let have = "one\ntwo\nthree";
/// let want = "one\nthree\nfour";
/// let result = contains_lines(have, want);
/// assert_eq!(result, vec!["four"]);
/// ```
pub fn contains_lines<'a>(_have: &'a str, _want: &str) -> Vec<&'a str> {
    vec![]
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
        let left = "one\ntwo";
        let right = "two\none";
        let have = contains_lines(left, right);
        let want: Vec<&str> = vec!["one"];
        assert_eq!(have, want);
    }
}
