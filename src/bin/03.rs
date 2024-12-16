advent_of_code::solution!(3);
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
    let re_digits = Regex::new(r"\d{1,3}").unwrap();
    let muls: Vec<&str> = re.find_iter(input).map(|m| m.as_str()).collect();
    let mut numbers: Vec<u32> = Vec::new();
    for line in muls {
        re_digits
            .find_iter(line)
            .for_each(|m| numbers.push(m.as_str().parse().unwrap()));
    }
    let result_sum: u32 = numbers.chunks(2).fold(0, |acc, m| acc + m[0] * m[1]);

    Some(result_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)|do\(\)|don't\(\)").unwrap();
    let re_digits = Regex::new(r"\d{1,3}").unwrap();
    let muls: Vec<&str> = re.find_iter(input).map(|m| m.as_str()).collect();
    let mut numbers: Vec<u32> = Vec::new();
    let mut enabled: bool = true;
    for line in muls {
        match line {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => {
                if enabled {
                    re_digits
                        .find_iter(line)
                        .for_each(|m| numbers.push(m.as_str().parse().unwrap()));
                }
            }
        }
    }
    let result_sum: u32 = numbers.chunks(2).fold(0, |acc, m| acc + m[0] * m[1]);

    Some(result_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
