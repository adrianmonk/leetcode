pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
        words.iter()
        .enumerate()
        .filter(|(_, word)| word.contains(x))
        .map(|(index, _)| index as i32)
        .collect()
}

#[cfg(test)]
mod find_words_containing_char_tests {
    use super::*;

    #[test]
    fn example1()
    {
        let result = find_words_containing(["leet".to_string(), "code".to_string()].to_vec(), 'e');
        assert_eq!(vec![0, 1], result);
    }

    #[test]
    fn example2()
    {
        let result = find_words_containing(vec!["abc".to_string(),"bcd".to_string(),"aaaa".to_string(),"cbc".to_string()], 'a');
        assert_eq!(vec![0, 2], result);
    }

    #[test]
    fn example3()
    {
        let result = find_words_containing(vec!["abc".to_string(),"bcd".to_string(),"aaaa".to_string(),"cbc".to_string()], 'a');
        assert_eq!(vec![0, 2], result);
    }
}
