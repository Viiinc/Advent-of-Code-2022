#Learnings AoC 2022

- Don't fight the language
	- If Rust does not want to have circular references, don't force it to
- Use libraries where it helps
	- Rust tree library: [Crates.io](https://crates.io/crates/indextree), used [here](https://github.com/tobidope/aoc-2022-rust/blob/main/day07/src/main.rs) (also a clinic on parsing with pattern matching)
	- Rust regex library: [Rust docs](https://docs.rs/regex/latest/regex/)
	- Rust JSON library: [Rust docs](https://docs.rs/json/latest/json/) - life saver on day 13