use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<usize> {
    let mut house_count: HashMap<(i32, i32), u32> = HashMap::new();
    house_count.insert((0, 0), 1);
    let mut current_house: (i32, i32) = (0, 0);

    for instruction in input.chars() {
        if instruction == '^' {
            let y = current_house.1 + 1;
            current_house = (current_house.0, y);
            *house_count.entry(current_house).or_insert(0) += 1;
        } else if instruction == 'v' {
            let y = current_house.1 - 1;
            current_house = (current_house.0, y);
            *house_count.entry(current_house).or_insert(0) += 1;
        } else if instruction == '>' {
            let x = current_house.0 + 1;
            current_house = (x, current_house.1);
            *house_count.entry(current_house).or_insert(0) += 1;
        } else if instruction == '<' {
            let x = current_house.0 - 1;
            current_house = (x, current_house.1);
            *house_count.entry(current_house).or_insert(0) += 1;
        }
    }
    Some(house_count.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut house_count: HashMap<(i32, i32), u32> = HashMap::new();
    house_count.insert((0, 0), 1);
    let mut s_current_house: (i32, i32) = (0, 0);
    let mut r_current_house: (i32, i32) = (0, 0);
    let mut current_s: char = 's';

    for instruction in input.chars() {
        if current_s == 's' {
            if instruction == '^' {
                let y = s_current_house.1 + 1;
                s_current_house = (s_current_house.0, y);
                *house_count.entry(s_current_house).or_insert(0) += 1;
            } else if instruction == 'v' {
                let y = s_current_house.1 - 1;
                s_current_house = (s_current_house.0, y);
                *house_count.entry(s_current_house).or_insert(0) += 1;
            } else if instruction == '>' {
                let x = s_current_house.0 + 1;
                s_current_house = (x, s_current_house.1);
                *house_count.entry(s_current_house).or_insert(0) += 1;
            } else if instruction == '<' {
                let x = s_current_house.0 - 1;
                s_current_house = (x, s_current_house.1);
                *house_count.entry(s_current_house).or_insert(0) += 1;
            }
            current_s = 'r';
        } else {
            if instruction == '^' {
                let y = r_current_house.1 + 1;
                r_current_house = (r_current_house.0, y);
                *house_count.entry(r_current_house).or_insert(0) += 1;
            } else if instruction == 'v' {
                let y = r_current_house.1 - 1;
                r_current_house = (r_current_house.0, y);
                *house_count.entry(r_current_house).or_insert(0) += 1;
            } else if instruction == '>' {
                let x = r_current_house.0 + 1;
                r_current_house = (x, r_current_house.1);
                *house_count.entry(r_current_house).or_insert(0) += 1;
            } else if instruction == '<' {
                let x = r_current_house.0 - 1;
                r_current_house = (x, r_current_house.1);
                *house_count.entry(r_current_house).or_insert(0) += 1;
            }
            current_s = 's';
        }
    }
    Some(house_count.len())
}

advent_of_code::main!(3);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 3));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 3));
        assert_eq!(result, Some(3));
    }
}
