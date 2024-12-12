fn longest_palindrome(s: String) -> String 
{
    fn expand_around_centre(s: &str, left: usize, right: usize) -> &str
    {
        let (mut l, mut r) = (left as isize, right as isize);

        while l >= 0 && r < s.len() as isize && s.as_bytes()[l as usize] == s.as_bytes()[r as usize]
        {
            l -= 1;
            r += 1;
        }

        &s[(l + 1) as usize..r as usize]
    }

    let len = s.len();
    if len <= 0 
    {
        return s;
    }

    let mut longest: &str = "";

    for i in 0..len 
    {
        let odd_palindrome = expand_around_centre(&s, i, i);

        if odd_palindrome.len() > longest.len() 
        {
            longest = odd_palindrome;
        }

        let even_palindrome = expand_around_centre(&s, i, i + 1);

        if even_palindrome.len() > longest.len()
        {
            longest = even_palindrome;
        }
            
    }
        
    return longest.to_string()
}

#[cfg(test)]
mod longest_palindromic_substring_tests {
    use super::*;

    #[test]
    fn example1()
    {
        let result: String = longest_palindrome("abcabcbb".to_string());
        assert_eq!("bcb", result);
    }

    #[test]
    fn example2()
    {
        let result: String = longest_palindrome("bbbbb".to_string());
        assert_eq!("bbbbb", result);
    }

    #[test]
    fn example3()
    {
        let result: String = longest_palindrome("pwwkew".to_string());
        assert_eq!("ww", result);
    }
}

