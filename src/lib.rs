use rand::Rng;
use std::collections::HashMap;

const START: &str = "START";

const STOP: &str = "STOP";

#[derive(Debug)]
struct Distribution {
    total: u32,
    amounts: HashMap<String, u32>,
}

impl Distribution {
    fn new() -> Distribution {
        let total = 0;
        let amounts = HashMap::new();
        Distribution { total, amounts }
    }

    fn add(&mut self, word: String) {
        self.total += 1;
        let count = self.amounts.entry(word).or_default();
        *count += 1;
    }

    fn get_random(&self) -> Option<&str> {
        let mut n = rand::thread_rng().gen_range(0, self.total);
        for (word, amount) in self.amounts.iter() {
            if n <= *amount {
                return Some(&word);
            }
            n = n - amount;
        }
        None
    }
}

#[derive(Debug)]
struct PTable {
    prefix_length: usize,
    table: HashMap<Vec<String>, Distribution>,
}
impl PTable {
    fn new(prefix_length: u32) -> PTable {
        PTable {
            prefix_length: prefix_length as usize,
            table: HashMap::new(),
        }
    }

    fn add_sentence(&mut self, sentence: Vec<String>) -> &mut Self {
        let mut prefix: Vec<String> = Vec::with_capacity(self.prefix_length);
        prefix.push(START.to_string());

        for word in sentence {
            let distribution = self
                .table
                .entry(prefix.clone())
                .or_insert(Distribution::new());
            distribution.add(word.clone());

            if prefix.len() == self.prefix_length {
                prefix.remove(0);
            }
            prefix.push(word);
        }

        let distribution = self
            .table
            .entry(prefix.clone())
            .or_insert(Distribution::new());
        distribution.add(STOP.to_string());

        self
    }

    fn walk(&self) -> Vec<String> {
        let mut prefix: Vec<String> = Vec::with_capacity(self.prefix_length);
        prefix.push(START.to_string());
        let mut result = Vec::new();

        while let Some(distribution) = self.table.get(&prefix) {
            match distribution.get_random() {
                Some(word) => {
                    if word == STOP {
                        return result;
                    }

                    result.push(word.to_string());

                    if prefix.len() == self.prefix_length {
                        prefix.remove(0);
                    }
                    prefix.push(word.to_string());
                }
                None => return result,
            }
        }

        result
    }
}

mod sentences {
    pub fn from_text(text: &str) -> Vec<Vec<String>> {
        let mut sentences: Vec<Vec<String>> = Vec::new();
        let mut current_sentence: Vec<String> = Vec::new();
        let mut word = String::new();
        for chr in text.chars() {
            if chr.is_whitespace() {
                maybe_add_word(&mut word, &mut current_sentence);
            } else if chr == ';'
                || chr == ','
                || chr == ':'
                || chr == '-'
                || chr == '"'
                // || chr == '\''
                || chr == '?'
                || chr == '!'
                || chr == '.'
            {
                maybe_add_word(&mut word, &mut current_sentence);

                maybe_add_word(&mut chr.to_string(), &mut current_sentence);

                if chr == '?' || chr == '!' || chr == '.' {
                    maybe_add_sentence(&mut current_sentence, &mut sentences);
                }
            } else {
                if chr.is_alphanumeric() || chr == '\'' {
                    word.push(chr);
                }
            }
        }

        // Add the maybe last captured word and sentence
        maybe_add_word(&mut word, &mut current_sentence);
        maybe_add_sentence(&mut current_sentence, &mut sentences);

        sentences
    }

    fn maybe_add_word(word: &mut String, sentence: &mut Vec<String>) {
        if !word.is_empty() {
            sentence.push(word.clone());
            word.clear();
        }
    }

    fn maybe_add_sentence(sentence: &mut Vec<String>, sentences: &mut Vec<Vec<String>>) {
        if !sentence.is_empty() {
            sentences.push(sentence.clone());
            sentence.clear();
        }
    }
}

pub struct SentenceGenerator {
    ptable: PTable,
}

impl SentenceGenerator {
    pub fn new(prefix_length: u32) -> SentenceGenerator {
        SentenceGenerator {
            ptable: PTable::new(prefix_length),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        let sentences = sentences::from_text(text);
        for sentence in sentences {
            self.ptable.add_sentence(sentence);
        }
    }

    pub fn get_random_sentence(&self) -> String {
        self.ptable.walk().join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_distribution() {
        let words = vec!["hi", "ho", "hi"];
        let mut distribution = Distribution::new();
        for word in words {
            distribution.add(word.to_string());
        }
        dbg!(distribution);
    }

    #[test]
    fn parsing_sentences() {
        let tests = vec![
            (
                "Hi, how are you?",
                vec![vec!["Hi", ",", "how", "are", "you", "?"]],
            ),
            (
                "Hi, how are you? I am fine thanks!",
                vec![
                    vec!["Hi", ",", "how", "are", "you", "?"],
                    vec!["I", "am", "fine", "thanks", "!"],
                ],
            ),
            (
                "Hi, how are you? Fine",
                vec![vec!["Hi", ",", "how", "are", "you", "?"], vec!["Fine"]],
            ),
        ];
        for (t1, s1) in tests {
            assert_eq!(sentences::from_text(t1), s1);
        }
    }

    #[test]
    fn ptable() {
        let text = "\
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!";
        let text2 = "Hi how are you. Hey, how are thee. Hi, how are they? No, how are thee!";
        let mut ptable = PTable::new(2);
        for sentence in sentences::from_text(text2) {
            ptable.add_sentence(sentence);
        }
        dbg!(ptable);
    }

    #[test]
    fn sentence_generator() {
        let text = "\
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!";
        let mut sentence_generator = SentenceGenerator::new(2);
        sentence_generator.add_text(text);
        dbg!(sentence_generator.get_random_sentence());
    }
}
