use fancy_regex::Regex;

fn has_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')
}

fn has_illegal_pair(c: &str) -> bool {
    let illegal_strings = vec!["ab", "cd", "pq", "xy"];

    for x in illegal_strings {
        if c.contains(x) {
            return true;
        }
    }
    false
}

fn has_double_letter(c: &str) -> bool {
    let mut last = ' ';
    for ch in c.chars() {
        if ch == last {
            return true;
        }
        last = ch;
    }
    false
}

// It contains a pair of any two letters that appears at least twice in the string
// without overlapping, like xyxy (xy) or aabcdefgaa (aa), but not like aaa (aa, but it overlaps).
fn has_two_pairs(c: &str) -> bool {
    Regex::new("(..).*\\1").unwrap().is_match(c).unwrap()
}

fn one_letter_repeating(c: &str) -> bool {
    Regex::new("(.).\\1").unwrap().is_match(c).unwrap()
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> i32 {
    input.lines().fold(0, |acc, line| {
        let mut is_nice = true;

        if !(line.chars().filter(|c| has_vowel(*c)).count() < 3) {
            if has_double_letter(line) {
                if !has_illegal_pair(line) {
                    return acc + 1;
                }
            }
        }
        acc
    })
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> i32 {
    input.lines().fold(0, |acc, line| {
        if has_two_pairs(line) {
            if one_letter_repeating(line) {
                return acc + 1;
            }
        }
        acc
    })
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test1() {
        assert_eq!(part1("jchzalrnumimnmhp"), 0)
    }

    #[test]
    fn test2() {
        assert_eq!(part1("haegwjzuvuyypxyu"), 0)
    }

    #[test]
    fn test3() {
        assert_eq!(part1("dvszwmarrgswjxmb"), 0)
    }

    #[test]
    fn test4() {
        assert_eq!(part1("aaa"), 1)
    }

    #[test]
    fn test5() {
        assert_eq!(part1("ugknbfddgicrmopn"), 1)
    }

    #[test]
    fn test6() {
        assert_eq!(part2("qjhvhtzxzqqjkmpb"), 1)
    }

    #[test]
    fn test7() {
        assert_eq!(part2("xxyxx"), 1)
    }

    #[test]
    fn test8() {
        assert_eq!(part2("uurcxstgmygtbstg"), 0)
    }

    #[test]
    fn test9() {
        assert_eq!(part2("ieodomkazucvgmuy"), 0)
    }
}
