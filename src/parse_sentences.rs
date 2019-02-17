pub fn from_text(text: &str) -> Vec<Vec<&str>> {
    let mut sentences = Vec::new();
    let mut current_sentence = Vec::new();
    let mut start = 0;
    let last_index = text.len() - 1;

    for (i, chr) in text.char_indices() {
        if chr.is_whitespace() {
            // Add previous word
            if start < i {
                let word = &text[start..i];
                current_sentence.push(word);
            }
            start = i + 1;
        } else if is_punctuation(chr) {
            // Add previous word
            if start < i {
                let word = &text[start..i];
                current_sentence.push(word);
            }
            start = i;
            // Add char itself as word
            let word = &text[start..i + 1];
            current_sentence.push(word);
            start = i + 1;

            if is_sentence_terminator(chr) {
                // Add sentence
                if !current_sentence.is_empty() {
                    sentences.push(current_sentence);
                    current_sentence = Vec::new();
                }
            }
        }

        if i == last_index {
            // Add word if any pending
            if start < i {
                let word = &text[start..=i];
                current_sentence.push(word);
            }
            // Add sentence if any pending
            if !current_sentence.is_empty() {
                sentences.push(current_sentence);
                current_sentence = Vec::new();
            }
        }
    }

    sentences
}

fn is_sentence_terminator(chr: char) -> bool {
    chr == '?' || chr == '!' || chr == '.'
}

fn is_punctuation(chr: char) -> bool {
    chr == ';'
    || chr == ','
    || chr == ':'
    || chr == '-'
    || chr == '"'
    // || chr == '\''
    || chr == '?'
    || chr == '!'
    || chr == '.'
}

#[cfg(test)]
mod sentence_tests {
    use super::*;

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
            assert_eq!(from_text(t1), s1);
        }
    }
}
