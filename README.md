# Contains_lines

A Rust library for checking if one string contains all lines of another string.

Example: You write a test for output like log files or terminal output.
Your test specifies lines that the output must contain.
The test should pass if the output contains other lines mixed in.

```rs
use contains_lines::contains_lines;

let have = "one\ntwo\nthree";
let want = "one\nthree\nfour";
let result = contains_lines(have, want);
assert_eq!(result, vec!["four"]);
```
