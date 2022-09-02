#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    input.chars().fold(0, |floor, c| match c {
        '(' => floor + 1,
        ')' => floor - 1,
        _ => unreachable!(),
    })
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> usize {
    let mut floor = 1; // We start at position 1

    // char c, index i
    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => unreachable!(),
        }

        if floor == -1 {
            return i;
        }
    }

    unreachable!()
}

// #[aoc(day1, part2)]
#[cfg(test)]
mod tests {
    use super::{part1, part2};

    // (()) and ()() both result in floor 0.
    #[test]
    fn test1() {
        assert_eq!(part1("(())"), 0);
        assert_eq!(part1("()()"), 0);
    }

    // ((( and (()(()( both result in floor 3.
    #[test]
    fn test2() {
        assert_eq!(part1("((("), 3);
        assert_eq!(part1("(()(()("), 3);
    }

    // ))((((( also results in floor 3.
    #[test]
    fn test3() {
        assert_eq!(part1("))((((("), 3);
    }

    // ()) and ))( both result in floor -1 (the first basement level).
    #[test]
    fn test4() {
        assert_eq!(part1("())"), -1);
        assert_eq!(part1("))("), -1);
    }

    // ))) and )())()) both result in floor -3.
    #[test]
    fn test5() {
        assert_eq!(part1(")))"), -3);
        assert_eq!(part1(")())())"), -3);
    }

    // ) causes him to enter the basement at character position 1.
    #[test]
    fn test6() {
        assert_eq!(part2(")))"), 1);
    }

    // ()()) causes him to enter the basement at character position 5.
    #[test]
    fn test7() {
        assert_eq!(part2("()())))"), 5);
    }
}
