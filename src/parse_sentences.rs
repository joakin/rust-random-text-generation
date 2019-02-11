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
