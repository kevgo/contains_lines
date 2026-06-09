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
}
