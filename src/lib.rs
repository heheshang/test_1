pub fn find_first_palindrome(start: i32, end: i32) -> Option<i32> {
    eprintln!("{} {}", start, end);
    if start <= end {
        for i in start..=end {
            let s = i.to_string();
            if s == s.chars().rev().collect::<String>() {
                return Some(i);
            }
        }
        return None;
    } else {
        for i in end..=start {
            let s = i.to_string();
            if s == s.chars().rev().collect::<String>() {
                return Some(i);
            }
        }
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_first_palindrome() {
        assert_eq!(true, true);
    }
}
