# Rustbook minigrep

Using chapter 12 of 
`rustup doc --book`

Follow the example command line app with small (sectionwise) commits. Then we can use diffs to see the evolution in the program structure, error handling, refactoring main to create a library crate, TDD, environment variables etc.

use `cargo check` for compilation checks, `cargo run` to run the tests. Run program with `cargo run -- args` so in this case something like `cargo run  -- body poem.txt`

##  env vars
Section 12.5 incorporates env vars. The example uses an env var `IGNORE_CASE`.  This can be used in the call as e.g. `IGNORE_CASE=5 cargo run -- To poem.txt` , try without the env var and also with lowercase `to` instead of `To`. 