use md5;

pub fn part_one(input: &str) -> Option<u32> {
    let mut md5_calc = String::from("123456789");
    let mut salt:u32 = 0;
    while &md5_calc[0..5] != "00000" {
        let key = format!("{}{}", input.trim(), salt);
        md5_calc = format!("{:?}", md5::compute(key));
        salt += 1
    }
    Some(salt - 1)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut md5_calc = String::from("123456789");
    let mut salt:u32 = 0;
    while &md5_calc[0..6] != "000000" {
        let key = format!("{}{}", input.trim(), salt);
        md5_calc = format!("{:?}", md5::compute(key));
        salt += 1
    }
    Some(salt - 1)
}

advent_of_code::main!(4);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 4));
        assert_eq!(result, Some(1048970));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 4));
        assert_eq!(result, Some(5714438));
    }
}
