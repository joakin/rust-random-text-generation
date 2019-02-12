use rand::Rng;
use std::collections::HashMap;

const START: &str = "START";
const STOP: &str = "STOP";

#[derive(Debug)]
pub struct PrefixTable {
    prefix_length: usize,
    table: HashMap<Vec<String>, WordDistribution>,
}

impl PrefixTable {
    pub fn new(prefix_length: u32) -> PrefixTable {
        PrefixTable {
            prefix_length: prefix_length as usize,
            table: HashMap::new(),
        }
    }

    pub fn add_sentence(&mut self, sentence: Vec<String>) -> &mut Self {
        let mut prefix = Prefix::new(self.prefix_length);
        prefix.push(START.to_string());

        for word in sentence {
            let distribution = self
                .table
                .entry(prefix.words.clone())
                .or_insert(WordDistribution::new());
            distribution.add(word.clone());

            prefix.push(word);
        }

        let distribution = self
            .table
            .entry(prefix.words.clone())
            .or_insert(WordDistribution::new());
        distribution.add(STOP.to_string());

        self
    }

    pub fn walk(&self) -> Vec<String> {
        let mut prefix = Prefix::new(self.prefix_length);
        prefix.push(START.to_string());
        let mut result = Vec::new();

        while let Some(distribution) = self.table.get(&prefix.words) {
            match distribution.get_random() {
                Some(word) => {
                    if word == STOP {
                        return result;
                    }

                    result.push(word.to_string());

                    prefix.push(word.to_string());
                }
                None => return result,
            }
        }

        result
    }
}

#[cfg(test)]
mod prefix_table_tests {
    use super::*;
    use crate::parse_sentences;

    #[test]
    fn prefix_table() {
        let text = "\
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!

Are you nobody, as well?";
        let mut prefix_table = PrefixTable::new(3);
        for sentence in parse_sentences::from_text(text) {
            prefix_table.add_sentence(sentence);
        }
        assert_eq!(prefix_table.prefix_length, 3);
        assert_eq!(prefix_table.table.len(), 57);
        assert_eq!(
            *prefix_table
                .table
                .entry(vec![
                    "Are".to_string(),
                    "you".to_string(),
                    "nobody".to_string()
                ])
                .or_insert(WordDistribution::new())
                .words
                .entry(",".to_string())
                .or_default(),
            2
        );
    }
}

struct Prefix {
    length: usize,
    words: Vec<String>,
}

impl Prefix {
    fn new(length: usize) -> Prefix {
        Prefix {
            length,
            words: Vec::with_capacity(length),
        }
    }

    fn push(&mut self, word: String) -> &mut Prefix {
        if self.words.len() == self.length {
            self.words.remove(0);
        }
        self.words.push(word.to_string());
        self
    }
}

#[cfg(test)]
mod prefix_tests {
    use super::*;

    #[test]
    fn prefix_test() {
        let mut prefix = Prefix::new(2);
        prefix.push("hi".to_string());
        prefix.push("ho".to_string());
        assert_eq!(prefix.words, vec!["hi".to_string(), "ho".to_string()]);

        prefix.push("lets go".to_string());
        assert_eq!(prefix.words, vec!["ho".to_string(), "lets go".to_string()]);
    }
}

#[derive(Debug)]
struct WordDistribution {
    total: u32,
    words: HashMap<String, u32>,
}

impl WordDistribution {
    fn new() -> WordDistribution {
        let total = 0;
        let words = HashMap::new();
        WordDistribution { total, words }
    }

    fn add(&mut self, word: String) {
        self.total += 1;
        let count = self.words.entry(word).or_default();
        *count += 1;
    }

    fn get_random(&self) -> Option<&str> {
        let mut n = rand::thread_rng().gen_range(0, self.total);
        for (word, amount) in self.words.iter() {
            if n <= *amount {
                return Some(&word);
            }
            n = n - amount;
        }
        None
    }
}

#[cfg(test)]
mod word_distribution_tests {
    use super::*;

    #[test]
    fn new_distribution() {
        let words = vec!["hi", "ho", "hi"];
        let mut distribution = WordDistribution::new();
        for word in words {
            distribution.add(word.to_string());
        }
        assert_eq!(distribution.total, 3, "Total should be 3");
        assert_eq!(
            *distribution.words.entry("hi".to_string()).or_default(),
            2,
            "hi should have frequency 2"
        );
    }
}
