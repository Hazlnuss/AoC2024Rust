advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let mut result_sum: u32 = 0;
    let mut order_rules: Vec<(u32, u32)> = Vec::new();

    for line in input.lines() {
        if line.contains('|') {
            let (a, b) = line.split_once('|').unwrap();
            order_rules.push((a.parse().unwrap(), b.parse().unwrap()));
        }
        if line.contains(',') {
            let numbers: Vec<u32> = line.split(',').map(|n| n.parse::<u32>().unwrap()).collect();
            let mut failed: bool = false;
            for pair in &order_rules {
                if numbers.contains(&pair.0) && numbers.contains(&pair.1) {
                    if !(numbers.iter().position(|n| *n == pair.0)
                        < numbers.iter().position(|n| *n == pair.1))
                    {
                        failed = true;
                        break;
                    }
                }
            }
            if !failed {
                result_sum += numbers[numbers.len() / 2];
            }
        }
    }
    Some(result_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result_sum: u32 = 0;
    let mut order_rules: Vec<(u32, u32)> = Vec::new();

    for line in input.lines() {
        if line.contains('|') {
            let (a, b) = line.split_once('|').unwrap();
            order_rules.push((a.parse().unwrap(), b.parse().unwrap()));
        }
        if line.contains(',') {
            let mut numbers: Vec<u32> =
                line.split(',').map(|n| n.parse::<u32>().unwrap()).collect();
            let mut failed: bool = false;
            let mut incorrect: bool = true;
            while incorrect {
                incorrect = false;
                for pair in &order_rules {
                    if numbers.contains(&pair.0) && numbers.contains(&pair.1) {
                        if !(numbers.iter().position(|n| *n == pair.0)
                            < numbers.iter().position(|n| *n == pair.1))
                        {
                            let pos_zero = numbers.iter().position(|n| n == &pair.0).unwrap();
                            let pos_one = numbers.iter().position(|n| n == &pair.1).unwrap();
                            numbers.swap(pos_zero, pos_one);
                            failed = true;
                            incorrect = true;
                        }
                    }
                }
            }
            if failed {
                result_sum += numbers[numbers.len() / 2];
            }
        }
    }
    Some(result_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
