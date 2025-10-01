/// Creates a score of how much the input and the pattern match
///
/// The higher the score the better. There is no max score.
pub fn fuzzy_match_score(input: &str, pattern: &str) -> i32 {
    let mut score = 0;
    if input
        .to_lowercase()
        .contains(pattern.to_lowercase().as_str())
    {
        score += 10;
    }
    let mut dir_name_mut = input.to_string();
    for c in pattern.chars() {
        if dir_name_mut.to_lowercase().contains(c.to_ascii_lowercase()) {
            score += 1;
            // strip the char to avoid multiple matches
            dir_name_mut = dir_name_mut.replacen(c, "", 1);
        } else {
            score -= 5;
        }
    }
    if input.to_lowercase() == pattern.to_lowercase() {
        score += 50;
    }
    if score < 0 {
        score = 0;
    }
    score
}

#[cfg(test)]
mod tests {
    use super::fuzzy_match_score as score;
    
    #[test]
    #[should_panic]
    fn not_yet_implemented() {
        // TODO: Should give more points if parts of query match
        // beginning parts of input parts
        assert!(score("test", "tt") > score("test", "t"));
        assert!(score("test-abc", "ta") > score("test-abc", "te"));
        assert!(score("test abc", "ta") > score("test abc", "te"));
        assert!(score("test_abc", "ta") > score("test_abc", "te"));

        assert!(score("test_abc_a", "taa") > score("test_abc_a", "te"));
        assert!(score("test_abc_a", "taa") > score("test_abc_a", "tea"));
        
        assert!(score("testAbc", "ta") > score("testAbc", "te"));
    }

    #[test]
    fn test_simple() {
        assert_eq!(score("test", "test"), score("test", "test"));
        assert_eq!(score("test", "uoa"), 0);

        assert!(score("test", "test") > score("test", "tes"));
        assert!(score("ttest", "tt") > score("ttest", "t"));
    }
    
    #[test]
    fn test_advanced() {
        assert!(score("helloworld", "world") > score("helloworld", "elwo"));
        assert!(score("helloworld", "hello") > score("helloworld", "hellohello"));
    }
    
    #[test]
    fn test_negative_queries() {
        assert!(score("helloworld", "ellovvvv") == 0);
        assert!(score("helloworld", "ww") == 0);
    }
}
