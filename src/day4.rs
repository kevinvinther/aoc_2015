#[aoc(day4, part1)]
pub fn part1(input: &str) -> i32 {
    for i in 0.. {
        if &format!("{:?}", md5::compute(format!("{}{}", input, i)))[0..5] == "00000" {
            return i;
        }
    }
    unreachable!()
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> i32 {
    for i in 0.. {
        if &format!("{:?}", md5::compute(format!("{}{}", input, i)))[0..6] == "000000" {
            return i;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test1() {
        assert_eq!(part1("abcdef"), 609043)
    }

    #[test]
    fn test2() {
        assert_eq!(part1("pqrstuv"), 1048970)
    }
}
