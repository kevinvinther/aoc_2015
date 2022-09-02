pub struct Present {
    l: i32,
    w: i32,
    h: i32,
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Present> {
    input
        .lines()
        .map(|line| {
            let mut present = line.trim().split('x').map(|d| d.parse::<i32>().unwrap());
            Present {
                l: present.next().unwrap(),
                w: present.next().unwrap(),
                h: present.next().unwrap(),
            }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(present: &[Present]) -> i32 {
    present
        .iter()
        .map(|p| {
            let sides: Vec<i32> = vec![p.w * p.l, p.w * p.h, p.h * p.l];
            let min = sides.iter().min().unwrap();
            let sum: i32 = sides.iter().map(|s| s * 2).sum();
            sum + min
        })
        .sum()
}

#[aoc(day2, part2)]
pub fn part2(present: &[Present]) -> i32 {
    present
        .iter()
        .map(|p| -> i32 {
            let mut sides: Vec<i32> = vec![p.l, p.w, p.h];
            sides.sort();
            let wrapping = sides[0] * 2 + sides[1] * 2;
            let bow: i32 = sides.iter().product();
            wrapping + bow
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    #[test]
    fn test1() {
        assert_eq!(part1(&input_generator("2x3x4")), 58);
    }

    #[test]
    fn test2() {
        assert_eq!(part1(&input_generator("1x1x10")), 43);
    }

    #[test]
    fn test3() {
        assert_eq!(part2(&input_generator("2x3x4")), 34);
    }

    #[test]
    fn test4() {
        assert_eq!(part2(&input_generator("1x1x10")), 14);
    }
}
