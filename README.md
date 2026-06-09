# `contains_lines`

A Rust library for comparing multiline strings
while allowing extra lines on the left side.

This is useful for testing output that may contain nondeterministic
or irrelevant lines, such as logs or terminal output.
Your expected value on the right lists the lines that must appear.
The comparison succeeds when those lines occur in the actual output,
in the same order as on the right, even if other lines appear between them.

The comparison fails if a required line is missing or appears out of order.


```rs
use contains_lines::contains_lines;

let have = "one\ntwo\nthree";
let want = "one\nthree\nfour";
let result = contains_lines(have, want);
assert_eq!(result, vec!["four"]);
```
