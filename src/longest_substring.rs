use std::collections::HashSet;

fn length_of_longest_substring(s: String) -> i32 {
    let mut seen: HashSet<char> = HashSet::new();

    let mut start= 0;
    let mut max_length = 0;

    for (end, ch) in s.chars().enumerate() 
    {
        while seen.contains(&ch) 
        {
            seen.remove(&s[start..].chars().next().unwrap());
            start += 1;
        }

        seen.insert(ch);
        max_length = max_length.max(end - start + 1);
    }

    max_length.try_into().unwrap()
}

#[cfg(test)]
mod length_of_longest_substring_tests {
    use super::*;

    #[test]
    fn example1()
    {
        let result: i32 = length_of_longest_substring("abcabcbb".to_string());
        assert_eq!(3, result);
    }

    #[test]
    fn example2()
    {
        let result: i32 = length_of_longest_substring("bbbbb".to_string());
        assert_eq!(1, result);
    }

    #[test]
    fn example3()
    {
        let result: i32 = length_of_longest_substring("pwwkew".to_string());
        assert_eq!(3, result);
    }
}