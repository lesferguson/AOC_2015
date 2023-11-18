pub fn part_one(input: &str) -> Option<usize> {
    //println!("{}", input);
    // let mut floor: u64 = 0;
    // for c in input.chars() {
    //     println!("{}", c);
    //     println!("{}", floor);
    //     if c == '(' {
    //         floor += 1;
    //     }
    //     else if c == ')' {
    //         floor -= 1
    //     }
    //
    // };
    // Some(floor)
    Some(input.chars().filter(|c| *c == '(').count() - input.chars().filter(|c| *c == ')').count())
}

pub fn part_two(input: &str) -> Option<u32> {
    //println!("{}", input);
    let mut floor: u32 = 0;
    let mut i: u32 = 0;
    for c in input.chars() {
        i += 1;
        // println!("{}", c);
        // println!("{}", floor);
        if c == '(' {
            floor += 1;
        } else if c == ')' {
            if floor == 0 {
                return Some(i);
            }
            floor -= 1
        }
    }
    Some(i)
}

advent_of_code::main!(1);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 1));

        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 1));
        assert_eq!(result, None);
    }
}
