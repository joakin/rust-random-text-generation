mod parse_sentences;
mod prefix_table;

use prefix_table::PrefixTable;

pub struct SentenceGenerator<'a> {
    prefix_table: PrefixTable<'a>,
}

impl<'a> SentenceGenerator<'a> {
    pub fn new(prefix_length: u32) -> SentenceGenerator<'a> {
        SentenceGenerator {
            prefix_table: PrefixTable::new(prefix_length),
        }
    }

    pub fn add_text(&mut self, text: &'a str) {
        let sentences = parse_sentences::from_text(text);
        for sentence in sentences {
            self.prefix_table.add_sentence(sentence);
        }
    }

    pub fn get_random_sentence(&self) -> String {
        self.prefix_table.walk().join(" ")
    }
}

#[cfg(test)]
mod sentence_generator_tests {
    use super::*;

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
        assert!(
            sentence_generator.get_random_sentence().len() > 0,
            "Should return a sentence with at least one word"
        );
    }
}
