pub fn part_one(input: &str) -> Option<u32> {
    let mut total_wp: u32 = 0;
    for present in input.split("\n") {
        if present.len() == 0 { continue };
        let mut dimensions: [u32; 3] = [0; 3] ;
        let mut side: usize = 0;
        // println!("{}", present);
        for dim in present.split("x") {
            // println!("{}", dim);
            let dim_c = dim.clone().trim();
            dimensions[side] = dim_c.parse::<u32>().unwrap();
            side += 1;
            // println!("{:?}", dimensions)
        };
        dimensions.sort();
        total_wp += 2*dimensions[0]*dimensions[1]+2*dimensions[1]*dimensions[2]+2*dimensions[0]*dimensions[2]+dimensions[0]*dimensions[1]

    }
    Some(total_wp)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total_r: u32 = 0;
    for present in input.split("\n") {
        if present.len() == 0 { continue };
        let mut dimensions: [u32; 3] = [0; 3] ;
        let mut side: usize = 0;
        // println!("{}", present);
        for dim in present.split("x") {
            // println!("{}", dim);
            let dim_c = dim.clone().trim();
            dimensions[side] = dim_c.parse::<u32>().unwrap();
            side += 1;
            // println!("{:?}", dimensions)
        };
        dimensions.sort();
        total_r += 2*(dimensions[0]+dimensions[1])+dimensions[0]*dimensions[1]*dimensions[2]

    }
    Some(total_r)
}

advent_of_code::main!(2);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 2));
        assert_eq!(result, Some(101));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 2));
        assert_eq!(result, Some(48));
    }
}
