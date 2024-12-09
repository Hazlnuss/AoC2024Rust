advent_of_code::solution!(1);

pub fn parse_and_sort(input: &str) -> (Vec<u32>, Vec<u32>) {
    let (mut left, mut right) = input
        .lines()
        .map(|line| line.split_ascii_whitespace())
        .map(|mut c| {
            (
                c.next().unwrap().parse::<u32>().unwrap(),
                c.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .collect::<(Vec<u32>, Vec<u32>)>();
    left.sort_unstable();
    right.sort_unstable();
    (left, right)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (left, right) = parse_and_sort(input);
    let result = left
        .iter()
        .zip(right.iter())
        .map(|(left, right)| left.abs_diff(*right))
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left, right) = parse_and_sort(input);
    let result = left
        .iter()
        .map(|l| l * right.iter().filter(|r| *r == l).count() as u32)
        .sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
