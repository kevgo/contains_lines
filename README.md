# Contains_lines

A Rust library for comparing strings ignoring lines on the left side.

Example: You write a test for output like log files or terminal output.
Your test specifies lines that the output must contain.
The test should pass if the output contains other lines mixed in.
But if a line documented in the test is missing, the test should fail.
Lines must occur in the order specified in the test.

```rs
use contains_lines::contains_lines;

let have = "one\ntwo\nthree";
let want = "one\nthree\nfour";
let result = contains_lines(have, want);
assert_eq!(result, vec!["four"]);
```
