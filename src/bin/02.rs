advent_of_code::solution!(2);

pub fn check_line(line: &Vec<u32>) -> bool {
    let mut ascending: bool = false;
    let mut descending: bool = false;
    let mut failed: bool = false;
    for pair in line.windows(2) {
        match pair[0].cmp(&pair[1]) {
            std::cmp::Ordering::Less => ascending = true,
            std::cmp::Ordering::Greater => descending = true,
            std::cmp::Ordering::Equal => {
                failed = true;
                break;
            }
        }
        if pair[0].abs_diff(pair[1]) > 3 || (ascending && descending) {
            failed = true;
            break;
        }
    }
    failed
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut safe_levels: u32 = 0;
    let levels: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|c| c.parse().unwrap())
                .collect()
        })
        .collect();
    for line in levels {
        let failed: bool;
        failed = check_line(&line);

        if !failed {
            safe_levels += 1;
        }
    }

    Some(safe_levels)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut safe_levels: u32 = 0;
    let levels: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|c| c.parse().unwrap())
                .collect()
        })
        .collect();
    for line in levels {
        if !check_line(&line) {
            safe_levels += 1;
            continue;
        }
        for i in 0..line.len() {
            let mut temp_line = line.to_vec();
            temp_line.remove(i);
            if !check_line(&temp_line) {
                safe_levels += 1;
                break;
            }
        }
    }

    Some(safe_levels)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
