advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let input_lines: Vec<&str> = input.split_terminator("\n").collect();
    // Initialize already with horizontally counted
    let mut counter: u32 = input
        .as_bytes()
        .windows(4)
        .filter(|m| *m == "XMAS".as_bytes() || *m == "SAMX".as_bytes())
        .count() as u32;

    let height = input_lines.len();
    let width = input_lines[0].len();

    for i in 0..height {
        for j in 0..width {
            if input_lines[i].as_bytes()[j] == b'X' {
                if i + 3 < height {
                    if input_lines[i + 1].as_bytes()[j] == b'M'
                        && input_lines[i + 2].as_bytes()[j] == b'A'
                        && input_lines[i + 3].as_bytes()[j] == b'S'
                    {
                        counter += 1;
                    }
                }
                // SE
                if i + 3 < height && j + 3 < width {
                    if input_lines[i + 1].as_bytes()[j + 1] == b'M'
                        && input_lines[i + 2].as_bytes()[j + 2] == b'A'
                        && input_lines[i + 3].as_bytes()[j + 3] == b'S'
                    {
                        counter += 1;
                    }
                }
                //SW
                if i + 3 < height && j as i32 - 3 >= 0 {
                    if input_lines[i + 1].as_bytes()[j - 1] == b'M'
                        && input_lines[i + 2].as_bytes()[j - 2] == b'A'
                        && input_lines[i + 3].as_bytes()[j - 3] == b'S'
                    {
                        counter += 1;
                    }
                }
                //NE
                if i as i32 - 3 >= 0 && j + 3 < width {
                    if input_lines[i - 1].as_bytes()[j + 1] == b'M'
                        && input_lines[i - 2].as_bytes()[j + 2] == b'A'
                        && input_lines[i - 3].as_bytes()[j + 3] == b'S'
                    {
                        counter += 1;
                    }
                }
                //NW
                if i as i32 - 3 >= 0 && j as i32 - 3 >= 0 {
                    if input_lines[i - 1].as_bytes()[j - 1] == b'M'
                        && input_lines[i - 2].as_bytes()[j - 2] == b'A'
                        && input_lines[i - 3].as_bytes()[j - 3] == b'S'
                    {
                        counter += 1;
                    }
                }
            }

            if input_lines[i].as_bytes()[j] == b'S' {
                if i + 3 < height {
                    if input_lines[i + 1].as_bytes()[j] == b'A'
                        && input_lines[i + 2].as_bytes()[j] == b'M'
                        && input_lines[i + 3].as_bytes()[j] == b'X'
                    {
                        counter += 1;
                    }
                }
            }
        }
    }
    Some(counter)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input_lines: Vec<&str> = input.split_terminator("\n").collect();
    let mut counter: u32 = 0;
    let height = input_lines.len();
    let width = input_lines[0].len();

    for i in 1..height - 1 {
        for j in 1..width - 1 {
            if input_lines[i].as_bytes()[j] == b'A' {
                // M.M
                // .A.
                // S.S
                if input_lines[i - 1].as_bytes()[j - 1] == b'M'
                    && input_lines[i - 1].as_bytes()[j + 1] == b'M'
                    && input_lines[i + 1].as_bytes()[j - 1] == b'S'
                    && input_lines[i + 1].as_bytes()[j + 1] == b'S'
                {
                    counter += 1;
                }
                // M.S
                // .A.
                // M.S
                if input_lines[i - 1].as_bytes()[j - 1] == b'M'
                    && input_lines[i - 1].as_bytes()[j + 1] == b'S'
                    && input_lines[i + 1].as_bytes()[j - 1] == b'M'
                    && input_lines[i + 1].as_bytes()[j + 1] == b'S'
                {
                    counter += 1;
                }
                // S.M
                // .A.
                // S.M
                if input_lines[i - 1].as_bytes()[j - 1] == b'S'
                    && input_lines[i - 1].as_bytes()[j + 1] == b'M'
                    && input_lines[i + 1].as_bytes()[j - 1] == b'S'
                    && input_lines[i + 1].as_bytes()[j + 1] == b'M'
                {
                    counter += 1;
                }
                // S.S
                // .A.
                // M.M
                if input_lines[i - 1].as_bytes()[j - 1] == b'S'
                    && input_lines[i - 1].as_bytes()[j + 1] == b'S'
                    && input_lines[i + 1].as_bytes()[j - 1] == b'M'
                    && input_lines[i + 1].as_bytes()[j + 1] == b'M'
                {
                    counter += 1;
                }
            }
        }
    }

    Some(counter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
