pub struct WordIterator<'a> {
    alphabet: &'a [&'a str],
    max_length: usize,
    current_word: Vec<usize>,
}

impl<'a> WordIterator<'a> {
    pub fn new(alphabet: &'a [&'a str], max_length: usize) -> Self {
        WordIterator {
            alphabet,
            max_length,
            current_word: Vec::new(),
        }
    }

    pub fn increment_word(&mut self) {
        let alphabet_size = self.alphabet.len();

        for i in (0..self.current_word.len()).rev() {
            self.current_word[i] += 1;

            if self.current_word[i] == alphabet_size {
                self.current_word[i] = 0;
            } else {
                return;
            }
        }

        self.current_word.insert(0, 0);
    }
}

impl<'a> Iterator for WordIterator<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_word.len() > self.max_length {
            return None;
        }

        let word: String = self
            .current_word
            .iter()
            .enumerate()
            .flat_map(|(i, &index)| {
                let letter = self.alphabet[index];
                if i > 0 {
                    vec![".".to_string(), letter.to_string()]
                } else {
                    vec![letter.to_string()]
                }
            })
            .collect();

        self.increment_word();

        Some(word)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_iterator_with_single_letter_alphabet() {
        let alphabet = &["a"];
        let max_length = 3;

        let mut iterator = WordIterator::new(alphabet, max_length);

        assert_eq!(iterator.next(), Some("".to_string()));
        assert_eq!(iterator.next(), Some("a".to_string()));
        assert_eq!(iterator.next(), Some("a.a".to_string()));
        assert_eq!(iterator.next(), Some("a.a.a".to_string()));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn test_word_iterator_with_multiple_letter_alphabet() {
        let alphabet = &["a", "b"];
        let max_length = 2;

        let mut iterator = WordIterator::new(alphabet, max_length);
        assert_eq!(iterator.next(), Some("".to_string()));
        assert_eq!(iterator.next(), Some("a".to_string()));
        assert_eq!(iterator.next(), Some("b".to_string()));
        assert_eq!(iterator.next(), Some("a.a".to_string()));
        assert_eq!(iterator.next(), Some("a.b".to_string()));
        assert_eq!(iterator.next(), Some("b.a".to_string()));
        assert_eq!(iterator.next(), Some("b.b".to_string()));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn test_multiletter_alphabet() {
        let alphabet = &["f1", "f2", "f3"];
        let max_length = 2;

        let mut iterator = WordIterator::new(alphabet, max_length);
        assert_eq!(iterator.next(), Some("".to_string()));
        assert_eq!(iterator.next(), Some("f1".to_string()));
        assert_eq!(iterator.next(), Some("f2".to_string()));
        assert_eq!(iterator.next(), Some("f3".to_string()));
        assert_eq!(iterator.next(), Some("f1.f1".to_string()));
        assert_eq!(iterator.next(), Some("f1.f2".to_string()));
        assert_eq!(iterator.next(), Some("f1.f3".to_string()));
        assert_eq!(iterator.next(), Some("f2.f1".to_string()));
        assert_eq!(iterator.next(), Some("f2.f2".to_string()));
        assert_eq!(iterator.next(), Some("f2.f3".to_string()));
        assert_eq!(iterator.next(), Some("f3.f1".to_string()));
        assert_eq!(iterator.next(), Some("f3.f2".to_string()));
        assert_eq!(iterator.next(), Some("f3.f3".to_string()));
    }
}
