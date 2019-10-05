use regex::Regex;
use std::collections::HashMap;
use std::io::BufRead;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CountOption {
    Char,
    Word,
    Line,
}

impl Default for CountOption {
    fn default() -> Self {
        CountOption::Word
    }
}

// ライブラリ外から参照するためpubにする
pub fn count(input: impl BufRead) -> HashMap<String, usize> {
    let re = Regex::new(r"\w+").unwrap();
    let mut freqs = HashMap::new();

    for line in input.lines() {
        let line = line.unwrap();
        // 4. その行を単語で分割する
        for m in re.find_iter(&line) {
            let word = m.as_str().to_string();
            // 5. 出現した単語の出現頻度を数える
            *freqs.entry(word).or_insert(0) += 1;
        }
    }
    freqs
}