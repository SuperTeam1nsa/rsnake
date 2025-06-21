#![forbid(unsafe_code)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]

use rsnaker::start_snake;
// Basic to get the number of lines without comments and blanks:
// find . -name '*.rs' -not -path "./target/*" -print0 | xargs -0 cat | grep -v '^\s*$' | grep -v '^\s*//' | wc -l
// including comments without blanks:
// find . -name '*.rs' -not -path "./target/*" -print0 | xargs -0 cat | grep -v '^\s*$' | wc -l
// To get all in lines in .rs:
// find . -name '*.rs' -print0 | xargs -0 cat | wc -l
// for more precise analyse: cargo install tokei && tokei
fn main() {
    start_snake();
}
