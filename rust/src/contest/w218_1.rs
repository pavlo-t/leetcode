#![allow(dead_code, unused_imports)]

/// ### 5617. Goal Parser Interpretation
///
/// https://leetcode.com/contest/weekly-contest-218/problems/goal-parser-interpretation/
struct Solution;

impl Solution {
    pub fn interpret(command: String) -> String {
        let mut result = String::new();
        let mut is_al = false;

        for c in command.chars() {
            if c == 'G' {
                result.push('G');
            } else if c == 'a' {
                is_al = true;
            } else if c == ')' {
                if is_al {
                    is_al = false;
                    result.push('a');
                    result.push('l');
                } else {
                    result.push('o');
                }
            }
        };

        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let command = String::from("G()(al)");
        let expected = String::from("Goal");
        assert_eq!(Solution::interpret(command), expected);
        // Explanation: The Goal Parser interprets the command as follows:
        // G -> G
        // () -> o
        // (al) -> al
        // The final concatenated result is "Goal".
    }

    #[test]
    fn example2() {
        let command = String::from("G()()()()(al)");
        let expected = String::from("Gooooal");
        assert_eq!(Solution::interpret(command), expected);
    }

    #[test]
    fn example3() {
        let command = String::from("(al)G(al)()()G");
        let expected = String::from("alGalooG");
        assert_eq!(Solution::interpret(command), expected);
    }
}