use std::collections::HashSet;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> i32 {
    let mut map: HashSet<(i32, i32)> = HashSet::new();
    let mut current_position = (0, 0);

    map.insert((0, 0));

    input.chars().for_each(|c| match c {
        '^' => {
            current_position.1 += 1;
            map.insert(current_position);
        }
        '<' => {
            current_position.0 -= 1;
            map.insert(current_position);
        }
        '>' => {
            current_position.0 += 1;
            map.insert(current_position);
        }
        'v' => {
            current_position.1 -= 1;
            map.insert(current_position);
        }
        _ => unreachable!(),
    });

    map.len() as i32
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> i32 {
    let mut map: HashSet<(i32, i32)> = HashSet::new();
    let mut current_position = (0, 0);
    let mut current_position_robo = (0, 0);

    let mut is_robo_santa = false;

    map.insert((0, 0));

    input.chars().for_each(|c| match c {
        '^' => {
            if !is_robo_santa {
                current_position.1 += 1;
                map.insert(current_position);
            } else {
                current_position_robo.1 += 1;
                map.insert(current_position_robo);
            }
            is_robo_santa = !is_robo_santa;
        }
        '<' => {
            if !is_robo_santa {
                current_position.0 -= 1;
                map.insert(current_position);
            } else {
                current_position_robo.0 -= 1;
                map.insert(current_position_robo);
            }
            is_robo_santa = !is_robo_santa;
        }
        '>' => {
            if !is_robo_santa {
                current_position.0 += 1;
                map.insert(current_position);
            } else {
                current_position_robo.0 += 1;
                map.insert(current_position_robo);
            }
            is_robo_santa = !is_robo_santa;
        }
        'v' => {
            if !is_robo_santa {
                current_position.1 -= 1;
                map.insert(current_position);
            } else {
                current_position_robo.1 -= 1;
                map.insert(current_position_robo);
            }
            is_robo_santa = !is_robo_santa;
        }
        _ => unreachable!(),
    });

    map.len() as i32
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test1() {
        assert_eq!(part1(">"), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(part1("^>v<"), 4);
    }

    #[test]
    fn test3() {
        assert_eq!(part1("^v^v^v^v^v"), 2);
    }

    #[test]
    fn test4() {
        assert_eq!(part2("^v"), 3);
    }

    #[test]
    fn test5() {
        assert_eq!(part2("^>v<"), 3);
    }

    #[test]
    fn test6() {
        assert_eq!(part2("^v^v^v^v^v"), 11);
    }
}
